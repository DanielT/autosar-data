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
    A2LDisplayText                                                         = 3860,
    /// AAF-AES-3-DATA-TYPE
    AafAes3DataType                                                        = 3747,
    /// AAF-FORMAT
    AafFormat                                                              = 5627,
    /// AAF-NOMINAL-RATE
    AafNominalRate                                                         = 2961,
    /// ABBR-NAME
    AbbrName                                                               = 3083,
    /// ABS
    Abs                                                                    = 559,
    /// ABSOLUTE
    Absolute                                                               = 2394,
    /// ABSOLUTE-TOLERANCE
    AbsoluteTolerance                                                      = 4865,
    /// ABSOLUTELY-SCHEDULED-TIMINGS
    AbsolutelyScheduledTimings                                             = 3978,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 3517,
    /// ACCEPT-ENCODING
    AcceptEncoding                                                         = 1217,
    /// ACCEPT-VARIABLE-DA
    AcceptVariableDa                                                       = 6008,
    /// ACCEPT-VARIABLE-SA
    AcceptVariableSa                                                       = 932,
    /// ACCEPTANCE-FIELD
    AcceptanceField                                                        = 23,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF
    AcceptedCryptoCipherSuiteWithPskRef                                    = 385,
    /// ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS
    AcceptedCryptoCipherSuiteWithPskRefs                                   = 6507,
    /// ACCEPTED-REMOTE-CERTIFICATE-REF
    AcceptedRemoteCertificateRef                                           = 3908,
    /// ACCEPTED-REMOTE-CERTIFICATE-REFS
    AcceptedRemoteCertificateRefs                                          = 6563,
    /// ACCEPTED-STARTUP-RANGE
    AcceptedStartupRange                                                   = 1902,
    /// ACCEPTS-ENCODINGS
    AcceptsEncodings                                                       = 5261,
    /// ACCESS-CONTROL
    AccessControl                                                          = 204,
    /// ACCESS-COUNT
    AccessCount                                                            = 3834,
    /// ACCESS-COUNT-SET
    AccessCountSet                                                         = 2489,
    /// ACCESS-COUNT-SETS
    AccessCountSets                                                        = 231,
    /// ACCESS-COUNTS
    AccessCounts                                                           = 4282,
    /// ACCESS-DATA-RECORD-SIZE
    AccessDataRecordSize                                                   = 3909,
    /// ACCESS-MODE
    AccessMode                                                             = 1516,
    /// ACCESS-PERMISSION-REF
    AccessPermissionRef                                                    = 4919,
    /// ACCESS-PERMISSION-VALIDITY
    AccessPermissionValidity                                               = 591,
    /// ACCESS-POINT-REF
    AccessPointRef                                                         = 6175,
    /// ACCESSED-DATA-PROTOTYPE-IREF
    AccessedDataPrototypeIref                                              = 3488,
    /// ACCESSED-MODE-GROUPS
    AccessedModeGroups                                                     = 4474,
    /// ACCESSED-PARAMETER
    AccessedParameter                                                      = 4752,
    /// ACCESSED-VARIABLE
    AccessedVariable                                                       = 4868,
    /// ACCESSED-VARIABLE-REF
    AccessedVariableRef                                                    = 3043,
    /// ACCURACY
    Accuracy                                                               = 5388,
    /// ACCURACY-EXT
    AccuracyExt                                                            = 4170,
    /// ACCURACY-INT
    AccuracyInt                                                            = 5911,
    /// ACF-BUS-PART-REF
    AcfBusPartRef                                                          = 3140,
    /// ACF-BUS-PART-REFS
    AcfBusPartRefs                                                         = 5377,
    /// ACF-BUS-REF
    AcfBusRef                                                              = 3567,
    /// ACF-BUS-REFS
    AcfBusRefs                                                             = 6090,
    /// ACF-MAX-TRANSIT-TIME
    AcfMaxTransitTime                                                      = 5733,
    /// ACF-PARTS
    AcfParts                                                               = 1180,
    /// ACF-TRANSPORTED-BUSS
    AcfTransportedBuss                                                     = 6340,
    /// ACK-REQUEST
    AckRequest                                                             = 4312,
    /// ACK-TYPE
    AckType                                                                = 101,
    /// ACL-CONTEXT
    AclContext                                                             = 2708,
    /// ACL-CONTEXTS
    AclContexts                                                            = 5516,
    /// ACL-OBJECT-CLASS
    AclObjectClass                                                         = 1193,
    /// ACL-OBJECT-CLASSS
    AclObjectClasss                                                        = 1553,
    /// ACL-OBJECT-REF
    AclObjectRef                                                           = 828,
    /// ACL-OBJECT-REFS
    AclObjectRefs                                                          = 146,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 1433,
    /// ACL-OPERATION
    AclOperation                                                           = 5299,
    /// ACL-OPERATION-REF
    AclOperationRef                                                        = 819,
    /// ACL-OPERATION-REFS
    AclOperationRefs                                                       = 3657,
    /// ACL-PERMISSION
    AclPermission                                                          = 296,
    /// ACL-ROLE
    AclRole                                                                = 3041,
    /// ACL-ROLE-REF
    AclRoleRef                                                             = 270,
    /// ACL-ROLE-REFS
    AclRoleRefs                                                            = 5039,
    /// ACL-SCOPE
    AclScope                                                               = 67,
    /// ACTION
    Action                                                                 = 2931,
    /// ACTION-ITEM-LISTS
    ActionItemLists                                                        = 1137,
    /// ACTION-ITEM-REF
    ActionItemRef                                                          = 509,
    /// ACTION-ITEMS
    ActionItems                                                            = 2446,
    /// ACTION-LIST
    ActionList                                                             = 5249,
    /// ACTION-LIST-EXECUTION
    ActionListExecution                                                    = 3411,
    /// ACTION-LIST-ITEM-REF
    ActionListItemRef                                                      = 6387,
    /// ACTION-LIST-ITEM-REFS
    ActionListItemRefs                                                     = 1348,
    /// ACTION-LIST-PROCESSING-FAILED-ERROR-REF
    ActionListProcessingFailedErrorRef                                     = 2374,
    /// ACTION-LISTS
    ActionLists                                                            = 2516,
    /// ACTION-POINT-OFFSET
    ActionPointOffset                                                      = 2422,
    /// ACTION-TYPE
    ActionType                                                             = 4864,
    /// ACTIONS
    Actions                                                                = 2686,
    /// ACTIVATE-AT-START
    ActivateAtStart                                                        = 4302,
    /// ACTIVATION
    Activation                                                             = 2096,
    /// ACTIVATION-ACTION
    ActivationAction                                                       = 1068,
    /// ACTIVATION-POINTS
    ActivationPoints                                                       = 4554,
    /// ACTIVATION-REASON-REPRESENTATION-REF
    ActivationReasonRepresentationRef                                      = 6037,
    /// ACTIVATION-REASONS
    ActivationReasons                                                      = 1407,
    /// ACTIVATION-SWITCH
    ActivationSwitch                                                       = 2135,
    /// ACTIVE-SUPERVISION-REF
    ActiveSupervisionRef                                                   = 4850,
    /// ACTIVE-SUPERVISION-REFS
    ActiveSupervisionRefs                                                  = 6130,
    /// ACTUAL-EVENT-REF
    ActualEventRef                                                         = 5443,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 3184,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 337,
    /// ADAPTIVE-FIREWALL-MODULE-INSTANTIATION
    AdaptiveFirewallModuleInstantiation                                    = 5212,
    /// ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING
    AdaptiveFirewallToPortPrototypeMapping                                 = 1607,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 4016,
    /// ADD-INFO-5
    AddInfo5                                                               = 2103,
    /// ADDITIONAL-INFORMATION
    AdditionalInformation                                                  = 4611,
    /// ADDITIONAL-NATIVE-TYPE-QUALIFIER
    AdditionalNativeTypeQualifier                                          = 2333,
    /// ADDRESS
    Address                                                                = 874,
    /// ADDRESS-CLAIM-ENABLED
    AddressClaimEnabled                                                    = 6209,
    /// ADDRESS-CONFIGURATION-CAPABILITY
    AddressConfigurationCapability                                         = 5872,
    /// ADDRESS-RANGE-LOWER-BOUND
    AddressRangeLowerBound                                                 = 5940,
    /// ADDRESS-RANGE-UPPER-BOUND
    AddressRangeUpperBound                                                 = 2326,
    /// ADDRESS-SEMANTICS
    AddressSemantics                                                       = 4973,
    /// ADDRESSING-FORMAT
    AddressingFormat                                                       = 1605,
    /// ADMIN-DATA
    AdminData                                                              = 5490,
    /// ADMIT-WITHOUT-MESSAGE-ID
    AdmitWithoutMessageId                                                  = 6527,
    /// AES-3-DATA-TYPE-H
    Aes3DataTypeH                                                          = 3013,
    /// AES-3-DATA-TYPE-L
    Aes3DataTypeL                                                          = 5376,
    /// AFFECTED-APPLICATION
    AffectedApplication                                                    = 1429,
    /// AFFECTED-BSW-MODULE
    AffectedBswModule                                                      = 2331,
    /// AFFECTED-BY-CONTROL-DTC-SETTING
    AffectedByControlDtcSetting                                            = 426,
    /// AFFECTED-FUNCTIONAL-CLUSTER
    AffectedFunctionalCluster                                              = 4645,
    /// AFFECTED-REF
    AffectedRef                                                            = 4743,
    /// AFFECTED-REFS
    AffectedRefs                                                           = 4093,
    /// AFFECTED-STATE-IREF
    AffectedStateIref                                                      = 1719,
    /// AFFECTION-KIND
    AffectionKind                                                          = 2105,
    /// AGE
    Age                                                                    = 3055,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 6475,
    /// AGGREGATION
    Aggregation                                                            = 840,
    /// AGGREGATION-CONDITION
    AggregationCondition                                                   = 363,
    /// AGGREGATION-REF
    AggregationRef                                                         = 4344,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 6309,
    /// AGING-ALLOWED
    AgingAllowed                                                           = 6207,
    /// AGING-CYCLES
    AgingCycles                                                            = 5434,
    /// AGING-REF
    AgingRef                                                               = 1241,
    /// AGING-REQUIRES-TESTED-CYCLE
    AgingRequiresTestedCycle                                               = 4029,
    /// AH-CIPHER-SUITE-NAME
    AhCipherSuiteName                                                      = 5535,
    /// AH-CIPHER-SUITE-NAMES
    AhCipherSuiteNames                                                     = 1996,
    /// ALGORITHM-DESCRIPTIONS
    AlgorithmDescriptions                                                  = 694,
    /// ALGORITHM-FAMILY
    AlgorithmFamily                                                        = 372,
    /// ALGORITHM-MODE
    AlgorithmMode                                                          = 1162,
    /// ALGORITHM-SECONDARY-FAMILY
    AlgorithmSecondaryFamily                                               = 116,
    /// ALIAS-EVENT-REF
    AliasEventRef                                                          = 6093,
    /// ALIAS-NAME-ASSIGNMENT
    AliasNameAssignment                                                    = 2663,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 4788,
    /// ALIAS-NAMES
    AliasNames                                                             = 370,
    /// ALIGNMENT
    Alignment                                                              = 71,
    /// ALIVE-CHECK-RESPONSE-TIMEOUT
    AliveCheckResponseTimeout                                              = 5055,
    /// ALIVE-REFERENCE-CYCLE
    AliveReferenceCycle                                                    = 269,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 6105,
    /// ALIVE-SUPERVISIONS
    AliveSupervisions                                                      = 3466,
    /// ALIVE-TIMEOUT
    AliveTimeout                                                           = 1528,
    /// ALL-CHANNELS-REF
    AllChannelsRef                                                         = 5946,
    /// ALL-CHANNELS-REFS
    AllChannelsRefs                                                        = 3152,
    /// ALL-NM-MESSAGES-KEEP-AWAKE
    AllNmMessagesKeepAwake                                                 = 3773,
    /// ALL-PHYSICAL-CHANNELS
    AllPhysicalChannels                                                    = 4206,
    /// ALL-PHYSICAL-CHANNELS-REF
    AllPhysicalChannelsRef                                                 = 3340,
    /// ALLOCATE-SHADOW-COPY
    AllocateShadowCopy                                                     = 6195,
    /// ALLOCATOR
    Allocator                                                              = 150,
    /// ALLOCATOR-REF
    AllocatorRef                                                           = 1426,
    /// ALLOW-CONTENT-TYPE-CHANGE
    AllowContentTypeChange                                                 = 1050,
    /// ALLOW-DYNAMIC-L-SDU-LENGTH
    AllowDynamicLSduLength                                                 = 2903,
    /// ALLOW-HALT-DUE-TO-CLOCK
    AllowHaltDueToClock                                                    = 193,
    /// ALLOW-MASTER-RATE-CORRECTION
    AllowMasterRateCorrection                                              = 5734,
    /// ALLOW-PASSIVE-TO-ACTIVE
    AllowPassiveToActive                                                   = 2377,
    /// ALLOW-PLAIN-TEXT-EXPORT
    AllowPlainTextExport                                                   = 5213,
    /// ALLOW-PLAIN-TEXT-IMPORT
    AllowPlainTextImport                                                   = 820,
    /// ALLOW-PROVIDER-RATE-CORRECTION
    AllowProviderRateCorrection                                            = 554,
    /// ALLOW-UNAUTHENTICATED-PARTICIPANTS
    AllowUnauthenticatedParticipants                                       = 6430,
    /// ALLOWED-I-PV-6-EXT-HEADER
    AllowedIPv6ExtHeader                                                   = 6295,
    /// ALLOWED-I-PV-6-EXT-HEADERS
    AllowedIPv6ExtHeaders                                                  = 1610,
    /// ALLOWED-I-PV-6-EXT-HEADERS-REF
    AllowedIPv6ExtHeadersRef                                               = 1722,
    /// ALLOWED-KEYSLOT-USAGE
    AllowedKeyslotUsage                                                    = 1350,
    /// ALLOWED-SERVICE-CONSUMERS
    AllowedServiceConsumers                                                = 2733,
    /// ALLOWED-SERVICE-PROVIDERS
    AllowedServiceProviders                                                = 4857,
    /// ALLOWED-TCP-OPTION
    AllowedTcpOption                                                       = 6265,
    /// ALLOWED-TCP-OPTIONS
    AllowedTcpOptions                                                      = 3385,
    /// ALLOWED-TCP-OPTIONS-REF
    AllowedTcpOptionsRef                                                   = 543,
    /// ALTERNATIVE-NAME
    AlternativeName                                                        = 5732,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 5067,
    /// ANNOTATION
    Annotation                                                             = 5115,
    /// ANNOTATION-ORIGIN
    AnnotationOrigin                                                       = 5487,
    /// ANNOTATION-TEXT
    AnnotationText                                                         = 6181,
    /// ANNOTATIONS
    Annotations                                                            = 4153,
    /// ANONYMOUS
    Anonymous                                                              = 4058,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 6531,
    /// AP-APPLICATION-ENDPOINTS
    ApApplicationEndpoints                                                 = 3203,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 735,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 5519,
    /// AP-APPLICATION-ERROR-REF
    ApApplicationErrorRef                                                  = 5379,
    /// AP-APPLICATION-ERROR-REFS
    ApApplicationErrorRefs                                                 = 688,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 2593,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 4930,
    /// API-PRINCIPLE
    ApiPrinciple                                                           = 2305,
    /// API-SERVICE-PREFIX
    ApiServicePrefix                                                       = 4467,
    /// APMC-BOOLEAN-PARAM-DEF
    ApmcBooleanParamDef                                                    = 2570,
    /// APMC-CHOICE-CONTAINER-DEF
    ApmcChoiceContainerDef                                                 = 4765,
    /// APMC-CHOICE-CONTAINER-REFERENCE-DEF
    ApmcChoiceContainerReferenceDef                                        = 5085,
    /// APMC-CONTAINER-REFERENCE-DEF
    ApmcContainerReferenceDef                                              = 4762,
    /// APMC-CONTAINER-REFERENCE-VALUE
    ApmcContainerReferenceValue                                            = 4066,
    /// APMC-CONTAINER-VALUE
    ApmcContainerValue                                                     = 4276,
    /// APMC-DEFINITION-COLLECTION
    ApmcDefinitionCollection                                               = 526,
    /// APMC-ENUMERATION-LITERAL-DEF
    ApmcEnumerationLiteralDef                                              = 4132,
    /// APMC-ENUMERATION-PARAM-DEF
    ApmcEnumerationParamDef                                                = 4101,
    /// APMC-FLOAT-PARAM-DEF
    ApmcFloatParamDef                                                      = 4137,
    /// APMC-FOREIGN-REFERENCE-DEF
    ApmcForeignReferenceDef                                                = 5805,
    /// APMC-FOREIGN-REFERENCE-VALUE
    ApmcForeignReferenceValue                                              = 5355,
    /// APMC-FUNCTIONAL-CLUSTER-DEF
    ApmcFunctionalClusterDef                                               = 1415,
    /// APMC-FUNCTIONAL-CLUSTER-VALUE
    ApmcFunctionalClusterValue                                             = 1579,
    /// APMC-INSTANCE-REFERENCE-DEF
    ApmcInstanceReferenceDef                                               = 4228,
    /// APMC-INSTANCE-REFERENCE-VALUE
    ApmcInstanceReferenceValue                                             = 1151,
    /// APMC-INTEGER-PARAM-DEF
    ApmcIntegerParamDef                                                    = 1804,
    /// APMC-IP-V4-ADDRESS-PARAM-DEF
    ApmcIpV4AddressParamDef                                                = 3768,
    /// APMC-IP-V4-ADDRESS-PARAM-VALUE
    ApmcIpV4AddressParamValue                                              = 2577,
    /// APMC-IP-V6-ADDRESS-PARAM-DEF
    ApmcIpV6AddressParamDef                                                = 512,
    /// APMC-IP-V6-ADDRESS-PARAM-VALUE
    ApmcIpV6AddressParamValue                                              = 1038,
    /// APMC-MAC-ADDRESS-PARAM-DEF
    ApmcMacAddressParamDef                                                 = 4514,
    /// APMC-MAC-ADDRESS-PARAM-VALUE
    ApmcMacAddressParamValue                                               = 2171,
    /// APMC-NUMERICAL-PARAM-VALUE
    ApmcNumericalParamValue                                                = 9,
    /// APMC-PARAM-CONF-CONTAINER-DEF
    ApmcParamConfContainerDef                                              = 4971,
    /// APMC-REVISION-LABEL-PARAM-DEF
    ApmcRevisionLabelParamDef                                              = 5668,
    /// APMC-REVISION-LABEL-PARAM-VALUE
    ApmcRevisionLabelParamValue                                            = 4526,
    /// APMC-STRING-PARAM-DEF
    ApmcStringParamDef                                                     = 1263,
    /// APMC-STRONG-REVISION-LABEL-PARAM-DEF
    ApmcStrongRevisionLabelParamDef                                        = 415,
    /// APMC-STRONG-REVISION-LABEL-PARAM-VALUE
    ApmcStrongRevisionLabelParamValue                                      = 2271,
    /// APMC-TEXTUAL-PARAM-VALUE
    ApmcTextualParamValue                                                  = 3688,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF
    ApmcUpstreamDocForeignReferenceDef                                     = 602,
    /// APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE
    ApmcUpstreamDocForeignReferenceValue                                   = 6549,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF
    ApmcUpstreamDocInstanceReferenceDef                                    = 5396,
    /// APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE
    ApmcUpstreamDocInstanceReferenceValue                                  = 2274,
    /// APMC-URI-FOREIGN-REFERENCE-DEF
    ApmcUriForeignReferenceDef                                             = 1673,
    /// APMC-URI-FOREIGN-REFERENCE-VALUE
    ApmcUriForeignReferenceValue                                           = 1106,
    /// APMC-URI-INSTANCE-REFERENCE-DEF
    ApmcUriInstanceReferenceDef                                            = 3998,
    /// APMC-URI-INSTANCE-REFERENCE-VALUE
    ApmcUriInstanceReferenceValue                                          = 4324,
    /// APMC-URI-PARAM-DEF
    ApmcUriParamDef                                                        = 5582,
    /// APMC-URI-PARAM-VALUE
    ApmcUriParamValue                                                      = 764,
    /// APMC-VALUE-COLLECTION
    ApmcValueCollection                                                    = 3149,
    /// APP-ID
    AppId                                                                  = 4175,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 5071,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS
    AppOsTaskProxyToEcuTaskProxyMappings                                   = 2711,
    /// APP-TASK-PROXY-REF
    AppTaskProxyRef                                                        = 2831,
    /// APPL-CONSTANT-REF
    ApplConstantRef                                                        = 4603,
    /// APPL-INIT-VALUE
    ApplInitValue                                                          = 5718,
    /// APPL-WATCHDOG-LIMIT
    ApplWatchdogLimit                                                      = 3079,
    /// APPLICABILITY-INFO
    ApplicabilityInfo                                                      = 3435,
    /// APPLICABILITY-INFO-SET
    ApplicabilityInfoSet                                                   = 2124,
    /// APPLICABILITY-INFOS
    ApplicabilityInfos                                                     = 2089,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 722,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 5246,
    /// APPLICATION-ARRAY-ELEMENT-REF
    ApplicationArrayElementRef                                             = 5025,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 5914,
    /// APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION
    ApplicationAssocMapElementValueSpecification                           = 498,
    /// APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION
    ApplicationAssocMapValueSpecification                                  = 307,
    /// APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF
    ApplicationCompositeDataTypeSubElementRef                              = 2565,
    /// APPLICATION-COMPOSITE-ELEMENT-IREF
    ApplicationCompositeElementIref                                        = 3265,
    /// APPLICATION-CONTEXT-REF
    ApplicationContextRef                                                  = 4840,
    /// APPLICATION-CONTEXT-REFS
    ApplicationContextRefs                                                 = 2895,
    /// APPLICATION-DATA-TYPE-REF
    ApplicationDataTypeRef                                                 = 3392,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 3420,
    /// APPLICATION-DESCRIPTION
    ApplicationDescription                                                 = 3199,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 2199,
    /// APPLICATION-ENDPOINT-REF
    ApplicationEndpointRef                                                 = 4169,
    /// APPLICATION-ENDPOINT-REF-CONDITIONAL
    ApplicationEndpointRefConditional                                      = 5126,
    /// APPLICATION-ENTRY
    ApplicationEntry                                                       = 2490,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 4049,
    /// APPLICATION-ID
    ApplicationId                                                          = 2643,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 4905,
    /// APPLICATION-MODE-IREF
    ApplicationModeIref                                                    = 1290,
    /// APPLICATION-MODE-MACHINE
    ApplicationModeMachine                                                 = 556,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 1959,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 161,
    /// APPLICATION-PARTITION-REF
    ApplicationPartitionRef                                                = 236,
    /// APPLICATION-PARTITION-REFS
    ApplicationPartitionRefs                                               = 3209,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 6535,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS
    ApplicationPartitionToEcuPartitionMappings                             = 4609,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 1496,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 5994,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 1126,
    /// APPLICATION-RECORD-ELEMENT-REF
    ApplicationRecordElementRef                                            = 6186,
    /// APPLICATION-RULE-BASED-VALUE-SPECIFICATION
    ApplicationRuleBasedValueSpecification                                 = 5572,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 1656,
    /// APPLICATION-TYPE-REF
    ApplicationTypeRef                                                     = 105,
    /// APPLICATION-TYPE-TREF
    ApplicationTypeTref                                                    = 2217,
    /// APPLICATION-VALUE-SPECIFICATION
    ApplicationValueSpecification                                          = 2117,
    /// APPLICATIONS
    Applications                                                           = 2972,
    /// APPLIED-STANDARD
    AppliedStandard                                                        = 2400,
    /// APPLIED-STANDARDS
    AppliedStandards                                                       = 1157,
    /// APPLIES-TO
    AppliesTo                                                              = 1425,
    /// APPLIES-TO-DEPENDENCIES
    AppliesToDependencies                                                  = 910,
    /// APPROVAL-REQUIRED
    ApprovalRequired                                                       = 4724,
    /// APPROVAL-STATUS
    ApprovalStatus                                                         = 4265,
    /// AR-PACKAGE
    ArPackage                                                              = 2626,
    /// AR-PACKAGES
    ArPackages                                                             = 4415,
    /// AR-PARAMETER
    ArParameter                                                            = 5366,
    /// AR-RELEASE-VERSION
    ArReleaseVersion                                                       = 4493,
    /// AR-TYPED-PER-INSTANCE-MEMORY-REF
    ArTypedPerInstanceMemoryRef                                            = 1353,
    /// AR-TYPED-PER-INSTANCE-MEMORYS
    ArTypedPerInstanceMemorys                                              = 837,
    /// ARBITRARY-ADDRESS-CAPABLE
    ArbitraryAddressCapable                                                = 5072,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 876,
    /// ARBITRATION
    Arbitration                                                            = 6092,
    /// ARBITRATIONS
    Arbitrations                                                           = 1625,
    /// AREA
    Area                                                                   = 3072,
    /// ARGUMENT
    Argument                                                               = 2301,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 4810,
    /// ARGUMENT-MAPPINGS
    ArgumentMappings                                                       = 4431,
    /// ARGUMENT-REF
    ArgumentRef                                                            = 6611,
    /// ARGUMENTS
    Arguments                                                              = 4889,
    /// ARGUMENTSS
    Argumentss                                                             = 871,
    /// ARP-PROPS
    ArpProps                                                               = 3535,
    /// ARRAY-ELEMENT-MAPPINGS
    ArrayElementMappings                                                   = 2040,
    /// ARRAY-ELEMENT-REF
    ArrayElementRef                                                        = 5892,
    /// ARRAY-IMPL-POLICY
    ArrayImplPolicy                                                        = 5971,
    /// ARRAY-SIZE
    ArraySize                                                              = 4682,
    /// ARRAY-SIZE-HANDLING
    ArraySizeHandling                                                      = 2034,
    /// ARRAY-SIZE-SEMANTICS
    ArraySizeSemantics                                                     = 921,
    /// ARRAY-VALUE-SPECIFICATION
    ArrayValueSpecification                                                = 4906,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 2285,
    /// ARTIFACT-CHECKSUM-REF
    ArtifactChecksumRef                                                    = 4073,
    /// ARTIFACT-CHECKSUM-REFS
    ArtifactChecksumRefs                                                   = 4933,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 1408,
    /// ARTIFACT-CHECKSUMS
    ArtifactChecksums                                                      = 3585,
    /// ARTIFACT-DESCRIPTOR
    ArtifactDescriptor                                                     = 6412,
    /// ARTIFACT-DESCRIPTORS
    ArtifactDescriptors                                                    = 2144,
    /// ARTIFACT-LOCATOR
    ArtifactLocator                                                        = 2993,
    /// ARTIFACT-LOCATORS
    ArtifactLocators                                                       = 5749,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 3780,
    /// ASSERTIONS
    Assertions                                                             = 6089,
    /// ASSIGN-FRAME-ID
    AssignFrameId                                                          = 4490,
    /// ASSIGN-FRAME-ID-RANGE
    AssignFrameIdRange                                                     = 195,
    /// ASSIGN-NAD
    AssignNad                                                              = 2662,
    /// ASSIGNED-CONTROLLER-REF
    AssignedControllerRef                                                  = 5378,
    /// ASSIGNED-DATA-TYPES
    AssignedDataTypes                                                      = 1247,
    /// ASSIGNED-DATAS
    AssignedDatas                                                          = 5545,
    /// ASSIGNED-ENTRY-REF
    AssignedEntryRef                                                       = 2550,
    /// ASSIGNED-ENTRY-ROLES
    AssignedEntryRoles                                                     = 1746,
    /// ASSIGNED-FRAME-TRIGGERING-REF
    AssignedFrameTriggeringRef                                             = 4336,
    /// ASSIGNED-LIN-SLAVE-CONFIG-REF
    AssignedLinSlaveConfigRef                                              = 2823,
    /// ASSIGNED-PORTS
    AssignedPorts                                                          = 3640,
    /// ASSIGNED-TRAFFIC-CLASS
    AssignedTrafficClass                                                   = 947,
    /// ASSIGNED-TRAFFIC-CLASSS
    AssignedTrafficClasss                                                  = 4952,
    /// ASSIGNMENT-PRIORITY
    AssignmentPriority                                                     = 6033,
    /// ASSOCIATED-COM-I-PDU-GROUP-REF
    AssociatedComIPduGroupRef                                              = 1174,
    /// ASSOCIATED-COM-I-PDU-GROUP-REFS
    AssociatedComIPduGroupRefs                                             = 1515,
    /// ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    AssociatedConsumedProvidedServiceInstanceGroups                        = 3975,
    /// ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF
    AssociatedCrossSwClusterComRtePluginRef                                = 5918,
    /// ASSOCIATED-EVENT-IDENTIFICATION
    AssociatedEventIdentification                                          = 6528,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REF
    AssociatedPdurIPduGroupRef                                             = 1343,
    /// ASSOCIATED-PDUR-I-PDU-GROUP-REFS
    AssociatedPdurIPduGroupRefs                                            = 6576,
    /// ASSOCIATED-RTE-PLUGIN-REF
    AssociatedRtePluginRef                                                 = 1270,
    /// ASSUMED-CURRENT-STATE-IREF
    AssumedCurrentStateIref                                                = 273,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 1548,
    /// ASYNCHRONOUS-SERVER-CALL-POINT-REF
    AsynchronousServerCallPointRef                                         = 4293,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 4403,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS
    AsynchronousServerCallResultPoints                                     = 920,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 1488,
    /// ASYNCHRONOUS-TRAFFIC-SHAPER-REF
    AsynchronousTrafficShaperRef                                           = 4929,
    /// ATS-INSTANCE-ENTRY-REF
    AtsInstanceEntryRef                                                    = 2960,
    /// ATS-INSTANCE-ENTRYS
    AtsInstanceEntrys                                                      = 3353,
    /// ATTRIBUTE-DEFS
    AttributeDefs                                                          = 456,
    /// ATTRIBUTE-NAME
    AttributeName                                                          = 6031,
    /// ATTRIBUTE-REF
    AttributeRef                                                           = 5263,
    /// ATTRIBUTE-TAILORINGS
    AttributeTailorings                                                    = 5801,
    /// ATTRIBUTE-VALUES
    AttributeValues                                                        = 5292,
    /// ATTRIBUTES
    Attributes                                                             = 6584,
    /// AUDIENCE
    Audience                                                               = 2364,
    /// AUDIENCES
    Audiences                                                              = 5811,
    /// AUTH-ALGORITHM
    AuthAlgorithm                                                          = 1268,
    /// AUTH-DATA-FRESHNESS-LENGTH
    AuthDataFreshnessLength                                                = 3570,
    /// AUTH-DATA-FRESHNESS-START-POSITION
    AuthDataFreshnessStartPosition                                         = 2529,
    /// AUTH-INFO-TX-LENGTH
    AuthInfoTxLength                                                       = 2950,
    /// AUTH-ROLE-REF
    AuthRoleRef                                                            = 1361,
    /// AUTH-ROLE-REFS
    AuthRoleRefs                                                           = 3715,
    /// AUTHENTIC-CONNECTION-PROPSS
    AuthenticConnectionPropss                                              = 5451,
    /// AUTHENTICATION-BUILD-ATTEMPTS
    AuthenticationBuildAttempts                                            = 2337,
    /// AUTHENTICATION-CLASS-REF
    AuthenticationClassRef                                                 = 6529,
    /// AUTHENTICATION-ENABLED
    AuthenticationEnabled                                                  = 390,
    /// AUTHENTICATION-KEY-REF
    AuthenticationKeyRef                                                   = 2387,
    /// AUTHENTICATION-PROPS-REF
    AuthenticationPropsRef                                                 = 1685,
    /// AUTHENTICATION-PROPSS
    AuthenticationPropss                                                   = 4512,
    /// AUTHENTICATION-REF
    AuthenticationRef                                                      = 6154,
    /// AUTHENTICATION-RETRIES
    AuthenticationRetries                                                  = 5294,
    /// AUTHENTICATION-ROLE-REF
    AuthenticationRoleRef                                                  = 5531,
    /// AUTHENTICATION-ROLE-REFS
    AuthenticationRoleRefs                                                 = 2876,
    /// AUTHENTICATION-TIMEOUT
    AuthenticationTimeout                                                  = 975,
    /// AUTHENTICATION-VERIFY-ATTEMPTS
    AuthenticationVerifyAttempts                                           = 4507,
    /// AUTO-AVAILABLE
    AutoAvailable                                                          = 3423,
    /// AUTO-COLLECT
    AutoCollect                                                            = 4692,
    /// AUTO-IP-PROPS
    AutoIpProps                                                            = 6176,
    /// AUTO-REQUIRE
    AutoRequire                                                            = 1301,
    /// AUTO-START
    AutoStart                                                              = 624,
    /// AUTOMATIC-END
    AutomaticEnd                                                           = 3027,
    /// AUTOSAR
    Autosar                                                                = 2820,
    /// AUTOSAR-CONNECTOR
    AutosarConnector                                                       = 6403,
    /// AUTOSAR-DATA-TYPE-REF
    AutosarDataTypeRef                                                     = 6099,
    /// AUTOSAR-DATA-TYPE-REF-CONDITIONAL
    AutosarDataTypeRefConditional                                          = 4245,
    /// AUTOSAR-ENGINEERING-OBJECT
    AutosarEngineeringObject                                               = 1248,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 94,
    /// AUTOSAR-PARAMETER-IREF
    AutosarParameterIref                                                   = 2511,
    /// AUTOSAR-VARIABLE
    AutosarVariable                                                        = 633,
    /// AUTOSAR-VARIABLE-IN-IMPL-DATATYPE
    AutosarVariableInImplDatatype                                          = 2408,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 5596,
    /// AUTOSAR-VARIABLE-IREF
    AutosarVariableIref                                                    = 4517,
    /// AUXILIARY-FIELD-DEFINITIONS
    AuxiliaryFieldDefinitions                                              = 3090,
    /// AUXILIARY-FIELDS
    AuxiliaryFields                                                        = 4203,
    /// AVERAGE-MEMORY-CONSUMPTION
    AverageMemoryConsumption                                               = 5907,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 1758,
    /// BAM-TIMING
    BamTiming                                                              = 4330,
    /// BANDWIDTH-LIMITATION
    BandwidthLimitation                                                    = 5979,
    /// BASE-COMPOSITION-REF
    BaseCompositionRef                                                     = 1332,
    /// BASE-CYCLE
    BaseCycle                                                              = 5641,
    /// BASE-FREQUENCY
    BaseFrequency                                                          = 4454,
    /// BASE-IS-THIS-PACKAGE
    BaseIsThisPackage                                                      = 2078,
    /// BASE-REF
    BaseRef                                                                = 331,
    /// BASE-REFERENCE-REF
    BaseReferenceRef                                                       = 5454,
    /// BASE-TYPE-ENCODING
    BaseTypeEncoding                                                       = 801,
    /// BASE-TYPE-REF
    BaseTypeRef                                                            = 3726,
    /// BASE-TYPE-SIZE
    BaseTypeSize                                                           = 374,
    /// BASIC-CYCLE-LENGTH
    BasicCycleLength                                                       = 5090,
    /// BAUDRATE
    Baudrate                                                               = 5288,
    /// BEHAVIOR
    Behavior                                                               = 3425,
    /// BEHAVIOR-REF
    BehaviorRef                                                            = 6587,
    /// BEST-CASE-EXECUTION-TIME
    BestCaseExecutionTime                                                  = 913,
    /// BGCOLOR
    Bgcolor                                                                = 3182,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 768,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 611,
    /// BINARY-MANIFEST-ITEM-NUMERICAL-VALUE
    BinaryManifestItemNumericalValue                                       = 2391,
    /// BINARY-MANIFEST-ITEM-POINTER-VALUE
    BinaryManifestItemPointerValue                                         = 5567,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 5860,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 3818,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 6471,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 1699,
    /// BIT
    Bit                                                                    = 2709,
    /// BIT-NUMBER
    BitNumber                                                              = 3270,
    /// BIT-OFFSET
    BitOffset                                                              = 2012,
    /// BIT-POSITION
    BitPosition                                                            = 4434,
    /// BITFIELD-TEXT-TABLE-MASK-FIRST
    BitfieldTextTableMaskFirst                                             = 266,
    /// BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR
    BitfieldTextTableMaskNvBlockDescriptor                                 = 3591,
    /// BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE
    BitfieldTextTableMaskPortPrototype                                     = 368,
    /// BITFIELD-TEXT-TABLE-MASK-SECOND
    BitfieldTextTableMaskSecond                                            = 3112,
    /// BLACKLISTED-VERSIONS
    BlacklistedVersions                                                    = 2614,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREF
    BlockIfStateActiveApIref                                               = 6121,
    /// BLOCK-IF-STATE-ACTIVE-AP-IREFS
    BlockIfStateActiveApIrefs                                              = 3059,
    /// BLOCK-IF-STATE-ACTIVE-CP-REF
    BlockIfStateActiveCpRef                                                = 2442,
    /// BLOCK-IF-STATE-ACTIVE-CP-REFS
    BlockIfStateActiveCpRefs                                               = 4362,
    /// BLOCK-INCONSISTENT
    BlockInconsistent                                                      = 1780,
    /// BLOCK-STATE
    BlockState                                                             = 4585,
    /// BLOCK-STATES
    BlockStates                                                            = 5770,
    /// BLOCKLISTED-VERSIONS
    BlocklistedVersions                                                    = 5630,
    /// BLUEPRINT-CONDITION
    BlueprintCondition                                                     = 3607,
    /// BLUEPRINT-DERIVATION-GUIDE
    BlueprintDerivationGuide                                               = 2441,
    /// BLUEPRINT-MAPPING
    BlueprintMapping                                                       = 1821,
    /// BLUEPRINT-MAPPING-GUIDE
    BlueprintMappingGuide                                                  = 815,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 1345,
    /// BLUEPRINT-MAPS
    BlueprintMaps                                                          = 4364,
    /// BLUEPRINT-POLICY-LIST
    BlueprintPolicyList                                                    = 2293,
    /// BLUEPRINT-POLICY-NOT-MODIFIABLE
    BlueprintPolicyNotModifiable                                           = 4116,
    /// BLUEPRINT-POLICY-SINGLE
    BlueprintPolicySingle                                                  = 458,
    /// BLUEPRINT-POLICYS
    BlueprintPolicys                                                       = 1251,
    /// BLUEPRINT-REF
    BlueprintRef                                                           = 2164,
    /// BOOLEAN-VALUE-VARIATION-POINT
    BooleanValueVariationPoint                                             = 5862,
    /// BOR-COUNTER-L-1-TO-L-2
    BorCounterL1ToL2                                                       = 2587,
    /// BOR-TIME-L-1
    BorTimeL1                                                              = 6307,
    /// BOR-TIME-L-2
    BorTimeL2                                                              = 3635,
    /// BOR-TIME-TX-ENSURED
    BorTimeTxEnsured                                                       = 1497,
    /// BR
    Br                                                                     = 2049,
    /// BROADCAST
    Broadcast                                                              = 2007,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 4382,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 1794,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 4172,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 2830,
    /// BSW-BEHAVIOR-REF
    BswBehaviorRef                                                         = 1672,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 941,
    /// BSW-CLIENT-POLICY
    BswClientPolicy                                                        = 4996,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 5773,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 1195,
    /// BSW-DATA-SEND-POLICY
    BswDataSendPolicy                                                      = 6468,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 4444,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 883,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 4247,
    /// BSW-ENTITY-REF
    BswEntityRef                                                           = 2267,
    /// BSW-ENTRY-KIND
    BswEntryKind                                                           = 5776,
    /// BSW-ENTRY-RELATIONSHIP
    BswEntryRelationship                                                   = 6097,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 654,
    /// BSW-ENTRY-RELATIONSHIP-TYPE
    BswEntryRelationshipType                                               = 4149,
    /// BSW-ENTRY-RELATIONSHIPS
    BswEntryRelationships                                                  = 3242,
    /// BSW-EXCLUSIVE-AREA-POLICY
    BswExclusiveAreaPolicy                                                 = 1552,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 2675,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 3025,
    /// BSW-IMPLEMENTATION-REF
    BswImplementationRef                                                   = 724,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 2133,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 4209,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 438,
    /// BSW-INTERNAL-TRIGGERING-POINT-POLICY
    BswInternalTriggeringPointPolicy                                       = 6496,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF
    BswInternalTriggeringPointRef                                          = 4285,
    /// BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL
    BswInternalTriggeringPointRefConditional                               = 2578,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 4534,
    /// BSW-INTERRUPT-EVENT
    BswInterruptEvent                                                      = 3382,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 1027,
    /// BSW-MODE-GROUP-REF
    BswModeGroupRef                                                        = 4553,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 2784,
    /// BSW-MODE-RECEIVER-POLICY
    BswModeReceiverPolicy                                                  = 4368,
    /// BSW-MODE-SENDER-POLICY
    BswModeSenderPolicy                                                    = 1259,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 2273,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 3051,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 3446,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 563,
    /// BSW-MODULE-DEPENDENCYS
    BswModuleDependencys                                                   = 1084,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 6432,
    /// BSW-MODULE-DESCRIPTION-REF
    BswModuleDescriptionRef                                                = 4641,
    /// BSW-MODULE-DESCRIPTION-REF-CONDITIONAL
    BswModuleDescriptionRefConditional                                     = 6073,
    /// BSW-MODULE-DOCUMENTATIONS
    BswModuleDocumentations                                                = 834,
    /// BSW-MODULE-ENTITY-REF
    BswModuleEntityRef                                                     = 3930,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 4393,
    /// BSW-MODULE-ENTRY-REF
    BswModuleEntryRef                                                      = 1087,
    /// BSW-MODULE-ENTRY-REF-CONDITIONAL
    BswModuleEntryRefConditional                                           = 3117,
    /// BSW-MODULE-INSTANCE-REF
    BswModuleInstanceRef                                                   = 5859,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 6233,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 5403,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 3272,
    /// BSW-PARAMETER-POLICY
    BswParameterPolicy                                                     = 4078,
    /// BSW-PER-INSTANCE-MEMORY-POLICY
    BswPerInstanceMemoryPolicy                                             = 1017,
    /// BSW-PER-INSTANCE-MEMORY-POLICYS
    BswPerInstanceMemoryPolicys                                            = 2729,
    /// BSW-QUEUED-DATA-RECEPTION-POLICY
    BswQueuedDataReceptionPolicy                                           = 1636,
    /// BSW-RELEASED-TRIGGER-POLICY
    BswReleasedTriggerPolicy                                               = 4658,
    /// BSW-RESOLUTION
    BswResolution                                                          = 3561,
    /// BSW-RESOURCE-ESTIMATION
    BswResourceEstimation                                                  = 890,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 410,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 6324,
    /// BSW-SERVICE-DEPENDENCY
    BswServiceDependency                                                   = 1755,
    /// BSW-SERVICE-DEPENDENCY-REF
    BswServiceDependencyRef                                                = 742,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 2443,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 6056,
    /// BSW-TRIGGER-DIRECT-IMPLEMENTATION
    BswTriggerDirectImplementation                                         = 4844,
    /// BSW-TRIGGER-REF
    BswTriggerRef                                                          = 1594,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 4962,
    /// BUCKET-SIZE
    BucketSize                                                             = 5152,
    /// BUFFER-COMPUTATION
    BufferComputation                                                      = 4229,
    /// BUFFER-OUTPUT
    BufferOutput                                                           = 2174,
    /// BUFFER-PROPERTIES
    BufferProperties                                                       = 256,
    /// BUFFER-RATIO
    BufferRatio                                                            = 3481,
    /// BUILD-ACTION
    BuildAction                                                            = 5307,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 5289,
    /// BUILD-ACTION-ENVIRONMENTS
    BuildActionEnvironments                                                = 1606,
    /// BUILD-ACTION-IO-ELEMENT
    BuildActionIoElement                                                   = 2291,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 5759,
    /// BUILD-ACTION-MANIFEST-REF
    BuildActionManifestRef                                                 = 635,
    /// BUILD-ACTION-MANIFEST-REF-CONDITIONAL
    BuildActionManifestRefConditional                                      = 6548,
    /// BUILD-ACTION-MANIFESTS
    BuildActionManifests                                                   = 2356,
    /// BUILD-ACTIONS
    BuildActions                                                           = 294,
    /// BUILD-TYPE
    BuildType                                                              = 3941,
    /// BULK-DATA-CATEGORY
    BulkDataCategory                                                       = 4503,
    /// BULK-NV-BLOCK
    BulkNvBlock                                                            = 5625,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 5611,
    /// BULK-NV-DATA-DESCRIPTORS
    BulkNvDataDescriptors                                                  = 1663,
    /// BUNDLED-CONNECTIONS
    BundledConnections                                                     = 5695,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 6485,
    /// BURST-SIZE
    BurstSize                                                              = 5150,
    /// BURST-SIZE-REQUEST
    BurstSizeRequest                                                       = 5385,
    /// BURST-SIZE-RESPONSE
    BurstSizeResponse                                                      = 5700,
    /// BUS-DEPENDENT-NM-ECUS
    BusDependentNmEcus                                                     = 4884,
    /// BUS-ID
    BusId                                                                  = 3934,
    /// BUS-IDLE-TIMEOUT-PERIOD
    BusIdleTimeoutPeriod                                                   = 1077,
    /// BUS-MIRROR-CAN-ID-RANGE-MAPPING
    BusMirrorCanIdRangeMapping                                             = 1514,
    /// BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING
    BusMirrorCanIdToCanIdMapping                                           = 3529,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 5845,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 3703,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 4899,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 719,
    /// BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING
    BusMirrorLinPidToCanIdMapping                                          = 3610,
    /// BUS-MIRROR-NETWORK-ID
    BusMirrorNetworkId                                                     = 5364,
    /// BUS-OFF-RECOVERY
    BusOffRecovery                                                         = 5162,
    /// BUS-SPECIFIC-NM-ECU
    BusSpecificNmEcu                                                       = 5831,
    /// BY-PASS-POINT-IREF
    ByPassPointIref                                                        = 5492,
    /// BY-PASS-POINT-IREFS
    ByPassPointIrefs                                                       = 5132,
    /// BYPASS-ETHER-TYPE
    BypassEtherType                                                        = 6001,
    /// BYPASS-ETHER-TYPES
    BypassEtherTypes                                                       = 6122,
    /// BYPASS-VLAN
    BypassVlan                                                             = 5266,
    /// BYPASS-VLANS
    BypassVlans                                                            = 6143,
    /// BYTE
    Byte                                                                   = 3303,
    /// BYTE-ORDER
    ByteOrder                                                              = 1977,
    /// BYTE-VALUE
    ByteValue                                                              = 1525,
    /// BYTE-VALUES
    ByteValues                                                             = 1057,
    /// C-CODE
    CCode                                                                  = 578,
    /// CAK-REF
    CakRef                                                                 = 178,
    /// CALC-RAM-BLOCK-CRC
    CalcRamBlockCrc                                                        = 6132,
    /// CALCULATION-FORMULA
    CalculationFormula                                                     = 1876,
    /// CALIBRATION-PARAMETER-VALUE
    CalibrationParameterValue                                              = 879,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 5553,
    /// CALIBRATION-PARAMETER-VALUE-SET-REF
    CalibrationParameterValueSetRef                                        = 5856,
    /// CALIBRATION-PARAMETER-VALUE-SET-REFS
    CalibrationParameterValueSetRefs                                       = 962,
    /// CALIBRATION-PARAMETER-VALUES
    CalibrationParameterValues                                             = 1286,
    /// CALL-POINTS
    CallPoints                                                             = 1448,
    /// CALL-SIGNAL-REF
    CallSignalRef                                                          = 3877,
    /// CALL-SIGNAL-TRIGGERING-REF
    CallSignalTriggeringRef                                                = 5915,
    /// CALL-TIMEOUT
    CallTimeout                                                            = 3190,
    /// CALL-TYPE
    CallType                                                               = 4019,
    /// CALLBACK-HEADER-REF
    CallbackHeaderRef                                                      = 278,
    /// CALLBACK-HEADER-REFS
    CallbackHeaderRefs                                                     = 1042,
    /// CALLED-ENTRY-REF
    CalledEntryRef                                                         = 5826,
    /// CALLED-ENTRYS
    CalledEntrys                                                           = 3104,
    /// CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF
    CalledFromWithinExclusiveAreaRef                                       = 5293,
    /// CAN-ADDRESSING-MODE
    CanAddressingMode                                                      = 3493,
    /// CAN-BAUDRATE
    CanBaudrate                                                            = 3862,
    /// CAN-BE-INVOKED-CONCURRENTLY
    CanBeInvokedConcurrently                                               = 1715,
    /// CAN-BIT-RATE-SWITCH
    CanBitRateSwitch                                                       = 5185,
    /// CAN-CLUSTER
    CanCluster                                                             = 4599,
    /// CAN-CLUSTER-CONDITIONAL
    CanClusterConditional                                                  = 3180,
    /// CAN-CLUSTER-VARIANTS
    CanClusterVariants                                                     = 6352,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 5905,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 5863,
    /// CAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    CanCommunicationControllerConditional                                  = 5780,
    /// CAN-COMMUNICATION-CONTROLLER-VARIANTS
    CanCommunicationControllerVariants                                     = 4199,
    /// CAN-CONFIG
    CanConfig                                                              = 4931,
    /// CAN-CONTROLLER-ATTRIBUTES
    CanControllerAttributes                                                = 2138,
    /// CAN-CONTROLLER-CONFIGURATION
    CanControllerConfiguration                                             = 898,
    /// CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS
    CanControllerConfigurationRequirements                                 = 301,
    /// CAN-CONTROLLER-FD-ATTRIBUTES
    CanControllerFdAttributes                                              = 2173,
    /// CAN-CONTROLLER-FD-REQUIREMENTS
    CanControllerFdRequirements                                            = 2776,
    /// CAN-CONTROLLER-XL-ATTRIBUTES
    CanControllerXlAttributes                                              = 5108,
    /// CAN-CONTROLLER-XL-REQUIREMENTS
    CanControllerXlRequirements                                            = 3810,
    /// CAN-ENTER-EXCLUSIVE-AREA-REF
    CanEnterExclusiveAreaRef                                               = 2066,
    /// CAN-ENTER-EXCLUSIVE-AREA-REFS
    CanEnterExclusiveAreaRefs                                              = 222,
    /// CAN-ENTERS
    CanEnters                                                              = 5935,
    /// CAN-FD-BAUDRATE
    CanFdBaudrate                                                          = 8,
    /// CAN-FD-CONFIG
    CanFdConfig                                                            = 2884,
    /// CAN-FD-FRAME-SUPPORT
    CanFdFrameSupport                                                      = 2454,
    /// CAN-FRAME
    CanFrame                                                               = 4346,
    /// CAN-FRAME-RX-BEHAVIOR
    CanFrameRxBehavior                                                     = 3447,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 22,
    /// CAN-FRAME-TX-BEHAVIOR
    CanFrameTxBehavior                                                     = 1029,
    /// CAN-GLOBAL-TIME-DOMAIN-PROPS
    CanGlobalTimeDomainProps                                               = 5787,
    /// CAN-ID-RANGE-MAPPINGS
    CanIdRangeMappings                                                     = 3702,
    /// CAN-ID-TO-CAN-ID-MAPPINGS
    CanIdToCanIdMappings                                                   = 6623,
    /// CAN-IDENTIFIER
    CanIdentifier                                                          = 5613,
    /// CAN-IDENTIFIER-MASK
    CanIdentifierMask                                                      = 3430,
    /// CAN-IDENTIFIER-RANGE
    CanIdentifierRange                                                     = 2727,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 4212,
    /// CAN-NM-CLUSTER-COUPLING
    CanNmClusterCoupling                                                   = 2156,
    /// CAN-NM-ECU
    CanNmEcu                                                               = 2182,
    /// CAN-NM-NODE
    CanNmNode                                                              = 1999,
    /// CAN-NM-TX-CAN-ID
    CanNmTxCanId                                                           = 2233,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 657,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 4552,
    /// CAN-TP-ADDRESS-REF
    CanTpAddressRef                                                        = 2779,
    /// CAN-TP-ADDRESS-REFS
    CanTpAddressRefs                                                       = 1088,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 2600,
    /// CAN-TP-CHANNEL-REF
    CanTpChannelRef                                                        = 1890,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 1826,
    /// CAN-TP-CONNECTION
    CanTpConnection                                                        = 3053,
    /// CAN-TP-ECU
    CanTpEcu                                                               = 5821,
    /// CAN-TP-NODE
    CanTpNode                                                              = 2033,
    /// CAN-XL-BAUDRATE
    CanXlBaudrate                                                          = 1786,
    /// CAN-XL-CONFIG
    CanXlConfig                                                            = 872,
    /// CAN-XL-CONFIG-REF
    CanXlConfigRef                                                         = 5118,
    /// CAN-XL-CONFIG-REQS
    CanXlConfigReqs                                                        = 4912,
    /// CAN-XL-FRAME-TRIGGERING-PROPS
    CanXlFrameTriggeringProps                                              = 2746,
    /// CAN-XL-NM-PROPS
    CanXlNmProps                                                           = 6148,
    /// CAN-XL-PROPS
    CanXlProps                                                             = 4737,
    /// CAN-XL-PROPS-REF
    CanXlPropsRef                                                          = 948,
    /// CAN-XL-PROPS-REFS
    CanXlPropsRefs                                                         = 3421,
    /// CANCELLATION
    Cancellation                                                           = 550,
    /// CAPABILITY
    Capability                                                             = 1397,
    /// CAPABILITY-RECORDS
    CapabilityRecords                                                      = 1092,
    /// CAPACITY
    Capacity                                                               = 4127,
    /// CAPTION
    Caption                                                                = 5939,
    /// CAS-RX-LOW-MAX
    CasRxLowMax                                                            = 5670,
    /// CAT-2-ISR
    Cat2Isr                                                                = 5646,
    /// CATEGORY
    Category                                                               = 5287,
    /// CDD-TYPE
    CddType                                                                = 4913,
    /// CERT-COMMON-NAME
    CertCommonName                                                         = 5283,
    /// CERTIFICATE-EVALUATIONS
    CertificateEvaluations                                                 = 2403,
    /// CERTIFICATE-GROUP-REF
    CertificateGroupRef                                                    = 4747,
    /// CERTIFICATE-REF
    CertificateRef                                                         = 2354,
    /// CERTIFICATE-TO-KEY-SLOT-MAPPINGS
    CertificateToKeySlotMappings                                           = 2717,
    /// CHANGE
    Change                                                                 = 2083,
    /// CHANNEL-ID
    ChannelId                                                              = 2854,
    /// CHANNEL-MODE
    ChannelMode                                                            = 1784,
    /// CHANNEL-NAME
    ChannelName                                                            = 806,
    /// CHANNEL-REF
    ChannelRef                                                             = 18,
    /// CHANNEL-SYNCHRONOUS-WAKEUP
    ChannelSynchronousWakeup                                               = 149,
    /// CHANNELS
    Channels                                                               = 1582,
    /// CHANNELS-PER-FRAME
    ChannelsPerFrame                                                       = 6137,
    /// CHAPTER
    Chapter                                                                = 2469,
    /// CHAPTERS
    Chapters                                                               = 1393,
    /// CHARGE-MANAGER-NEEDS
    ChargeManagerNeeds                                                     = 2874,
    /// CHECK-PER-SOURCE-ID
    CheckPerSourceId                                                       = 5137,
    /// CHECK-STATIC-BLOCK-ID
    CheckStaticBlockId                                                     = 794,
    /// CHECKPOINT-FINAL-REF
    CheckpointFinalRef                                                     = 3038,
    /// CHECKPOINT-FINAL-REFS
    CheckpointFinalRefs                                                    = 5312,
    /// CHECKPOINT-ID
    CheckpointId                                                           = 6625,
    /// CHECKPOINT-INITIAL-REF
    CheckpointInitialRef                                                   = 1680,
    /// CHECKPOINT-INITIAL-REFS
    CheckpointInitialRefs                                                  = 2768,
    /// CHECKPOINT-IREF
    CheckpointIref                                                         = 4657,
    /// CHECKPOINT-REF
    CheckpointRef                                                          = 6511,
    /// CHECKPOINT-REFS
    CheckpointRefs                                                         = 3049,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 6170,
    /// CHECKPOINT-TRANSITION-REF
    CheckpointTransitionRef                                                = 5802,
    /// CHECKPOINTS
    Checkpoints                                                            = 129,
    /// CHECKPOINTSS
    Checkpointss                                                           = 859,
    /// CHECKSUM-VALUE
    ChecksumValue                                                          = 3968,
    /// CHECKSUM-VERIFICATION
    ChecksumVerification                                                   = 416,
    /// CHOICES
    Choices                                                                = 1192,
    /// CIPHER-SUITE
    CipherSuite                                                            = 1056,
    /// CIPHER-SUITE-CONFIGS
    CipherSuiteConfigs                                                     = 5303,
    /// CIPHER-SUITE-ID
    CipherSuiteId                                                          = 3235,
    /// CIPHER-SUITE-NAME
    CipherSuiteName                                                        = 4594,
    /// CIPHER-SUITE-PRIORITY
    CipherSuitePriority                                                    = 4413,
    /// CIPHER-SUITE-SHORT-LABEL
    CipherSuiteShortLabel                                                  = 3010,
    /// CKN-REF
    CknRef                                                                 = 365,
    /// CLAIMED-FUNCTION-GROUP-REF
    ClaimedFunctionGroupRef                                                = 4879,
    /// CLAIMED-FUNCTION-GROUP-REFS
    ClaimedFunctionGroupRefs                                               = 5106,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 6039,
    /// CLASS-CONTENTS
    ClassContents                                                          = 1208,
    /// CLASS-TAILORINGS
    ClassTailorings                                                        = 1690,
    /// CLASSIFICATION
    Classification                                                         = 6042,
    /// CLASSIFICATIONS
    Classifications                                                        = 4712,
    /// CLEAR-CONDITION-GROUP-REF
    ClearConditionGroupRef                                                 = 3632,
    /// CLEAR-CONDITION-REF
    ClearConditionRef                                                      = 5945,
    /// CLEAR-CONDITION-REFS
    ClearConditionRefs                                                     = 497,
    /// CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF
    ClearDiagnosticInformationClassRef                                     = 4405,
    /// CLEAR-DTC-LIMITATION
    ClearDtcLimitation                                                     = 6047,
    /// CLEAR-EVENT-ALLOWED-BEHAVIOR
    ClearEventAllowedBehavior                                              = 6550,
    /// CLEAR-EVENT-BEHAVIOR
    ClearEventBehavior                                                     = 2840,
    /// CLEAR-FROM-VALID-TO-INVALID
    ClearFromValidToInvalid                                                = 4377,
    /// CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF
    ClearResetEmissionRelatedDiagnosticInfoClassRef                        = 2622,
    /// CLIENT-CAPABILITY
    ClientCapability                                                       = 223,
    /// CLIENT-COM-SPEC
    ClientComSpec                                                          = 131,
    /// CLIENT-ID
    ClientId                                                               = 5698,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 6595,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 5092,
    /// CLIENT-ID-DEFINITION-SET-REF
    ClientIdDefinitionSetRef                                               = 6382,
    /// CLIENT-ID-DEFINITION-SET-REFS
    ClientIdDefinitionSetRefs                                              = 3638,
    /// CLIENT-ID-DEFINITIONS
    ClientIdDefinitions                                                    = 3680,
    /// CLIENT-ID-RANGE
    ClientIdRange                                                          = 1370,
    /// CLIENT-INTENT
    ClientIntent                                                           = 19,
    /// CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST
    ClientIpAddrFromConnectionRequest                                      = 5998,
    /// CLIENT-POLICYS
    ClientPolicys                                                          = 5852,
    /// CLIENT-PORT-FROM-CONNECTION-REQUEST
    ClientPortFromConnectionRequest                                        = 1026,
    /// CLIENT-PORT-REF
    ClientPortRef                                                          = 4071,
    /// CLIENT-SERVER-ANNOTATION
    ClientServerAnnotation                                                 = 4303,
    /// CLIENT-SERVER-ANNOTATIONS
    ClientServerAnnotations                                                = 2724,
    /// CLIENT-SERVER-APPLICATION-ERROR-MAPPING
    ClientServerApplicationErrorMapping                                    = 1709,
    /// CLIENT-SERVER-ARRAY-ELEMENT-MAPPING
    ClientServerArrayElementMapping                                        = 5579,
    /// CLIENT-SERVER-ARRAY-TYPE-MAPPING
    ClientServerArrayTypeMapping                                           = 92,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 449,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 2764,
    /// CLIENT-SERVER-INTERFACE-REF
    ClientServerInterfaceRef                                               = 6271,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 3074,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 5810,
    /// CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING
    ClientServerOperationBlueprintMapping                                  = 958,
    /// CLIENT-SERVER-OPERATION-COM-PROPS
    ClientServerOperationComProps                                          = 1428,
    /// CLIENT-SERVER-OPERATION-IREF
    ClientServerOperationIref                                              = 5960,
    /// CLIENT-SERVER-OPERATION-MAPPING
    ClientServerOperationMapping                                           = 3227,
    /// CLIENT-SERVER-OPERATION-REF
    ClientServerOperationRef                                               = 6169,
    /// CLIENT-SERVER-PORTS
    ClientServerPorts                                                      = 4584,
    /// CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING
    ClientServerPrimitiveTypeMapping                                       = 1896,
    /// CLIENT-SERVER-RECORD-ELEMENT-MAPPING
    ClientServerRecordElementMapping                                       = 900,
    /// CLIENT-SERVER-RECORD-TYPE-MAPPING
    ClientServerRecordTypeMapping                                          = 183,
    /// CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING
    ClientServerToSignalGroupMapping                                       = 2940,
    /// CLIENT-SERVER-TO-SIGNAL-MAPPING
    ClientServerToSignalMapping                                            = 766,
    /// CLIENT-SERVICE-MAJOR-VERSION
    ClientServiceMajorVersion                                              = 5660,
    /// CLIENT-SERVICE-MINOR-VERSION
    ClientServiceMinorVersion                                              = 1450,
    /// CLOCK-REFERENCE-REF
    ClockReferenceRef                                                      = 1489,
    /// CLUSTER-DRIFT-DAMPING
    ClusterDriftDamping                                                    = 3096,
    /// CLUSTERED-COMPONENT-IREF
    ClusteredComponentIref                                                 = 4644,
    /// CLUSTERED-COMPONENT-IREFS
    ClusteredComponentIrefs                                                = 300,
    /// CM-MODULE-INSTANTIATION
    CmModuleInstantiation                                                  = 5356,
    /// CODE
    Code                                                                   = 5888,
    /// CODE-DESCRIPTORS
    CodeDescriptors                                                        = 483,
    /// CODE-LABEL
    CodeLabel                                                              = 4661,
    /// COLD-START-ATTEMPTS
    ColdStartAttempts                                                      = 5562,
    /// COLLECTED-INSTANCE-IREF
    CollectedInstanceIref                                                  = 2300,
    /// COLLECTED-INSTANCE-IREFS
    CollectedInstanceIrefs                                                 = 4731,
    /// COLLECTION
    Collection                                                             = 5415,
    /// COLLECTION-PROPS
    CollectionProps                                                        = 3525,
    /// COLLECTION-REF
    CollectionRef                                                          = 524,
    /// COLLECTION-SEMANTICS
    CollectionSemantics                                                    = 6361,
    /// COLLECTION-THRESHOLD
    CollectionThreshold                                                    = 1287,
    /// COLLECTION-TIMEOUT
    CollectionTimeout                                                      = 1378,
    /// COLLECTION-TRIGGER
    CollectionTrigger                                                      = 5585,
    /// COLLISION-RESOLVING-SCHEDULE-REF
    CollisionResolvingScheduleRef                                          = 3479,
    /// COLOR-MODE
    ColorMode                                                              = 5861,
    /// COLSPEC
    Colspec                                                                = 1617,
    /// COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS
    ComBasedSignalGroupTransformations                                     = 521,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 4685,
    /// COM-CONFIGURATION-GW-TIME-BASE
    ComConfigurationGwTimeBase                                             = 180,
    /// COM-CONFIGURATION-ID
    ComConfigurationId                                                     = 1500,
    /// COM-CONFIGURATION-RX-TIME-BASE
    ComConfigurationRxTimeBase                                             = 6457,
    /// COM-CONFIGURATION-TX-TIME-BASE
    ComConfigurationTxTimeBase                                             = 1686,
    /// COM-CONTROL-CLASS-REF
    ComControlClassRef                                                     = 989,
    /// COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION
    ComEnableMdtForCyclicTransmission                                      = 1769,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 1531,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 2315,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 2153,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 1684,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 6467,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 5380,
    /// COM-HANDLER-TASK-MAPPING-ENABLED
    ComHandlerTaskMappingEnabled                                           = 3932,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 5958,
    /// COM-MANAGEMENT-GROUP-REF
    ComManagementGroupRef                                                  = 2213,
    /// COM-MANAGEMENT-GROUP-REFS
    ComManagementGroupRefs                                                 = 1799,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 3118,
    /// COM-MANAGEMENT-MAPPINGS
    ComManagementMappings                                                  = 5722,
    /// COM-MANAGEMENT-PORT-GROUP-IREF
    ComManagementPortGroupIref                                             = 4666,
    /// COM-MANAGEMENT-PORT-GROUP-IREFS
    ComManagementPortGroupIrefs                                            = 3322,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 4668,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 6100,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 1178,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 549,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 3849,
    /// COM-PROCESSING-PERIOD
    ComProcessingPeriod                                                    = 934,
    /// COM-PROPS
    ComProps                                                               = 199,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 2258,
    /// COM-TIMEOUT-ERROR-REF
    ComTimeoutErrorRef                                                     = 1385,
    /// COM-TRIGGER-GRANT
    ComTriggerGrant                                                        = 741,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 408,
    /// COMBINED-NO-DATA-INIT-COUNT
    CombinedNoDataInitCount                                                = 4129,
    /// COMM-CONNECTORS
    CommConnectors                                                         = 112,
    /// COMM-CONTROLLER-MAPPINGS
    CommControllerMappings                                                 = 3898,
    /// COMM-CONTROLLER-REF
    CommControllerRef                                                      = 2038,
    /// COMM-CONTROLLERS
    CommControllers                                                        = 3299,
    /// COMMAND
    Command                                                                = 2252,
    /// COMMANDS
    Commands                                                               = 5433,
    /// COMMENT
    Comment                                                                = 2414,
    /// COMMITTED-BURST-SIZE
    CommittedBurstSize                                                     = 4183,
    /// COMMITTED-INFORMATION-RATE
    CommittedInformationRate                                               = 2296,
    /// COMMON-PROPERTIES
    CommonProperties                                                       = 493,
    /// COMMON-SIGNAL-PATH
    CommonSignalPath                                                       = 6075,
    /// COMMUNICATION-BUFFER-LOCKING
    CommunicationBufferLocking                                             = 3589,
    /// COMMUNICATION-CLUSTER-REF
    CommunicationClusterRef                                                = 6048,
    /// COMMUNICATION-CLUSTER-REFS
    CommunicationClusterRefs                                               = 2327,
    /// COMMUNICATION-CONNECTOR-REF
    CommunicationConnectorRef                                              = 1667,
    /// COMMUNICATION-CONNECTOR-REF-CONDITIONAL
    CommunicationConnectorRefConditional                                   = 4567,
    /// COMMUNICATION-CONNECTOR-REFS
    CommunicationConnectorRefs                                             = 1897,
    /// COMMUNICATION-CONNECTORS
    CommunicationConnectors                                                = 667,
    /// COMMUNICATION-CONTROLLER-MAPPING
    CommunicationControllerMapping                                         = 1867,
    /// COMMUNICATION-CONTROLLER-REF
    CommunicationControllerRef                                             = 2804,
    /// COMMUNICATION-CONTROLLERS
    CommunicationControllers                                               = 1979,
    /// COMMUNICATION-CYCLE
    CommunicationCycle                                                     = 4232,
    /// COMMUNICATION-DIRECTION
    CommunicationDirection                                                 = 5893,
    /// COMMUNICATION-MODE
    CommunicationMode                                                      = 2773,
    /// COMMUNICATION-RESOURCE-PROPS
    CommunicationResourceProps                                             = 1417,
    /// COMMUNICATION-RESOURCE-REF
    CommunicationResourceRef                                               = 3878,
    /// COMPARE-TYPE
    CompareType                                                            = 6593,
    /// COMPARE-VALUE
    CompareValue                                                           = 6608,
    /// COMPILER
    Compiler                                                               = 5741,
    /// COMPILERS
    Compilers                                                              = 3955,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 53,
    /// COMPLEX-TYPE-MAPPING
    ComplexTypeMapping                                                     = 3371,
    /// COMPONENT-CLUSTERING
    ComponentClustering                                                    = 617,
    /// COMPONENT-IMPLEMENTATION-REF
    ComponentImplementationRef                                             = 6323,
    /// COMPONENT-IREF
    ComponentIref                                                          = 481,
    /// COMPONENT-IREFS
    ComponentIrefs                                                         = 316,
    /// COMPONENT-QUALIFIER
    ComponentQualifier                                                     = 5273,
    /// COMPONENT-REF
    ComponentRef                                                           = 663,
    /// COMPONENT-SEPARATION
    ComponentSeparation                                                    = 114,
    /// COMPONENTS
    Components                                                             = 4188,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 1240,
    /// COMPOSITE-NETWORK-REPRESENTATION
    CompositeNetworkRepresentation                                         = 2808,
    /// COMPOSITE-NETWORK-REPRESENTATIONS
    CompositeNetworkRepresentations                                        = 4469,
    /// COMPOSITE-RULE-BASED-VALUE-SPECIFICATION
    CompositeRuleBasedValueSpecification                                   = 2109,
    /// COMPOSITE-SERVICE-INTERFACE-REF
    CompositeServiceInterfaceRef                                           = 4578,
    /// COMPOSITE-TYPE-MAPPINGS
    CompositeTypeMappings                                                  = 1852,
    /// COMPOSITION-DATA-PROTOTYPE-REF
    CompositionDataPrototypeRef                                            = 1075,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 631,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 5008,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 2424,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF
    CompositionSwComponentTypeRef                                          = 6007,
    /// COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL
    CompositionSwComponentTypeRefConditional                               = 1609,
    /// COMPOUND-PRIMITIVE-ARGUMENTS
    CompoundPrimitiveArguments                                             = 83,
    /// COMPRESSED-SOFTWARE-PACKAGE-SIZE
    CompressedSoftwarePackageSize                                          = 5087,
    /// COMPU-CONST
    CompuConst                                                             = 6413,
    /// COMPU-DEFAULT-VALUE
    CompuDefaultValue                                                      = 2806,
    /// COMPU-DENOMINATOR
    CompuDenominator                                                       = 4080,
    /// COMPU-INTERNAL-TO-PHYS
    CompuInternalToPhys                                                    = 4322,
    /// COMPU-INVERSE-VALUE
    CompuInverseValue                                                      = 2534,
    /// COMPU-METHOD
    CompuMethod                                                            = 3745,
    /// COMPU-METHOD-REF
    CompuMethodRef                                                         = 2260,
    /// COMPU-NUMERATOR
    CompuNumerator                                                         = 5259,
    /// COMPU-PHYS-TO-INTERNAL
    CompuPhysToInternal                                                    = 6393,
    /// COMPU-RATIONAL-COEFFS
    CompuRationalCoeffs                                                    = 4871,
    /// COMPU-SCALE
    CompuScale                                                             = 2032,
    /// COMPU-SCALES
    CompuScales                                                            = 3957,
    /// COMPUTED
    Computed                                                               = 2518,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 2902,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 3207,
    /// COND
    Cond                                                                   = 4404,
    /// CONDITION
    Condition                                                              = 4218,
    /// CONDITION-ACCESS
    ConditionAccess                                                        = 3774,
    /// CONDITION-FORMULA
    ConditionFormula                                                       = 86,
    /// CONDITIONAL-CHANGE-NAD
    ConditionalChangeNad                                                   = 1543,
    /// CONDITIONS
    Conditions                                                             = 1318,
    /// CONFIDENCE-INTERVAL
    ConfidenceInterval                                                     = 156,
    /// CONFIDENCE-INTERVALS
    ConfidenceIntervals                                                    = 1421,
    /// CONFIDENTIALITY-OFFSET
    ConfidentialityOffset                                                  = 6421,
    /// CONFIG-CLASS
    ConfigClass                                                            = 4091,
    /// CONFIG-ELEMENT-DEF-GLOBAL-REF
    ConfigElementDefGlobalRef                                              = 56,
    /// CONFIG-ELEMENT-DEF-LOCAL-REF
    ConfigElementDefLocalRef                                               = 3234,
    /// CONFIG-VARIANT
    ConfigVariant                                                          = 4656,
    /// CONFIGURATION-CLASS-AFFECTION
    ConfigurationClassAffection                                            = 886,
    /// CONFIGURATION-HANDLING
    ConfigurationHandling                                                  = 1974,
    /// CONFIGURED-NAD
    ConfiguredNad                                                          = 986,
    /// CONFIRMATION-THRESHOLD
    ConfirmationThreshold                                                  = 2060,
    /// CONFLICTS
    Conflicts                                                              = 4769,
    /// CONFLICTS-TO
    ConflictsTo                                                            = 2720,
    /// CONNECT
    Connect                                                                = 466,
    /// CONNECTED-INDICATORS
    ConnectedIndicators                                                    = 1025,
    /// CONNECTION-BUNDLES
    ConnectionBundles                                                      = 973,
    /// CONNECTION-IS-MANDATORY
    ConnectionIsMandatory                                                  = 4869,
    /// CONNECTION-NEGOTIATION-BEHAVIOR
    ConnectionNegotiationBehavior                                          = 5559,
    /// CONNECTION-PRIO-PDUS
    ConnectionPrioPdus                                                     = 994,
    /// CONNECTION-TYPE
    ConnectionType                                                         = 6425,
    /// CONNECTIONS
    Connections                                                            = 4572,
    /// CONNECTIVITY-KEY-NAME
    ConnectivityKeyName                                                    = 2453,
    /// CONNECTOR-REF
    ConnectorRef                                                           = 2926,
    /// CONNECTOR-REFS
    ConnectorRefs                                                          = 5573,
    /// CONNECTORS
    Connectors                                                             = 5809,
    /// CONSIDER-BUILD-NUMBER
    ConsiderBuildNumber                                                    = 4965,
    /// CONSIDER-PTO-STATUS
    ConsiderPtoStatus                                                      = 796,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 1593,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 1059,
    /// CONSISTENCY-NEEDSS
    ConsistencyNeedss                                                      = 1281,
    /// CONSTANT-MAPPING-REF
    ConstantMappingRef                                                     = 2592,
    /// CONSTANT-MAPPING-REFS
    ConstantMappingRefs                                                    = 2157,
    /// CONSTANT-MEMORYS
    ConstantMemorys                                                        = 583,
    /// CONSTANT-REF
    ConstantRef                                                            = 5238,
    /// CONSTANT-REFERENCE
    ConstantReference                                                      = 6327,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 981,
    /// CONSTANT-SPECIFICATION-MAPPING
    ConstantSpecificationMapping                                           = 496,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 4470,
    /// CONSTANT-VALUE-MAPPING-REF
    ConstantValueMappingRef                                                = 5173,
    /// CONSTANT-VALUE-MAPPING-REFS
    ConstantValueMappingRefs                                               = 6414,
    /// CONSTR-LEVEL
    ConstrLevel                                                            = 2638,
    /// CONSTRAINT-REF
    ConstraintRef                                                          = 789,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 421,
    /// CONSTRAINT-TAILORINGS
    ConstraintTailorings                                                   = 1228,
    /// CONSUMED-DDS-OPERATIONS
    ConsumedDdsOperations                                                  = 6494,
    /// CONSUMED-DDS-SERVICE-EVENTS
    ConsumedDdsServiceEvents                                               = 3841,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 740,
    /// CONSUMED-EVENT-GROUP-REF
    ConsumedEventGroupRef                                                  = 5170,
    /// CONSUMED-EVENT-GROUP-REFS
    ConsumedEventGroupRefs                                                 = 1968,
    /// CONSUMED-EVENT-GROUPS
    ConsumedEventGroups                                                    = 6150,
    /// CONSUMED-EVENTS
    ConsumedEvents                                                         = 1402,
    /// CONSUMED-FIELDS
    ConsumedFields                                                         = 6275,
    /// CONSUMED-OPERATIONS
    ConsumedOperations                                                     = 378,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 2048,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF
    ConsumedProvidedServiceInstanceGroupRef                                = 1074,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL
    ConsumedProvidedServiceInstanceGroupRefConditional                     = 3764,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 4309,
    /// CONSUMED-SERVICE-INSTANCE-REF
    ConsumedServiceInstanceRef                                             = 1797,
    /// CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL
    ConsumedServiceInstanceRefConditional                                  = 4332,
    /// CONSUMED-SERVICE-INSTANCES
    ConsumedServiceInstances                                               = 1387,
    /// CONTAINED-AR-ELEMENT-REF
    ContainedArElementRef                                                  = 5013,
    /// CONTAINED-AR-ELEMENT-REFS
    ContainedArElementRefs                                                 = 1888,
    /// CONTAINED-FIBEX-ELEMENT-REF
    ContainedFibexElementRef                                               = 6565,
    /// CONTAINED-FIBEX-ELEMENT-REFS
    ContainedFibexElementRefs                                              = 2821,
    /// CONTAINED-I-PDU-PROPS
    ContainedIPduProps                                                     = 1982,
    /// CONTAINED-I-PDU-TRIGGERING-PROPSS
    ContainedIPduTriggeringPropss                                          = 993,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REF
    ContainedISignalIPduGroupRef                                           = 1015,
    /// CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS
    ContainedISignalIPduGroupRefs                                          = 6419,
    /// CONTAINED-PACKAGE-ELEMENT-REF
    ContainedPackageElementRef                                             = 1940,
    /// CONTAINED-PACKAGE-ELEMENT-REFS
    ContainedPackageElementRefs                                            = 1472,
    /// CONTAINED-PDU-TRIGGERING-REF
    ContainedPduTriggeringRef                                              = 4445,
    /// CONTAINED-PDU-TRIGGERING-REFS
    ContainedPduTriggeringRefs                                             = 2521,
    /// CONTAINED-PROCESS-REF
    ContainedProcessRef                                                    = 587,
    /// CONTAINED-PROCESS-REFS
    ContainedProcessRefs                                                   = 5706,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 4721,
    /// CONTAINER-I-PDU-HEADER-BYTE-ORDER
    ContainerIPduHeaderByteOrder                                           = 3397,
    /// CONTAINER-TIMEOUT
    ContainerTimeout                                                       = 6172,
    /// CONTAINER-TRIGGER
    ContainerTrigger                                                       = 4427,
    /// CONTAINERS
    Containers                                                             = 4893,
    /// CONTENT-ELEMENT-REF
    ContentElementRef                                                      = 2019,
    /// CONTENT-OBJECT
    ContentObject                                                          = 1292,
    /// CONTENT-REF
    ContentRef                                                             = 1340,
    /// CONTENT-TYPE
    ContentType                                                            = 6478,
    /// CONTENT-URI
    ContentUri                                                             = 2580,
    /// CONTEXT-APPLICATION-RECORD-ELEMENT-REF
    ContextApplicationRecordElementRef                                     = 6335,
    /// CONTEXT-ATOMIC-COMPONENT-REF
    ContextAtomicComponentRef                                              = 2499,
    /// CONTEXT-BSW-IMPLEMENTATION-REF
    ContextBswImplementationRef                                            = 4863,
    /// CONTEXT-COMPONENT-PROTOTYPE-REF
    ContextComponentPrototypeRef                                           = 4450,
    /// CONTEXT-COMPONENT-REF
    ContextComponentRef                                                    = 93,
    /// CONTEXT-COMPONENT-REFS
    ContextComponentRefs                                                   = 230,
    /// CONTEXT-COMPOSITION-REF
    ContextCompositionRef                                                  = 4120,
    /// CONTEXT-DATA-ELEMENT-IDENTIFICATIONS
    ContextDataElementIdentifications                                      = 5359,
    /// CONTEXT-DATA-ELEMENTS
    ContextDataElements                                                    = 5250,
    /// CONTEXT-DATA-PROTOTYPE-IN-CS-REF
    ContextDataPrototypeInCsRef                                            = 3940,
    /// CONTEXT-DATA-PROTOTYPE-IN-SR-REF
    ContextDataPrototypeInSrRef                                            = 4156,
    /// CONTEXT-DATA-PROTOTYPE-REF
    ContextDataPrototypeRef                                                = 2905,
    /// CONTEXT-DATA-PROTOTYPE-REFS
    ContextDataPrototypeRefs                                               = 6442,
    /// CONTEXT-DATA-SOURCE
    ContextDataSource                                                      = 2827,
    /// CONTEXT-DATAS
    ContextDatas                                                           = 2045,
    /// CONTEXT-DESCRIPTION
    ContextDescription                                                     = 5586,
    /// CONTEXT-ELEMENT-REF
    ContextElementRef                                                      = 6108,
    /// CONTEXT-ELEMENT-REFS
    ContextElementRefs                                                     = 6559,
    /// CONTEXT-ID
    ContextId                                                              = 3797,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF
    ContextImplementationDataElementRef                                    = 697,
    /// CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS
    ContextImplementationDataElementRefs                                   = 2322,
    /// CONTEXT-LIMITATION-REF
    ContextLimitationRef                                                   = 4299,
    /// CONTEXT-LIMITATION-REFS
    ContextLimitationRefs                                                  = 511,
    /// CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ContextModeDeclarationGroupPrototypeRef                                = 4793,
    /// CONTEXT-MODE-DECLARATION-GROUP-REF
    ContextModeDeclarationGroupRef                                         = 827,
    /// CONTEXT-OPERATION-REF
    ContextOperationRef                                                    = 821,
    /// CONTEXT-P-PORT-PROTOTYPE-REF
    ContextPPortPrototypeRef                                               = 4579,
    /// CONTEXT-P-PORT-REF
    ContextPPortRef                                                        = 1383,
    /// CONTEXT-PORT-PROTOTYPE-REF
    ContextPortPrototypeRef                                                = 5460,
    /// CONTEXT-PORT-REF
    ContextPortRef                                                         = 1895,
    /// CONTEXT-R-PORT-PROTOTYPE-REF
    ContextRPortPrototypeRef                                               = 842,
    /// CONTEXT-R-PORT-REF
    ContextRPortRef                                                        = 1564,
    /// CONTEXT-REF
    ContextRef                                                             = 3193,
    /// CONTEXT-ROOT-COMPONENT-REF
    ContextRootComponentRef                                                = 3403,
    /// CONTEXT-ROOT-COMPOSITION-REF
    ContextRootCompositionRef                                              = 4535,
    /// CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF
    ContextRootSwClusterDesignComponentPrototypeRef                        = 1420,
    /// CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF
    ContextRootSwComponentPrototypeRef                                     = 2962,
    /// CONTEXT-ROOT-SW-COMPOSITION-REF
    ContextRootSwCompositionRef                                            = 2451,
    /// CONTEXT-SW-COMPONENT-PROTOTYPE-REF
    ContextSwComponentPrototypeRef                                         = 4095,
    /// CONTEXT-SW-COMPONENT-REF
    ContextSwComponentRef                                                  = 2411,
    /// CONTEXT-SWC-PROTOTYPE-REF
    ContextSwcPrototypeRef                                                 = 2071,
    /// CONTEXT-SWC-PROTOTYPE-REFS
    ContextSwcPrototypeRefs                                                = 5197,
    /// CONTEXT-TYPE-LIST
    ContextTypeList                                                        = 1604,
    /// CONTEXT-TYPE-LISTS
    ContextTypeLists                                                       = 4882,
    /// CONTEXTS
    Contexts                                                               = 6014,
    /// CONTRACT-VERSION
    ContractVersion                                                        = 3273,
    /// CONTROL-CONSUMED-EVENT-GROUP-REF
    ControlConsumedEventGroupRef                                           = 5909,
    /// CONTROL-CONSUMED-EVENT-GROUP-REFS
    ControlConsumedEventGroupRefs                                          = 2206,
    /// CONTROL-ENABLE-MASK-BITS
    ControlEnableMaskBits                                                  = 2872,
    /// CONTROL-OPTION-RECORD-PRESENT
    ControlOptionRecordPresent                                             = 1523,
    /// CONTROL-PNC-REF
    ControlPncRef                                                          = 4350,
    /// CONTROL-PNC-REFS
    ControlPncRefs                                                         = 4805,
    /// CONTROL-PROVIDED-EVENT-GROUP-REF
    ControlProvidedEventGroupRef                                           = 3352,
    /// CONTROL-PROVIDED-EVENT-GROUP-REFS
    ControlProvidedEventGroupRefs                                          = 5822,
    /// CONTROLLED-DATA-ELEMENT-REF
    ControlledDataElementRef                                               = 6437,
    /// CONTROLLED-DATA-ELEMENT-REFS
    ControlledDataElementRefs                                              = 6136,
    /// CONTROLLED-FID-REF
    ControlledFidRef                                                       = 747,
    /// CONTROLLED-HW-ELEMENT-REF
    ControlledHwElementRef                                                 = 3032,
    /// CONTROLLER-REF
    ControllerRef                                                          = 3622,
    /// CORE-ID
    CoreId                                                                 = 2664,
    /// CORES
    Cores                                                                  = 1053,
    /// COUNT-PROFILE
    CountProfile                                                           = 3432,
    /// COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE
    CounterBasedFdcThresholdStorageValue                                   = 4205,
    /// COUNTER-DECREMENT-STEP-SIZE
    CounterDecrementStepSize                                               = 109,
    /// COUNTER-FAILED-THRESHOLD
    CounterFailedThreshold                                                 = 2718,
    /// COUNTER-INCREMENT-STEP-SIZE
    CounterIncrementStepSize                                               = 4728,
    /// COUNTER-JUMP-DOWN
    CounterJumpDown                                                        = 1124,
    /// COUNTER-JUMP-DOWN-VALUE
    CounterJumpDownValue                                                   = 881,
    /// COUNTER-JUMP-UP
    CounterJumpUp                                                          = 640,
    /// COUNTER-JUMP-UP-VALUE
    CounterJumpUpValue                                                     = 4363,
    /// COUNTER-OFFSET
    CounterOffset                                                          = 4155,
    /// COUNTER-PASSED-THRESHOLD
    CounterPassedThreshold                                                 = 2166,
    /// COUPLED-CLUSTER-REF
    CoupledClusterRef                                                      = 5036,
    /// COUPLED-CLUSTER-REFS
    CoupledClusterRefs                                                     = 5002,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 5258,
    /// COUPLING-ELEMENT-DETAILS
    CouplingElementDetails                                                 = 2295,
    /// COUPLING-ELEMENT-SWITCH-DETAILS
    CouplingElementSwitchDetails                                           = 5202,
    /// COUPLING-FLAG
    CouplingFlag                                                           = 1624,
    /// COUPLING-PORT
    CouplingPort                                                           = 491,
    /// COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER
    CouplingPortAsynchronousTrafficShaper                                  = 582,
    /// COUPLING-PORT-CONNECTION
    CouplingPortConnection                                                 = 4022,
    /// COUPLING-PORT-CONNECTIONS
    CouplingPortConnections                                                = 1293,
    /// COUPLING-PORT-CREDIT-BASED-SHAPER
    CouplingPortCreditBasedShaper                                          = 6189,
    /// COUPLING-PORT-DETAILS
    CouplingPortDetails                                                    = 2588,
    /// COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER
    CouplingPortEnhancedTrafficShaper                                      = 202,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 464,
    /// COUPLING-PORT-RATE-POLICY
    CouplingPortRatePolicy                                                 = 5896,
    /// COUPLING-PORT-REF
    CouplingPortRef                                                        = 6429,
    /// COUPLING-PORT-REF-CONDITIONAL
    CouplingPortRefConditional                                             = 1539,
    /// COUPLING-PORT-ROLE
    CouplingPortRole                                                       = 1486,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 952,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 3269,
    /// COUPLING-PORT-SPEED
    CouplingPortSpeed                                                      = 4479,
    /// COUPLING-PORT-STARTUP-ACTIVE-TIME
    CouplingPortStartupActiveTime                                          = 2606,
    /// COUPLING-PORT-STRUCTURAL-ELEMENTS
    CouplingPortStructuralElements                                         = 1905,
    /// COUPLING-PORT-SWITCHOFF-DELAY
    CouplingPortSwitchoffDelay                                             = 5256,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 4115,
    /// COUPLING-PORTS
    CouplingPorts                                                          = 1889,
    /// COUPLING-TYPE
    CouplingType                                                           = 1454,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 4482,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 1372,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 4334,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 394,
    /// CP-SOFTWARE-CLUSTER-REF
    CpSoftwareClusterRef                                                   = 5631,
    /// CP-SOFTWARE-CLUSTER-REF-CONDITIONAL
    CpSoftwareClusterRefConditional                                        = 703,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 2075,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-REF
    CpSoftwareClusterResourceRef                                           = 2423,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 562,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 5950,
    /// CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterToApplicationPartitionMapping                         = 4829,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 3372,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 3578,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 3419,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 2275,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 6057,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 3511,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 1572,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 4696,
    /// CPP-TEMPLATE-ARGUMENT
    CppTemplateArgument                                                    = 5048,
    /// CPU-USAGE
    CpuUsage                                                               = 2755,
    /// CRC-CORRECTION-FIELD
    CrcCorrectionField                                                     = 6044,
    /// CRC-DOMAIN-NUMBER
    CrcDomainNumber                                                        = 95,
    /// CRC-FLAGS
    CrcFlags                                                               = 6284,
    /// CRC-MESSAGE-LENGTH
    CrcMessageLength                                                       = 4748,
    /// CRC-OFFSET
    CrcOffset                                                              = 3485,
    /// CRC-PRECISE-ORIGIN-TIMESTAMP
    CrcPreciseOriginTimestamp                                              = 371,
    /// CRC-SECURED
    CrcSecured                                                             = 4352,
    /// CRC-SEQUENCE-ID
    CrcSequenceId                                                          = 6538,
    /// CRC-SOURCE-PORT-IDENTITY
    CrcSourcePortIdentity                                                  = 3641,
    /// CRC-VALIDATED
    CrcValidated                                                           = 4874,
    /// CREATE-ECU-WAKEUP-SOURCE
    CreateEcuWakeupSource                                                  = 76,
    /// CREATED-DATAS
    CreatedDatas                                                           = 4516,
    /// CRF-PULL
    CrfPull                                                                = 6388,
    /// CRF-TYPE
    CrfType                                                                = 6322,
    /// CRYPTO-ALG-ID
    CryptoAlgId                                                            = 1461,
    /// CRYPTO-ALGO-CONFIG
    CryptoAlgoConfig                                                       = 773,
    /// CRYPTO-ALGORITHM-DESCRIPTION
    CryptoAlgorithmDescription                                             = 720,
    /// CRYPTO-ALGORITHM-ID
    CryptoAlgorithmId                                                      = 2150,
    /// CRYPTO-ALGORITHM-STRING
    CryptoAlgorithmString                                                  = 2294,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 2847,
    /// CRYPTO-CERTIFICATE-GROUP
    CryptoCertificateGroup                                                 = 1145,
    /// CRYPTO-CERTIFICATE-GROUP-INTERFACE
    CryptoCertificateGroupInterface                                        = 2200,
    /// CRYPTO-CERTIFICATE-GROUP-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateGroupToPortPrototypeMapping                           = 5107,
    /// CRYPTO-CERTIFICATE-GROUPS
    CryptoCertificateGroups                                                = 5358,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 3602,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 357,
    /// CRYPTO-CERTIFICATE-REF
    CryptoCertificateRef                                                   = 3986,
    /// CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING
    CryptoCertificateToCryptoKeySlotMapping                                = 3165,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 2483,
    /// CRYPTO-CERTIFICATES
    CryptoCertificates                                                     = 3557,
    /// CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS
    CryptoDesignToCryptoDriverMappings                                     = 6314,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 1934,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING
    CryptoDriverToCryptoJobMapping                                         = 729,
    /// CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS
    CryptoDriverToCryptoJobMappings                                        = 2471,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 5413,
    /// CRYPTO-JOB
    CryptoJob                                                              = 4938,
    /// CRYPTO-JOB-REF
    CryptoJobRef                                                           = 3725,
    /// CRYPTO-JOBS
    CryptoJobs                                                             = 2376,
    /// CRYPTO-KEY-DESCRIPTION
    CryptoKeyDescription                                                   = 5306,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 3806,
    /// CRYPTO-KEY-REF
    CryptoKeyRef                                                           = 2463,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 2743,
    /// CRYPTO-KEY-SLOT-CLIENT-INTERFACE
    CryptoKeySlotClientInterface                                           = 765,
    /// CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE
    CryptoKeySlotContentAllowedUsage                                       = 4877,
    /// CRYPTO-KEY-SLOT-DESIGN
    CryptoKeySlotDesign                                                    = 3777,
    /// CRYPTO-KEY-SLOT-DESIGN-DESCRIPTION
    CryptoKeySlotDesignDescription                                         = 3399,
    /// CRYPTO-KEY-SLOT-DESIGN-REF
    CryptoKeySlotDesignRef                                                 = 825,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 6482,
    /// CRYPTO-KEY-SLOT-REF
    CryptoKeySlotRef                                                       = 3076,
    /// CRYPTO-KEY-SLOT-REFS
    CryptoKeySlotRefs                                                      = 409,
    /// CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToClientPortPrototypeMapping                              = 908,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 733,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN
    CryptoKeySlotUsageDesign                                               = 914,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING
    CryptoKeySlotUsageDesignMapping                                        = 6484,
    /// CRYPTO-KEY-SLOT-USAGE-DESIGN-REF
    CryptoKeySlotUsageDesignRef                                            = 4142,
    /// CRYPTO-KEY-SLOT-USER-DESIGN
    CryptoKeySlotUserDesign                                                = 5653,
    /// CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING
    CryptoKeySlotUserDesignMapping                                         = 2581,
    /// CRYPTO-KEY-SLOT-USER-DESIGN-REF
    CryptoKeySlotUserDesignRef                                             = 5444,
    /// CRYPTO-KEY-SLOTS
    CryptoKeySlots                                                         = 1313,
    /// CRYPTO-KEY-TYPE
    CryptoKeyType                                                          = 2349,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 2612,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 4908,
    /// CRYPTO-NEED-REF
    CryptoNeedRef                                                          = 2137,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 902,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 5766,
    /// CRYPTO-OBJECT-TYPE
    CryptoObjectType                                                       = 5794,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 5633,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 3514,
    /// CRYPTO-PROVIDER-DOCUMENTATION-REF
    CryptoProviderDocumentationRef                                         = 1144,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 5239,
    /// CRYPTO-PROVIDER-REF
    CryptoProviderRef                                                      = 2180,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 6585,
    /// CRYPTO-PROVIDERS
    CryptoProviders                                                        = 6446,
    /// CRYPTO-R-PORT-COM-SPEC
    CryptoRPortComSpec                                                     = 2292,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 1832,
    /// CRYPTO-SERVICE-CERTIFICATE-REF
    CryptoServiceCertificateRef                                            = 2857,
    /// CRYPTO-SERVICE-CERTIFICATE-REFS
    CryptoServiceCertificateRefs                                           = 4430,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 322,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 35,
    /// CRYPTO-SERVICE-KEY-REF
    CryptoServiceKeyRef                                                    = 4378,
    /// CRYPTO-SERVICE-MAPPINGS
    CryptoServiceMappings                                                  = 6129,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 2183,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 3972,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 2290,
    /// CRYPTO-SERVICE-QUEUE-REF
    CryptoServiceQueueRef                                                  = 4674,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 169,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 6426,
    /// CS-ERROR-REACTION
    CsErrorReaction                                                        = 3650,
    /// CSE-CODE
    CseCode                                                                = 381,
    /// CSE-CODE-FACTOR
    CseCodeFactor                                                          = 3042,
    /// CURRENT-DATA-TYPE-REF
    CurrentDataTypeRef                                                     = 2945,
    /// CURRENT-EXP
    CurrentExp                                                             = 4220,
    /// CURRENT-VALUE-REF
    CurrentValueRef                                                        = 544,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 5508,
    /// CUSTOM-DOCUMENT-ELEMENT-REF
    CustomDocumentElementRef                                               = 1167,
    /// CUSTOM-DOCUMENTATION-REF
    CustomDocumentationRef                                                 = 141,
    /// CUSTOM-SDG-DEF-REF
    CustomSdgDefRef                                                        = 309,
    /// CUSTOM-SDG-DEF-REFS
    CustomSdgDefRefs                                                       = 2584,
    /// CUSTOM-SERVICE-CLASS-REF
    CustomServiceClassRef                                                  = 3309,
    /// CUSTOM-SERVICE-ID
    CustomServiceId                                                        = 5778,
    /// CUSTOM-SPECIFICATION-REF
    CustomSpecificationRef                                                 = 2405,
    /// CUSTOM-SPECIFICATION-REFS
    CustomSpecificationRefs                                                = 4790,
    /// CUSTOM-SUB-FUNCTION-NUMBER
    CustomSubFunctionNumber                                                = 3788,
    /// CUSTOM-TRIGGER
    CustomTrigger                                                          = 2676,
    /// CYCLE
    Cycle                                                                  = 3045,
    /// CYCLE-AUTOSTART
    CycleAutostart                                                         = 702,
    /// CYCLE-COUNT-MAX
    CycleCountMax                                                          = 4523,
    /// CYCLE-COUNTER
    CycleCounter                                                           = 1629,
    /// CYCLE-REPETITION
    CycleRepetition                                                        = 172,
    /// CYCLE-STATUS-STORAGE
    CycleStatusStorage                                                     = 6353,
    /// CYCLE-TIME-MAIN-FUNCTION
    CycleTimeMainFunction                                                  = 6094,
    /// CYCLE-TIME-VALUE
    CycleTimeValue                                                         = 2575,
    /// CYCLIC-BACKUP-INTERVAL
    CyclicBackupInterval                                                   = 78,
    /// CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPING
    CyclicHandlingComDataToOsTaskProxyMapping                              = 6333,
    /// CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPINGS
    CyclicHandlingComDataToOsTaskProxyMappings                             = 6642,
    /// CYCLIC-TIMING
    CyclicTiming                                                           = 3212,
    /// CYCLIC-WRITING-PERIOD
    CyclicWritingPeriod                                                    = 5538,
    /// DATA-COM-PROPS
    DataComProps                                                           = 3839,
    /// DATA-CONSISTENCY-POLICY
    DataConsistencyPolicy                                                  = 118,
    /// DATA-CONSTR
    DataConstr                                                             = 3762,
    /// DATA-CONSTR-REF
    DataConstrRef                                                          = 2372,
    /// DATA-CONSTR-RULE
    DataConstrRule                                                         = 753,
    /// DATA-CONSTR-RULES
    DataConstrRules                                                        = 4390,
    /// DATA-DUMP-ENTRY
    DataDumpEntry                                                          = 1097,
    /// DATA-ELEMENT-IREF
    DataElementIref                                                        = 480,
    /// DATA-ELEMENT-IREFS
    DataElementIrefs                                                       = 4038,
    /// DATA-ELEMENT-REF
    DataElementRef                                                         = 861,
    /// DATA-ELEMENT-REFS
    DataElementRefs                                                        = 2538,
    /// DATA-ELEMENTS
    DataElements                                                           = 1122,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 754,
    /// DATA-FILTER
    DataFilter                                                             = 262,
    /// DATA-FILTER-TYPE
    DataFilterType                                                         = 6600,
    /// DATA-FORMAT-TAILORING
    DataFormatTailoring                                                    = 4778,
    /// DATA-ID
    DataId                                                                 = 2668,
    /// DATA-ID-MODE
    DataIdMode                                                             = 5575,
    /// DATA-ID-NIBBLE-OFFSET
    DataIdNibbleOffset                                                     = 3593,
    /// DATA-IDENTIFIER-REF
    DataIdentifierRef                                                      = 5740,
    /// DATA-IDENTIFIER-REFS
    DataIdentifierRefs                                                     = 5128,
    /// DATA-IDS
    DataIds                                                                = 6257,
    /// DATA-IREF
    DataIref                                                               = 1502,
    /// DATA-LENGTH
    DataLength                                                             = 2367,
    /// DATA-LENGTH-REQUEST
    DataLengthRequest                                                      = 785,
    /// DATA-LENGTH-RESPONSE
    DataLengthResponse                                                     = 2907,
    /// DATA-LINK-LAYER-RULE
    DataLinkLayerRule                                                      = 479,
    /// DATA-MAPPINGS
    DataMappings                                                           = 566,
    /// DATA-OFFSET
    DataOffset                                                             = 3129,
    /// DATA-PDU-REF
    DataPduRef                                                             = 1994,
    /// DATA-PROTECTION-KIND
    DataProtectionKind                                                     = 4331,
    /// DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF
    DataProtototypeInPortInterfaceRef                                      = 1971,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 4010,
    /// DATA-PROTOTYPE-GROUP-IREF
    DataPrototypeGroupIref                                                 = 4313,
    /// DATA-PROTOTYPE-GROUP-IREFS
    DataPrototypeGroupIrefs                                                = 5327,
    /// DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF
    DataPrototypeInClientServerInterfaceIref                               = 3067,
    /// DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF
    DataPrototypeInMethodArgumentInstanceRefIref                           = 3893,
    /// DATA-PROTOTYPE-IN-PORT-INTERFACE-REF
    DataPrototypeInPortInterfaceRef                                        = 1819,
    /// DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF
    DataPrototypeInSenderReceiverInterfaceIref                             = 3153,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF
    DataPrototypeInServiceInterfaceIref                                    = 3443,
    /// DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF
    DataPrototypeInServiceInterfaceRef                                     = 3086,
    /// DATA-PROTOTYPE-IREF
    DataPrototypeIref                                                      = 782,
    /// DATA-PROTOTYPE-MAPPING
    DataPrototypeMapping                                                   = 1223,
    /// DATA-PROTOTYPE-REF
    DataPrototypeRef                                                       = 126,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPS
    DataPrototypeTransformationProps                                       = 936,
    /// DATA-PROTOTYPE-TRANSFORMATION-PROPSS
    DataPrototypeTransformationPropss                                      = 6622,
    /// DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF
    DataPrototypeWithApplicationDataTypeInSystemRef                        = 6177,
    /// DATA-PROTOTYPES
    DataPrototypes                                                         = 2254,
    /// DATA-PROVIDER
    DataProvider                                                           = 1521,
    /// DATA-READ-ACCESSS
    DataReadAccesss                                                        = 529,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 3718,
    /// DATA-RECEIVE-POINT-BY-ARGUMENTS
    DataReceivePointByArguments                                            = 2826,
    /// DATA-RECEIVE-POINT-BY-VALUES
    DataReceivePointByValues                                               = 2861,
    /// DATA-RECEIVE-POINTS
    DataReceivePoints                                                      = 4808,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 1537,
    /// DATA-REF
    DataRef                                                                = 4546,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 3325,
    /// DATA-SEND-POINTS
    DataSendPoints                                                         = 614,
    /// DATA-TRANSFER-CLASS-REF
    DataTransferClassRef                                                   = 2751,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 2126,
    /// DATA-TRANSFORMATION-KIND
    DataTransformationKind                                                 = 1225,
    /// DATA-TRANSFORMATION-REF
    DataTransformationRef                                                  = 3919,
    /// DATA-TRANSFORMATION-REF-CONDITIONAL
    DataTransformationRefConditional                                       = 1330,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 1644,
    /// DATA-TRANSFORMATIONS
    DataTransformations                                                    = 3323,
    /// DATA-TYPE-FOR-SERIALIZATION-REF
    DataTypeForSerializationRef                                            = 2695,
    /// DATA-TYPE-FOR-SERIALIZATION-REFS
    DataTypeForSerializationRefs                                           = 5381,
    /// DATA-TYPE-MAP
    DataTypeMap                                                            = 6149,
    /// DATA-TYPE-MAPPING
    DataTypeMapping                                                        = 5469,
    /// DATA-TYPE-MAPPING-REF
    DataTypeMappingRef                                                     = 3651,
    /// DATA-TYPE-MAPPING-REFS
    DataTypeMappingRefs                                                    = 4738,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 2710,
    /// DATA-TYPE-MAPPING-SET-REF
    DataTypeMappingSetRef                                                  = 5825,
    /// DATA-TYPE-MAPPING-SET-REFS
    DataTypeMappingSetRefs                                                 = 1945,
    /// DATA-TYPE-MAPS
    DataTypeMaps                                                           = 5143,
    /// DATA-TYPE-POLICY
    DataTypePolicy                                                         = 4957,
    /// DATA-TYPE-REF
    DataTypeRef                                                            = 6281,
    /// DATA-TYPE-REFS
    DataTypeRefs                                                           = 3321,
    /// DATA-UPDATE-PERIOD
    DataUpdatePeriod                                                       = 1206,
    /// DATA-VALUE-ERROR-REF
    DataValueErrorRef                                                      = 5751,
    /// DATA-WRITE-ACCESSS
    DataWriteAccesss                                                       = 2978,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 5549,
    /// DATE
    Date                                                                   = 2462,
    /// DCM-BUS-TYPE
    DcmBusType                                                             = 3974,
    /// DCM-I-PDU
    DcmIPdu                                                                = 283,
    /// DDS-CP-CONFIG
    DdsCpConfig                                                            = 5922,
    /// DDS-CP-CONSUMED-SERVICE-INSTANCE
    DdsCpConsumedServiceInstance                                           = 2735,
    /// DDS-CP-DOMAIN
    DdsCpDomain                                                            = 972,
    /// DDS-CP-I-SIGNAL-TO-DDS-TOPIC-MAPPING
    DdsCpISignalToDdsTopicMapping                                          = 5000,
    /// DDS-CP-PARTITION
    DdsCpPartition                                                         = 2901,
    /// DDS-CP-PROVIDED-SERVICE-INSTANCE
    DdsCpProvidedServiceInstance                                           = 3215,
    /// DDS-CP-QOS-PROFILE
    DdsCpQosProfile                                                        = 5121,
    /// DDS-CP-SERVICE-INSTANCE-EVENT
    DdsCpServiceInstanceEvent                                              = 4896,
    /// DDS-CP-SERVICE-INSTANCE-OPERATION
    DdsCpServiceInstanceOperation                                          = 5956,
    /// DDS-CP-TOPIC
    DdsCpTopic                                                             = 6041,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 4858,
    /// DDS-DOMAINS
    DdsDomains                                                             = 239,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 6487,
    /// DDS-EVENT-QOS-PROFILE-REF
    DdsEventQosProfileRef                                                  = 1708,
    /// DDS-EVENT-QOS-PROPS
    DdsEventQosProps                                                       = 84,
    /// DDS-EVENT-REF
    DdsEventRef                                                            = 1643,
    /// DDS-EVENT-TOPIC-REF
    DdsEventTopicRef                                                       = 4928,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 1094,
    /// DDS-FIELD-QOS-PROPS
    DdsFieldQosProps                                                       = 4802,
    /// DDS-FIELD-REPLY-TOPIC-REF
    DdsFieldReplyTopicRef                                                  = 64,
    /// DDS-FIELD-REQUEST-TOPIC-REF
    DdsFieldRequestTopicRef                                                = 937,
    /// DDS-I-SIGNAL-TO-TOPIC-MAPPINGS
    DdsISignalToTopicMappings                                              = 1577,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 3700,
    /// DDS-METHOD-QOS-PROPS
    DdsMethodQosProps                                                      = 187,
    /// DDS-METHOD-REPLY-TOPIC-REF
    DdsMethodReplyTopicRef                                                 = 4575,
    /// DDS-METHOD-REQUEST-TOPIC-REF
    DdsMethodRequestTopicRef                                               = 6365,
    /// DDS-OPERATION-REQUEST-TRIGGERING-REF
    DdsOperationRequestTriggeringRef                                       = 3944,
    /// DDS-OPERATION-RESPONSE-TRIGGERING-REF
    DdsOperationResponseTriggeringRef                                      = 4238,
    /// DDS-PARTITION-REF
    DdsPartitionRef                                                        = 1664,
    /// DDS-PARTITIONS
    DdsPartitions                                                          = 6517,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 3310,
    /// DDS-QOS-PROFILE-REF
    DdsQosProfileRef                                                       = 2953,
    /// DDS-QOS-PROFILES
    DdsQosProfiles                                                         = 2553,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 293,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 2856,
    /// DDS-RPC-SERVICE-REF
    DdsRpcServiceRef                                                       = 4088,
    /// DDS-RPC-SERVICES
    DdsRpcServices                                                         = 1748,
    /// DDS-RULE
    DdsRule                                                                = 1981,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 852,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 209,
    /// DDS-SERVICE-INSTANCE-EVENT-CP
    DdsServiceInstanceEventCp                                              = 5326,
    /// DDS-SERVICE-INSTANCE-FIELD-CP
    DdsServiceInstanceFieldCp                                              = 781,
    /// DDS-SERVICE-INSTANCE-OPERATION-CP
    DdsServiceInstanceOperationCp                                          = 3677,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 5455,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 5341,
    /// DDS-SERVICE-QOS-PROFILE-REF
    DdsServiceQosProfileRef                                                = 1723,
    /// DDS-SERVICE-VERSION
    DdsServiceVersion                                                      = 6059,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 6179,
    /// DDS-TOPIC-REF
    DdsTopicRef                                                            = 621,
    /// DDS-TOPICS
    DdsTopics                                                              = 3125,
    /// DDS-TRANSFORMATION-DESCRIPTION
    DdsTransformationDescription                                           = 4264,
    /// DDS-TRANSFORMATION-I-SIGNAL-PROPS
    DdsTransformationISignalProps                                          = 800,
    /// DDS-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    DdsTransformationISignalPropsConditional                               = 3973,
    /// DDS-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    DdsTransformationISignalPropsVariants                                  = 5236,
    /// DEADLINE
    Deadline                                                               = 3942,
    /// DEADLINE-MAX
    DeadlineMax                                                            = 1917,
    /// DEADLINE-MIN
    DeadlineMin                                                            = 3461,
    /// DEADLINE-PERIOD
    DeadlinePeriod                                                         = 6319,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 1315,
    /// DEADLINE-SUPERVISIONS
    DeadlineSupervisions                                                   = 4744,
    /// DEBOUNCE-ALGORITHM
    DebounceAlgorithm                                                      = 6641,
    /// DEBOUNCE-ALGORITHM-PROPSS
    DebounceAlgorithmPropss                                                = 5240,
    /// DEBOUNCE-ALGORITHM-REF
    DebounceAlgorithmRef                                                   = 3856,
    /// DEBOUNCE-BEHAVIOR
    DebounceBehavior                                                       = 2886,
    /// DEBOUNCE-COUNTER-STORAGE
    DebounceCounterStorage                                                 = 4502,
    /// DEBOUNCE-TIME
    DebounceTime                                                           = 345,
    /// DEBUG-INFOS
    DebugInfos                                                             = 4888,
    /// DECODING-CORRECTION
    DecodingCorrection                                                     = 4471,
    /// DECOMPOSITIONS
    Decompositions                                                         = 2309,
    /// DEF
    Def                                                                    = 4718,
    /// DEF-CALPRM-SET
    DefCalprmSet                                                           = 4764,
    /// DEF-ITEM
    DefItem                                                                = 6605,
    /// DEF-LIST
    DefList                                                                = 4562,
    /// DEFAULT-ACTION
    DefaultAction                                                          = 1834,
    /// DEFAULT-APPLICATION-TIMEOUT
    DefaultApplicationTimeout                                              = 6598,
    /// DEFAULT-ENDIANNESS
    DefaultEndianness                                                      = 2514,
    /// DEFAULT-GATEWAY
    DefaultGateway                                                         = 6516,
    /// DEFAULT-LC-STATE-REF
    DefaultLcStateRef                                                      = 3143,
    /// DEFAULT-LEASE-TIME
    DefaultLeaseTime                                                       = 954,
    /// DEFAULT-LOG-THRESHOLD
    DefaultLogThreshold                                                    = 3814,
    /// DEFAULT-MODE-REF
    DefaultModeRef                                                         = 3418,
    /// DEFAULT-PATTERN
    DefaultPattern                                                         = 265,
    /// DEFAULT-PERIOD-BEGIN
    DefaultPeriodBegin                                                     = 1601,
    /// DEFAULT-PERIOD-END
    DefaultPeriodEnd                                                       = 3662,
    /// DEFAULT-PRIORITY
    DefaultPriority                                                        = 211,
    /// DEFAULT-REPORTING-MODE
    DefaultReportingMode                                                   = 6594,
    /// DEFAULT-ROUTER
    DefaultRouter                                                          = 2882,
    /// DEFAULT-TRACE-STATE
    DefaultTraceState                                                      = 176,
    /// DEFAULT-TRAFFIC-CLASS
    DefaultTrafficClass                                                    = 1184,
    /// DEFAULT-VALUE
    DefaultValue                                                           = 2805,
    /// DEFAULT-VALUE-ELEMENT
    DefaultValueElement                                                    = 6334,
    /// DEFAULT-VALUE-ELEMENTS
    DefaultValueElements                                                   = 1550,
    /// DEFAULT-VALUE-HANDLING
    DefaultValueHandling                                                   = 2560,
    /// DEFAULT-VLAN-REF
    DefaultVlanRef                                                         = 4139,
    /// DEFERRING-FID-REF
    DeferringFidRef                                                        = 3467,
    /// DEFERRING-FID-REFS
    DeferringFidRefs                                                       = 2125,
    /// DEFINITION-REF
    DefinitionRef                                                          = 2507,
    /// DELAY
    Delay                                                                  = 4591,
    /// DELAY-COMPENSATION-A
    DelayCompensationA                                                     = 1317,
    /// DELAY-COMPENSATION-B
    DelayCompensationB                                                     = 6493,
    /// DELEGATED-PORT-ANNOTATION
    DelegatedPortAnnotation                                                = 6,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 4259,
    /// DELETES
    Deletes                                                                = 81,
    /// DELIVERY-ARTIFACTS
    DeliveryArtifacts                                                      = 1326,
    /// DELTA-PACKAGE-APPLICABLE-VERSION
    DeltaPackageApplicableVersion                                          = 144,
    /// DENOMINATOR-CONDITION
    DenominatorCondition                                                   = 1139,
    /// DENOMINATOR-GROUP
    DenominatorGroup                                                       = 5476,
    /// DEPENDENCIES
    Dependencies                                                           = 4678,
    /// DEPENDENCY
    Dependency                                                             = 1078,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 5864,
    /// DEPENDENT-RESOURCES
    DependentResources                                                     = 287,
    /// DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF
    DependentSoftwareClusterDesignRef                                      = 5164,
    /// DEPENDENT-SOFTWARE-CLUSTER-REF
    DependentSoftwareClusterRef                                            = 692,
    /// DEPENDS-ON
    DependsOn                                                              = 3147,
    /// DEPENDS-ON-REF
    DependsOnRef                                                           = 5485,
    /// DEPENDS-ON-REFS
    DependsOnRefs                                                          = 145,
    /// DEPENDS-ONS
    DependsOns                                                             = 6338,
    /// DEPLOYMENT-REF
    DeploymentRef                                                          = 315,
    /// DEPLOYMENT-URIS
    DeploymentUris                                                         = 3665,
    /// DERIVATION
    Derivation                                                             = 2787,
    /// DERIVED-CERTIFICATE-ACCEPTED
    DerivedCertificateAccepted                                             = 6294,
    /// DERIVED-FROM-BLUEPRINT-REF
    DerivedFromBlueprintRef                                                = 787,
    /// DERIVED-FROM-BLUEPRINT-REFS
    DerivedFromBlueprintRefs                                               = 302,
    /// DERIVED-INTERFACE-REF
    DerivedInterfaceRef                                                    = 1967,
    /// DERIVED-OBJECT-REF
    DerivedObjectRef                                                       = 41,
    /// DERIVED-PORT-INTERFACE-REF
    DerivedPortInterfaceRef                                                = 5849,
    /// DERIVED-PORT-PROTOTYPE-REF
    DerivedPortPrototypeRef                                                = 3954,
    /// DESC
    Desc                                                                   = 4509,
    /// DESCRIPTION
    Description                                                            = 2277,
    /// DESIGN-REF
    DesignRef                                                              = 5222,
    /// DESIGN-REFS
    DesignRefs                                                             = 1465,
    /// DEST-META-CLASS
    DestMetaClass                                                          = 1874,
    /// DEST-SDG-REF
    DestSdgRef                                                             = 4934,
    /// DESTINATION-BASE-ID
    DestinationBaseId                                                      = 1731,
    /// DESTINATION-CONTEXT
    DestinationContext                                                     = 3746,
    /// DESTINATION-IP-ADDRESS
    DestinationIpAddress                                                   = 899,
    /// DESTINATION-IPV-4-ADDRESS
    DestinationIpv4Address                                                 = 1305,
    /// DESTINATION-IPV-6-ADDRESS
    DestinationIpv6Address                                                 = 4026,
    /// DESTINATION-MAC-ADDRESS
    DestinationMacAddress                                                  = 344,
    /// DESTINATION-MAC-ADDRESS-MASK
    DestinationMacAddressMask                                              = 6109,
    /// DESTINATION-MAX-ADDRESS
    DestinationMaxAddress                                                  = 1447,
    /// DESTINATION-MIN-ADDRESS
    DestinationMinAddress                                                  = 4819,
    /// DESTINATION-NETWORK-MASK
    DestinationNetworkMask                                                 = 4292,
    /// DESTINATION-ORDER
    DestinationOrder                                                       = 4215,
    /// DESTINATION-ORDER-KIND
    DestinationOrderKind                                                   = 1414,
    /// DESTINATION-PHYSICAL-ADDRESS
    DestinationPhysicalAddress                                             = 343,
    /// DESTINATION-PORTS
    DestinationPorts                                                       = 4044,
    /// DESTINATION-REF
    DestinationRef                                                         = 2601,
    /// DESTINATION-REFS
    DestinationRefs                                                        = 983,
    /// DESTINATION-TYPE
    DestinationType                                                        = 6521,
    /// DESTINATION-URI-DEFS
    DestinationUriDefs                                                     = 2910,
    /// DESTINATION-URI-NESTING-CONTRACT
    DestinationUriNestingContract                                          = 2119,
    /// DESTINATION-URI-POLICY
    DestinationUriPolicy                                                   = 2862,
    /// DESTINATION-URI-REF
    DestinationUriRef                                                      = 5866,
    /// DESTINATION-URI-REFS
    DestinationUriRefs                                                     = 5840,
    /// DETECT-NIT-ERROR
    DetectNitError                                                         = 2358,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 527,
    /// DETERMINISTIC-CLIENT-REF
    DeterministicClientRef                                                 = 4478,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 642,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDSS
    DeterministicClientResourceNeedss                                      = 3939,
    /// DETERMINISTIC-SYNC-M-OUT-OF-N
    DeterministicSyncMOutOfN                                               = 1980,
    /// DETERMINISTIC-SYNC-MASTER
    DeterministicSyncMaster                                                = 3515,
    /// DETERMINISTIC-SYNC-MASTER-REF
    DeterministicSyncMasterRef                                             = 2777,
    /// DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING
    DeterministicSyncMasterToTimeBaseConsumerMapping                       = 3685,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 4663,
    /// DEVELOPMENT-VALUE
    DevelopmentValue                                                       = 923,
    /// DHCP-ADDRESS-ASSIGNMENT
    DhcpAddressAssignment                                                  = 6376,
    /// DHCP-PROPS
    DhcpProps                                                              = 5757,
    /// DHCP-SERVER-CONFIGURATION
    DhcpServerConfiguration                                                = 1207,
    /// DIAG-ARG-INTEGRITY
    DiagArgIntegrity                                                       = 1238,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 1632,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 2205,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 5836,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 4138,
    /// DIAG-FAULT-DETECTION-COUNTER-PORT-REF
    DiagFaultDetectionCounterPortRef                                       = 6616,
    /// DIAG-PDU-TYPE
    DiagPduType                                                            = 5332,
    /// DIAG-REQUIREMENT
    DiagRequirement                                                        = 6350,
    /// DIAG-ROUTINE-TYPE
    DiagRoutineType                                                        = 318,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 4410,
    /// DIAGNOSTIC-ADDRESS
    DiagnosticAddress                                                      = 2725,
    /// DIAGNOSTIC-ADDRESSES
    DiagnosticAddresses                                                    = 4035,
    /// DIAGNOSTIC-ADDRESSS
    DiagnosticAddresss                                                     = 2568,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 2283,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 2728,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE
    DiagnosticAuthTransmitCertificate                                      = 2666,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION
    DiagnosticAuthTransmitCertificateEvaluation                            = 5688,
    /// DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING
    DiagnosticAuthTransmitCertificateMapping                               = 6283,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 1660,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 4583,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 4494,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 5707,
    /// DIAGNOSTIC-AUTHENTICATION-REF
    DiagnosticAuthenticationRef                                            = 3595,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 2591,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 4620,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 6260,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 6631,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 977,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 4063,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 5241,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 2223,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 4665,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 5224,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 155,
    /// DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL
    DiagnosticComControlSpecificChannel                                    = 2051,
    /// DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL
    DiagnosticComControlSubNodeChannel                                     = 3387,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF
    DiagnosticCommonElementRef                                             = 212,
    /// DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL
    DiagnosticCommonElementRefConditional                                  = 4177,
    /// DIAGNOSTIC-COMMON-PROPS-CONDITIONAL
    DiagnosticCommonPropsConditional                                       = 3343,
    /// DIAGNOSTIC-COMMON-PROPS-VARIANTS
    DiagnosticCommonPropsVariants                                          = 1166,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 3070,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 5753,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 3506,
    /// DIAGNOSTIC-CONDITION-REF
    DiagnosticConditionRef                                                 = 5937,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 6052,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 5924,
    /// DIAGNOSTIC-CONNECTION-REF
    DiagnosticConnectionRef                                                = 1597,
    /// DIAGNOSTIC-CONNECTION-REF-CONDITIONAL
    DiagnosticConnectionRefConditional                                     = 1416,
    /// DIAGNOSTIC-CONNECTIONS
    DiagnosticConnections                                                  = 619,
    /// DIAGNOSTIC-CONTRIBUTION-REF
    DiagnosticContributionRef                                              = 2734,
    /// DIAGNOSTIC-CONTRIBUTION-REFS
    DiagnosticContributionRefs                                             = 2175,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 1152,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 2634,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 2361,
    /// DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT
    DiagnosticControlEnableMaskBit                                         = 2013,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 2225,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 866,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 5109,
    /// DIAGNOSTIC-DATA-CHANGE-TRIGGER
    DiagnosticDataChangeTrigger                                            = 892,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 5191,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 419,
    /// DIAGNOSTIC-DATA-ELEMENT-REF
    DiagnosticDataElementRef                                               = 1391,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 586,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 2065,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 6128,
    /// DIAGNOSTIC-DATA-IDENTIFIER-REF
    DiagnosticDataIdentifierRef                                            = 3377,
    /// DIAGNOSTIC-DATA-IDENTIFIER-REF-CONDITIONAL
    DiagnosticDataIdentifierRefConditional                                 = 2429,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 6267,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF
    DiagnosticDataIdentifierSetRef                                         = 843,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL
    DiagnosticDataIdentifierSetRefConditional                              = 2191,
    /// DIAGNOSTIC-DATA-IDENTIFIERS
    DiagnosticDataIdentifiers                                              = 4117,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 848,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 4894,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 3490,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 1869,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 4976,
    /// DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT
    DiagnosticDebounceBehaviorEnumValueVariationPoint                      = 1863,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 2002,
    /// DIAGNOSTIC-DEPLOYMENT-PROPS-REF
    DiagnosticDeploymentPropsRef                                           = 2028,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 4191,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING
    DiagnosticDoIpActivationLinePortMapping                                = 6491,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE
    DiagnosticDoIpEntityIdentificationInterface                            = 6062,
    /// DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpEntityIdentificationPortMapping                          = 854,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 2703,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING
    DiagnosticDoIpGroupIdentificationPortMapping                           = 3474,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 4287,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING
    DiagnosticDoIpPowerModePortMapping                                     = 1110,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 2079,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING
    DiagnosticDoIpTriggerVehicleAnnouncementPortMapping                    = 6543,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 3133,
    /// DIAGNOSTIC-DTC-CHANGE-TRIGGER
    DiagnosticDtcChangeTrigger                                             = 1080,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 968,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 2067,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 4119,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 3618,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 1155,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 3881,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 5649,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 5059,
    /// DIAGNOSTIC-EDR-DATA-PROVIDER-MAPPING
    DiagnosticEdrDataProviderMapping                                       = 944,
    /// DIAGNOSTIC-EDR-SENDER-PORT-MAPPING
    DiagnosticEdrSenderPortMapping                                         = 2084,
    /// DIAGNOSTIC-EDR-SERVER-PORT-MAPPING
    DiagnosticEdrServerPortMapping                                         = 5537,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 1297,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 5226,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 1256,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 1010,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF
    DiagnosticEnableConditionRef                                           = 2969,
    /// DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL
    DiagnosticEnableConditionRefConditional                                = 4343,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 1570,
    /// DIAGNOSTIC-ENV-CONDITION-FORMULA
    DiagnosticEnvConditionFormula                                          = 2163,
    /// DIAGNOSTIC-ENV-DATA-CONDITION
    DiagnosticEnvDataCondition                                             = 5488,
    /// DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION
    DiagnosticEnvDataElementCondition                                      = 1138,
    /// DIAGNOSTIC-ENV-MODE-CONDITION
    DiagnosticEnvModeCondition                                             = 6343,
    /// DIAGNOSTIC-ENV-SOVD-DATA-CONDITION
    DiagnosticEnvSovdDataCondition                                         = 3971,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 3538,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 5744,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 686,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 4472,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 3509,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 4639,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 2037,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 2222,
    /// DIAGNOSTIC-EVENT-REF
    DiagnosticEventRef                                                     = 4401,
    /// DIAGNOSTIC-EVENT-REF-CONDITIONAL
    DiagnosticEventRefConditional                                          = 4836,
    /// DIAGNOSTIC-EVENT-REFS
    DiagnosticEventRefs                                                    = 2465,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 1921,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 6628,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 5357,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 4677,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 5928,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 6572,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 2018,
    /// DIAGNOSTIC-EVENT-WINDOW
    DiagnosticEventWindow                                                  = 4735,
    /// DIAGNOSTIC-EVENTS
    DiagnosticEvents                                                       = 5400,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 475,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-CLIENT-PORT-MAPPING
    DiagnosticExtendedDataRecordClientPortMapping                          = 106,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-ELEMENT
    DiagnosticExtendedDataRecordElement                                    = 1194,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-INTERFACE
    DiagnosticExtendedDataRecordInterface                                  = 2528,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-NEEDS
    DiagnosticExtendedDataRecordNeeds                                      = 362,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF
    DiagnosticExtendedDataRecordRef                                        = 3915,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL
    DiagnosticExtendedDataRecordRefConditional                             = 3388,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION
    DiagnosticExternalAuthenticationIdentification                         = 3476,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 3441,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 4224,
    /// DIAGNOSTIC-EXTRACT-REF
    DiagnosticExtractRef                                                   = 1588,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 6146,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 4281,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 4258,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 463,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 6253,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 17,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 1181,
    /// DIAGNOSTIC-FREEZE-FRAME-REF
    DiagnosticFreezeFrameRef                                               = 1202,
    /// DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL
    DiagnosticFreezeFrameRefConditional                                    = 3102,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 2691,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 282,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 1898,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 6562,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 3071,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 4367,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 3267,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 572,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 5374,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 1086,
    /// DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT
    DiagnosticIndicatorTypeEnumValueVariationPoint                         = 1200,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 1811,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 1462,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 1374,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 3164,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 1232,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 5713,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 5592,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 1894,
    /// DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER
    DiagnosticIumprGroupIdentifier                                         = 1249,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 6205,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 3720,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 6029,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 2848,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 3233,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 2232,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 759,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 537,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 1116,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 5411,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 1410,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 1704,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 2332,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 1363,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 5204,
    /// DIAGNOSTIC-MEMORY-REF
    DiagnosticMemoryRef                                                    = 6464,
    /// DIAGNOSTIC-MESSAGE-TYPE
    DiagnosticMessageType                                                  = 5155,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 1950,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 3429,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE
    DiagnosticMultipleConditionInterface                                   = 4669,
    /// DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING
    DiagnosticMultipleConditionPortMapping                                 = 1878,
    /// DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE
    DiagnosticMultipleEventInterface                                       = 1100,
    /// DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING
    DiagnosticMultipleEventPortMapping                                     = 3157,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE
    DiagnosticMultipleMonitorInterface                                     = 4160,
    /// DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING
    DiagnosticMultipleMonitorPortMapping                                   = 577,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 5969,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 3327,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 2859,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 3057,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF
    DiagnosticOperationCycleRef                                            = 749,
    /// DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL
    DiagnosticOperationCycleRefConditional                                 = 3799,
    /// DIAGNOSTIC-OPERATION-CYCLES
    DiagnosticOperationCycles                                              = 3712,
    /// DIAGNOSTIC-PARAMETER
    DiagnosticParameter                                                    = 2046,
    /// DIAGNOSTIC-PARAMETER-ELEMENT
    DiagnosticParameterElement                                             = 3146,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 2318,
    /// DIAGNOSTIC-PARAMETER-REF
    DiagnosticParameterRef                                                 = 957,
    /// DIAGNOSTIC-PERIODIC-RATE
    DiagnosticPeriodicRate                                                 = 351,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 992,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 5541,
    /// DIAGNOSTIC-PROPS
    DiagnosticProps                                                        = 6120,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 2185,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 906,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 3188,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 2468,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 2479,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 5193,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 4197,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 6032,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 1271,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 3349,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 4700,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 3389,
    /// DIAGNOSTIC-RELEVANCE
    DiagnosticRelevance                                                    = 127,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 6011,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 6362,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 6147,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 4616,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 5473,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 6545,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 5281,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 540,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 4754,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 1405,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 5693,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 850,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE
    DiagnosticRequestFileTransferInterface                                 = 2723,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 6466,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 1352,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 5604,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 4885,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 4460,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 1047,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 1873,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 3192,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 725,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 5885,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 1975,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 5823,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 1903,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 1991,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 6112,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 3156,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 1965,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 2056,
    /// DIAGNOSTIC-SECURE-CODING-MAPPING
    DiagnosticSecureCodingMapping                                          = 1866,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 5504,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 4999,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 2946,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 3220,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 2485,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 3084,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 2692,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 4301,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 5991,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 3093,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF
    DiagnosticServiceInstanceRef                                           = 4458,
    /// DIAGNOSTIC-SERVICE-INSTANCE-REF-CONDITIONAL
    DiagnosticServiceInstanceRefConditional                                = 57,
    /// DIAGNOSTIC-SERVICE-INSTANCES
    DiagnosticServiceInstances                                             = 1008,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 4571,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 3704,
    /// DIAGNOSTIC-SERVICE-TABLE-REF
    DiagnosticServiceTableRef                                              = 1403,
    /// DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL
    DiagnosticServiceTableRefConditional                                   = 44,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 2954,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 6524,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 4051,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 3880,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 3807,
    /// DIAGNOSTIC-SESSION-REF
    DiagnosticSessionRef                                                   = 4961,
    /// DIAGNOSTIC-SESSION-REFS
    DiagnosticSessionRefs                                                  = 1253,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 325,
    /// DIAGNOSTIC-SOVD-ACCESS-ARGUMENT
    DiagnosticSovdAccessArgument                                           = 4690,
    /// DIAGNOSTIC-SOVD-ARRAY-CONTENT-ELEMENT
    DiagnosticSovdArrayContentElement                                      = 627,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE
    DiagnosticSovdAuthorizationInterface                                   = 2633,
    /// DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING
    DiagnosticSovdAuthorizationPortMapping                                 = 6499,
    /// DIAGNOSTIC-SOVD-BULK-DATA
    DiagnosticSovdBulkData                                                 = 376,
    /// DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE
    DiagnosticSovdBulkDataInterface                                        = 3283,
    /// DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING
    DiagnosticSovdBulkDataPortMapping                                      = 4164,
    /// DIAGNOSTIC-SOVD-CONFIG-CONTENT-MAPPING
    DiagnosticSovdConfigContentMapping                                     = 6465,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA
    DiagnosticSovdConfigurationBulkData                                    = 1791,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING
    DiagnosticSovdConfigurationDataIdentifierMapping                       = 584,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE
    DiagnosticSovdConfigurationInterface                                   = 5491,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER
    DiagnosticSovdConfigurationParameter                                   = 2999,
    /// DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING
    DiagnosticSovdConfigurationPortMapping                                 = 4532,
    /// DIAGNOSTIC-SOVD-CONTENT
    DiagnosticSovdContent                                                  = 518,
    /// DIAGNOSTIC-SOVD-CONTENT-ELEMENT-INTERFACE
    DiagnosticSovdContentElementInterface                                  = 2818,
    /// DIAGNOSTIC-SOVD-CONTENT-GROUP
    DiagnosticSovdContentGroup                                             = 6396,
    /// DIAGNOSTIC-SOVD-CONTENT-INTERFACE
    DiagnosticSovdContentInterface                                         = 1303,
    /// DIAGNOSTIC-SOVD-CONTENT-PORT-MAPPING
    DiagnosticSovdContentPortMapping                                       = 1016,
    /// DIAGNOSTIC-SOVD-DATA
    DiagnosticSovdData                                                     = 618,
    /// DIAGNOSTIC-SOVD-DATA-CATEGORY
    DiagnosticSovdDataCategory                                             = 1119,
    /// DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS
    DiagnosticSovdFaultMemoryAccess                                        = 6569,
    /// DIAGNOSTIC-SOVD-GROUP
    DiagnosticSovdGroup                                                    = 34,
    /// DIAGNOSTIC-SOVD-LOCK
    DiagnosticSovdLock                                                     = 1626,
    /// DIAGNOSTIC-SOVD-LOG
    DiagnosticSovdLog                                                      = 6159,
    /// DIAGNOSTIC-SOVD-METHOD
    DiagnosticSovdMethod                                                   = 6134,
    /// DIAGNOSTIC-SOVD-METHOD-PRIMITIVE
    DiagnosticSovdMethodPrimitive                                          = 5277,
    /// DIAGNOSTIC-SOVD-OPERATION
    DiagnosticSovdOperation                                                = 6199,
    /// DIAGNOSTIC-SOVD-OPERATION-INTERFACE
    DiagnosticSovdOperationInterface                                       = 1432,
    /// DIAGNOSTIC-SOVD-OPERATION-PORT-MAPPING
    DiagnosticSovdOperationPortMapping                                     = 903,
    /// DIAGNOSTIC-SOVD-PRIMITIVE-CONTENT-ELEMENT
    DiagnosticSovdPrimitiveContentElement                                  = 778,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE
    DiagnosticSovdProximityChallengeInterface                              = 1976,
    /// DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING
    DiagnosticSovdProximityChallengePortMapping                            = 1273,
    /// DIAGNOSTIC-SOVD-RECORD-CONTENT-ELEMENT
    DiagnosticSovdRecordContentElement                                     = 6461,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE
    DiagnosticSovdServiceValidationInterface                               = 3784,
    /// DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING
    DiagnosticSovdServiceValidationPortMapping                             = 174,
    /// DIAGNOSTIC-SOVD-UPDATE
    DiagnosticSovdUpdate                                                   = 3300,
    /// DIAGNOSTIC-SOVD-UPDATE-INTERFACE
    DiagnosticSovdUpdateInterface                                          = 4614,
    /// DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING
    DiagnosticSovdUpdatePortMapping                                        = 3210,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 4640,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 573,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 777,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 1480,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF
    DiagnosticStorageConditionRef                                          = 6407,
    /// DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL
    DiagnosticStorageConditionRefConditional                               = 6103,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 3833,
    /// DIAGNOSTIC-TEST-RESULT-REF
    DiagnosticTestResultRef                                                = 4576,
    /// DIAGNOSTIC-TEST-RESULT-REFS
    DiagnosticTestResultRefs                                               = 4944,
    /// DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT
    DiagnosticTestResultUpdateEnumValueVariationPoint                      = 1066,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 2868,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 2168,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 1214,
    /// DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE
    DiagnosticTransmitCertificateInterface                                 = 1132,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 6015,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 605,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 3328,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD-PROPS
    DiagnosticTroubleCodeObdProps                                          = 678,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 2750,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS-REF
    DiagnosticTroubleCodePropsRef                                          = 1085,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS-REF-CONDITIONAL
    DiagnosticTroubleCodePropsRefConditional                               = 3495,
    /// DIAGNOSTIC-TROUBLE-CODE-REF
    DiagnosticTroubleCodeRef                                               = 4498,
    /// DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL
    DiagnosticTroubleCodeRefConditional                                    = 616,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 6157,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 3808,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 1257,
    /// DIAGNOSTIC-UDS-SEVERITY-ENUM-VALUE-VARIATION-POINT
    DiagnosticUdsSeverityEnumValueVariationPoint                           = 1239,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 1519,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 5157,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 3792,
    /// DIAGNOSTIC-VALUE-ACCESS
    DiagnosticValueAccess                                                  = 6313,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 2648,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 5027,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 3369,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 2698,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 5614,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 4726,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 5188,
    /// DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT
    DiagnosticWwhObdDtcClassEnumValueVariationPoint                        = 700,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 47,
    /// DID-NUMBER
    DidNumber                                                              = 2172,
    /// DID-SIZE
    DidSize                                                                = 4204,
    /// DIFFERENTIATED-SERVICE-CODE-POINT
    DifferentiatedServiceCodePoint                                         = 3842,
    /// DIFFERENTIATED-SERVICE-FIELD
    DifferentiatedServiceField                                             = 5179,
    /// DIRECT-PDU-REF
    DirectPduRef                                                           = 763,
    /// DIRECT-SUCCESSOR-REF
    DirectSuccessorRef                                                     = 6522,
    /// DIRECT-SUCCESSOR-REFS
    DirectSuccessorRefs                                                    = 4192,
    /// DIRECT-TP-SDU-REF
    DirectTpSduRef                                                         = 4354,
    /// DIRECTION
    Direction                                                              = 1444,
    /// DISABLE-END-TO-END-CHECK
    DisableEndToEndCheck                                                   = 3820,
    /// DISABLE-END-TO-END-STATE-MACHINE
    DisableEndToEndStateMachine                                            = 2706,
    /// DISABLED-IN-MODE-IREF
    DisabledInModeIref                                                     = 3354,
    /// DISABLED-IN-MODE-IREFS
    DisabledInModeIrefs                                                    = 1473,
    /// DISABLED-MODE-IREF
    DisabledModeIref                                                       = 5393,
    /// DISABLED-MODE-IREFS
    DisabledModeIrefs                                                      = 3922,
    /// DISCOVERY-PROTECTION-KIND
    DiscoveryProtectionKind                                                = 3913,
    /// DISCOVERY-TECHNOLOGY
    DiscoveryTechnology                                                    = 5763,
    /// DISCOVERY-TYPE
    DiscoveryType                                                          = 2264,
    /// DISPLAY-FORMAT
    DisplayFormat                                                          = 3901,
    /// DISPLAY-IDENTIFIER
    DisplayIdentifier                                                      = 4486,
    /// DISPLAY-NAME
    DisplayName                                                            = 5974,
    /// DISPLAY-PRESENTATION
    DisplayPresentation                                                    = 6554,
    /// DISTINGUISHED-PARTITIONS
    DistinguishedPartitions                                                = 6591,
    /// DLT-APPLICATION
    DltApplication                                                         = 5645,
    /// DLT-APPLICATION-REF
    DltApplicationRef                                                      = 2535,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 460,
    /// DLT-ARGUMENT
    DltArgument                                                            = 2981,
    /// DLT-ARGUMENT-ENTRYS
    DltArgumentEntrys                                                      = 6251,
    /// DLT-ARGUMENT-PROPS
    DltArgumentProps                                                       = 5248,
    /// DLT-ARGUMENT-PROPS-REF
    DltArgumentPropsRef                                                    = 5526,
    /// DLT-ARGUMENT-PROPS-REF-CONDITIONAL
    DltArgumentPropsRefConditional                                         = 6445,
    /// DLT-ARGUMENT-PROPS-SET
    DltArgumentPropsSet                                                    = 5192,
    /// DLT-ARGUMENT-PROPSS
    DltArgumentPropss                                                      = 2382,
    /// DLT-ARGUMENTS
    DltArguments                                                           = 6477,
    /// DLT-CONFIG
    DltConfig                                                              = 712,
    /// DLT-CONTEXT
    DltContext                                                             = 721,
    /// DLT-CONTEXT-REF
    DltContextRef                                                          = 4511,
    /// DLT-CONTEXT-REF-CONDITIONAL
    DltContextRefConditional                                               = 3727,
    /// DLT-ECU
    DltEcu                                                                 = 2496,
    /// DLT-ECU-ID
    DltEcuId                                                               = 5370,
    /// DLT-ECU-REF
    DltEcuRef                                                              = 4536,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 2888,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 5913,
    /// DLT-LOG-CHANNEL-DESIGN-REF
    DltLogChannelDesignRef                                                 = 1998,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 1380,
    /// DLT-LOG-CHANNEL-REF
    DltLogChannelRef                                                       = 2058,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 2500,
    /// DLT-LOG-CHANNELS
    DltLogChannels                                                         = 4635,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 2997,
    /// DLT-LOG-SINK-REF
    DltLogSinkRef                                                          = 4622,
    /// DLT-LOG-SINK-REFS
    DltLogSinkRefs                                                         = 2865,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 5257,
    /// DLT-MESSAGE
    DltMessage                                                             = 849,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 5208,
    /// DLT-MESSAGE-REF
    DltMessageRef                                                          = 4619,
    /// DLT-MESSAGE-REF-CONDITIONAL
    DltMessageRefConditional                                               = 1645,
    /// DLT-MESSAGE-REFS
    DltMessageRefs                                                         = 2381,
    /// DLT-MESSAGES
    DltMessages                                                            = 5708,
    /// DLT-SESSION-ID
    DltSessionId                                                           = 5745,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 576,
    /// DNS-SERVER-ADDRESS
    DnsServerAddress                                                       = 988,
    /// DNS-SERVER-ADDRESSES
    DnsServerAddresses                                                     = 5648,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 1198,
    /// DO-IP-CONFIG
    DoIpConfig                                                             = 4773,
    /// DO-IP-DESIGN-REF
    DoIpDesignRef                                                          = 6410,
    /// DO-IP-ENTITY
    DoIpEntity                                                             = 4848,
    /// DO-IP-ENTITY-ROLE
    DoIpEntityRole                                                         = 4681,
    /// DO-IP-FUNCTIONAL-CLUSTER-DESIGN
    DoIpFunctionalClusterDesign                                            = 5282,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 4694,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 2576,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 2461,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 4987,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 312,
    /// DO-IP-LOGIC-ADDRESS-PROPS
    DoIpLogicAddressProps                                                  = 1003,
    /// DO-IP-LOGIC-ADDRESSS
    DoIpLogicAddresss                                                      = 3838,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 6456,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 4801,
    /// DO-IP-LOGICAL-ADDRESS
    DoIpLogicalAddress                                                     = 696,
    /// DO-IP-LOGICAL-ADDRESS-REF
    DoIpLogicalAddressRef                                                  = 4485,
    /// DO-IP-NETWORK-CONFIGURATION
    DoIpNetworkConfiguration                                               = 3179,
    /// DO-IP-NETWORK-CONFIGURATION-DESIGN
    DoIpNetworkConfigurationDesign                                         = 5066,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 6504,
    /// DO-IP-PROTOCOL-VERSION
    DoIpProtocolVersion                                                    = 2288,
    /// DO-IP-REQUEST-CONFIGURATION
    DoIpRequestConfiguration                                               = 5158,
    /// DO-IP-REQUEST-CONFIGURATION-DESIGN
    DoIpRequestConfigurationDesign                                         = 5042,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 4335,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 4110,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 3740,
    /// DO-IP-ROUTING-ACTIVATIONS
    DoIpRoutingActivations                                                 = 1062,
    /// DO-IP-RULE
    DoIpRule                                                               = 6358,
    /// DO-IP-SOURCE-ADDRESS-REF
    DoIpSourceAddressRef                                                   = 6125,
    /// DO-IP-TARGET-ADDRESS-REF
    DoIpTargetAddressRef                                                   = 342,
    /// DO-IP-TARGET-ADDRESS-REFS
    DoIpTargetAddressRefs                                                  = 5054,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REF
    DoIpTesterRoutingActivationRef                                         = 6404,
    /// DO-IP-TESTER-ROUTING-ACTIVATION-REFS
    DoIpTesterRoutingActivationRefs                                        = 2531,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 1123,
    /// DO-IP-TP-CONNECTION
    DoIpTpConnection                                                       = 3246,
    /// DO-NOT-FRAGMENT
    DoNotFragment                                                          = 3320,
    /// DOC-REVISION
    DocRevision                                                            = 5987,
    /// DOC-REVISIONS
    DocRevisions                                                           = 6648,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 5672,
    /// DOCUMENT-ELEMENT-SCOPES
    DocumentElementScopes                                                  = 504,
    /// DOCUMENTATION
    Documentation                                                          = 5667,
    /// DOCUMENTATION-CONTENT
    DocumentationContent                                                   = 4804,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 3951,
    /// DOIP-CHANNEL-COLLECTION-REF
    DoipChannelCollectionRef                                               = 4898,
    /// DOIP-CONNECTION-REF
    DoipConnectionRef                                                      = 2159,
    /// DOIP-CONNECTION-REFS
    DoipConnectionRefs                                                     = 6255,
    /// DOIP-INTERFACES
    DoipInterfaces                                                         = 235,
    /// DOMAIN
    Domain                                                                 = 6021,
    /// DOMAIN-ID
    DomainId                                                               = 1596,
    /// DOMAIN-IDS
    DomainIds                                                              = 82,
    /// DPD-ACTION
    DpdAction                                                              = 3604,
    /// DPD-DELAY
    DpdDelay                                                               = 3413,
    /// DPG-DOES-NOT-REQUIRE-COHERENCYS
    DpgDoesNotRequireCoherencys                                            = 1375,
    /// DPG-REQUIRES-COHERENCYS
    DpgRequiresCoherencys                                                  = 5600,
    /// DRIVER-NOTIFICATIONS
    DriverNotifications                                                    = 490,
    /// DRIVER-PRIMITIVE-REF
    DriverPrimitiveRef                                                     = 443,
    /// DROP-NOT-REQUESTED-NAD
    DropNotRequestedNad                                                    = 1925,
    /// DTC-FORMAT-TYPE
    DtcFormatType                                                          = 1099,
    /// DTC-KIND
    DtcKind                                                                = 2807,
    /// DTC-NUMBER
    DtcNumber                                                              = 5517,
    /// DTC-PROPS-REF
    DtcPropsRef                                                            = 6065,
    /// DTC-SETTING-CLASS-REF
    DtcSettingClassRef                                                     = 4838,
    /// DTC-SETTING-PARAMETER
    DtcSettingParameter                                                    = 2480,
    /// DTC-STATUS-AVAILABILITY-MASK
    DtcStatusAvailabilityMask                                              = 538,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 3095,
    /// DTC-STATUS-MASK
    DtcStatusMask                                                          = 2253,
    /// DTCS
    Dtcs                                                                   = 3275,
    /// DURABILITY
    Durability                                                             = 3178,
    /// DURABILITY-KIND
    DurabilityKind                                                         = 5199,
    /// DURABILITY-SERVICE
    DurabilityService                                                      = 4234,
    /// DURABILITY-SERVICE-CLEANUP-DELAY
    DurabilityServiceCleanupDelay                                          = 6505,
    /// DURABILITY-SERVICE-HISTORY-DEPTH
    DurabilityServiceHistoryDepth                                          = 6539,
    /// DURABILITY-SERVICE-HISTORY-KIND
    DurabilityServiceHistoryKind                                           = 5784,
    /// DURABILITY-SERVICE-MAX-INSTANCES
    DurabilityServiceMaxInstances                                          = 5673,
    /// DURABILITY-SERVICE-MAX-SAMPLES
    DurabilityServiceMaxSamples                                            = 6431,
    /// DURABILITY-SERVICE-MAX-SAMPLES-PER-INSTANCE
    DurabilityServiceMaxSamplesPerInstance                                 = 6640,
    /// DYNAMIC-ACTION-REF
    DynamicActionRef                                                       = 6161,
    /// DYNAMIC-ACTION-REFS
    DynamicActionRefs                                                      = 6301,
    /// DYNAMIC-ARRAY-SIZE-PROFILE
    DynamicArraySizeProfile                                                = 77,
    /// DYNAMIC-BS
    DynamicBs                                                              = 1650,
    /// DYNAMIC-LENGTH
    DynamicLength                                                          = 2867,
    /// DYNAMIC-PART
    DynamicPart                                                            = 1419,
    /// DYNAMIC-PART-ALTERNATIVE
    DynamicPartAlternative                                                 = 4052,
    /// DYNAMIC-PART-ALTERNATIVES
    DynamicPartAlternatives                                                = 6502,
    /// DYNAMIC-PARTS
    DynamicParts                                                           = 3348,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REF
    DynamicPncMappingPduGroupRef                                           = 3222,
    /// DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS
    DynamicPncMappingPduGroupRefs                                          = 1993,
    /// DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED
    DynamicPncToChannelMappingEnabled                                      = 5959,
    /// DYNAMIC-RUNTIME-LENGTH-HANDLING
    DynamicRuntimeLengthHandling                                           = 715,
    /// DYNAMIC-SLOT-IDLE-PHASE
    DynamicSlotIdlePhase                                                   = 1269,
    /// DYNAMICALLY-ASSIGNED
    DynamicallyAssigned                                                    = 3553,
    /// DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF
    DynamicallyDefineDataIdentifierClassRef                                = 5827,
    /// E
    E                                                                      = 1720,
    /// E-2-E-COUNTER-ERROR-REF
    E2ECounterErrorRef                                                     = 4964,
    /// E-2-E-CRC-ERROR-REF
    E2ECrcErrorRef                                                         = 4356,
    /// E-2-E-EVENT-PROTECTION-PROPSS
    E2EEventProtectionPropss                                               = 5285,
    /// E-2-E-METHOD-PROTECTION-PROPSS
    E2EMethodProtectionPropss                                              = 1935,
    /// E-2-E-NO-NEW-DATA-EXTENSION-FACTOR
    E2ENoNewDataExtensionFactor                                            = 1836,
    /// E-2-E-OVERSAMPLING-PERIOD
    E2EOversamplingPeriod                                                  = 6558,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 2475,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS-REF
    E2EProfileCompatibilityPropsRef                                        = 4136,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 3201,
    /// E-2-E-PROFILE-CONFIGURATION-REF
    E2EProfileConfigurationRef                                             = 6614,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 10,
    /// E-2-E-PROFILE-CONFIGURATIONS
    E2EProfileConfigurations                                               = 1493,
    /// E-2-E-TIMEOUT-ERROR-REF
    E2ETimeoutErrorRef                                                     = 620,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 3080,
    /// ECU-COMM-PORT-INSTANCES
    EcuCommPortInstances                                                   = 5302,
    /// ECU-CONFIGURATION-REF
    EcuConfigurationRef                                                    = 3254,
    /// ECU-EXTRACT-REF
    EcuExtractRef                                                          = 3454,
    /// ECU-EXTRACT-REFERENCE-IREF
    EcuExtractReferenceIref                                                = 5245,
    /// ECU-EXTRACT-VERSION
    EcuExtractVersion                                                      = 1220,
    /// ECU-ID
    EcuId                                                                  = 5243,
    /// ECU-INSTANCE
    EcuInstance                                                            = 5163,
    /// ECU-INSTANCE-PROPS
    EcuInstanceProps                                                       = 4822,
    /// ECU-INSTANCE-PROPSS
    EcuInstancePropss                                                      = 4384,
    /// ECU-INSTANCE-REF
    EcuInstanceRef                                                         = 2196,
    /// ECU-INSTANCE-REF-CONDITIONAL
    EcuInstanceRefConditional                                              = 1808,
    /// ECU-INSTANCE-REFS
    EcuInstanceRefs                                                        = 2460,
    /// ECU-INSTANCES
    EcuInstances                                                           = 3802,
    /// ECU-MAPPING
    EcuMapping                                                             = 226,
    /// ECU-PARTITION
    EcuPartition                                                           = 4916,
    /// ECU-PARTITION-REF
    EcuPartitionRef                                                        = 2617,
    /// ECU-PARTITION-TO-CORE-MAPPING
    EcuPartitionToCoreMapping                                              = 4459,
    /// ECU-PARTITION-TO-CORE-MAPPINGS
    EcuPartitionToCoreMappings                                             = 69,
    /// ECU-REF
    EcuRef                                                                 = 6402,
    /// ECU-RESET-CLASS-REF
    EcuResetClassRef                                                       = 5626,
    /// ECU-RESOURCE-ESTIMATION
    EcuResourceEstimation                                                  = 5858,
    /// ECU-RESOURCE-MAPPINGS
    EcuResourceMappings                                                    = 4573,
    /// ECU-SCOPE-REF
    EcuScopeRef                                                            = 1335,
    /// ECU-SCOPE-REFS
    EcuScopeRefs                                                           = 5966,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 2741,
    /// ECU-TASK-PROXY-REF
    EcuTaskProxyRef                                                        = 142,
    /// ECU-TASK-PROXY-REFS
    EcuTaskProxyRefs                                                       = 2210,
    /// ECU-TIMING
    EcuTiming                                                              = 4176,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 3219,
    /// ECUC-ADD-INFO-PARAM-VALUE
    EcucAddInfoParamValue                                                  = 732,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 39,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 4159,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 4266,
    /// ECUC-COND
    EcucCond                                                               = 305,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 3711,
    /// ECUC-DEF-EDITION
    EcucDefEdition                                                         = 4042,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 2136,
    /// ECUC-DEFINITION-REF
    EcucDefinitionRef                                                      = 3803,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 2434,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 3351,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 3381,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 4664,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 6390,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 4179,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 2713,
    /// ECUC-FUNCTION-NAME-DEF-CONDITIONAL
    EcucFunctionNameDefConditional                                         = 1495,
    /// ECUC-FUNCTION-NAME-DEF-VARIANTS
    EcucFunctionNameDefVariants                                            = 3089,
    /// ECUC-IMPLEMENTATION-CONFIGURATION-CLASS
    EcucImplementationConfigurationClass                                   = 1476,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 2583,
    /// ECUC-INSTANCE-REFERENCE-VALUE
    EcucInstanceReferenceValue                                             = 6424,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 6405,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 4636,
    /// ECUC-LINKER-SYMBOL-DEF-CONDITIONAL
    EcucLinkerSymbolDefConditional                                         = 4483,
    /// ECUC-LINKER-SYMBOL-DEF-VARIANTS
    EcucLinkerSymbolDefVariants                                            = 2965,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 13,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF
    EcucModuleConfigurationValuesRef                                       = 5607,
    /// ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL
    EcucModuleConfigurationValuesRefConditional                            = 4242,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 1168,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 3816,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL
    EcucMultilineStringParamDefConditional                                 = 3251,
    /// ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS
    EcucMultilineStringParamDefVariants                                    = 2877,
    /// ECUC-MULTIPLICITY-CONFIGURATION-CLASS
    EcucMultiplicityConfigurationClass                                     = 1554,
    /// ECUC-NUMERICAL-PARAM-VALUE
    EcucNumericalParamValue                                                = 3606,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 6084,
    /// ECUC-QUERY
    EcucQuery                                                              = 6553,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 1342,
    /// ECUC-QUERY-REF
    EcucQueryRef                                                           = 3370,
    /// ECUC-QUERY-STRING-REF
    EcucQueryStringRef                                                     = 5999,
    /// ECUC-QUERYS
    EcucQuerys                                                             = 4342,
    /// ECUC-REF
    EcucRef                                                                = 1688,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 2935,
    /// ECUC-REFERENCE-VALUE
    EcucReferenceValue                                                     = 3767,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 5020,
    /// ECUC-STRING-PARAM-DEF-CONDITIONAL
    EcucStringParamDefConditional                                          = 1262,
    /// ECUC-STRING-PARAM-DEF-VARIANTS
    EcucStringParamDefVariants                                             = 160,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 1052,
    /// ECUC-TEXTUAL-PARAM-VALUE
    EcucTextualParamValue                                                  = 1608,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 3136,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 2256,
    /// ECUC-VALIDATION-CONDS
    EcucValidationConds                                                    = 4329,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 1837,
    /// ECUC-VALUE-CONFIGURATION-CLASS
    EcucValueConfigurationClass                                            = 3891,
    /// ECUC-VALUES
    EcucValues                                                             = 4255,
    /// EGRESS-PORT-REF
    EgressPortRef                                                          = 3244,
    /// EGRESS-PORT-REFS
    EgressPortRefs                                                         = 5156,
    /// EID
    Eid                                                                    = 1071,
    /// EID-RETRIEVAL
    EidRetrieval                                                           = 3531,
    /// EID-USE-MAC
    EidUseMac                                                              = 3289,
    /// ELEMENT
    Element                                                                = 1418,
    /// ELEMENT-BYTE-VALUE
    ElementByteValue                                                       = 3985,
    /// ELEMENT-CREATION
    ElementCreation                                                        = 2448,
    /// ELEMENT-GROUPS
    ElementGroups                                                          = 6223,
    /// ELEMENT-IN-IMPL-DATATYPE
    ElementInImplDatatype                                                  = 1265,
    /// ELEMENT-MAPPINGS
    ElementMappings                                                        = 6612,
    /// ELEMENT-POSITION
    ElementPosition                                                        = 387,
    /// ELEMENT-PROPSS
    ElementPropss                                                          = 1130,
    /// ELEMENT-REF
    ElementRef                                                             = 3575,
    /// ELEMENT-REFS
    ElementRefs                                                            = 856,
    /// ELEMENT-ROLE
    ElementRole                                                            = 4998,
    /// ELEMENTS
    Elements                                                               = 3341,
    /// ELLIPTIC-CURVE-REF
    EllipticCurveRef                                                       = 1036,
    /// ELLIPTIC-CURVE-REFS
    EllipticCurveRefs                                                      = 4104,
    /// EMPTY-SIGNAL
    EmptySignal                                                            = 4484,
    /// EMULATION-SUPPORTS
    EmulationSupports                                                      = 5014,
    /// ENABLE-ANYCAST
    EnableAnycast                                                          = 2938,
    /// ENABLE-CONDITION-GROUP-REF
    EnableConditionGroupRef                                                = 3664,
    /// ENABLE-CONDITION-REF
    EnableConditionRef                                                     = 388,
    /// ENABLE-CONDITIONS
    EnableConditions                                                       = 4824,
    /// ENABLE-DEACTIVATION
    EnableDeactivation                                                     = 2425,
    /// ENABLE-DISCOVERY-PROTECTION
    EnableDiscoveryProtection                                              = 2845,
    /// ENABLE-JOIN-ACCESS-CONTROL
    EnableJoinAccessControl                                                = 1475,
    /// ENABLE-LIVELINESS-PROTECTION
    EnableLivelinessProtection                                             = 4291,
    /// ENABLE-READ-ACCESS-CONTROL
    EnableReadAccessControl                                                = 683,
    /// ENABLE-TAKE-ADDRESS
    EnableTakeAddress                                                      = 672,
    /// ENABLE-UPDATE
    EnableUpdate                                                           = 5475,
    /// ENABLE-WRITE-ACCESS-CONTROL
    EnableWriteAccessControl                                               = 2186,
    /// ENABLES-LOG-TRACE
    EnablesLogTrace                                                        = 1696,
    /// ENCAPSULATED-ENTRY-REF
    EncapsulatedEntryRef                                                   = 4369,
    /// ENCODING
    Encoding                                                               = 6274,
    /// ENCRYPTION-REF
    EncryptionRef                                                          = 4803,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 1785,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 431,
    /// END-ADDRESS
    EndAddress                                                             = 5736,
    /// END-TO-END-CALL-RESPONSE-TIMEOUT
    EndToEndCallResponseTimeout                                            = 6443,
    /// END-TO-END-PROFILE
    EndToEndProfile                                                        = 6588,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 1612,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 454,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDUS
    EndToEndProtectionISignalIPdus                                         = 6067,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 535,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 5781,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPES
    EndToEndProtectionVariablePrototypes                                   = 2770,
    /// END-TO-END-PROTECTIONS
    EndToEndProtections                                                    = 33,
    /// END-TO-END-TRANSFORMATION-COM-SPEC-PROPS
    EndToEndTransformationComSpecProps                                     = 2689,
    /// END-TO-END-TRANSFORMATION-DESCRIPTION
    EndToEndTransformationDescription                                      = 1694,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS
    EndToEndTransformationISignalProps                                     = 4988,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    EndToEndTransformationISignalPropsConditional                          = 3405,
    /// END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    EndToEndTransformationISignalPropsVariants                             = 3248,
    /// ENDPOINT-CONFIGURATION-REF
    EndpointConfigurationRef                                               = 6010,
    /// ENDPOINTS
    Endpoints                                                              = 3611,
    /// ENGINEERING-OBJECT
    EngineeringObject                                                      = 664,
    /// ENGINEERING-OBJECTS
    EngineeringObjects                                                     = 6096,
    /// ENHANCED-MODE-API
    EnhancedModeApi                                                        = 4173,
    /// ENTER-SUSPEND-TO-RAM-ERROR-REF
    EnterSuspendToRamErrorRef                                              = 4062,
    /// ENTER-TIMEOUT-VALUE
    EnterTimeoutValue                                                      = 4974,
    /// ENTERED-MODE-REF
    EnteredModeRef                                                         = 289,
    /// ENTITY-STATUS-MAX-BYTE-FIELD-USE
    EntityStatusMaxByteFieldUse                                            = 3290,
    /// ENTITYS
    Entitys                                                                = 915,
    /// ENTRY
    Entry                                                                  = 3518,
    /// ENTRY-MODE-DECLARATION-REF
    EntryModeDeclarationRef                                                = 2650,
    /// ENTRY-REF
    EntryRef                                                               = 2811,
    /// ENTRY-TYPE
    EntryType                                                              = 4588,
    /// ENTRYS
    Entrys                                                                 = 2395,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 2722,
    /// ENUMERATOR-VALUE
    EnumeratorValue                                                        = 4125,
    /// ENUMS
    Enums                                                                  = 5555,
    /// ENVIRONMENT-CAPTURE-TO-REPORTING
    EnvironmentCaptureToReporting                                          = 6163,
    /// ENVIRONMENT-DATA-CAPTURE
    EnvironmentDataCapture                                                 = 4015,
    /// ENVIRONMENT-VARIABLES
    EnvironmentVariables                                                   = 98,
    /// ENVIRONMENTAL-CONDITION-REF
    EnvironmentalConditionRef                                              = 1456,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 4745,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 1951,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 6389,
    /// ERROR-CODE
    ErrorCode                                                              = 4953,
    /// ERROR-CONTEXT-REF
    ErrorContextRef                                                        = 3101,
    /// ERROR-CONTEXT-REFS
    ErrorContextRefs                                                       = 6427,
    /// ERROR-DOMAIN-REF
    ErrorDomainRef                                                         = 3953,
    /// ERROR-HANDLING
    ErrorHandling                                                          = 3181,
    /// ERROR-MAPPINGS
    ErrorMappings                                                          = 6098,
    /// ERROR-REACTION-POLICY
    ErrorReactionPolicy                                                    = 4814,
    /// ERROR-SIGNALING-ENABLED
    ErrorSignalingEnabled                                                  = 3933,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 1468,
    /// ESP-CIPHER-SUITE-NAME
    EspCipherSuiteName                                                     = 2673,
    /// ESP-CIPHER-SUITE-NAMES
    EspCipherSuiteNames                                                    = 3111,
    /// ESTIMATED-DURATION-OF-CAMPAIGN
    EstimatedDurationOfCampaign                                            = 5180,
    /// ESTIMATED-DURATION-OF-OPERATION
    EstimatedDurationOfOperation                                           = 5270,
    /// ESTIMATED-EXECUTION-TIME
    EstimatedExecutionTime                                                 = 1689,
    /// ETH-GLOBAL-TIME-DOMAIN-PROPS
    EthGlobalTimeDomainProps                                               = 15,
    /// ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT
    EthGlobalTimeManagedCouplingPort                                       = 4399,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 5349,
    /// ETH-IP-PROPS-REF
    EthIpPropsRef                                                          = 3344,
    /// ETH-IP-PROPS-REFS
    EthIpPropsRefs                                                         = 1379,
    /// ETH-SWITCH-PORT-GROUP-DERIVATION
    EthSwitchPortGroupDerivation                                           = 5184,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 3302,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 1962,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 709,
    /// ETH-TP-CONNECTION
    EthTpConnection                                                        = 349,
    /// ETHER-TYPE
    EtherType                                                              = 1054,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 6523,
    /// ETHERNET-CLUSTER-CONDITIONAL
    EthernetClusterConditional                                             = 1768,
    /// ETHERNET-CLUSTER-REF
    EthernetClusterRef                                                     = 3624,
    /// ETHERNET-CLUSTER-VARIANTS
    EthernetClusterVariants                                                = 814,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 3433,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 5314,
    /// ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL
    EthernetCommunicationControllerConditional                             = 6215,
    /// ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS
    EthernetCommunicationControllerVariants                                = 2170,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 1430,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 6034,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 3574,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 3620,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 1477,
    /// ETHERNET-PRIORITY-REGENERATIONS
    EthernetPriorityRegenerations                                          = 5534,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 5687,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 3277,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 1549,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING-REF
    EthernetRawDataStreamMappingRef                                        = 4284,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 2982,
    /// ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS
    EthernetTrafficClassAssignments                                        = 192,
    /// ETHERNET-VLAN-TRANSLATION-TABLE
    EthernetVlanTranslationTable                                           = 2979,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 2975,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 3048,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS
    EthernetWakeupSleepOnDatalineConfigs                                   = 734,
    /// ETS-AVAILABLE-BANDWIDTH-IN-PERCENT
    EtsAvailableBandwidthInPercent                                         = 3202,
    /// ETS-AVAILABLE-BANDWIDTH-IN-WEIGHT-VALUE
    EtsAvailableBandwidthInWeightValue                                     = 5923,
    /// EVALUATED-ELEMENT-REF
    EvaluatedElementRef                                                    = 5703,
    /// EVALUATED-ELEMENT-REFS
    EvaluatedElementRefs                                                   = 3065,
    /// EVALUATED-VARIANT-REF
    EvaluatedVariantRef                                                    = 2775,
    /// EVALUATED-VARIANT-REFS
    EvaluatedVariantRefs                                                   = 911,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 4673,
    /// EVALUATION-ID
    EvaluationId                                                           = 2810,
    /// EVENT-CLEAR-ALLOWED
    EventClearAllowed                                                      = 1859,
    /// EVENT-COMBINATION-REPORTING-BEHAVIOR
    EventCombinationReportingBehavior                                      = 2738,
    /// EVENT-CONTROLLED-TIMING
    EventControlledTiming                                                  = 5779,
    /// EVENT-DEFAULT-VALUE
    EventDefaultValue                                                      = 3362,
    /// EVENT-DEPLOYMENTS
    EventDeployments                                                       = 4937,
    /// EVENT-DISPLACEMENT-STRATEGY
    EventDisplacementStrategy                                              = 5963,
    /// EVENT-ELEMENT-MAPPINGS
    EventElementMappings                                                   = 5948,
    /// EVENT-ELEMENTS
    EventElements                                                          = 3927,
    /// EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD
    EventFailureCycleCounterThreshold                                      = 1864,
    /// EVENT-GROUP-CONTROL-TYPE
    EventGroupControlType                                                  = 279,
    /// EVENT-GROUP-ID
    EventGroupId                                                           = 447,
    /// EVENT-GROUP-IDENTIFIER
    EventGroupIdentifier                                                   = 3831,
    /// EVENT-GROUP-REF
    EventGroupRef                                                          = 4558,
    /// EVENT-GROUP-REFS
    EventGroupRefs                                                         = 4623,
    /// EVENT-GROUPS
    EventGroups                                                            = 4720,
    /// EVENT-HANDLER
    EventHandler                                                           = 5223,
    /// EVENT-HANDLER-REF
    EventHandlerRef                                                        = 5899,
    /// EVENT-HANDLER-REFS
    EventHandlerRefs                                                       = 2564,
    /// EVENT-HANDLERS
    EventHandlers                                                          = 2015,
    /// EVENT-ID
    EventId                                                                = 4842,
    /// EVENT-IREF
    EventIref                                                              = 2970,
    /// EVENT-KIND
    EventKind                                                              = 641,
    /// EVENT-MAPPING
    EventMapping                                                           = 1947,
    /// EVENT-MAPPINGS
    EventMappings                                                          = 1861,
    /// EVENT-MULTICAST-ADDRESSS
    EventMulticastAddresss                                                 = 1621,
    /// EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS
    EventMulticastSubscriptionAddresss                                     = 1737,
    /// EVENT-MULTICAST-UDP-PORT
    EventMulticastUdpPort                                                  = 6285,
    /// EVENT-NEEDS-REF
    EventNeedsRef                                                          = 1355,
    /// EVENT-OBD-READINESS-GROUP
    EventObdReadinessGroup                                                 = 4223,
    /// EVENT-OCCURRENCE-KIND
    EventOccurrenceKind                                                    = 177,
    /// EVENT-PROPSS
    EventPropss                                                            = 6306,
    /// EVENT-QOS-PROPSS
    EventQosPropss                                                         = 4097,
    /// EVENT-READINESS-GROUPS
    EventReadinessGroups                                                   = 54,
    /// EVENT-RECEPTION-DEFAULT-VALUE
    EventReceptionDefaultValue                                             = 5311,
    /// EVENT-REF
    EventRef                                                               = 5211,
    /// EVENT-REFS
    EventRefs                                                              = 1302,
    /// EVENT-SOURCE-REF
    EventSourceRef                                                         = 6318,
    /// EVENT-SYMBOL-NAME
    EventSymbolName                                                        = 2214,
    /// EVENT-TOPIC-ACCESS-RULE-REF
    EventTopicAccessRuleRef                                                = 4233,
    /// EVENT-TRIGGERING-REF
    EventTriggeringRef                                                     = 6238,
    /// EVENT-WINDOW-TIME
    EventWindowTime                                                        = 6197,
    /// EVENT-WINDOWS
    EventWindows                                                           = 3453,
    /// EVENTS
    Events                                                                 = 1730,
    /// EXCESS-BURST-SIZE
    ExcessBurstSize                                                        = 1773,
    /// EXCESS-INFORMATION-RATE
    ExcessInformationRate                                                  = 3374,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 3091,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 2640,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REF
    ExclusiveAreaNestingOrderRef                                           = 4972,
    /// EXCLUSIVE-AREA-NESTING-ORDER-REFS
    ExclusiveAreaNestingOrderRefs                                          = 6248,
    /// EXCLUSIVE-AREA-NESTING-ORDERS
    ExclusiveAreaNestingOrders                                             = 6236,
    /// EXCLUSIVE-AREA-POLICYS
    ExclusiveAreaPolicys                                                   = 1129,
    /// EXCLUSIVE-AREA-REF
    ExclusiveAreaRef                                                       = 6392,
    /// EXCLUSIVE-AREA-REF-CONDITIONAL
    ExclusiveAreaRefConditional                                            = 2492,
    /// EXCLUSIVE-AREA-REFS
    ExclusiveAreaRefs                                                      = 6259,
    /// EXCLUSIVE-AREAS
    ExclusiveAreas                                                         = 1090,
    /// EXEC-IN-USER-MODE
    ExecInUserMode                                                         = 5062,
    /// EXECUTABLE
    Executable                                                             = 1140,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 1930,
    /// EXECUTABLE-ENTITY-REF
    ExecutableEntityRef                                                    = 3883,
    /// EXECUTABLE-ENTITY-REFS
    ExecutableEntityRefs                                                   = 3668,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 247,
    /// EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS
    ExecutableLoggingImplementationProps                                   = 5425,
    /// EXECUTABLE-PROVIDED-PORT-IREF
    ExecutableProvidedPortIref                                             = 1733,
    /// EXECUTABLE-REF
    ExecutableRef                                                          = 1319,
    /// EXECUTABLE-REFS
    ExecutableRefs                                                         = 392,
    /// EXECUTABLE-REQUIRED-PORT-IREF
    ExecutableRequiredPortIref                                             = 5479,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 2231,
    /// EXECUTE-DESPITE-DATA-UNAVAILABILITY
    ExecuteDespiteDataUnavailability                                       = 4515,
    /// EXECUTION-CONTEXT
    ExecutionContext                                                       = 4491,
    /// EXECUTION-CONTEXT-REF
    ExecutionContextRef                                                    = 6220,
    /// EXECUTION-CONTEXT-REFS
    ExecutionContextRefs                                                   = 2678,
    /// EXECUTION-CONTEXTS
    ExecutionContexts                                                      = 939,
    /// EXECUTION-DEPENDENCY
    ExecutionDependency                                                    = 4783,
    /// EXECUTION-DEPENDENCYS
    ExecutionDependencys                                                   = 5772,
    /// EXECUTION-ERROR
    ExecutionError                                                         = 1369,
    /// EXECUTION-ERROR-REF
    ExecutionErrorRef                                                      = 5070,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 6158,
    /// EXECUTION-ORDER-CONSTRAINT-TYPE
    ExecutionOrderConstraintType                                           = 1682,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 5093,
    /// EXECUTION-TIME-TYPE
    ExecutionTimeType                                                      = 89,
    /// EXECUTION-TIMES
    ExecutionTimes                                                         = 259,
    /// EXIT-MODE-DECLARATION-REF
    ExitModeDeclarationRef                                                 = 3255,
    /// EXIT-TIMEOUT-VALUE
    ExitTimeoutValue                                                       = 2853,
    /// EXITED-MODE-REF
    ExitedModeRef                                                          = 2973,
    /// EXPECTED-ALIVE-CYCLE
    ExpectedAliveCycle                                                     = 5515,
    /// EXPECTED-ALIVE-INDICATIONS
    ExpectedAliveIndications                                               = 5244,
    /// EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE
    ExpectedAliveIndicationsInSupervisionCycle                             = 1358,
    /// EXPECTED-CALLBACKS
    ExpectedCallbacks                                                      = 3100,
    /// EXPECTED-ENTRYS
    ExpectedEntrys                                                         = 2685,
    /// EXPECTED-TX-TRIGGER
    ExpectedTxTrigger                                                      = 2421,
    /// EXPIRED-SUPERVISION-CYCLE-TOL
    ExpiredSupervisionCycleTol                                             = 2509,
    /// EXPIRED-SUPERVISION-CYCLES-TOLERANCE
    ExpiredSupervisionCyclesTolerance                                      = 28,
    /// EXPIRED-SUPERVISION-TOLERANCE
    ExpiredSupervisionTolerance                                            = 4653,
    /// EXPLICIT-CONGESTION-NOTIFICATION
    ExplicitCongestionNotification                                         = 5073,
    /// EXPLICIT-INTER-RUNNABLE-VARIABLES
    ExplicitInterRunnableVariables                                         = 1117,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REF
    ExplicitRptProfileSelectionRef                                         = 1306,
    /// EXPLICIT-RPT-PROFILE-SELECTION-REFS
    ExplicitRptProfileSelectionRefs                                        = 356,
    /// EXPLICIT-WAKEUP-CHANNELS
    ExplicitWakeupChannels                                                 = 1000,
    /// EXPLICIT-WAKEUP-PNCS
    ExplicitWakeupPncs                                                     = 4533,
    /// EXPORTABILITY
    Exportability                                                          = 3887,
    /// EXPRESSION
    Expression                                                             = 2259,
    /// EXPRESSION-REF
    ExpressionRef                                                          = 5523,
    /// EXT-HEADER-FILTER-LISTS
    ExtHeaderFilterLists                                                   = 4543,
    /// EXTENDED-ADDRESSING
    ExtendedAddressing                                                     = 2986,
    /// EXTENDED-DATA-RECORD-ELEMENT-REF
    ExtendedDataRecordElementRef                                           = 3417,
    /// EXTENDED-DATA-RECORDS
    ExtendedDataRecords                                                    = 6573,
    /// EXTENDS-META-CLASS
    ExtendsMetaClass                                                       = 2607,
    /// EXTERN-OFFSET-CORRECTION
    ExternOffsetCorrection                                                 = 6016,
    /// EXTERN-RATE-CORRECTION
    ExternRateCorrection                                                   = 2004,
    /// EXTERNAL-AUTHENTICATIONS
    ExternalAuthentications                                                = 3920,
    /// EXTERNAL-CLOCK-SYNCHRONISATION
    ExternalClockSynchronisation                                           = 4464,
    /// EXTERNAL-REPLACEMENT-REF
    ExternalReplacementRef                                                 = 3044,
    /// EXTERNAL-SYNC
    ExternalSync                                                           = 3678,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 123,
    /// EXTERNAL-TRIGGERING-POINT
    ExternalTriggeringPoint                                                = 4560,
    /// EXTERNAL-TRIGGERING-POINTS
    ExternalTriggeringPoints                                               = 1679,
    /// FACTOR-SI-TO-UNIT
    FactorSiToUnit                                                         = 3827,
    /// FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL
    FailedAliveSupervisionRefCycleTol                                      = 5300,
    /// FAILED-REFERENCE-CYCLES-TOLERANCE
    FailedReferenceCyclesTolerance                                         = 5368,
    /// FAILED-SUPERVISION-CYCLES-TOLERANCE
    FailedSupervisionCyclesTolerance                                       = 2832,
    /// FAILURE-MONITORING-REF
    FailureMonitoringRef                                                   = 648,
    /// FALL-BACK-INTERNAL
    FallBackInternal                                                       = 2682,
    /// FALSE-ACTION-LIST-REF
    FalseActionListRef                                                     = 1927,
    /// FDC-THRESHOLD-STORAGE-VALUE
    FdcThresholdStorageValue                                               = 455,
    /// FEATURE-IREF
    FeatureIref                                                            = 4004,
    /// FEATURE-MODEL-REF
    FeatureModelRef                                                        = 858,
    /// FEATURE-MODEL-REFS
    FeatureModelRefs                                                       = 1830,
    /// FEATURE-REF
    FeatureRef                                                             = 3017,
    /// FEATURE-REFS
    FeatureRefs                                                            = 3007,
    /// FIBEX-ELEMENT-REF
    FibexElementRef                                                        = 3580,
    /// FIBEX-ELEMENT-REF-CONDITIONAL
    FibexElementRefConditional                                             = 3428,
    /// FIBEX-ELEMENTS
    FibexElements                                                          = 5834,
    /// FIELD
    Field                                                                  = 5472,
    /// FIELD-DEPLOYMENTS
    FieldDeployments                                                       = 726,
    /// FIELD-ELEMENTS
    FieldElements                                                          = 3274,
    /// FIELD-GET-SET-QOS-PROPSS
    FieldGetSetQosPropss                                                   = 6459,
    /// FIELD-IREF
    FieldIref                                                              = 3573,
    /// FIELD-MAPPING
    FieldMapping                                                           = 5041,
    /// FIELD-MAPPINGS
    FieldMappings                                                          = 4443,
    /// FIELD-NOTIFIER-QOS-PROPSS
    FieldNotifierQosPropss                                                 = 1108,
    /// FIELD-NOTIFIER-REF
    FieldNotifierRef                                                       = 2416,
    /// FIELD-REF
    FieldRef                                                               = 1778,
    /// FIELD-REFS
    FieldRefs                                                              = 2815,
    /// FIELD-REPLY-TOPIC-NAME
    FieldReplyTopicName                                                    = 2762,
    /// FIELD-REQUEST-TOPIC-NAME
    FieldRequestTopicName                                                  = 5981,
    /// FIELD-SENDER-COM-SPEC
    FieldSenderComSpec                                                     = 938,
    /// FIELD-TOPICS-ACCESS-RULE-REF
    FieldTopicsAccessRuleRef                                               = 1471,
    /// FIELDS
    Fields                                                                 = 6596,
    /// FIFO-DEPTH
    FifoDepth                                                              = 2834,
    /// FIFO-RANGES
    FifoRanges                                                             = 5557,
    /// FIGURE
    Figure                                                                 = 4550,
    /// FIGURE-CAPTION
    FigureCaption                                                          = 43,
    /// FILE-ELEMENTS
    FileElements                                                           = 4009,
    /// FILE-INFO-COMMENT
    FileInfoComment                                                        = 3094,
    /// FILE-NAME
    FileName                                                               = 1222,
    /// FILE-PROXYS
    FileProxys                                                             = 560,
    /// FILE-REF
    FileRef                                                                = 4956,
    /// FILE-REFS
    FileRefs                                                               = 6023,
    /// FILE-STORAGE-REF
    FileStorageRef                                                         = 4628,
    /// FILE-TYPE
    FileType                                                               = 5903,
    /// FILE-TYPE-PATTERN
    FileTypePattern                                                        = 4451,
    /// FILES
    Files                                                                  = 311,
    /// FILTER
    Filter                                                                 = 6620,
    /// FILTER-ACTION-BLOCK-SOURCE
    FilterActionBlockSource                                                = 2627,
    /// FILTER-ACTION-DEST-PORT-MODIFICATION
    FilterActionDestPortModification                                       = 3615,
    /// FILTER-ACTION-DROP-FRAME
    FilterActionDropFrame                                                  = 2087,
    /// FILTER-ACTION-VLAN-MODIFICATION
    FilterActionVlanModification                                           = 746,
    /// FILTER-CHAINS
    FilterChains                                                           = 5804,
    /// FILTER-PRIORITY
    FilterPriority                                                         = 3549,
    /// FILTERING-DEBOUNCING
    FilteringDebouncing                                                    = 3278,
    /// FINAL-CHECKPOINT-REF
    FinalCheckpointRef                                                     = 96,
    /// FINAL-CHECKPOINT-REFS
    FinalCheckpointRefs                                                    = 557,
    /// FIRE-AND-FORGET
    FireAndForget                                                          = 6003,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 1513,
    /// FIRE-AND-FORGET-MAPPINGS
    FireAndForgetMappings                                                  = 5983,
    /// FIRE-AND-FORGET-METHOD-MAPPING
    FireAndForgetMethodMapping                                             = 1218,
    /// FIRE-AND-FORGET-METHOD-MAPPINGS
    FireAndForgetMethodMappings                                            = 3869,
    /// FIREWALL-REF
    FirewallRef                                                            = 6508,
    /// FIREWALL-RULE
    FirewallRule                                                           = 66,
    /// FIREWALL-RULE-PROPS
    FirewallRuleProps                                                      = 5347,
    /// FIREWALL-RULE-PROPSS
    FirewallRulePropss                                                     = 3174,
    /// FIREWALL-RULE-REF
    FirewallRuleRef                                                        = 3750,
    /// FIREWALL-RULE-REFS
    FirewallRuleRefs                                                       = 2540,
    /// FIREWALL-STATE-IREF
    FirewallStateIref                                                      = 4165,
    /// FIREWALL-STATE-IREFS
    FirewallStateIrefs                                                     = 880,
    /// FIREWALL-STATE-MACHINES
    FirewallStateMachines                                                  = 1960,
    /// FIREWALL-STATE-MODE-DECLARATION-REF
    FirewallStateModeDeclarationRef                                        = 2916,
    /// FIREWALL-STATE-MODE-DECLARATION-REFS
    FirewallStateModeDeclarationRefs                                       = 5710,
    /// FIREWALL-STATE-SWITCH-INTERFACE
    FirewallStateSwitchInterface                                           = 6258,
    /// FIRST-APPLICATION-ERROR-REF
    FirstApplicationErrorRef                                               = 3500,
    /// FIRST-DATA-PROTOTYPE-REF
    FirstDataPrototypeRef                                                  = 2630,
    /// FIRST-ELEMENT-INSTANCE-IREF
    FirstElementInstanceIref                                               = 2237,
    /// FIRST-ELEMENT-INSTANCE-IREFS
    FirstElementInstanceIrefs                                              = 5560,
    /// FIRST-ELEMENT-REF
    FirstElementRef                                                        = 3721,
    /// FIRST-ELEMENT-REFS
    FirstElementRefs                                                       = 5897,
    /// FIRST-ELEMENTS
    FirstElements                                                          = 1221,
    /// FIRST-MODE-GROUP-REF
    FirstModeGroupRef                                                      = 2585,
    /// FIRST-MODE-REF
    FirstModeRef                                                           = 40,
    /// FIRST-MODE-REFS
    FirstModeRefs                                                          = 4154,
    /// FIRST-OPERATION-REF
    FirstOperationRef                                                      = 451,
    /// FIRST-PHYSICAL-DIMENSION-REF
    FirstPhysicalDimensionRef                                              = 4041,
    /// FIRST-PORT-REF
    FirstPortRef                                                           = 1524,
    /// FIRST-TIMEOUT
    FirstTimeout                                                           = 4387,
    /// FIRST-TO-SECOND-DATA-TRANSFORMATION-REF
    FirstToSecondDataTransformationRef                                     = 267,
    /// FIRST-TRIGGER-REF
    FirstTriggerRef                                                        = 2537,
    /// FIRST-VALUE
    FirstValue                                                             = 1307,
    /// FIXED-LENGTH
    FixedLength                                                            = 1782,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 2983,
    /// FLAT-INSTANCE-REF
    FlatInstanceRef                                                        = 3994,
    /// FLAT-MAP
    FlatMap                                                                = 3396,
    /// FLAT-MAP-ENTRY-REF
    FlatMapEntryRef                                                        = 3171,
    /// FLAT-MAP-ENTRY-REFS
    FlatMapEntryRefs                                                       = 3213,
    /// FLAT-MAP-REF
    FlatMapRef                                                             = 5146,
    /// FLAT-OBJECT-REF
    FlatObjectRef                                                          = 5539,
    /// FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING
    FlexrayAbsolutelyScheduledTiming                                       = 2050,
    /// FLEXRAY-AR-TP-CHANNEL
    FlexrayArTpChannel                                                     = 186,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 6363,
    /// FLEXRAY-AR-TP-CONNECTION
    FlexrayArTpConnection                                                  = 6086,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 2355,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 1210,
    /// FLEXRAY-CLUSTER-CONDITIONAL
    FlexrayClusterConditional                                              = 3789,
    /// FLEXRAY-CLUSTER-VARIANTS
    FlexrayClusterVariants                                                 = 391,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 2956,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 6423,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL
    FlexrayCommunicationControllerConditional                              = 2001,
    /// FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS
    FlexrayCommunicationControllerVariants                                 = 1351,
    /// FLEXRAY-FIFO-CONFIGURATION
    FlexrayFifoConfiguration                                               = 5510,
    /// FLEXRAY-FIFO-RANGE
    FlexrayFifoRange                                                       = 85,
    /// FLEXRAY-FIFOS
    FlexrayFifos                                                           = 4374,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 4487,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 6156,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 5884,
    /// FLEXRAY-NM-CLUSTER-COUPLING
    FlexrayNmClusterCoupling                                               = 5996,
    /// FLEXRAY-NM-ECU
    FlexrayNmEcu                                                           = 3211,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 3763,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 3875,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 4948,
    /// FLEXRAY-TP-CONNECTION
    FlexrayTpConnection                                                    = 919,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 5624,
    /// FLEXRAY-TP-ECU
    FlexrayTpEcu                                                           = 6472,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 3438,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 5699,
    /// FLOAT
    Float                                                                  = 5587,
    /// FLOAT-VALUE-VARIATION-POINT
    FloatValueVariationPoint                                               = 6326,
    /// FLOW-CONTROL-PDU-REF
    FlowControlPduRef                                                      = 3825,
    /// FLOW-CONTROL-PDU-REFS
    FlowControlPduRefs                                                     = 336,
    /// FLOW-CONTROL-REF
    FlowControlRef                                                         = 1795,
    /// FLOW-LABEL
    FlowLabel                                                              = 3681,
    /// FLOW-METERING-REF
    FlowMeteringRef                                                        = 3475,
    /// FLOW-METERINGS
    FlowMeterings                                                          = 5521,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 6568,
    /// FM-ATTRIBUTE-VALUE
    FmAttributeValue                                                       = 151,
    /// FM-COND
    FmCond                                                                 = 4710,
    /// FM-FEATURE
    FmFeature                                                              = 6481,
    /// FM-FEATURE-DECOMPOSITION
    FmFeatureDecomposition                                                 = 546,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 4018,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 3502,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 124,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 271,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 3390,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 1095,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 2892,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 3984,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 4924,
    /// FM-SYSCOND
    FmSyscond                                                              = 829,
    /// FMI
    Fmi                                                                    = 4547,
    /// FOLLOW-UP-ACTION-REF
    FollowUpActionRef                                                      = 2152,
    /// FOLLOW-UP-ACTION-REFS
    FollowUpActionRefs                                                     = 2106,
    /// FOLLOW-UP-OFFSET
    FollowUpOffset                                                         = 2761,
    /// FOLLOW-UP-TIMEOUT-VALUE
    FollowUpTimeoutValue                                                   = 3867,
    /// FORBIDDEN-SIGNAL-PATH
    ForbiddenSignalPath                                                    = 3455,
    /// FOREIGN-MODEL-REFERENCE
    ForeignModelReference                                                  = 2108,
    /// FORMAL-BLUEPRINT-CONDITION
    FormalBlueprintCondition                                               = 3755,
    /// FORMAL-BLUEPRINT-GENERATOR
    FormalBlueprintGenerator                                               = 3230,
    /// FORMAT
    Format                                                                 = 5305,
    /// FORMULA
    Formula                                                                = 1561,
    /// FORMULA-CAPTION
    FormulaCaption                                                         = 3231,
    /// FR-CLUSTER-REF
    FrClusterRef                                                           = 395,
    /// FR-GLOBAL-TIME-DOMAIN-PROPS
    FrGlobalTimeDomainProps                                                = 3426,
    /// FRAGMENT
    Fragment                                                               = 5018,
    /// FRAGMENTATION-PROPS
    FragmentationProps                                                     = 2871,
    /// FRAME-LENGTH
    FrameLength                                                            = 6188,
    /// FRAME-MAPPING
    FrameMapping                                                           = 1865,
    /// FRAME-MAPPINGS
    FrameMappings                                                          = 5511,
    /// FRAME-PID
    FramePid                                                               = 5855,
    /// FRAME-PIDS
    FramePids                                                              = 2494,
    /// FRAME-PORT
    FramePort                                                              = 2082,
    /// FRAME-PORT-REF
    FramePortRef                                                           = 1920,
    /// FRAME-PORT-REFS
    FramePortRefs                                                          = 2493,
    /// FRAME-PREEMPTION-SUPPORT
    FramePreemptionSupport                                                 = 2261,
    /// FRAME-REF
    FrameRef                                                               = 168,
    /// FRAME-SYNC-ENABLED
    FrameSyncEnabled                                                       = 5530,
    /// FRAME-TRIGGERING-REF
    FrameTriggeringRef                                                     = 6321,
    /// FRAME-TRIGGERINGS
    FrameTriggerings                                                       = 5111,
    /// FREE-FORMAT
    FreeFormat                                                             = 6581,
    /// FREEZE-CURRENT-STATE
    FreezeCurrentState                                                     = 676,
    /// FREEZE-CURRENT-STATE-SUPPORTED
    FreezeCurrentStateSupported                                            = 2081,
    /// FREEZE-FRAME-CONTENT-REF
    FreezeFrameContentRef                                                  = 4767,
    /// FREEZE-FRAME-CONTENT-WWH-OBD-REF
    FreezeFrameContentWwhObdRef                                            = 5748,
    /// FREEZE-FRAME-REF
    FreezeFrameRef                                                         = 1546,
    /// FREEZE-FRAMES
    FreezeFrames                                                           = 2383,
    /// FRESHNESS-COUNTER-SYNC-ATTEMPTS
    FreshnessCounterSyncAttempts                                           = 3563,
    /// FRESHNESS-PROPS-REF
    FreshnessPropsRef                                                      = 1810,
    /// FRESHNESS-PROPSS
    FreshnessPropss                                                        = 6556,
    /// FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR
    FreshnessTimestampTimePeriodFactor                                     = 2370,
    /// FRESHNESS-VALUE-ID
    FreshnessValueId                                                       = 5405,
    /// FRESHNESS-VALUE-LENGTH
    FreshnessValueLength                                                   = 2263,
    /// FRESHNESS-VALUE-TX-LENGTH
    FreshnessValueTxLength                                                 = 3173,
    /// FROM-REF
    FromRef                                                                = 3956,
    /// FT
    Ft                                                                     = 6204,
    /// FULL-DUPLEX-ENABLED
    FullDuplexEnabled                                                      = 2297,
    /// FULLY-QUALIFIED-DOMAIN-NAME
    FullyQualifiedDomainName                                               = 2026,
    /// FUNCTION
    Function                                                               = 3837,
    /// FUNCTION-CLUSTER-AFFILIATION
    FunctionClusterAffiliation                                             = 505,
    /// FUNCTION-GROUP-ERROR-MAPPING
    FunctionGroupErrorMapping                                              = 693,
    /// FUNCTION-GROUP-ERROR-MAPPINGS
    FunctionGroupErrorMappings                                             = 3555,
    /// FUNCTION-GROUP-IREF
    FunctionGroupIref                                                      = 6190,
    /// FUNCTION-GROUP-IREFS
    FunctionGroupIrefs                                                     = 1466,
    /// FUNCTION-GROUP-MODE-IREF
    FunctionGroupModeIref                                                  = 1366,
    /// FUNCTION-GROUP-MODE-IREFS
    FunctionGroupModeIrefs                                                 = 1710,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 3969,
    /// FUNCTION-GROUP-PHM-STATE-REFERENCE
    FunctionGroupPhmStateReference                                         = 5737,
    /// FUNCTION-GROUP-PORT-MAPPING
    FunctionGroupPortMapping                                               = 2503,
    /// FUNCTION-GROUP-REF
    FunctionGroupRef                                                       = 2459,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 90,
    /// FUNCTION-GROUP-STATE-IREF
    FunctionGroupStateIref                                                 = 4347,
    /// FUNCTION-GROUP-STATE-IREFS
    FunctionGroupStateIrefs                                                = 5756,
    /// FUNCTION-GROUP-TRANSITION-REQUEST-FAILED-ERROR-REF
    FunctionGroupTransitionRequestFailedErrorRef                           = 6187,
    /// FUNCTION-GROUP-TYPE-REF
    FunctionGroupTypeRef                                                   = 2824,
    /// FUNCTION-GROUPS
    FunctionGroups                                                         = 263,
    /// FUNCTION-ID
    FunctionId                                                             = 3757,
    /// FUNCTION-IDENTIFIER-REF
    FunctionIdentifierRef                                                  = 3148,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 6138,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 2991,
    /// FUNCTION-INSTANCE
    FunctionInstance                                                       = 2470,
    /// FUNCTION-NAME
    FunctionName                                                           = 1652,
    /// FUNCTION-NAMES
    FunctionNames                                                          = 5887,
    /// FUNCTION-POINTER-SIGNATURE-REF
    FunctionPointerSignatureRef                                            = 201,
    /// FUNCTION-PROTOTYPE-EMITTER
    FunctionPrototypeEmitter                                               = 1154,
    /// FUNCTIONAL-CLUSTER-DESIGNS
    FunctionalClusterDesigns                                               = 705,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING
    FunctionalClusterInteractsWithDiagnosticEventMapping                   = 4034,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING
    FunctionalClusterInteractsWithPersistencyDeploymentMapping             = 1942,
    /// FUNCTIONAL-CLUSTER-REF
    FunctionalClusterRef                                                   = 3062,
    /// FUNCTIONAL-CLUSTER-REFS
    FunctionalClusterRefs                                                  = 5298,
    /// FUNCTIONAL-CLUSTER-TO-DLT-LOG-SINK-MAPPING
    FunctionalClusterToDltLogSinkMapping                                   = 4610,
    /// FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING
    FunctionalClusterToSecurityEventDefinitionMapping                      = 1409,
    /// FUNCTIONAL-REQUEST-REF
    FunctionalRequestRef                                                   = 2911,
    /// FUNCTIONAL-REQUEST-REFS
    FunctionalRequestRefs                                                  = 6229,
    /// FUNCTIONAL-UNIT
    FunctionalUnit                                                         = 5161,
    /// FUP-DATA-ID-LIST
    FupDataIdList                                                          = 5267,
    /// FUP-DATA-ID-LISTS
    FupDataIdLists                                                         = 2190,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 5661,
    /// GATEWAY
    Gateway                                                                = 3019,
    /// GATEWAYS
    Gateways                                                               = 1885,
    /// GENERAL-INACTIVITY-TIME
    GeneralInactivityTime                                                  = 536,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 3347,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 2472,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 2921,
    /// GENERAL-PURPOSE-TIMER-SERVICE-NEEDS
    GeneralPurposeTimerServiceNeeds                                        = 6368,
    /// GENERATED-ARTIFACTS
    GeneratedArtifacts                                                     = 594,
    /// GENERATOR
    Generator                                                              = 3066,
    /// GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION
    GenericDiagnosticTransportInstantiation                                = 3510,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 242,
    /// GENERIC-MATH
    GenericMath                                                            = 3719,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 5932,
    /// GENERIC-TP
    GenericTp                                                              = 2788,
    /// GENERIC-TP-CONNECTION
    GenericTpConnection                                                    = 6567,
    /// GET
    Get                                                                    = 6542,
    /// GETS
    Gets                                                                   = 2858,
    /// GETTER-CALL-REF
    GetterCallRef                                                          = 4275,
    /// GETTER-CALL-SIGNAL-REF
    GetterCallSignalRef                                                    = 3676,
    /// GETTER-OPERATION-REF
    GetterOperationRef                                                     = 3243,
    /// GETTER-REF
    GetterRef                                                              = 2281,
    /// GETTER-RETURN-REF
    GetterReturnRef                                                        = 5578,
    /// GETTER-RETURN-SIGNAL-REF
    GetterReturnSignalRef                                                  = 1573,
    /// GID
    Gid                                                                    = 4366,
    /// GID-INVALIDITY-PATTERN
    GidInvalidityPattern                                                   = 252,
    /// GLOBAL-ELEMENT
    GlobalElement                                                          = 5083,
    /// GLOBAL-ELEMENTS
    GlobalElements                                                         = 2362,
    /// GLOBAL-IN-PACKAGE-REF
    GlobalInPackageRef                                                     = 5336,
    /// GLOBAL-IN-PACKAGE-REFS
    GlobalInPackageRefs                                                    = 799,
    /// GLOBAL-KAY-PROPS-REF
    GlobalKayPropsRef                                                      = 1322,
    /// GLOBAL-RESOURCE-ID
    GlobalResourceId                                                       = 2201,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 3950,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 2701,
    /// GLOBAL-SUPERVISION-ENTITYS
    GlobalSupervisionEntitys                                               = 5334,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 3020,
    /// GLOBAL-SUPERVISIONS
    GlobalSupervisions                                                     = 4914,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 4100,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 710,
    /// GLOBAL-TIME-CORRECTION-PROPS
    GlobalTimeCorrectionProps                                              = 3599,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 588,
    /// GLOBAL-TIME-DOMAIN-PROPERTYS
    GlobalTimeDomainPropertys                                              = 5917,
    /// GLOBAL-TIME-DOMAIN-PROPS
    GlobalTimeDomainProps                                                  = 3532,
    /// GLOBAL-TIME-DOMAIN-REF
    GlobalTimeDomainRef                                                    = 6557,
    /// GLOBAL-TIME-DOMAIN-REF-CONDITIONAL
    GlobalTimeDomainRefConditional                                         = 2559,
    /// GLOBAL-TIME-DOMAINS
    GlobalTimeDomains                                                      = 3728,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 1275,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 6064,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 3058,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 953,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 5046,
    /// GLOBAL-TIME-MASTERS
    GlobalTimeMasters                                                      = 5043,
    /// GLOBAL-TIME-PDU-REF
    GlobalTimePduRef                                                       = 608,
    /// GLOBAL-TIME-PDU-TRIGGERING-REF
    GlobalTimePduTriggeringRef                                             = 3366,
    /// GLOBAL-TIME-PORT-ROLE
    GlobalTimePortRole                                                     = 1395,
    /// GLOBAL-TIME-PROPS
    GlobalTimeProps                                                        = 3170,
    /// GLOBAL-TIME-SUB-DOMAINS
    GlobalTimeSubDomains                                                   = 5790,
    /// GLOBAL-TIME-TX-PERIOD
    GlobalTimeTxPeriod                                                     = 2504,
    /// GOVERNANCE-REF
    GovernanceRef                                                          = 1986,
    /// GRANT-REF
    GrantRef                                                               = 6213,
    /// GRANT-REFS
    GrantRefs                                                              = 3778,
    /// GRAPHIC
    Graphic                                                                = 5978,
    /// GROUP-ID
    GroupId                                                                = 6356,
    /// GROUP-IDENTIFIER
    GroupIdentifier                                                        = 6344,
    /// GROUP-NUMBER
    GroupNumber                                                            = 959,
    /// GROUP-REF
    GroupRef                                                               = 1923,
    /// GROUP-REFS
    GroupRefs                                                              = 5022,
    /// GROUPED-ALIAS-EVENT-REF
    GroupedAliasEventRef                                                   = 6079,
    /// GROUPED-ALIAS-EVENT-REFS
    GroupedAliasEventRefs                                                  = 5321,
    /// HANDLE-DATA-STATUS
    HandleDataStatus                                                       = 4522,
    /// HANDLE-INVALID
    HandleInvalid                                                          = 1764,
    /// HANDLE-NEVER-RECEIVED
    HandleNeverReceived                                                    = 5387,
    /// HANDLE-OUT-OF-RANGE
    HandleOutOfRange                                                       = 1772,
    /// HANDLE-OUT-OF-RANGE-STATUS
    HandleOutOfRangeStatus                                                 = 5808,
    /// HANDLE-TERMINATION-AND-RESTART
    HandleTerminationAndRestart                                            = 210,
    /// HANDLE-TIMEOUT-TYPE
    HandleTimeoutType                                                      = 6182,
    /// HARDWARE-CONFIGURATION
    HardwareConfiguration                                                  = 552,
    /// HARDWARE-ELEMENT-REF
    HardwareElementRef                                                     = 113,
    /// HARDWARE-ELEMENT-REFS
    HardwareElementRefs                                                    = 401,
    /// HARDWARE-PLATFORM
    HardwarePlatform                                                       = 1365,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 5053,
    /// HAS-DYNAMIC-LENGTH
    HasDynamicLength                                                       = 896,
    /// HAS-GETTER
    HasGetter                                                              = 3073,
    /// HAS-INTERNAL-STATE
    HasInternalState                                                       = 6006,
    /// HAS-NOTIFIER
    HasNotifier                                                            = 1146,
    /// HAS-SETTER
    HasSetter                                                              = 1063,
    /// HEADER-FILE
    HeaderFile                                                             = 4671,
    /// HEADER-ID
    HeaderId                                                               = 3294,
    /// HEADER-ID-LONG-HEADER
    HeaderIdLongHeader                                                     = 5620,
    /// HEADER-ID-SHORT-HEADER
    HeaderIdShortHeader                                                    = 1150,
    /// HEADER-LENGTH
    HeaderLength                                                           = 1325,
    /// HEADER-TYPE
    HeaderType                                                             = 3141,
    /// HEALING-CYCLE-COUNTER-THRESHOLD
    HealingCycleCounterThreshold                                           = 4021,
    /// HEALING-CYCLE-REF
    HealingCycleRef                                                        = 4557,
    /// HEALTH-CHANNEL-ARGUMENT-REF
    HealthChannelArgumentRef                                               = 5337,
    /// HEALTH-CHANNEL-ARGUMENT-REFS
    HealthChannelArgumentRefs                                              = 1666,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 5593,
    /// HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS
    HealthChannelExternalReportedStatus                                    = 350,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 3647,
    /// HEALTH-CHANNEL-ID
    HealthChannelId                                                        = 4144,
    /// HEALTH-CHANNEL-IREF
    HealthChannelIref                                                      = 943,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 4060,
    /// HEALTH-CHANNELS
    HealthChannels                                                         = 3464,
    /// HEAP-USAGES
    HeapUsages                                                             = 5061,
    /// HISTORY
    History                                                                = 5785,
    /// HISTORY-KIND
    HistoryKind                                                            = 3963,
    /// HISTORY-ORDER-DEPTH
    HistoryOrderDepth                                                      = 4655,
    /// HOLD-OVER-TIME
    HoldOverTime                                                           = 1824,
    /// HOP-COUNT
    HopCount                                                               = 1300,
    /// HOP-LIMIT
    HopLimit                                                               = 3373,
    /// HOST-ID
    HostId                                                                 = 2053,
    /// HOST-REF
    HostRef                                                                = 6532,
    /// HOST-SYSTEM-REF
    HostSystemRef                                                          = 478,
    /// HTTP-ACCEPT-ENCODING
    HttpAcceptEncoding                                                     = 894,
    /// HTTP-TP
    HttpTp                                                                 = 5058,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 2014,
    /// HW-ATTRIBUTE-DEF-REF
    HwAttributeDefRef                                                      = 2373,
    /// HW-ATTRIBUTE-DEFS
    HwAttributeDefs                                                        = 4316,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 1712,
    /// HW-ATTRIBUTE-LITERALS
    HwAttributeLiterals                                                    = 292,
    /// HW-ATTRIBUTE-VALUE
    HwAttributeValue                                                       = 1751,
    /// HW-ATTRIBUTE-VALUES
    HwAttributeValues                                                      = 3361,
    /// HW-CATEGORY
    HwCategory                                                             = 4196,
    /// HW-CATEGORY-REF
    HwCategoryRef                                                          = 3843,
    /// HW-CATEGORY-REFS
    HwCategoryRefs                                                         = 4830,
    /// HW-COMMUNICATION-CONTROLLER-REF
    HwCommunicationControllerRef                                           = 6180,
    /// HW-COMMUNICATION-PORT-REF
    HwCommunicationPortRef                                                 = 4288,
    /// HW-ELEMENT
    HwElement                                                              = 432,
    /// HW-ELEMENT-CONNECTIONS
    HwElementConnections                                                   = 2984,
    /// HW-ELEMENT-CONNECTOR
    HwElementConnector                                                     = 327,
    /// HW-ELEMENT-REF
    HwElementRef                                                           = 6422,
    /// HW-ELEMENT-REF-CONDITIONAL
    HwElementRefConditional                                                = 4065,
    /// HW-ELEMENT-REFS
    HwElementRefs                                                          = 191,
    /// HW-PIN
    HwPin                                                                  = 6452,
    /// HW-PIN-CONNECTIONS
    HwPinConnections                                                       = 3200,
    /// HW-PIN-CONNECTOR
    HwPinConnector                                                         = 439,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 3917,
    /// HW-PIN-GROUP-CONNECTIONS
    HwPinGroupConnections                                                  = 6577,
    /// HW-PIN-GROUP-CONNECTOR
    HwPinGroupConnector                                                    = 1818,
    /// HW-PIN-GROUP-CONTENT
    HwPinGroupContent                                                      = 1914,
    /// HW-PIN-GROUP-REF
    HwPinGroupRef                                                          = 1877,
    /// HW-PIN-GROUP-REFS
    HwPinGroupRefs                                                         = 1390,
    /// HW-PIN-GROUPS
    HwPinGroups                                                            = 3077,
    /// HW-PIN-REF
    HwPinRef                                                               = 3520,
    /// HW-PIN-REFS
    HwPinRefs                                                              = 5838,
    /// HW-PORT-MAPPING
    HwPortMapping                                                          = 5880,
    /// HW-PORT-MAPPINGS
    HwPortMappings                                                         = 5427,
    /// HW-TYPE
    HwType                                                                 = 2,
    /// HW-TYPE-REF
    HwTypeRef                                                              = 6299,
    /// I-PDU-IDENTIFIER-TCP-REF
    IPduIdentifierTcpRef                                                   = 14,
    /// I-PDU-IDENTIFIER-TCP-REFS
    IPduIdentifierTcpRefs                                                  = 4361,
    /// I-PDU-IDENTIFIER-UDP-REF
    IPduIdentifierUdpRef                                                   = 2976,
    /// I-PDU-IDENTIFIER-UDP-REFS
    IPduIdentifierUdpRefs                                                  = 5439,
    /// I-PDU-IDENTIFIERS
    IPduIdentifiers                                                        = 396,
    /// I-PDU-MAPPING
    IPduMapping                                                            = 1274,
    /// I-PDU-MAPPINGS
    IPduMappings                                                           = 6367,
    /// I-PDU-PORT
    IPduPort                                                               = 4337,
    /// I-PDU-PORT-REF
    IPduPortRef                                                            = 4903,
    /// I-PDU-PORT-REFS
    IPduPortRefs                                                           = 2908,
    /// I-PDU-REF
    IPduRef                                                                = 6178,
    /// I-PDU-SIGNAL-PROCESSING
    IPduSignalProcessing                                                   = 3527,
    /// I-PDU-TIMING
    IPduTiming                                                             = 5927,
    /// I-PDU-TIMING-SPECIFICATIONS
    IPduTimingSpecifications                                               = 4970,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 2375,
    /// I-PDU-TRIGGERING-REF
    IPduTriggeringRef                                                      = 4568,
    /// I-PDU-TRIGGERING-REF-CONDITIONAL
    IPduTriggeringRefConditional                                           = 3976,
    /// I-PDU-TRIGGERINGS
    IPduTriggerings                                                        = 5943,
    /// I-PDUS
    IPdus                                                                  = 5255,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 748,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 3263,
    /// I-SIGNAL
    ISignal                                                                = 4601,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 646,
    /// I-SIGNAL-GROUP-REF
    ISignalGroupRef                                                        = 6381,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 638,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 600,
    /// I-SIGNAL-I-PDU-REF
    ISignalIPduRef                                                         = 107,
    /// I-SIGNAL-I-PDU-REF-CONDITIONAL
    ISignalIPduRefConditional                                              = 5209,
    /// I-SIGNAL-I-PDUS
    ISignalIPdus                                                           = 2343,
    /// I-SIGNAL-IN-I-PDU-REF
    ISignalInIPduRef                                                       = 2488,
    /// I-SIGNAL-MAPPING
    ISignalMapping                                                         = 5242,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 6070,
    /// I-SIGNAL-PORT-REF
    ISignalPortRef                                                         = 6272,
    /// I-SIGNAL-PORT-REFS
    ISignalPortRefs                                                        = 5704,
    /// I-SIGNAL-PORT-TO-DIAGNOSTIC-EVENT-MAPPING
    ISignalPortToDiagnosticEventMapping                                    = 6304,
    /// I-SIGNAL-PROPS
    ISignalProps                                                           = 6470,
    /// I-SIGNAL-REF
    ISignalRef                                                             = 4171,
    /// I-SIGNAL-REFS
    ISignalRefs                                                            = 2154,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 2272,
    /// I-SIGNAL-TO-I-PDU-MAPPINGS
    ISignalToIPduMappings                                                  = 6266,
    /// I-SIGNAL-TO-PDU-MAPPINGS
    ISignalToPduMappings                                                   = 1620,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 6574,
    /// I-SIGNAL-TRIGGERING-REF
    ISignalTriggeringRef                                                   = 4318,
    /// I-SIGNAL-TRIGGERING-REF-CONDITIONAL
    ISignalTriggeringRefConditional                                        = 341,
    /// I-SIGNAL-TRIGGERINGS
    ISignalTriggerings                                                     = 2812,
    /// I-SIGNAL-TYPE
    ISignalType                                                            = 2006,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 3166,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF
    IamRelevantTlsSecureComPropsRef                                        = 6621,
    /// IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS
    IamRelevantTlsSecureComPropsRefs                                       = 6035,
    /// ICMP-RULE
    IcmpRule                                                               = 412,
    /// ICMP-V-4-PROPS
    IcmpV4Props                                                            = 6192,
    /// ICMP-V-6-PROPS
    IcmpV6Props                                                            = 3754,
    /// ICV-FRESHNESS-VALUE-ID
    IcvFreshnessValueId                                                    = 1749,
    /// ICV-SECURE-COM-PROPSS
    IcvSecureComPropss                                                     = 3478,
    /// ICV-SECURED
    IcvSecured                                                             = 1854,
    /// ICV-VERIFICATION
    IcvVerification                                                        = 6198,
    /// ID
    Id                                                                     = 786,
    /// IDENT
    Ident                                                                  = 3258,
    /// IDENTICAL-MAPPING
    IdenticalMapping                                                       = 2388,
    /// IDENTIFIABLE-REF
    IdentifiableRef                                                        = 6590,
    /// IDENTIFIER
    Identifier                                                             = 1659,
    /// IDENTITIY-NUMBER
    IdentitiyNumber                                                        = 523,
    /// IDENTITY
    Identity                                                               = 3865,
    /// IDENTITY-CERTIFICATE-AUTHORITY-REF
    IdentityCertificateAuthorityRef                                        = 5842,
    /// IDENTITY-REF
    IdentityRef                                                            = 3786,
    /// IDLE-SLOPE
    IdleSlope                                                              = 673,
    /// IDS-COMMON-ELEMENT-REF
    IdsCommonElementRef                                                    = 677,
    /// IDS-COMMON-ELEMENT-REF-CONDITIONAL
    IdsCommonElementRefConditional                                         = 1346,
    /// IDS-DESIGN
    IdsDesign                                                              = 3437,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 0,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 3884,
    /// IDS-PLATFORM-INSTANTIATION-REF
    IdsPlatformInstantiationRef                                            = 4396,
    /// IDS-TIME-BASE-REF
    IdsTimeBaseRef                                                         = 1058,
    /// IDS-TIME-BASE-REFS
    IdsTimeBaseRefs                                                        = 5548,
    /// IDSM-CONTEXT-PROVIDER-INTERFACE
    IdsmContextProviderInterface                                           = 4545,
    /// IDSM-CONTEXT-PROVIDER-MAPPING
    IdsmContextProviderMapping                                             = 6444,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 6575,
    /// IDSM-INSTANCE-ID
    IdsmInstanceId                                                         = 5666,
    /// IDSM-INSTANCE-REF
    IdsmInstanceRef                                                        = 632,
    /// IDSM-INSTANCE-REF-CONDITIONAL
    IdsmInstanceRefConditional                                             = 3355,
    /// IDSM-INSTANCES
    IdsmInstances                                                          = 70,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 5726,
    /// IDSM-MODULE-INSTANTIATION-REF
    IdsmModuleInstantiationRef                                             = 1760,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 2783,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE
    IdsmQualifiedEventReceiverInterface                                    = 1439,
    /// IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING
    IdsmQualifiedEventReceiverMapping                                      = 3639,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 698,
    /// IDSM-RATE-LIMITATION-REF
    IdsmRateLimitationRef                                                  = 5970,
    /// IDSM-RATE-LIMITATION-REF-CONDITIONAL
    IdsmRateLimitationRefConditional                                       = 662,
    /// IDSM-REPORTING-MODE-PROVIDER-INTERFACE
    IdsmReportingModeProviderInterface                                     = 6636,
    /// IDSM-REPORTING-MODE-PROVIDER-MAPPING
    IdsmReportingModeProviderMapping                                       = 2763,
    /// IDSM-TIMESTAMP-PROVIDER-INTERFACE
    IdsmTimestampProviderInterface                                         = 2958,
    /// IDSM-TIMESTAMP-PROVIDER-MAPPING
    IdsmTimestampProviderMapping                                           = 6610,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 1141,
    /// IDSM-TRAFFIC-LIMITATION-REF
    IdsmTrafficLimitationRef                                               = 6242,
    /// IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL
    IdsmTrafficLimitationRefConditional                                    = 5414,
    /// IE
    Ie                                                                     = 3766,
    /// IEE-E-1722-TP-CONNECTION-REF
    IeeE1722TpConnectionRef                                                = 1093,
    /// IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusPartRawDataStreamConsumerMapping                         = 4541,
    /// IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722AcfBusRawDataStreamConsumerMapping                             = 6219,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE
    Ieee1722RawDataStreamConsumerInterface                                 = 158,
    /// IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING
    Ieee1722RawDataStreamConsumerMapping                                   = 2456,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE
    Ieee1722RawDataStreamProducerInterface                                 = 5829,
    /// IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING
    Ieee1722RawDataStreamProducerMapping                                   = 907,
    /// IEEE-1722-STREAM-REF
    Ieee1722StreamRef                                                      = 5203,
    /// IEEE-1722-TP
    Ieee1722Tp                                                             = 5457,
    /// IEEE-1722-TP-AAF-CONNECTION
    Ieee1722TpAafConnection                                                = 1321,
    /// IEEE-1722-TP-ACF-CAN
    Ieee1722TpAcfCan                                                       = 5178,
    /// IEEE-1722-TP-ACF-CAN-PART
    Ieee1722TpAcfCanPart                                                   = 3966,
    /// IEEE-1722-TP-ACF-CONNECTION
    Ieee1722TpAcfConnection                                                = 1061,
    /// IEEE-1722-TP-ACF-LIN
    Ieee1722TpAcfLin                                                       = 2891,
    /// IEEE-1722-TP-ACF-LIN-PART
    Ieee1722TpAcfLinPart                                                   = 4279,
    /// IEEE-1722-TP-CONFIG
    Ieee1722TpConfig                                                       = 1575,
    /// IEEE-1722-TP-CONNECTION-REF-CONDITIONAL
    Ieee1722TpConnectionRefConditional                                     = 3183,
    /// IEEE-1722-TP-CRF-CONNECTION
    Ieee1722TpCrfConnection                                                = 1633,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 5929,
    /// IEEE-1722-TP-IIDC-CONNECTION
    Ieee1722TpIidcConnection                                               = 3646,
    /// IEEE-1722-TP-RULE
    Ieee1722TpRule                                                         = 5874,
    /// IEEE-1722-TP-RVF-CONNECTION
    Ieee1722TpRvfConnection                                                = 2765,
    /// IGNORE-AFTER-TX
    IgnoreAfterTx                                                          = 2208,
    /// IGNORE-ORDER-ALLOWED
    IgnoreOrderAllowed                                                     = 1254,
    /// IIDC-CHANNEL
    IidcChannel                                                            = 4297,
    /// IIDC-DATA-BLOCK-SIZE
    IidcDataBlockSize                                                      = 4742,
    /// IIDC-FRACTION-NUMBER
    IidcFractionNumber                                                     = 2435,
    /// IIDC-SOURCE-PACKET-HEADER
    IidcSourcePacketHeader                                                 = 1337,
    /// IIDC-STREAM-FORMAT
    IidcStreamFormat                                                       = 4638,
    /// IIDC-SY
    IidcSy                                                                 = 4715,
    /// IIDC-T-CODE
    IidcTCode                                                              = 3436,
    /// IIDC-TAG
    IidcTag                                                                = 472,
    /// IKE-AUTHENTICATION-METHOD
    IkeAuthenticationMethod                                                = 6460,
    /// IKE-CIPHER-SUITE-NAME
    IkeCipherSuiteName                                                     = 2336,
    /// IKE-OVER-TIME
    IkeOverTime                                                            = 3997,
    /// IKE-RAND-TIME
    IkeRandTime                                                            = 803,
    /// IKE-REAUTH-TIME
    IkeReauthTime                                                          = 863,
    /// IKE-REKEY-TIME
    IkeRekeyTime                                                           = 570,
    /// IMMEDIATE-NV-DATA-STORAGE
    ImmediateNvDataStorage                                                 = 2145,
    /// IMMEDIATE-RESUME-TIME
    ImmediateResumeTime                                                    = 2649,
    /// IMPL-CONSTANT-REF
    ImplConstantRef                                                        = 2596,
    /// IMPL-INIT-VALUE
    ImplInitValue                                                          = 4794,
    /// IMPLEMENTATION-ARRAY-ELEMENT-REF
    ImplementationArrayElementRef                                          = 332,
    /// IMPLEMENTATION-CONFIG-CLASSES
    ImplementationConfigClasses                                            = 4691,
    /// IMPLEMENTATION-CONFIG-VARIANT
    ImplementationConfigVariant                                            = 1334,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 448,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 4612,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 3460,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF
    ImplementationDataTypeElementInAutosarDataPrototypeRef                 = 1756,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF
    ImplementationDataTypeElementInPortInterfaceRef                        = 6501,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF
    ImplementationDataTypeElementInSystemRef                               = 4670,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    ImplementationDataTypeElementRef                                       = 5788,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 5068,
    /// IMPLEMENTATION-DATA-TYPE-REF
    ImplementationDataTypeRef                                              = 5782,
    /// IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF
    ImplementationDataTypeSubElementRef                                    = 6375,
    /// IMPLEMENTATION-PROPSS
    ImplementationPropss                                                   = 5738,
    /// IMPLEMENTATION-RECORD-ELEMENT-REF
    ImplementationRecordElementRef                                         = 4083,
    /// IMPLEMENTATION-REF
    ImplementationRef                                                      = 5254,
    /// IMPLEMENTATION-REFS
    ImplementationRefs                                                     = 1104,
    /// IMPLEMENTED-ENTRY-REF
    ImplementedEntryRef                                                    = 380,
    /// IMPLEMENTED-ENTRYS
    ImplementedEntrys                                                      = 1823,
    /// IMPLEMENTED-IN-REF
    ImplementedInRef                                                       = 6297,
    /// IMPLEMENTS-LEGACY-STRING-SERIALIZATION
    ImplementsLegacyStringSerialization                                    = 290,
    /// IMPLEMENTS-SOMEIP-STRING-HANDLING
    ImplementsSomeipStringHandling                                         = 2202,
    /// IMPLICIT-DATA-ACCESS-IREF
    ImplicitDataAccessIref                                                 = 6486,
    /// IMPLICIT-DATA-ACCESS-IREFS
    ImplicitDataAccessIrefs                                                = 136,
    /// IMPLICIT-INTER-RUNNABLE-VARIABLES
    ImplicitInterRunnableVariables                                         = 2390,
    /// IMPLIED-OPERATION-REF
    ImpliedOperationRef                                                    = 3853,
    /// IMPLIED-OPERATION-REFS
    ImpliedOperationRefs                                                   = 533,
    /// IMPORTANCE
    Importance                                                             = 3393,
    /// IMPOSITION-TIME
    ImpositionTime                                                         = 3888,
    /// IMPOSITION-TIME-DEFINITION-GROUP
    ImpositionTimeDefinitionGroup                                          = 565,
    /// IMPOSITION-TIMES
    ImpositionTimes                                                        = 1657,
    /// IN-MEASUREMENT-SET
    InMeasurementSet                                                       = 4604,
    /// IN-PLACE
    InPlace                                                                = 3445,
    /// IN-SCOPE
    InScope                                                                = 4475,
    /// INCLUDE-REF
    IncludeRef                                                             = 987,
    /// INCLUDE-REFS
    IncludeRefs                                                            = 3105,
    /// INCLUDED-CERTIFICATE-REF
    IncludedCertificateRef                                                 = 1384,
    /// INCLUDED-CERTIFICATE-REFS
    IncludedCertificateRefs                                                = 1481,
    /// INCLUDED-DATA-TYPE-SET
    IncludedDataTypeSet                                                    = 3457,
    /// INCLUDED-DATA-TYPE-SETS
    IncludedDataTypeSets                                                   = 2378,
    /// INCLUDED-LIBRARY-REF
    IncludedLibraryRef                                                     = 2679,
    /// INCLUDED-LIBRARY-REFS
    IncludedLibraryRefs                                                    = 5881,
    /// INCLUDED-MODE-DECLARATION-GROUP-SET
    IncludedModeDeclarationGroupSet                                        = 3760,
    /// INCLUDED-MODE-DECLARATION-GROUP-SETS
    IncludedModeDeclarationGroupSets                                       = 1623,
    /// INCLUDED-VARIANT-REF
    IncludedVariantRef                                                     = 5171,
    /// INCLUDED-VARIANT-REFS
    IncludedVariantRefs                                                    = 1285,
    /// INDENT-SAMPLE
    IndentSample                                                           = 4580,
    /// INDEX
    Index                                                                  = 1822,
    /// INDEX-DATA-TYPE-REF
    IndexDataTypeRef                                                       = 360,
    /// INDEXED-ARRAY-ELEMENT
    IndexedArrayElement                                                    = 4520,
    /// INDICATIONS
    Indications                                                            = 2659,
    /// INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD
    IndicatorFailureCycleCounterThreshold                                  = 982,
    /// INDICATOR-REF
    IndicatorRef                                                           = 4551,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 2620,
    /// INDIRECT-API
    IndirectApi                                                            = 2512,
    /// INDUSTRY-GROUP
    IndustryGroup                                                          = 120,
    /// INFO-TYPE
    InfoType                                                               = 3601,
    /// INFO-TYPE-REF
    InfoTypeRef                                                            = 985,
    /// INFORMAL-FORMULA
    InformalFormula                                                        = 6609,
    /// INFRASTRUCTURE-SERVICES
    InfrastructureServices                                                 = 5547,
    /// INGRESS-PORT-REF
    IngressPortRef                                                         = 6018,
    /// INGRESS-PORT-REFS
    IngressPortRefs                                                        = 960,
    /// INGRESS-PRIORITY
    IngressPriority                                                        = 3238,
    /// INGRESS-VLAN-ID
    IngressVlanId                                                          = 4113,
    /// INHIBIT-SOURCES
    InhibitSources                                                         = 5401,
    /// INHIBITING-FID-REF
    InhibitingFidRef                                                       = 6626,
    /// INHIBITING-SECONDARY-FID-REF
    InhibitingSecondaryFidRef                                              = 5133,
    /// INHIBITING-SECONDARY-FID-REFS
    InhibitingSecondaryFidRefs                                             = 6017,
    /// INHIBITION-MASK
    InhibitionMask                                                         = 3889,
    /// INHIBITION-SOURCE-REF
    InhibitionSourceRef                                                    = 1067,
    /// INIT-EVENT
    InitEvent                                                              = 885,
    /// INIT-RESOURCE
    InitResource                                                           = 3496,
    /// INIT-VALUE
    InitValue                                                              = 4986,
    /// INIT-VALUES
    InitValues                                                             = 5655,
    /// INITIAL-CHECKPOINT-REF
    InitialCheckpointRef                                                   = 1033,
    /// INITIAL-CHECKPOINT-REFS
    InitialCheckpointRefs                                                  = 221,
    /// INITIAL-DELAY-MAX-VALUE
    InitialDelayMaxValue                                                   = 2549,
    /// INITIAL-DELAY-MIN-VALUE
    InitialDelayMinValue                                                   = 4600,
    /// INITIAL-DYNAMIC-PART
    InitialDynamicPart                                                     = 6225,
    /// INITIAL-EVENT-STATUS
    InitialEventStatus                                                     = 2415,
    /// INITIAL-FIND-BEHAVIOR
    InitialFindBehavior                                                    = 3859,
    /// INITIAL-INACTIVITY-TIME
    InitialInactivityTime                                                  = 181,
    /// INITIAL-MODE-REF
    InitialModeRef                                                         = 610,
    /// INITIAL-NAD
    InitialNad                                                             = 1246,
    /// INITIAL-OFFER-BEHAVIOR
    InitialOfferBehavior                                                   = 3588,
    /// INITIAL-REF-OFFSET
    InitialRefOffset                                                       = 58,
    /// INITIAL-REPETITIONS-BASE-DELAY
    InitialRepetitionsBaseDelay                                            = 2843,
    /// INITIAL-REPETITIONS-MAX
    InitialRepetitionsMax                                                  = 6607,
    /// INITIAL-STATUS
    InitialStatus                                                          = 3462,
    /// INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    InitialVehicleAnnouncementTime                                         = 2427,
    /// INITIALIZATION-VECTOR-LENGTH
    InitializationVectorLength                                             = 4570,
    /// INITIALIZED-PARAMETER-REF
    InitializedParameterRef                                                = 3982,
    /// INNER-GROUP-IREF
    InnerGroupIref                                                         = 1809,
    /// INNER-GROUP-IREFS
    InnerGroupIrefs                                                        = 6644,
    /// INNER-PORT-IREF
    InnerPortIref                                                          = 3410,
    /// INPLACE
    Inplace                                                                = 3056,
    /// INPUT
    Input                                                                  = 4341,
    /// INPUT-DATAS
    InputDatas                                                             = 4627,
    /// INPUT-VARIABLE-TYPE-REF
    InputVariableTypeRef                                                   = 4556,
    /// INSTALLATION-BEHAVIOR
    InstallationBehavior                                                   = 1304,
    /// INSTANCE-ID
    InstanceId                                                             = 869,
    /// INSTANCE-ID-PROPS
    InstanceIdProps                                                        = 1584,
    /// INSTANCE-IDENTIFIER
    InstanceIdentifier                                                     = 2923,
    /// INSTANCE-IDS
    InstanceIds                                                            = 612,
    /// INSTANCE-IN-MEMORY
    InstanceInMemory                                                       = 3061,
    /// INSTANCES
    Instances                                                              = 752,
    /// INSTANTIATION-DATA-DEF-PROPS
    InstantiationDataDefProps                                              = 4990,
    /// INSTANTIATION-DATA-DEF-PROPSS
    InstantiationDataDefPropss                                             = 4782,
    /// INSTANTIATION-RTE-EVENT-PROPSS
    InstantiationRteEventPropss                                            = 3241,
    /// INSTANTIATION-TIMING-EVENT-PROPS
    InstantiationTimingEventProps                                          = 3542,
    /// INTEGER-VALUE-VARIATION-POINT
    IntegerValueVariationPoint                                             = 933,
    /// INTENDED-FILENAME
    IntendedFilename                                                       = 3472,
    /// INTENDED-PARTIAL-INITIALIZATION-COUNT
    IntendedPartialInitializationCount                                     = 3937,
    /// INTENDED-TARGET-MACHINE-REF
    IntendedTargetMachineRef                                               = 5139,
    /// INTENDED-TARGET-MACHINE-REFS
    IntendedTargetMachineRefs                                              = 3187,
    /// INTER-MESSAGE-TIME
    InterMessageTime                                                       = 6498,
    /// INTERFACE-BLUEPRINT-MAPPING
    InterfaceBlueprintMapping                                              = 2099,
    /// INTERFACE-BLUEPRINT-MAPPINGS
    InterfaceBlueprintMappings                                             = 5015,
    /// INTERFACE-BLUEPRINT-REF
    InterfaceBlueprintRef                                                  = 428,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 1503,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 3132,
    /// INTERFACE-MAPPINGS
    InterfaceMappings                                                      = 6643,
    /// INTERFACE-REF
    InterfaceRef                                                           = 6603,
    /// INTERFACE-VERSION
    InterfaceVersion                                                       = 3756,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 1219,
    /// INTERNAL-BEHAVIORS
    InternalBehaviors                                                      = 4012,
    /// INTERNAL-CONSTRS
    InternalConstrs                                                        = 955,
    /// INTERNAL-PRIORITY-VALUE
    InternalPriorityValue                                                  = 4917,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 4881,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 4307,
    /// INTERNAL-TRIGGERING-POINT-POLICYS
    InternalTriggeringPointPolicys                                         = 4501,
    /// INTERNAL-TRIGGERING-POINTS
    InternalTriggeringPoints                                               = 4907,
    /// INTERNET-HEADER-LENGTH
    InternetHeaderLength                                                   = 3471,
    /// INTERPOLATION-ROUTINE
    InterpolationRoutine                                                   = 3513,
    /// INTERPOLATION-ROUTINE-MAPPING
    InterpolationRoutineMapping                                            = 758,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 3444,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REF
    InterpolationRoutineMappingSetRef                                      = 4103,
    /// INTERPOLATION-ROUTINE-MAPPING-SET-REFS
    InterpolationRoutineMappingSetRefs                                     = 2515,
    /// INTERPOLATION-ROUTINE-MAPPINGS
    InterpolationRoutineMappings                                           = 2719,
    /// INTERPOLATION-ROUTINE-REF
    InterpolationRoutineRef                                                = 6066,
    /// INTERPOLATION-ROUTINES
    InterpolationRoutines                                                  = 1464,
    /// INTERRUPT-CATEGORY
    InterruptCategory                                                      = 996,
    /// INTERRUPT-SOURCE
    InterruptSource                                                        = 613,
    /// INTERVAL-LENGTH
    IntervalLength                                                         = 1362,
    /// INTRODUCTION
    Introduction                                                           = 6345,
    /// INVALID-VALUE
    InvalidValue                                                           = 6355,
    /// INVALID-VALUE-BITFIELD-ERRORS-ENABLED
    InvalidValueBitfieldErrorsEnabled                                      = 2329,
    /// INVALIDATION-POLICY
    InvalidationPolicy                                                     = 3120,
    /// INVALIDATION-POLICYS
    InvalidationPolicys                                                    = 4161,
    /// INVERSE-PROTOCOL-VERSION
    InverseProtocolVersion                                                 = 1693,
    /// INVERT
    Invert                                                                 = 5728,
    /// INVERT-CONDITION
    InvertCondition                                                        = 3126,
    /// INVOCATION
    Invocation                                                             = 3701,
    /// IO-CONTROL-CLASS-REF
    IoControlClassRef                                                      = 1175,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATION
    IoHwAbstractionServerAnnotation                                        = 6341,
    /// IO-HW-ABSTRACTION-SERVER-ANNOTATIONS
    IoHwAbstractionServerAnnotations                                       = 3590,
    /// IP-ADDRESS
    IpAddress                                                              = 4991,
    /// IP-ADDRESS-KEEP-BEHAVIOR
    IpAddressKeepBehavior                                                  = 2793,
    /// IP-ADDRESS-PREFIX-LENGTH
    IpAddressPrefixLength                                                  = 4231,
    /// IP-IAM-AUTHENTIC-CONNECTION-PROPS
    IpIamAuthenticConnectionProps                                          = 542,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 2396,
    /// IP-PROTOCOL
    IpProtocol                                                             = 310,
    /// IP-SEC-CONFIG
    IpSecConfig                                                            = 3890,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 1388,
    /// IP-SEC-CONFIG-PROPS-REF
    IpSecConfigPropsRef                                                    = 103,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 4821,
    /// IP-SEC-RULE
    IpSecRule                                                              = 4977,
    /// IP-SEC-RULES
    IpSecRules                                                             = 323,
    /// IP-TP-RULE
    IpTpRule                                                               = 1478,
    /// IP-V-4-ADDRESS
    IpV4Address                                                            = 6263,
    /// IP-V-6-ADDRESS
    IpV6Address                                                            = 2239,
    /// IP-V-6-PATH-MTU-ENABLED
    IpV6PathMtuEnabled                                                     = 6360,
    /// IP-V-6-PATH-MTU-TIMEOUT
    IpV6PathMtuTimeout                                                     = 1812,
    /// IPV-4-ADDRESS
    Ipv4Address                                                            = 1800,
    /// IPV-4-ADDRESS-MASK
    Ipv4AddressMask                                                        = 386,
    /// IPV-4-ADDRESS-SOURCE
    Ipv4AddressSource                                                      = 630,
    /// IPV-4-CONFIGURATION
    Ipv4Configuration                                                      = 5568,
    /// IPV-4-DHCP-SERVER-CONFIGURATION
    Ipv4DhcpServerConfiguration                                            = 658,
    /// IPV-4-MULTICAST-IP-ADDRESS
    Ipv4MulticastIpAddress                                                 = 1912,
    /// IPV-4-PROPS
    Ipv4Props                                                              = 339,
    /// IPV-4-RULE
    Ipv4Rule                                                               = 1882,
    /// IPV-6-ADDRESS
    Ipv6Address                                                            = 5716,
    /// IPV-6-ADDRESS-MASK
    Ipv6AddressMask                                                        = 3697,
    /// IPV-6-ADDRESS-SOURCE
    Ipv6AddressSource                                                      = 808,
    /// IPV-6-CONFIGURATION
    Ipv6Configuration                                                      = 1835,
    /// IPV-6-DHCP-SERVER-CONFIGURATION
    Ipv6DhcpServerConfiguration                                            = 6450,
    /// IPV-6-MULTICAST-IP-ADDRESS
    Ipv6MulticastIpAddress                                                 = 4448,
    /// IPV-6-PROPS
    Ipv6Props                                                              = 1825,
    /// IPV-6-RULE
    Ipv6Rule                                                               = 400,
    /// IS-ACTIVATION-LINE-DEPENDENT
    IsActivationLineDependent                                              = 2212,
    /// IS-AUTO-VALUE
    IsAutoValue                                                            = 4380,
    /// IS-DEFAULT
    IsDefault                                                              = 2510,
    /// IS-DELTA-PACKAGE
    IsDeltaPackage                                                         = 816,
    /// IS-DYNAMIC-LENGTH-FIELD-SIZE
    IsDynamicLengthFieldSize                                               = 4719,
    /// IS-EVENT
    IsEvent                                                                = 453,
    /// IS-EXTERNAL
    IsExternal                                                             = 1741,
    /// IS-FUNC-CLUSTER-ENDPOINT
    IsFuncClusterEndpoint                                                  = 450,
    /// IS-FUNC-CLUSTER-ENDPOINT-REF
    IsFuncClusterEndpointRef                                               = 4455,
    /// IS-GLOBAL
    IsGlobal                                                               = 991,
    /// IS-MANDATORY
    IsMandatory                                                            = 4323,
    /// IS-OBD-RELEVANT
    IsObdRelevant                                                          = 261,
    /// IS-OPTIONAL
    IsOptional                                                             = 1328,
    /// IS-PIPELINING-PERMITTED
    IsPipeliningPermitted                                                  = 313,
    /// IS-PRIVATE
    IsPrivate                                                              = 4121,
    /// IS-REENTRANT
    IsReentrant                                                            = 2937,
    /// IS-REQUIRED
    IsRequired                                                             = 1031,
    /// IS-SERVICE
    IsService                                                              = 4105,
    /// IS-STRUCT-WITH-OPTIONAL-ELEMENT
    IsStructWithOptionalElement                                            = 6454,
    /// IS-SYNCHRONOUS
    IsSynchronous                                                          = 1381,
    /// IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER
    IsSystemWideGlobalTimeMaster                                           = 3449,
    /// IS-UNUSED
    IsUnused                                                               = 1231,
    /// IS-VARIADIC-TEMPLATE
    IsVariadicTemplate                                                     = 324,
    /// ISSUED-BY
    IssuedBy                                                               = 354,
    /// ISSUED-TRIGGERS
    IssuedTriggers                                                         = 218,
    /// IT-REF
    ItRef                                                                  = 607,
    /// ITEM
    Item                                                                   = 4178,
    /// ITEM-DEFINITIONS
    ItemDefinitions                                                        = 2234,
    /// ITEM-LABEL
    ItemLabel                                                              = 5174,
    /// ITEMS
    Items                                                                  = 6269,
    /// IUMPR-GROUP
    IumprGroup                                                             = 691,
    /// IUMPR-GROUP-IDENTIFIERS
    IumprGroupIdentifiers                                                  = 6618,
    /// IUMPR-REF
    IumprRef                                                               = 5219,
    /// IUMPR-REFS
    IumprRefs                                                              = 4791,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 5659,
    /// J-1939-CLUSTER-CONDITIONAL
    J1939ClusterConditional                                                = 5934,
    /// J-1939-CLUSTER-VARIANTS
    J1939ClusterVariants                                                   = 4892,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 1360,
    /// J-1939-CONTROLLER-APPLICATION-REF
    J1939ControllerApplicationRef                                          = 2749,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING
    J1939ControllerApplicationToJ1939NmNodeMapping                         = 1665,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS
    J1939ControllerApplicationToJ1939NmNodeMappings                        = 5310,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NODE-MAPPING
    J1939ControllerApplicationToJ1939NodeMapping                           = 72,
    /// J-1939-CONTROLLER-APPLICATION-TO-J-1939-NODE-MAPPINGS
    J1939ControllerApplicationToJ1939NodeMappings                          = 4271,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 1096,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 1158,
    /// J-1939-DTC-VALUE
    J1939DtcValue                                                          = 2384,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 4468,
    /// J-1939-NM-ECU
    J1939NmEcu                                                             = 1970,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 2062,
    /// J-1939-NM-NODE-REF
    J1939NmNodeRef                                                         = 762,
    /// J-1939-NODE
    J1939Node                                                              = 2968,
    /// J-1939-NODE-REF
    J1939NodeRef                                                           = 4727,
    /// J-1939-NODES
    J1939Nodes                                                             = 338,
    /// J-1939-PROTECTED-I-PDU
    J1939ProtectedIPdu                                                     = 5001,
    /// J-1939-REQUESTABLE
    J1939Requestable                                                       = 5297,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1156,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 792,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 5464,
    /// J-1939-SHARED-ADDRESS-CLUSTERS
    J1939SharedAddressClusters                                             = 1386,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 4457,
    /// J-1939-TP-CONNECTION
    J1939TpConnection                                                      = 2193,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 4698,
    /// J-1939-TP-PG
    J1939TpPg                                                              = 714,
    /// JITTER
    Jitter                                                                 = 3546,
    /// JOB-PRIMITIVE-REF
    JobPrimitiveRef                                                        = 6311,
    /// JOB-REF
    JobRef                                                                 = 3592,
    /// JOB-REQUIREMENTS
    JobRequirements                                                        = 4327,
    /// JUMP-TO-BOOT-LOADER
    JumpToBootLoader                                                       = 1234,
    /// K-INIT-RESOURCE
    KInitResource                                                          = 4047,
    /// K-RUN-RESOURCE
    KRunResource                                                           = 1183,
    /// KEEP-ALIVE-INTERVAL
    KeepAliveInterval                                                      = 6395,
    /// KEEP-ALIVE-PROBES-MAX
    KeepAliveProbesMax                                                     = 1492,
    /// KEEP-ALIVE-TIME
    KeepAliveTime                                                          = 2069,
    /// KEEP-ALIVES
    KeepAlives                                                             = 1105,
    /// KEY
    Key                                                                    = 929,
    /// KEY-EXCHANGE-AUTHENTICATION-REF
    KeyExchangeAuthenticationRef                                           = 4827,
    /// KEY-EXCHANGE-AUTHENTICATION-REFS
    KeyExchangeAuthenticationRefs                                          = 6063,
    /// KEY-EXCHANGE-REF
    KeyExchangeRef                                                         = 4709,
    /// KEY-EXCHANGE-REFS
    KeyExchangeRefs                                                        = 5271,
    /// KEY-GENERATION
    KeyGeneration                                                          = 1264,
    /// KEY-ID
    KeyId                                                                  = 3416,
    /// KEY-IDENTITY-REF
    KeyIdentityRef                                                         = 1995,
    /// KEY-REF
    KeyRef                                                                 = 5215,
    /// KEY-SERVER-PRIORITY
    KeyServerPriority                                                      = 1585,
    /// KEY-SIZE
    KeySize                                                                = 4902,
    /// KEY-SLOT-ALLOWED-MODIFICATION
    KeySlotAllowedModification                                             = 5637,
    /// KEY-SLOT-CONTENT-ALLOWED-USAGES
    KeySlotContentAllowedUsages                                            = 5957,
    /// KEY-SLOT-ID
    KeySlotId                                                              = 3271,
    /// KEY-SLOT-ONLY-ENABLED
    KeySlotOnlyEnabled                                                     = 5144,
    /// KEY-SLOT-REF
    KeySlotRef                                                             = 590,
    /// KEY-SLOT-USAGE
    KeySlotUsage                                                           = 1535,
    /// KEY-SLOT-USAGES
    KeySlotUsages                                                          = 5850,
    /// KEY-SLOT-USED-FOR-START-UP
    KeySlotUsedForStartUp                                                  = 5296,
    /// KEY-SLOT-USED-FOR-SYNC
    KeySlotUsedForSync                                                     = 1212,
    /// KEY-SLOTS
    KeySlots                                                               = 2059,
    /// KEY-STORAGE-TYPE
    KeyStorageType                                                         = 4286,
    /// KEY-VALUE-PAIRS
    KeyValuePairs                                                          = 4978,
    /// KEY-VALUE-STORAGE-REF
    KeyValueStorageRef                                                     = 1677,
    /// KEYWORD
    Keyword                                                                = 3569,
    /// KEYWORD-SET
    KeywordSet                                                             = 2712,
    /// KEYWORDS
    Keywords                                                               = 5494,
    /// KIND
    Kind                                                                   = 2909,
    /// L-1
    L1                                                                     = 817,
    /// L-10
    L10                                                                    = 4506,
    /// L-2
    L2                                                                     = 5389,
    /// L-4
    L4                                                                     = 3110,
    /// L-5
    L5                                                                     = 2482,
    /// L-GRAPHIC
    LGraphic                                                               = 1186,
    /// LABEL
    Label                                                                  = 6286,
    /// LABEL-1
    Label1                                                                 = 3958,
    /// LABELED-ITEM
    LabeledItem                                                            = 2766,
    /// LABELED-LIST
    LabeledList                                                            = 3291,
    /// LANGUAGE
    Language                                                               = 6239,
    /// LAST-EGRESS-SCHEDULER-REF
    LastEgressSchedulerRef                                                 = 4207,
    /// LATENCY-BUDGET
    LatencyBudget                                                          = 2363,
    /// LATENCY-BUDGET-DURATION
    LatencyBudgetDuration                                                  = 2317,
    /// LATENCY-CONSTRAINT-TYPE
    LatencyConstraintType                                                  = 299,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 4050,
    /// LATEST-TX
    LatestTx                                                               = 4811,
    /// LC-OBJECT-REF
    LcObjectRef                                                            = 6650,
    /// LC-STATE-REF
    LcStateRef                                                             = 1102,
    /// LC-STATES
    LcStates                                                               = 1267,
    /// LDAP-URL
    LdapUrl                                                                = 3714,
    /// LEAF-ELEMENT-IREF
    LeafElementIref                                                        = 3648,
    /// LEAVE-SUSPEND-TO-RAM-ERROR-REF
    LeaveSuspendToRamErrorRef                                              = 6027,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS
    LegislatedFreezeFrameContentUdsObds                                    = 2837,
    /// LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS
    LegislatedFreezeFrameContentWwhObds                                    = 2883,
    /// LEGISLATED-SOVD-FREEZE-FRAME-CONTENT-REF
    LegislatedSovdFreezeFrameContentRef                                    = 5345,
    /// LENGTH
    Length                                                                 = 377,
    /// LENGTH-CLIENT-ID
    LengthClientId                                                         = 2198,
    /// LENGTH-EXP
    LengthExp                                                              = 2631,
    /// LENGTH-SEQUENCE-COUNTER
    LengthSequenceCounter                                                  = 3682,
    /// LENGTH-VERIFICATION
    LengthVerification                                                     = 3907,
    /// LET-DATA-EXCHANGE-PARADIGM
    LetDataExchangeParadigm                                                = 61,
    /// LET-INTERVAL-REF
    LetIntervalRef                                                         = 1767,
    /// LET-INTERVAL-REFS
    LetIntervalRefs                                                        = 4462,
    /// LICENSE-REF
    LicenseRef                                                             = 21,
    /// LICENSE-REFS
    LicenseRefs                                                            = 2671,
    /// LIFE-CYCLE-INFO
    LifeCycleInfo                                                          = 6262,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 3223,
    /// LIFE-CYCLE-INFOS
    LifeCycleInfos                                                         = 5,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 916,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 5833,
    /// LIFESPAN
    Lifespan                                                               = 5598,
    /// LIFESPAN-DURATION
    LifespanDuration                                                       = 3717,
    /// LIMIT
    Limit                                                                  = 3345,
    /// LIMIT-KIND
    LimitKind                                                              = 5813,
    /// LIN-CHECKSUM
    LinChecksum                                                            = 5973,
    /// LIN-CLUSTER
    LinCluster                                                             = 5584,
    /// LIN-CLUSTER-CONDITIONAL
    LinClusterConditional                                                  = 5844,
    /// LIN-CLUSTER-VARIANTS
    LinClusterVariants                                                     = 2203,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 423,
    /// LIN-CONFIGURABLE-FRAME
    LinConfigurableFrame                                                   = 5812,
    /// LIN-CONFIGURABLE-FRAMES
    LinConfigurableFrames                                                  = 1512,
    /// LIN-ERROR-RESPONSE
    LinErrorResponse                                                       = 2939,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 5941,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 2920,
    /// LIN-IDENTIFIER
    LinIdentifier                                                          = 51,
    /// LIN-MASTER
    LinMaster                                                              = 6434,
    /// LIN-MASTER-CONDITIONAL
    LinMasterConditional                                                   = 3628,
    /// LIN-MASTER-VARIANTS
    LinMasterVariants                                                      = 4642,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 2216,
    /// LIN-ORDERED-CONFIGURABLE-FRAME
    LinOrderedConfigurableFrame                                            = 5262,
    /// LIN-ORDERED-CONFIGURABLE-FRAMES
    LinOrderedConfigurableFrames                                           = 574,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 865,
    /// LIN-PID-TO-CAN-ID-MAPPINGS
    LinPidToCanIdMappings                                                  = 2586,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 1044,
    /// LIN-SLAVE
    LinSlave                                                               = 1201,
    /// LIN-SLAVE-CONDITIONAL
    LinSlaveConditional                                                    = 237,
    /// LIN-SLAVE-CONFIG
    LinSlaveConfig                                                         = 3571,
    /// LIN-SLAVE-ECU-REF
    LinSlaveEcuRef                                                         = 6544,
    /// LIN-SLAVE-VARIANTS
    LinSlaveVariants                                                       = 62,
    /// LIN-SLAVES
    LinSlaves                                                              = 4587,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 888,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 4867,
    /// LIN-TP-CONNECTION
    LinTpConnection                                                        = 1949,
    /// LIN-TP-N-SDU-REF
    LinTpNSduRef                                                           = 4722,
    /// LIN-TP-NODE
    LinTpNode                                                              = 3052,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 4261,
    /// LIN-UNCONDITIONAL-FRAME-REF
    LinUnconditionalFrameRef                                               = 6173,
    /// LIN-UNCONDITIONAL-FRAME-REFS
    LinUnconditionalFrameRefs                                              = 3033,
    /// LINKER
    Linker                                                                 = 1683,
    /// LINKERS
    Linkers                                                                = 4000,
    /// LIST
    List                                                                   = 5478,
    /// LISTEN-NOISE
    ListenNoise                                                            = 727,
    /// LISTEN-TIMEOUT
    ListenTimeout                                                          = 1592,
    /// LITERAL-PREFIX
    LiteralPrefix                                                          = 3673,
    /// LITERALS
    Literals                                                               = 6317,
    /// LIVELINESS
    Liveliness                                                             = 5497,
    /// LIVELINESS-LEASE-DURATION
    LivelinessLeaseDuration                                                = 3683,
    /// LIVELINESS-PROTECTION-KIND
    LivelinessProtectionKind                                               = 6398,
    /// LIVENESS-KIND
    LivenessKind                                                           = 761,
    /// LOAD-BALANCING-PRIORITY
    LoadBalancingPriority                                                  = 2330,
    /// LOAD-BALANCING-WEIGHT
    LoadBalancingWeight                                                    = 5495,
    /// LOC-MEASUREMENT-SET
    LocMeasurementSet                                                      = 1109,
    /// LOCAL-CERTIFICATE-REF
    LocalCertificateRef                                                    = 3556,
    /// LOCAL-CERTIFICATE-REFS
    LocalCertificateRefs                                                   = 2041,
    /// LOCAL-COM-ACCESS-CONTROL-ENABLED
    LocalComAccessControlEnabled                                           = 5470,
    /// LOCAL-COMM-CONNECTOR-REF
    LocalCommConnectorRef                                                  = 2211,
    /// LOCAL-DEBUG-DATAS
    LocalDebugDatas                                                        = 5138,
    /// LOCAL-ENDPOINT-CONFIG
    LocalEndpointConfig                                                    = 6166,
    /// LOCAL-ID
    LocalId                                                                = 2302,
    /// LOCAL-IP-SEC-RULE-REF
    LocalIpSecRuleRef                                                      = 4883,
    /// LOCAL-IP-SEC-RULE-REFS
    LocalIpSecRuleRefs                                                     = 3761,
    /// LOCAL-NETWORK-ENDPOINT-REF
    LocalNetworkEndpointRef                                                = 976,
    /// LOCAL-PARAMETER-REF
    LocalParameterRef                                                      = 2192,
    /// LOCAL-PORT-RANGE-END
    LocalPortRangeEnd                                                      = 4147,
    /// LOCAL-PORT-RANGE-START
    LocalPortRangeStart                                                    = 5730,
    /// LOCAL-PORT-REF
    LocalPortRef                                                           = 367,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 4958,
    /// LOCAL-SUPERVISION-ENTITY-REF
    LocalSupervisionEntityRef                                              = 5951,
    /// LOCAL-SUPERVISION-ENTITY-REFS
    LocalSupervisionEntityRefs                                             = 6447,
    /// LOCAL-SUPERVISION-REF
    LocalSupervisionRef                                                    = 4672,
    /// LOCAL-SUPERVISION-REFS
    LocalSupervisionRefs                                                   = 671,
    /// LOCAL-SUPERVISIONS
    LocalSupervisions                                                      = 2748,
    /// LOCAL-TCP-PORT-REF
    LocalTcpPortRef                                                        = 4982,
    /// LOCAL-UDP-PORT-REF
    LocalUdpPortRef                                                        = 4023,
    /// LOCAL-UNICAST-ADDRESSES
    LocalUnicastAddresses                                                  = 5663,
    /// LOCAL-UNICAST-ADDRESSS
    LocalUnicastAddresss                                                   = 3356,
    /// LOCAL-VARIABLE-REF
    LocalVariableRef                                                       = 284,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 4180,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 2792,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 1023,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF
    LogAndTraceMessageCollectionSetRef                                     = 818,
    /// LOG-CHANNEL-ID
    LogChannelId                                                           = 3900,
    /// LOG-SINK-REF
    LogSinkRef                                                             = 2116,
    /// LOG-SINK-REFS
    LogSinkRefs                                                            = 1691,
    /// LOG-TRACE-DEFAULT-LOG-LEVEL
    LogTraceDefaultLogLevel                                                = 2280,
    /// LOG-TRACE-DEFAULT-LOG-THRESHOLD
    LogTraceDefaultLogThreshold                                            = 2825,
    /// LOG-TRACE-FILE-PATH
    LogTraceFilePath                                                       = 6369,
    /// LOG-TRACE-LOG-MODE
    LogTraceLogMode                                                        = 6252,
    /// LOG-TRACE-LOG-MODES
    LogTraceLogModes                                                       = 522,
    /// LOG-TRACE-PORT-ID
    LogTracePortId                                                         = 366,
    /// LOG-TRACE-PROCESS-DESC
    LogTraceProcessDesc                                                    = 5169,
    /// LOG-TRACE-PROCESS-ID
    LogTraceProcessId                                                      = 2646,
    /// LOGGING-BEHAVIOR
    LoggingBehavior                                                        = 4134,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 1630,
    /// LOGIC-ADDRESSS
    LogicAddresss                                                          = 4182,
    /// LOGICAL-ADDRESS
    LogicalAddress                                                         = 5962,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 5177,
    /// LOGICAL-EXPRESSION-ARGUMENT-REF
    LogicalExpressionArgumentRef                                           = 2786,
    /// LOGICAL-EXPRESSION-ARGUMENT-REFS
    LogicalExpressionArgumentRefs                                          = 4061,
    /// LOGICAL-EXPRESSIONS
    LogicalExpressions                                                     = 5595,
    /// LOGICAL-OPERATOR
    LogicalOperator                                                        = 5561,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 1681,
    /// LOGICAL-SUPERVISIONS
    LogicalSupervisions                                                    = 4985,
    /// LONG-NAME
    LongName                                                               = 839,
    /// LONG-NAME-1
    LongName1                                                              = 877,
    /// LOWER-BOUND
    LowerBound                                                             = 5795,
    /// LOWER-BOUNDARY
    LowerBoundary                                                          = 1862,
    /// LOWER-CAN-ID
    LowerCanId                                                             = 4391,
    /// LOWER-LIMIT
    LowerLimit                                                             = 4488,
    /// LOWER-MULTIPLICITY
    LowerMultiplicity                                                      = 4887,
    /// LOWER-REF
    LowerRef                                                               = 1721,
    /// LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING
    LtMessageCollectionToPortPrototypeMapping                              = 16,
    /// LUMINOUS-INTENSITY-EXP
    LuminousIntensityExp                                                   = 2552,
    /// M
    M                                                                      = 5528,
    /// MAC-ADDRESS
    MacAddress                                                             = 5691,
    /// MAC-ADDRESS-MASK
    MacAddressMask                                                         = 1586,
    /// MAC-ADDRESS-STREAM-ID
    MacAddressStreamId                                                     = 3006,
    /// MAC-ADDRESS-VLAN-ASSIGNMENTS
    MacAddressVlanAssignments                                              = 4438,
    /// MAC-ADDRESS-VLAN-MEMBERSHIP
    MacAddressVlanMembership                                               = 4148,
    /// MAC-LAYER-TYPE
    MacLayerType                                                           = 5912,
    /// MAC-MULTICAST-ADDRESS
    MacMulticastAddress                                                    = 3523,
    /// MAC-MULTICAST-ADDRESS-REF
    MacMulticastAddressRef                                                 = 3357,
    /// MAC-MULTICAST-ADDRESS-REFS
    MacMulticastAddressRefs                                                = 3063,
    /// MAC-MULTICAST-CONFIGURATION
    MacMulticastConfiguration                                              = 3979,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 4394,
    /// MAC-MULTICAST-GROUP-REF
    MacMulticastGroupRef                                                   = 3736,
    /// MAC-MULTICAST-GROUPS
    MacMulticastGroups                                                     = 4077,
    /// MAC-SEC-CIPHER-SUITE-CONFIG
    MacSecCipherSuiteConfig                                                = 2134,
    /// MAC-SEC-GLOBAL-KAY-PROPS
    MacSecGlobalKayProps                                                   = 5676,
    /// MAC-SEC-KAY-CONFIG
    MacSecKayConfig                                                        = 5117,
    /// MAC-SEC-KAY-PARTICIPANT
    MacSecKayParticipant                                                   = 809,
    /// MAC-SEC-PARTICIPANT-SET
    MacSecParticipantSet                                                   = 4452,
    /// MAC-SEC-PROPS
    MacSecProps                                                            = 6091,
    /// MAC-SEC-PROPSS
    MacSecPropss                                                           = 728,
    /// MAC-UNICAST-ADDRESS
    MacUnicastAddress                                                      = 6249,
    /// MACHINE
    Machine                                                                = 1011,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 2120,
    /// MACHINE-DESIGN-REF
    MachineDesignRef                                                       = 5742,
    /// MACHINE-ID
    MachineId                                                              = 6536,
    /// MACHINE-MODE-IREF
    MachineModeIref                                                        = 3983,
    /// MACHINE-MODE-IREFS
    MachineModeIrefs                                                       = 5754,
    /// MACHINE-MODE-MACHINES
    MachineModeMachines                                                    = 4211,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 2835,
    /// MACHINE-REF
    MachineRef                                                             = 5629,
    /// MACHINE-TIMING
    MachineTiming                                                          = 598,
    /// MACRO-INITIAL-OFFSET-A
    MacroInitialOffsetA                                                    = 3798,
    /// MACRO-INITIAL-OFFSET-B
    MacroInitialOffsetB                                                    = 1931,
    /// MACRO-PER-CYCLE
    MacroPerCycle                                                          = 20,
    /// MACROTICK-DURATION
    MacrotickDuration                                                      = 3783,
    /// MAIN-FUNCTION-PERIOD
    MainFunctionPeriod                                                     = 531,
    /// MAJOR-PROTOCOL-VERSION
    MajorProtocolVersion                                                   = 622,
    /// MAJOR-VERSION
    MajorVersion                                                           = 5976,
    /// MANAGED-COUPLING-PORTS
    ManagedCouplingPorts                                                   = 2131,
    /// MANAGED-MODE-GROUPS
    ManagedModeGroups                                                      = 5949,
    /// MANAGED-PHYSICAL-CHANNEL-REF
    ManagedPhysicalChannelRef                                              = 3293,
    /// MANAGED-PHYSICAL-CHANNEL-REFS
    ManagedPhysicalChannelRefs                                             = 2801,
    /// MANDATORY
    Mandatory                                                              = 2251,
    /// MANUFACTURER-CODE
    ManufacturerCode                                                       = 1312,
    /// MANUFACTURER-VALIDATION-ORDER-REF
    ManufacturerValidationOrderRef                                         = 3970,
    /// MANUFACTURER-VALIDATION-ORDER-REFS
    ManufacturerValidationOrderRefs                                        = 5902,
    /// MAP
    Map                                                                    = 3945,
    /// MAP-ELEMENT-TUPLES
    MapElementTuples                                                       = 6552,
    /// MAPPED-AP-DATA-ELEMENT-IREF
    MappedApDataElementIref                                                = 2455,
    /// MAPPED-BSW-SERVICE-DEPENDENCY-REF
    MappedBswServiceDependencyRef                                          = 500,
    /// MAPPED-DATA-ELEMENT-IREF
    MappedDataElementIref                                                  = 4949,
    /// MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF
    MappedFlatSwcServiceDependencyRef                                      = 4268,
    /// MAPPED-FUNCTION-REF
    MappedFunctionRef                                                      = 6602,
    /// MAPPED-OPERATION-IREF
    MappedOperationIref                                                    = 5731,
    /// MAPPED-SECURITY-EVENTS
    MappedSecurityEvents                                                   = 4298,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    MappedSwcServiceDependencyInExecutableIref                             = 3029,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    MappedSwcServiceDependencyInSystemIref                                 = 317,
    /// MAPPED-SWC-SERVICE-DEPENDENCY-IREF
    MappedSwcServiceDependencyIref                                         = 4809,
    /// MAPPING-CONSTRAINTS
    MappingConstraints                                                     = 1714,
    /// MAPPING-DIRECTION
    MappingDirection                                                       = 1490,
    /// MAPPING-REF
    MappingRef                                                             = 4529,
    /// MAPPING-SCOPE
    MappingScope                                                           = 2308,
    /// MAPPINGS
    Mappings                                                               = 2228,
    /// MASK
    Mask                                                                   = 1692,
    /// MASS-EXP
    MassExp                                                                = 6291,
    /// MASTER
    Master                                                                 = 111,
    /// MASTER-EVENT-REF
    MasterEventRef                                                         = 1019,
    /// MASTER-REF
    MasterRef                                                              = 5251,
    /// MASTER-TO-SLAVE-EVENT-MAPPINGS
    MasterToSlaveEventMappings                                             = 3163,
    /// MASTERED-TRIGGER-REF
    MasteredTriggerRef                                                     = 5423,
    /// MATCHING-CRITERION-REF
    MatchingCriterionRef                                                   = 5930,
    /// MATCHING-EGRESS-RULE-REF
    MatchingEgressRuleRef                                                  = 2039,
    /// MATCHING-EGRESS-RULE-REFS
    MatchingEgressRuleRefs                                                 = 5669,
    /// MATCHING-INGRESS-RULE-REF
    MatchingIngressRuleRef                                                 = 3292,
    /// MATCHING-INGRESS-RULE-REFS
    MatchingIngressRuleRefs                                                = 3326,
    /// MATCHING-RULE-REF
    MatchingRuleRef                                                        = 5569,
    /// MATCHING-RULE-REFS
    MatchingRuleRefs                                                       = 190,
    /// MAX
    Max                                                                    = 4453,
    /// MAX-ACTION-ITEM-DURATION
    MaxActionItemDuration                                                  = 5437,
    /// MAX-ACTION-LIST-DURATION
    MaxActionListDuration                                                  = 4198,
    /// MAX-ALIVE-CYCLE
    MaxAliveCycle                                                          = 3262,
    /// MAX-AR
    MaxAr                                                                  = 655,
    /// MAX-AS
    MaxAs                                                                  = 3552,
    /// MAX-AVAILABLE-PERSISTENCY-STORAGE-SPACE
    MaxAvailablePersistencyStorageSpace                                    = 2817,
    /// MAX-BASE-TYPE-SIZE
    MaxBaseTypeSize                                                        = 5777,
    /// MAX-BLOCK-SIZE
    MaxBlockSize                                                           = 3547,
    /// MAX-BS
    MaxBs                                                                  = 2628,
    /// MAX-BUFFER-REQUEST
    MaxBufferRequest                                                       = 2036,
    /// MAX-BUFFER-SIZE
    MaxBufferSize                                                          = 3659,
    /// MAX-BYTES-IN-INTERVAL
    MaxBytesInInterval                                                     = 6250,
    /// MAX-COMM-MODE
    MaxCommMode                                                            = 4760,
    /// MAX-CONVERSATIONS
    MaxConversations                                                       = 949,
    /// MAX-CYCLE-REPETITIONS
    MaxCycleRepetitions                                                    = 2464,
    /// MAX-CYCLES
    MaxCycles                                                              = 4566,
    /// MAX-DATA-LENGTH
    MaxDataLength                                                          = 4194,
    /// MAX-DEADLINE
    MaxDeadline                                                            = 4886,
    /// MAX-DELTA-COUNTER
    MaxDeltaCounter                                                        = 5505,
    /// MAX-DELTA-COUNTER-INIT
    MaxDeltaCounterInit                                                    = 3559,
    /// MAX-DESTINATION-PORT-NUMBER
    MaxDestinationPortNumber                                               = 4210,
    /// MAX-DID-TO-READ
    MaxDidToRead                                                           = 2621,
    /// MAX-DIFF
    MaxDiff                                                                = 1538,
    /// MAX-DURATION-EXCEEDED-ERROR-REF
    MaxDurationExceededErrorRef                                            = 5520,
    /// MAX-ERROR-STATE-INIT
    MaxErrorStateInit                                                      = 6401,
    /// MAX-ERROR-STATE-INVALID
    MaxErrorStateInvalid                                                   = 2179,
    /// MAX-ERROR-STATE-VALID
    MaxErrorStateValid                                                     = 276,
    /// MAX-EVENTS-IN-INTERVAL
    MaxEventsInInterval                                                    = 215,
    /// MAX-EXP-BS
    MaxExpBs                                                               = 3008,
    /// MAX-FC-WAIT
    MaxFcWait                                                              = 5814,
    /// MAX-FR-IF
    MaxFrIf                                                                = 1356,
    /// MAX-GRADIENT
    MaxGradient                                                            = 2616,
    /// MAX-HEADER-ID
    MaxHeaderId                                                            = 999,
    /// MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME
    MaxInitialVehicleAnnouncementTime                                      = 5252,
    /// MAX-INSTANCES
    MaxInstances                                                           = 4979,
    /// MAX-LENGTH
    MaxLength                                                              = 3541,
    /// MAX-MAJOR-VERSION
    MaxMajorVersion                                                        = 1759,
    /// MAX-MARGIN
    MaxMargin                                                              = 3075,
    /// MAX-MINOR-VERSION
    MaxMinorVersion                                                        = 494,
    /// MAX-NO-NEW-OR-REPEATED-DATA
    MaxNoNewOrRepeatedData                                                 = 6534,
    /// MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS
    MaxNumChangeOfDataIdentfierEvents                                      = 1647,
    /// MAX-NUM-COMPARISION-OF-VALUE-EVENTS
    MaxNumComparisionOfValueEvents                                         = 6530,
    /// MAX-NUMBER-FREEZE-FRAME-RECORDS
    MaxNumberFreezeFrameRecords                                            = 4150,
    /// MAX-NUMBER-OF-ALLOWED-UPDATES
    MaxNumberOfAllowedUpdates                                              = 1118,
    /// MAX-NUMBER-OF-CONNECTIONS
    MaxNumberOfConnections                                                 = 2158,
    /// MAX-NUMBER-OF-ELEMENTS
    MaxNumberOfElements                                                    = 3208,
    /// MAX-NUMBER-OF-EVENT-ENTRIES
    MaxNumberOfEventEntries                                                = 288,
    /// MAX-NUMBER-OF-FILES
    MaxNumberOfFiles                                                       = 603,
    /// MAX-NUMBER-OF-NPDU-PER-CYCLE
    MaxNumberOfNpduPerCycle                                                = 2990,
    /// MAX-NUMBER-OF-OCCURRENCES
    MaxNumberOfOccurrences                                                 = 644,
    /// MAX-NUMBER-OF-PARALLEL-TRANSFERS
    MaxNumberOfParallelTransfers                                           = 645,
    /// MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING
    MaxNumberOfRequestCorrectlyReceivedResponsePending                     = 1045,
    /// MAX-NUMBER-OF-RESP-PENDING-FRAMES
    MaxNumberOfRespPendingFrames                                           = 2445,
    /// MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS
    MaxNumberOfStoredDtcStatusChangedEvents                                = 3087,
    /// MAX-NUMBER-OF-TIME-QUANTA-PER-BIT
    MaxNumberOfTimeQuantaPerBit                                            = 4966,
    /// MAX-PERIODIC-DID-TO-READ
    MaxPeriodicDidToRead                                                   = 6515,
    /// MAX-PROGRESSION-MISMATCH-THRESHOLD
    MaxProgressionMismatchThreshold                                        = 5876,
    /// MAX-PWM-L
    MaxPwmL                                                                = 4496,
    /// MAX-PWM-O
    MaxPwmO                                                                = 2933,
    /// MAX-PWM-S
    MaxPwmS                                                                = 6525,
    /// MAX-REQUEST-BYTES
    MaxRequestBytes                                                        = 707,
    /// MAX-RESIDENCE-TIME
    MaxResidenceTime                                                       = 4311,
    /// MAX-RETRIES
    MaxRetries                                                             = 2589,
    /// MAX-RPT-EVENT-ID
    MaxRptEventId                                                          = 4717,
    /// MAX-RUN-CYCLE-LOOP
    MaxRunCycleLoop                                                        = 3450,
    /// MAX-SAMPLE-POINT
    MaxSamplePoint                                                         = 1646,
    /// MAX-SAMPLES
    MaxSamples                                                             = 6518,
    /// MAX-SAMPLES-PER-INSTANCE
    MaxSamplesPerInstance                                                  = 6082,
    /// MAX-SDU-SIZE
    MaxSduSize                                                             = 1039,
    /// MAX-SERVICE-POINT-ID
    MaxServicePointId                                                      = 389,
    /// MAX-SIZE-TO-FILL
    MaxSizeToFill                                                          = 3427,
    /// MAX-SLOTS
    MaxSlots                                                               = 1669,
    /// MAX-SLOTS-PER-CYCLE
    MaxSlotsPerCycle                                                       = 4817,
    /// MAX-SOURCE-ELEMENT
    MaxSourceElement                                                       = 1793,
    /// MAX-SOURCE-PORT-NUMBER
    MaxSourcePortNumber                                                    = 2594,
    /// MAX-SUPPORTED-DID-LENGTH
    MaxSupportedDidLength                                                  = 5166,
    /// MAX-SYNC-JUMP-WIDTH
    MaxSyncJumpWidth                                                       = 3609,
    /// MAX-TESTER-CONNECTIONS
    MaxTesterConnections                                                   = 1569,
    /// MAX-TRANSIT-TIME
    MaxTransitTime                                                         = 3952,
    /// MAX-TRCV-DELAY-COMPENSATION-OFFSET
    MaxTrcvDelayCompensationOffset                                         = 6519,
    /// MAX-VALUE
    MaxValue                                                               = 882,
    /// MAX-WITHOUT-CLOCK-CORRECTION-FATAL
    MaxWithoutClockCorrectionFatal                                         = 446,
    /// MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE
    MaxWithoutClockCorrectionPassive                                       = 1469,
    /// MAXIMUM
    Maximum                                                                = 3870,
    /// MAXIMUM-ALLOWED-SIZE
    MaximumAllowedSize                                                     = 790,
    /// MAXIMUM-DISTANCES
    MaximumDistances                                                       = 2063,
    /// MAXIMUM-DURATION-OF-CAMPAIGN
    MaximumDurationOfCampaign                                              = 593,
    /// MAXIMUM-DYNAMIC-PAYLOAD-LENGTH
    MaximumDynamicPayloadLength                                            = 4085,
    /// MAXIMUM-EXECUTION-TIME
    MaximumExecutionTime                                                   = 5752,
    /// MAXIMUM-INTENDED-BINDING-TIME
    MaximumIntendedBindingTime                                             = 3543,
    /// MAXIMUM-INTER-ARRIVAL-TIME
    MaximumInterArrivalTime                                                = 4633,
    /// MAXIMUM-KEY-LENGTH
    MaximumKeyLength                                                       = 3168,
    /// MAXIMUM-LENGTH
    MaximumLength                                                          = 963,
    /// MAXIMUM-MEMORY-CONSUMPTION
    MaximumMemoryConsumption                                               = 6331,
    /// MAXIMUM-MESSAGE-LENGTH
    MaximumMessageLength                                                   = 1341,
    /// MAXIMUM-NUMBER-OF-RETRIES
    MaximumNumberOfRetries                                                 = 1929,
    /// MAXIMUM-RECEIVE-BUFFER-LENGTH
    MaximumReceiveBufferLength                                             = 4230,
    /// MAXIMUM-RESIDENCE-TIME
    MaximumResidenceTime                                                   = 1463,
    /// MAXIMUM-SEGMENT-LENGTH
    MaximumSegmentLength                                                   = 4310,
    /// MAXIMUM-SEGMENT-LENGTH-REQUEST
    MaximumSegmentLengthRequest                                            = 5686,
    /// MAXIMUM-SEGMENT-LENGTH-RESPONSE
    MaximumSegmentLengthResponse                                           = 887,
    /// MAXIMUM-SELECTED-BINDING-TIME
    MaximumSelectedBindingTime                                             = 5705,
    /// MAXIMUM-SUPPORTED-UCM-MASTER-VERSION
    MaximumSupportedUcmMasterVersion                                       = 3634,
    /// MAXIMUM-SUPPORTED-UCM-VERSION
    MaximumSupportedUcmVersion                                             = 1470,
    /// MAXIMUM-TRANSMISSION-UNIT
    MaximumTransmissionUnit                                                = 4003,
    /// MAXIMUM-TRANSMIT-BUFFER-LENGTH
    MaximumTransmitBufferLength                                            = 6077,
    /// MAY-BE-UNCONNECTED
    MayBeUnconnected                                                       = 1872,
    /// MC-DATA-ACCESS-DETAILS
    McDataAccessDetails                                                    = 4555,
    /// MC-DATA-ASSIGNMENTS
    McDataAssignments                                                      = 5634,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 4815,
    /// MC-DATA-INSTANCE-REF
    McDataInstanceRef                                                      = 2875,
    /// MC-DATA-INSTANCE-REFS
    McDataInstanceRefs                                                     = 2870,
    /// MC-DATA-INSTANCE-VAR-REF
    McDataInstanceVarRef                                                   = 4201,
    /// MC-FUNCTION
    McFunction                                                             = 280,
    /// MC-FUNCTION-DATA-REF-SET-CONDITIONAL
    McFunctionDataRefSetConditional                                        = 1771,
    /// MC-FUNCTION-DATA-REF-SET-VARIANTS
    McFunctionDataRefSetVariants                                           = 731,
    /// MC-FUNCTION-REF
    McFunctionRef                                                          = 4643,
    /// MC-FUNCTION-REFS
    McFunctionRefs                                                         = 2000,
    /// MC-GROUP
    McGroup                                                                = 6639,
    /// MC-GROUP-DATA-REF-SET-CONDITIONAL
    McGroupDataRefSetConditional                                           = 5692,
    /// MC-GROUP-DATA-REF-SET-VARIANTS
    McGroupDataRefSetVariants                                              = 5029,
    /// MC-PARAMETER-ELEMENT-GROUP
    McParameterElementGroup                                                = 5210,
    /// MC-PARAMETER-INSTANCES
    McParameterInstances                                                   = 3060,
    /// MC-SUPPORT
    McSupport                                                              = 2270,
    /// MC-SW-EMULATION-METHOD-SUPPORT
    McSwEmulationMethodSupport                                             = 3463,
    /// MC-VARIABLE-INSTANCES
    McVariableInstances                                                    = 1583,
    /// MCD-IDENTIFIER
    McdIdentifier                                                          = 55,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REF
    MeasurableSystemConstantValuesRef                                      = 5806,
    /// MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS
    MeasurableSystemConstantValuesRefs                                     = 5677,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 3499,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 135,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 5407,
    /// MEM-ALIGNMENT
    MemAlignment                                                           = 6473,
    /// MEM-CLASS-SYMBOL
    MemClassSymbol                                                         = 5564,
    /// MEM-USAGE
    MemUsage                                                               = 2505,
    /// MEMORY-ALLOCATION-KEYWORD-POLICY
    MemoryAllocationKeywordPolicy                                          = 4676,
    /// MEMORY-CONSUMPTION
    MemoryConsumption                                                      = 2121,
    /// MEMORY-DESTINATION-REF
    MemoryDestinationRef                                                   = 2236,
    /// MEMORY-DESTINATION-REFS
    MemoryDestinationRefs                                                  = 3395,
    /// MEMORY-ENTRY-STORAGE-TRIGGER
    MemoryEntryStorageTrigger                                              = 5799,
    /// MEMORY-HIGH-ADDRESS
    MemoryHighAddress                                                      = 1049,
    /// MEMORY-HIGH-ADDRESS-LABEL
    MemoryHighAddressLabel                                                 = 2603,
    /// MEMORY-ID
    MemoryId                                                               = 6152,
    /// MEMORY-LOW-ADDRESS
    MemoryLowAddress                                                       = 6049,
    /// MEMORY-LOW-ADDRESS-LABEL
    MemoryLowAddressLabel                                                  = 5697,
    /// MEMORY-RANGE-REF
    MemoryRangeRef                                                         = 4424,
    /// MEMORY-RANGE-REFS
    MemoryRangeRefs                                                        = 5295,
    /// MEMORY-SECTION
    MemorySection                                                          = 1040,
    /// MEMORY-SECTION-LOCATION
    MemorySectionLocation                                                  = 1911,
    /// MEMORY-SECTION-LOCATIONS
    MemorySectionLocations                                                 = 1185,
    /// MEMORY-SECTIONS
    MemorySections                                                         = 1734,
    /// MEMORY-USAGE
    MemoryUsage                                                            = 5140,
    /// MEMORY-USAGES
    MemoryUsages                                                           = 2027,
    /// MESSAGE-COMPLIANCE
    MessageCompliance                                                      = 2197,
    /// MESSAGE-ID
    MessageId                                                              = 1021,
    /// MESSAGE-LINE-NUMBER
    MessageLineNumber                                                      = 4915,
    /// MESSAGE-LINK-LENGTH
    MessageLinkLength                                                      = 6287,
    /// MESSAGE-LINK-POSITION
    MessageLinkPosition                                                    = 3134,
    /// MESSAGE-SOURCE-FILE
    MessageSourceFile                                                      = 1196,
    /// MESSAGE-TYPE
    MessageType                                                            = 2368,
    /// MESSAGE-TYPE-INFO
    MessageTypeInfo                                                        = 3696,
    /// META-DATA-FIELDS
    MetaDataFields                                                         = 2057,
    /// META-DATA-ITEM
    MetaDataItem                                                           = 5942,
    /// META-DATA-ITEM-SET
    MetaDataItemSet                                                        = 2582,
    /// META-DATA-ITEM-SETS
    MetaDataItemSets                                                       = 824,
    /// META-DATA-ITEM-TYPE
    MetaDataItemType                                                       = 4841,
    /// META-DATA-ITEMS
    MetaDataItems                                                          = 3626,
    /// META-DATA-LENGTH
    MetaDataLength                                                         = 6167,
    /// METADATA-PROTECTION-KIND
    MetadataProtectionKind                                                 = 4651,
    /// METHOD-ACTIVATION-ROUTING-GROUPS
    MethodActivationRoutingGroups                                          = 5886,
    /// METHOD-CALL-REF
    MethodCallRef                                                          = 2900,
    /// METHOD-CALL-REFS
    MethodCallRefs                                                         = 3897,
    /// METHOD-DEPLOYMENTS
    MethodDeployments                                                      = 745,
    /// METHOD-ID
    MethodId                                                               = 4939,
    /// METHOD-IREF
    MethodIref                                                             = 3313,
    /// METHOD-MAPPING
    MethodMapping                                                          = 4189,
    /// METHOD-MAPPINGS
    MethodMappings                                                         = 5278,
    /// METHOD-QOS-PROPSS
    MethodQosPropss                                                        = 4500,
    /// METHOD-REF
    MethodRef                                                              = 393,
    /// METHOD-REFS
    MethodRefs                                                             = 6055,
    /// METHOD-REPLY-TOPIC-NAME
    MethodReplyTopicName                                                   = 3706,
    /// METHOD-REQUEST-PROPSS
    MethodRequestPropss                                                    = 5524,
    /// METHOD-REQUEST-TOPIC-NAME
    MethodRequestTopicName                                                 = 6185,
    /// METHOD-RESPONSE-PROPSS
    MethodResponsePropss                                                   = 1389,
    /// METHOD-RETURN-REF
    MethodReturnRef                                                        = 4832,
    /// METHOD-RETURN-REFS
    MethodReturnRefs                                                       = 4184,
    /// METHOD-TOPICS-ACCESS-RULE-REF
    MethodTopicsAccessRuleRef                                              = 3980,
    /// METHODS
    Methods                                                                = 4272,
    /// MICRO-INITIAL-OFFSET-A
    MicroInitialOffsetA                                                    = 3931,
    /// MICRO-INITIAL-OFFSET-B
    MicroInitialOffsetB                                                    = 3568,
    /// MICRO-PER-CYCLE
    MicroPerCycle                                                          = 1900,
    /// MICROTICK-DURATION
    MicrotickDuration                                                      = 4111,
    /// MIN
    Min                                                                    = 5290,
    /// MIN-ALIVE-CYCLE
    MinAliveCycle                                                          = 889,
    /// MIN-DATA-LENGTH
    MinDataLength                                                          = 1339,
    /// MIN-DEADLINE
    MinDeadline                                                            = 6101,
    /// MIN-DESTINATION-PORT-NUMBER
    MinDestinationPortNumber                                               = 1529,
    /// MIN-HEADER-ID
    MinHeaderId                                                            = 1028,
    /// MIN-LENGTH
    MinLength                                                              = 5459,
    /// MIN-MAJOR-VERSION
    MinMajorVersion                                                        = 179,
    /// MIN-MARGIN
    MinMargin                                                              = 4005,
    /// MIN-MINOR-VERSION
    MinMinorVersion                                                        = 2473,
    /// MIN-NUMBER-OF-ELEMENTS
    MinNumberOfElements                                                    = 1775,
    /// MIN-NUMBER-OF-OCCURRENCES
    MinNumberOfOccurrences                                                 = 3394,
    /// MIN-NUMBER-OF-TIME-QUANTA-PER-BIT
    MinNumberOfTimeQuantaPerBit                                            = 404,
    /// MIN-OK-STATE-INIT
    MinOkStateInit                                                         = 1753,
    /// MIN-OK-STATE-INVALID
    MinOkStateInvalid                                                      = 6510,
    /// MIN-OK-STATE-VALID
    MinOkStateValid                                                        = 4011,
    /// MIN-PWM-L
    MinPwmL                                                                = 684,
    /// MIN-PWM-O
    MinPwmO                                                                = 3406,
    /// MIN-PWM-S
    MinPwmS                                                                = 5134,
    /// MIN-RPT-EVENT-ID
    MinRptEventId                                                          = 495,
    /// MIN-SAMPLE-POINT
    MinSamplePoint                                                         = 6024,
    /// MIN-SERVICE-POINT-ID
    MinServicePointId                                                      = 2517,
    /// MIN-SOURCE-PORT-NUMBER
    MinSourcePortNumber                                                    = 2177,
    /// MIN-SYNC-JUMP-WIDTH
    MinSyncJumpWidth                                                       = 5877,
    /// MIN-TRCV-DELAY-COMPENSATION-OFFSET
    MinTrcvDelayCompensationOffset                                         = 1142,
    /// MIN-VALUE
    MinValue                                                               = 5328,
    /// MINIMUM
    Minimum                                                                = 2008,
    /// MINIMUM-DELAY
    MinimumDelay                                                           = 2758,
    /// MINIMUM-DISTANCES
    MinimumDistances                                                       = 1541,
    /// MINIMUM-EXECUTION-TIME
    MinimumExecutionTime                                                   = 6397,
    /// MINIMUM-FIFO-LENGTH
    MinimumFifoLength                                                      = 1671,
    /// MINIMUM-INTENDED-BINDING-TIME
    MinimumIntendedBindingTime                                             = 6012,
    /// MINIMUM-INTER-ARRIVAL-TIME
    MinimumInterArrivalTime                                                = 2407,
    /// MINIMUM-INTERVAL-LENGTH
    MinimumIntervalLength                                                  = 2572,
    /// MINIMUM-MEMORY-CONSUMPTION
    MinimumMemoryConsumption                                               = 5131,
    /// MINIMUM-MULTICAST-SEPERATION-TIME
    MinimumMulticastSeperationTime                                         = 4420,
    /// MINIMUM-NUMBER-OF-REQUESTS
    MinimumNumberOfRequests                                                = 304,
    /// MINIMUM-RX-CONTAINER-QUEUE-SIZE
    MinimumRxContainerQueueSize                                            = 2450,
    /// MINIMUM-SELECTED-BINDING-TIME
    MinimumSelectedBindingTime                                             = 3469,
    /// MINIMUM-SEND-INTERVAL
    MinimumSendInterval                                                    = 1662,
    /// MINIMUM-SEPARATION-TIME
    MinimumSeparationTime                                                  = 1868,
    /// MINIMUM-START-INTERVAL
    MinimumStartInterval                                                   = 1770,
    /// MINIMUM-SUPPORTED-UCM-MASTER-VERSION
    MinimumSupportedUcmMasterVersion                                       = 4353,
    /// MINIMUM-SUPPORTED-UCM-VERSION
    MinimumSupportedUcmVersion                                             = 2229,
    /// MINIMUM-SUSTAINED-SIZE
    MinimumSustainedSize                                                   = 4686,
    /// MINIMUM-TIMER-GRANULARITY
    MinimumTimerGranularity                                                = 208,
    /// MINIMUM-TX-CONTAINER-QUEUE-SIZE
    MinimumTxContainerQueueSize                                            = 467,
    /// MINISLOT-ACTION-POINT-OFFSET
    MinislotActionPointOffset                                              = 1041,
    /// MINISLOT-DURATION
    MinislotDuration                                                       = 2215,
    /// MINOR-PROTOCOL-VERSION
    MinorProtocolVersion                                                   = 6193,
    /// MINOR-VERSION
    MinorVersion                                                           = 353,
    /// MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID
    MirrorSourceLinToCanRangeBaseId                                        = 1336,
    /// MIRROR-STATUS-CAN-ID
    MirrorStatusCanId                                                      = 6234,
    /// MIRRORING-PROTOCOL
    MirroringProtocol                                                      = 5518,
    /// MIXED-BUS-TYPE-COLLECTION
    MixedBusTypeCollection                                                 = 5375,
    /// MKA-PARTICIPANT-REF
    MkaParticipantRef                                                      = 6416,
    /// MKA-PARTICIPANT-REFS
    MkaParticipantRefs                                                     = 1423,
    /// MKA-PARTICIPANTS
    MkaParticipants                                                        = 551,
    /// ML-GRAPHIC
    MlGraphic                                                              = 3026,
    /// MODE
    Mode                                                                   = 5301,
    /// MODE-ACCESS-POINT
    ModeAccessPoint                                                        = 4054,
    /// MODE-ACCESS-POINTS
    ModeAccessPoints                                                       = 1295,
    /// MODE-CONDITION
    ModeCondition                                                          = 1226,
    /// MODE-CONDITION-REF
    ModeConditionRef                                                       = 532,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 1205,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 6020,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 2536,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-IREF
    ModeDeclarationGroupPrototypeIref                                      = 4716,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF
    ModeDeclarationGroupPrototypeRef                                       = 117,
    /// MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL
    ModeDeclarationGroupPrototypeRefConditional                            = 319,
    /// MODE-DECLARATION-GROUP-REF
    ModeDeclarationGroupRef                                                = 5216,
    /// MODE-DECLARATION-GROUP-REFS
    ModeDeclarationGroupRefs                                               = 4852,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 2948,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 2899,
    /// MODE-DECLARATION-MAPPING-SET-REF
    ModeDeclarationMappingSetRef                                           = 4519,
    /// MODE-DECLARATION-MAPPINGS
    ModeDeclarationMappings                                                = 6261,
    /// MODE-DECLARATION-REF
    ModeDeclarationRef                                                     = 3652,
    /// MODE-DECLARATION-REFS
    ModeDeclarationRefs                                                    = 1574,
    /// MODE-DECLARATIONS
    ModeDeclarations                                                       = 6168,
    /// MODE-DEPENDENT-STARTUP-CONFIG
    ModeDependentStartupConfig                                             = 2437,
    /// MODE-DEPENDENT-STARTUP-CONFIGS
    ModeDependentStartupConfigs                                            = 4984,
    /// MODE-DRIVEN-FALSE-CONDITIONS
    ModeDrivenFalseConditions                                              = 5122,
    /// MODE-DRIVEN-TRANSMISSION-MODE-CONDITION
    ModeDrivenTransmissionModeCondition                                    = 5933,
    /// MODE-DRIVEN-TRUE-CONDITIONS
    ModeDrivenTrueConditions                                               = 6043,
    /// MODE-ELEMENT-REF
    ModeElementRef                                                         = 3855,
    /// MODE-ELEMENTS
    ModeElements                                                           = 3375,
    /// MODE-GROUP
    ModeGroup                                                              = 5453,
    /// MODE-GROUP-IREF
    ModeGroupIref                                                          = 3813,
    /// MODE-GROUP-REF
    ModeGroupRef                                                           = 5218,
    /// MODE-IN-BSW-INSTANCE-REF
    ModeInBswInstanceRef                                                   = 3346,
    /// MODE-IN-SWC-INSTANCE-REF
    ModeInSwcInstanceRef                                                   = 1382,
    /// MODE-INSTANCE
    ModeInstance                                                           = 1136,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 207,
    /// MODE-IREF
    ModeIref                                                               = 2980,
    /// MODE-IREFS
    ModeIrefs                                                              = 971,
    /// MODE-MANAGER-ERROR-BEHAVIOR
    ModeManagerErrorBehavior                                               = 5080,
    /// MODE-MAPPING
    ModeMapping                                                            = 3987,
    /// MODE-PORT-ANNOTATION
    ModePortAnnotation                                                     = 379,
    /// MODE-PORT-ANNOTATIONS
    ModePortAnnotations                                                    = 2694,
    /// MODE-RECEIVER-POLICYS
    ModeReceiverPolicys                                                    = 2047,
    /// MODE-REF
    ModeRef                                                                = 2167,
    /// MODE-REPORTER-PORT-IREF
    ModeReporterPortIref                                                   = 3484,
    /// MODE-REQUEST-TYPE-MAP
    ModeRequestTypeMap                                                     = 6520,
    /// MODE-REQUEST-TYPE-MAPS
    ModeRequestTypeMaps                                                    = 5235,
    /// MODE-SENDER-POLICYS
    ModeSenderPolicys                                                      = 6113,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY
    ModeSwitchEventTriggeredActivity                                       = 1394,
    /// MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS
    ModeSwitchEventTriggeredActivitys                                      = 5477,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 2652,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 5467,
    /// MODE-SWITCH-POINTS
    ModeSwitchPoints                                                       = 2086,
    /// MODE-SWITCH-RECEIVER-COM-SPEC
    ModeSwitchReceiverComSpec                                              = 3949,
    /// MODE-SWITCH-SENDER-COM-SPEC
    ModeSwitchSenderComSpec                                                = 838,
    /// MODE-SWITCH-SENDER-COM-SPEC-PROPS
    ModeSwitchSenderComSpecProps                                           = 2520,
    /// MODE-SWITCH-SENDER-COM-SPEC-PROPSS
    ModeSwitchSenderComSpecPropss                                          = 2325,
    /// MODE-SWITCHED-ACK
    ModeSwitchedAck                                                        = 1441,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 2597,
    /// MODE-TRANSITION
    ModeTransition                                                         = 2936,
    /// MODE-TRANSITIONS
    ModeTransitions                                                        = 2838,
    /// MODE-USER-ERROR-BEHAVIOR
    ModeUserErrorBehavior                                                  = 2866,
    /// MODEL-OBJECT-REFERENCE
    ModelObjectReference                                                   = 6533,
    /// MODES
    Modes                                                                  = 1739,
    /// MODIFICATION
    Modification                                                           = 743,
    /// MODIFICATIONS
    Modifications                                                          = 2707,
    /// MODIFIED-DATAS
    ModifiedDatas                                                          = 3790,
    /// MODULE-DESCRIPTION-REF
    ModuleDescriptionRef                                                   = 5846,
    /// MODULE-ID
    ModuleId                                                               = 4629,
    /// MODULE-INSTANTIATION-REF
    ModuleInstantiationRef                                                 = 5847,
    /// MODULE-INSTANTIATION-REFS
    ModuleInstantiationRefs                                                = 4477,
    /// MODULE-INSTANTIATIONS
    ModuleInstantiations                                                   = 3081,
    /// MODULE-REF
    ModuleRef                                                              = 5422,
    /// MODULE-REFS
    ModuleRefs                                                             = 29,
    /// MOLAR-AMOUNT-EXP
    MolarAmountExp                                                         = 1411,
    /// MONITORED-IDENTIFIER-REF
    MonitoredIdentifierRef                                                 = 5416,
    /// MONOTONY
    Monotony                                                               = 2730,
    /// MORE-FRAGMENTS
    MoreFragments                                                          = 6373,
    /// MSG-ID-MASK
    MsgIdMask                                                              = 1675,
    /// MSG-ID-MATCH
    MsgIdMatch                                                             = 3554,
    /// MSR-QUERY-ARG
    MsrQueryArg                                                            = 6303,
    /// MSR-QUERY-CHAPTER
    MsrQueryChapter                                                        = 1638,
    /// MSR-QUERY-NAME
    MsrQueryName                                                           = 3793,
    /// MSR-QUERY-P-1
    MsrQueryP1                                                             = 4186,
    /// MSR-QUERY-P-2
    MsrQueryP2                                                             = 6268,
    /// MSR-QUERY-PROPS
    MsrQueryProps                                                          = 4213,
    /// MSR-QUERY-RESULT-CHAPTER
    MsrQueryResultChapter                                                  = 2061,
    /// MSR-QUERY-RESULT-P-1
    MsrQueryResultP1                                                       = 5589,
    /// MSR-QUERY-RESULT-P-2
    MsrQueryResultP2                                                       = 737,
    /// MSR-QUERY-RESULT-TOPIC-1
    MsrQueryResultTopic1                                                   = 4784,
    /// MSR-QUERY-TOPIC-1
    MsrQueryTopic1                                                         = 5621,
    /// MULTICAST-CONNECTOR-REF
    MulticastConnectorRef                                                  = 3175,
    /// MULTICAST-CONNECTOR-REFS
    MulticastConnectorRefs                                                 = 3923,
    /// MULTICAST-CREDENTIALS
    MulticastCredentials                                                   = 6076,
    /// MULTICAST-REF
    MulticastRef                                                           = 5340,
    /// MULTICAST-SD-IP-ADDRESS-REF
    MulticastSdIpAddressRef                                                = 5815,
    /// MULTICAST-SECURE-COM-PROPS-REF
    MulticastSecureComPropsRef                                             = 268,
    /// MULTICAST-SEGMENTATION
    MulticastSegmentation                                                  = 5260,
    /// MULTICAST-THRESHOLD
    MulticastThreshold                                                     = 128,
    /// MULTICAST-UDP-PORT
    MulticastUdpPort                                                       = 3113,
    /// MULTIPLE-CONFIGURATION-CONTAINER
    MultipleConfigurationContainer                                         = 2665,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 675,
    /// MULTIPLICITY-CONFIG-CLASSES
    MultiplicityConfigClasses                                              = 5616,
    /// MULTIPLICITY-RESTRICTION
    MultiplicityRestriction                                                = 6087,
    /// N
    N                                                                      = 4859,
    /// N-DATA-SETS
    NDataSets                                                              = 6117,
    /// N-PDU
    NPdu                                                                   = 1937,
    /// N-PDU-REF
    NPduRef                                                                = 1209,
    /// N-PDU-REFS
    NPduRefs                                                               = 2555,
    /// N-ROM-BLOCKS
    NRomBlocks                                                             = 751,
    /// NAGLES-ALGORITHM
    NaglesAlgorithm                                                        = 1485,
    /// NAME
    Name                                                                   = 2803,
    /// NAME-TOKEN-VALUE-VARIATION-POINT
    NameTokenValueVariationPoint                                           = 811,
    /// NAMED-CURVE-ID
    NamedCurveId                                                           = 1616,
    /// NAMESPACES
    Namespaces                                                             = 384,
    /// NAS-TIMEOUT
    NasTimeout                                                             = 4007,
    /// NATIVE-DECLARATION
    NativeDeclaration                                                      = 5536,
    /// NDP-PROPS
    NdpProps                                                               = 3539,
    /// NEEDS-ORIGINAL-DATA
    NeedsOriginalData                                                      = 716,
    /// NEIGHBOR-CACHE-SIZE
    NeighborCacheSize                                                      = 6449,
    /// NESTED-CONTEXT-DATAS
    NestedContextDatas                                                     = 1642,
    /// NESTED-ELEMENT-REF
    NestedElementRef                                                       = 1589,
    /// NESTED-ELEMENT-REFS
    NestedElementRefs                                                      = 3995,
    /// NESTED-ELEMENTS
    NestedElements                                                         = 5875,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 6497,
    /// NETWORK-CONFIGURATION-DESIGN-REF
    NetworkConfigurationDesignRef                                          = 4910,
    /// NETWORK-CONFIGURATION-REF
    NetworkConfigurationRef                                                = 1851,
    /// NETWORK-CONFIGURATION-REFS
    NetworkConfigurationRefs                                               = 1932,
    /// NETWORK-CONFIGURATIONS
    NetworkConfigurations                                                  = 6378,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 6036,
    /// NETWORK-ENDPOINT-ADDRESSES
    NetworkEndpointAddresses                                               = 6244,
    /// NETWORK-ENDPOINT-REF
    NetworkEndpointRef                                                     = 723,
    /// NETWORK-ENDPOINT-REF-CONDITIONAL
    NetworkEndpointRefConditional                                          = 2566,
    /// NETWORK-ENDPOINT-REFS
    NetworkEndpointRefs                                                    = 5369,
    /// NETWORK-ENDPOINTS
    NetworkEndpoints                                                       = 3365,
    /// NETWORK-HANDLE-PORT-MAPPING
    NetworkHandlePortMapping                                               = 6290,
    /// NETWORK-HANDLE-REF
    NetworkHandleRef                                                       = 4349,
    /// NETWORK-HANDLES
    NetworkHandles                                                         = 2076,
    /// NETWORK-ID
    NetworkId                                                              = 580,
    /// NETWORK-IDLE-TIME
    NetworkIdleTime                                                        = 3812,
    /// NETWORK-INTERFACE-ID
    NetworkInterfaceId                                                     = 3771,
    /// NETWORK-INTERFACE-REF
    NetworkInterfaceRef                                                    = 2914,
    /// NETWORK-INTERFACE-REFS
    NetworkInterfaceRefs                                                   = 3722,
    /// NETWORK-INTERFACES
    NetworkInterfaces                                                      = 1576,
    /// NETWORK-LAYER-RULE
    NetworkLayerRule                                                       = 1431,
    /// NETWORK-MANAGEMENT-PORT-INTERFACE
    NetworkManagementPortInterface                                         = 2798,
    /// NETWORK-MANAGEMENT-VECTOR-LENGTH
    NetworkManagementVectorLength                                          = 5394,
    /// NETWORK-MASK
    NetworkMask                                                            = 5064,
    /// NETWORK-REPRESENTATION
    NetworkRepresentation                                                  = 2366,
    /// NETWORK-REPRESENTATION-PROPS
    NetworkRepresentationProps                                             = 5986,
    /// NETWORK-REPRESENTATIONS
    NetworkRepresentations                                                 = 3990,
    /// NETWORK-SEGMENT-ID
    NetworkSegmentId                                                       = 2365,
    /// NETWORK-TIME-CONSUMER-REF
    NetworkTimeConsumerRef                                                 = 6013,
    /// NETWORK-TIME-MASTER-REF
    NetworkTimeMasterRef                                                   = 2849,
    /// NETWORK-TIME-PROVIDER-REF
    NetworkTimeProviderRef                                                 = 930,
    /// NETWORK-TIME-SLAVE-REF
    NetworkTimeSlaveRef                                                    = 3964,
    /// NEW-NAD
    NewNad                                                                 = 5532,
    /// NEXT-HEADER
    NextHeader                                                             = 6118,
    /// NEXT-HIGHER-CERTIFICATE-REF
    NextHigherCertificateRef                                               = 3206,
    /// NEXT-STATE-IREF
    NextStateIref                                                          = 119,
    /// NM-ACTIVE-COORDINATOR
    NmActiveCoordinator                                                    = 3691,
    /// NM-BUS-LOAD-REDUCTION-ENABLED
    NmBusLoadReductionEnabled                                              = 2313,
    /// NM-BUS-SYNCHRONIZATION-ENABLED
    NmBusSynchronizationEnabled                                            = 2487,
    /// NM-BUSLOAD-REDUCTION-ACTIVE
    NmBusloadReductionActive                                               = 6597,
    /// NM-BUSLOAD-REDUCTION-ENABLED
    NmBusloadReductionEnabled                                              = 3666,
    /// NM-CAR-WAKE-UP-BIT-POSITION
    NmCarWakeUpBitPosition                                                 = 2974,
    /// NM-CAR-WAKE-UP-FILTER-ENABLED
    NmCarWakeUpFilterEnabled                                               = 5652,
    /// NM-CAR-WAKE-UP-FILTER-NODE-ID
    NmCarWakeUpFilterNodeId                                                = 413,
    /// NM-CAR-WAKE-UP-RX-ENABLED
    NmCarWakeUpRxEnabled                                                   = 845,
    /// NM-CBV-POSITION
    NmCbvPosition                                                          = 2431,
    /// NM-CHANNEL-ACTIVE
    NmChannelActive                                                        = 140,
    /// NM-CHANNEL-ID
    NmChannelId                                                            = 6081,
    /// NM-CHANNEL-SLEEP-MASTER
    NmChannelSleepMaster                                                   = 3000,
    /// NM-CLUSTER-COUPLINGS
    NmClusterCouplings                                                     = 5113,
    /// NM-CLUSTERS
    NmClusters                                                             = 653,
    /// NM-COM-CONTROL-ENABLED
    NmComControlEnabled                                                    = 2802,
    /// NM-CONFIG
    NmConfig                                                               = 4781,
    /// NM-CONTROL-BIT-VECTOR-ACTIVE
    NmControlBitVectorActive                                               = 2696,
    /// NM-CONTROL-BIT-VECTOR-ENABLED
    NmControlBitVectorEnabled                                              = 4521,
    /// NM-COORD-CLUSTER
    NmCoordCluster                                                         = 2740,
    /// NM-COORD-SYNC-SUPPORT
    NmCoordSyncSupport                                                     = 1113,
    /// NM-COORDINATOR
    NmCoordinator                                                          = 1266,
    /// NM-COORDINATOR-ROLE
    NmCoordinatorRole                                                      = 5694,
    /// NM-CYCLETIME-MAIN-FUNCTION
    NmCycletimeMainFunction                                                = 1413,
    /// NM-DATA-CYCLE
    NmDataCycle                                                            = 6273,
    /// NM-DATA-DISABLED
    NmDataDisabled                                                         = 690,
    /// NM-DATA-ENABLED
    NmDataEnabled                                                          = 2947,
    /// NM-DATA-INFORMATION
    NmDataInformation                                                      = 2495,
    /// NM-DETECTION-LOCK
    NmDetectionLock                                                        = 6541,
    /// NM-ECU
    NmEcu                                                                  = 5690,
    /// NM-GLOBAL-COORDINATOR-TIME
    NmGlobalCoordinatorTime                                                = 6264,
    /// NM-HANDLE-REF
    NmHandleRef                                                            = 3122,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 5074,
    /// NM-HW-VOTE-ENABLED
    NmHwVoteEnabled                                                        = 5867,
    /// NM-IF-ECU-REF
    NmIfEcuRef                                                             = 3512,
    /// NM-IF-ECUS
    NmIfEcus                                                               = 4880,
    /// NM-IMMEDIATE-NM-CYCLE-TIME
    NmImmediateNmCycleTime                                                 = 4927,
    /// NM-IMMEDIATE-NM-TRANSMISSIONS
    NmImmediateNmTransmissions                                             = 1511,
    /// NM-IMMEDIATE-RESTART-ENABLED
    NmImmediateRestartEnabled                                              = 2430,
    /// NM-INSTANCE-ID
    NmInstanceId                                                           = 2148,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 2922,
    /// NM-INTERACTS-WITH-SM-MAPPING
    NmInteractsWithSmMapping                                               = 171,
    /// NM-LIGHT-TIMEOUT
    NmLightTimeout                                                         = 5099,
    /// NM-MAIN-FUNCTION-ACROSS-FR-CYCLE
    NmMainFunctionAcrossFrCycle                                            = 1022,
    /// NM-MAIN-FUNCTION-PERIOD
    NmMainFunctionPeriod                                                   = 2732,
    /// NM-MESSAGE-TIMEOUT-TIME
    NmMessageTimeoutTime                                                   = 1700,
    /// NM-MSG-CYCLE-OFFSET
    NmMsgCycleOffset                                                       = 6243,
    /// NM-MSG-CYCLE-TIME
    NmMsgCycleTime                                                         = 3918,
    /// NM-MSG-REDUCED-TIME
    NmMsgReducedTime                                                       = 1333,
    /// NM-MULTIPLE-CHANNELS-ENABLED
    NmMultipleChannelsEnabled                                              = 4975,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 5395,
    /// NM-NETWORK-HANDLE-REF
    NmNetworkHandleRef                                                     = 1727,
    /// NM-NETWORK-TIMEOUT
    NmNetworkTimeout                                                       = 4096,
    /// NM-NID-POSITION
    NmNidPosition                                                          = 2602,
    /// NM-NODE-DETECTION-ENABLED
    NmNodeDetectionEnabled                                                 = 200,
    /// NM-NODE-ID
    NmNodeId                                                               = 4008,
    /// NM-NODE-ID-ENABLED
    NmNodeIdEnabled                                                        = 835,
    /// NM-NODE-REF
    NmNodeRef                                                              = 167,
    /// NM-NODE-REFS
    NmNodeRefs                                                             = 6270,
    /// NM-NODES
    NmNodes                                                                = 3603,
    /// NM-PASSIVE-MODE-ENABLED
    NmPassiveModeEnabled                                                   = 4946,
    /// NM-PDU
    NmPdu                                                                  = 760,
    /// NM-PDU-REF
    NmPduRef                                                               = 3669,
    /// NM-PDU-REF-CONDITIONAL
    NmPduRefConditional                                                    = 4321,
    /// NM-PDU-RX-INDICATION-ENABLED
    NmPduRxIndicationEnabled                                               = 4131,
    /// NM-PDUS
    NmPdus                                                                 = 5330,
    /// NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS
    NmPnHandleMultipleNetworkRequests                                      = 666,
    /// NM-PNC-PARTICIPATION
    NmPncParticipation                                                     = 4820,
    /// NM-RANGE-CONFIG
    NmRangeConfig                                                          = 2988,
    /// NM-READY-SLEEP-COUNT
    NmReadySleepCount                                                      = 5153,
    /// NM-READY-SLEEP-TIME
    NmReadySleepTime                                                       = 4145,
    /// NM-REMOTE-SLEEP-IND-ENABLED
    NmRemoteSleepIndEnabled                                                = 5552,
    /// NM-REMOTE-SLEEP-INDICATION-TIME
    NmRemoteSleepIndicationTime                                            = 5335,
    /// NM-REPEAT-MESSAGE-BIT-ACTIVE
    NmRepeatMessageBitActive                                               = 5601,
    /// NM-REPEAT-MESSAGE-BIT-ENABLE
    NmRepeatMessageBitEnable                                               = 5324,
    /// NM-REPEAT-MESSAGE-TIME
    NmRepeatMessageTime                                                    = 6328,
    /// NM-REPEAT-MSG-IND-ENABLED
    NmRepeatMsgIndEnabled                                                  = 5308,
    /// NM-REPEAT-MSG-INDICATION-ENABLED
    NmRepeatMsgIndicationEnabled                                           = 108,
    /// NM-REPETITION-CYCLE
    NmRepetitionCycle                                                      = 4436,
    /// NM-SCHEDULE-VARIANT
    NmScheduleVariant                                                      = 2864,
    /// NM-SHUTDOWN-DELAY-TIMER
    NmShutdownDelayTimer                                                   = 3692,
    /// NM-STATE-CHANGE-IND-ENABLED
    NmStateChangeIndEnabled                                                = 681,
    /// NM-STATE-REQUEST
    NmStateRequest                                                         = 6483,
    /// NM-SYNCHRONIZATION-POINT-ENABLED
    NmSynchronizationPointEnabled                                          = 185,
    /// NM-SYNCHRONIZING-NETWORK
    NmSynchronizingNetwork                                                 = 6127,
    /// NM-USER-DATA-ENABLED
    NmUserDataEnabled                                                      = 2716,
    /// NM-USER-DATA-LENGTH
    NmUserDataLength                                                       = 6308,
    /// NM-USER-DATA-OFFSET
    NmUserDataOffset                                                       = 5919,
    /// NM-VARIANT
    NmVariant                                                              = 1695,
    /// NM-VECTOR-EARLY-UPDATE
    NmVectorEarlyUpdate                                                    = 1517,
    /// NM-VOTE-INFORMATION
    NmVoteInformation                                                      = 1327,
    /// NM-VOTING-CYCLE
    NmVotingCycle                                                          = 46,
    /// NM-WAIT-BUS-SLEEP-TIME
    NmWaitBusSleepTime                                                     = 2250,
    /// NO-CHECKPOINT-SUPERVISION
    NoCheckpointSupervision                                                = 1842,
    /// NO-CHECKPOINT-SUPERVISIONS
    NoCheckpointSupervisions                                               = 197,
    /// NO-SUPERVISION
    NoSupervision                                                          = 3886,
    /// NO-SUPERVISIONS
    NoSupervisions                                                         = 4508,
    /// NODE-IDENTIFIER
    NodeIdentifier                                                         = 3959,
    /// NODE-NAME
    NodeName                                                               = 2800,
    /// NODE-PORTS
    NodePorts                                                              = 255,
    /// NODE-REF
    NodeRef                                                                = 5671,
    /// NOMINAL
    Nominal                                                                = 6218,
    /// NOMINAL-EXECUTION-TIME
    NominalExecutionTime                                                   = 5591,
    /// NON-OS-MODULE-INSTANTIATION-REF
    NonOsModuleInstantiationRef                                            = 6061,
    /// NON-VERBOSE-MODE
    NonVerboseMode                                                         = 3616,
    /// NONQUEUED-RECEIVER-COM-SPEC
    NonqueuedReceiverComSpec                                               = 4130,
    /// NONQUEUED-SENDER-COM-SPEC
    NonqueuedSenderComSpec                                                 = 3832,
    /// NOT-AVAILABLE-VALUE-SPECIFICATION
    NotAvailableValueSpecification                                         = 1806,
    /// NOTATION
    Notation                                                               = 744,
    /// NOTE
    Note                                                                   = 2556,
    /// NOTIFICATION-PORT-IREF
    NotificationPortIref                                                   = 669,
    /// NOTIFICATION-STATE
    NotificationState                                                      = 6349,
    /// NOTIFICATION-TIME
    NotificationTime                                                       = 2181,
    /// NOTIFICATIONS
    Notifications                                                          = 5839,
    /// NOTIFIED-STATUSS
    NotifiedStatuss                                                        = 3873,
    /// NOTIFIER
    Notifier                                                               = 4792,
    /// NOTIFIER-DATA-ELEMENT-REF
    NotifierDataElementRef                                                 = 2896,
    /// NOTIFIER-REF
    NotifierRef                                                            = 1252,
    /// NOTIFIER-SIGNAL-TRIGGERING-REF
    NotifierSignalTriggeringRef                                            = 3864,
    /// NRC-VALUE
    NrcValue                                                               = 1838,
    /// NTU
    Ntu                                                                    = 3194,
    /// NUM-FAILED-SECURITY-ACCESS
    NumFailedSecurityAccess                                                = 2269,
    /// NUMBER
    Number                                                                 = 1111,
    /// NUMBER-OF-BITS
    NumberOfBits                                                           = 5507,
    /// NUMBER-OF-CONNECTED-CLIENTS
    NumberOfConnectedClients                                               = 626,
    /// NUMBER-OF-ELEMENTS
    NumberOfElements                                                       = 884,
    /// NUMBER-OF-INSTRUCTIONS
    NumberOfInstructions                                                   = 2545,
    /// NUMBER-OF-MINISLOTS
    NumberOfMinislots                                                      = 6578,
    /// NUMBER-OF-NOTIFIER-SETS
    NumberOfNotifierSets                                                   = 4646,
    /// NUMBER-OF-PARALLEL-TCP-SESSIONS
    NumberOfParallelTcpSessions                                            = 4713,
    /// NUMBER-OF-REPETITIONS
    NumberOfRepetitions                                                    = 5608,
    /// NUMBER-OF-RESTART-ATTEMPTS
    NumberOfRestartAttempts                                                = 4695,
    /// NUMBER-OF-STATIC-SLOTS
    NumberOfStaticSlots                                                    = 358,
    /// NUMBER-OF-WORKERS
    NumberOfWorkers                                                        = 2632,
    /// NUMERICAL-RULE-BASED-VALUE-SPECIFICATION
    NumericalRuleBasedValueSpecification                                   = 1187,
    /// NUMERICAL-VALUE
    NumericalValue                                                         = 1841,
    /// NUMERICAL-VALUE-SPECIFICATION
    NumericalValueSpecification                                            = 2068,
    /// NUMERICAL-VALUE-VARIATION-POINT
    NumericalValueVariationPoint                                           = 275,
    /// NV-BLOCK-DATA-MAPPING
    NvBlockDataMapping                                                     = 3196,
    /// NV-BLOCK-DATA-MAPPINGS
    NvBlockDataMappings                                                    = 4497,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 6162,
    /// NV-BLOCK-DESCRIPTORS
    NvBlockDescriptors                                                     = 6051,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 2551,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 6599,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 5610,
    /// NV-DATA-PORT-ANNOTATION
    NvDataPortAnnotation                                                   = 5181,
    /// NV-DATA-PORT-ANNOTATIONS
    NvDataPortAnnotations                                                  = 1127,
    /// NV-DATAS
    NvDatas                                                                = 3329,
    /// NV-PROVIDE-COM-SPEC
    NvProvideComSpec                                                       = 1634,
    /// NV-RAM-BLOCK-ELEMENT
    NvRamBlockElement                                                      = 6408,
    /// NV-REQUIRE-COM-SPEC
    NvRequireComSpec                                                       = 2615,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 6615,
    /// OBD-DTC-NUMBER
    ObdDtcNumber                                                           = 4834,
    /// OBD-DTC-VALUE
    ObdDtcValue                                                            = 5452,
    /// OBD-DTC-VALUE-3-BYTE
    ObdDtcValue3Byte                                                       = 1670,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 5679,
    /// OBD-MID
    ObdMid                                                                 = 2771,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 5264,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 5176,
    /// OBD-PROPSS
    ObdPropss                                                              = 2497,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 6347,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 1278,
    /// OBD-RELEVANCE
    ObdRelevance                                                           = 1724,
    /// OBD-SUPPORT
    ObdSupport                                                             = 5727,
    /// OBJECT-DEFINITION-REF
    ObjectDefinitionRef                                                    = 4687,
    /// OBJECT-DEFINITION-REFS
    ObjectDefinitionRefs                                                   = 1713,
    /// OBJECT-DEFINTION-REF
    ObjectDefintionRef                                                     = 162,
    /// OBJECT-DEFINTION-REFS
    ObjectDefintionRefs                                                    = 3936,
    /// OBJECT-IREF
    ObjectIref                                                             = 6040,
    /// OBJECT-REF
    ObjectRef                                                              = 482,
    /// OBJECT-REFS
    ObjectRefs                                                             = 534,
    /// OCCURRENCE-COUNTER-PROCESSING
    OccurrenceCounterProcessing                                            = 804,
    /// OCCURRENCE-EXPRESSION
    OccurrenceExpression                                                   = 5196,
    /// OFFER-CYCLIC-DELAY
    OfferCyclicDelay                                                       = 6638,
    /// OFFSET
    Offset                                                                 = 4983,
    /// OFFSET-CORRECTION-ADAPTION-INTERVAL
    OffsetCorrectionAdaptionInterval                                       = 4675,
    /// OFFSET-CORRECTION-JUMP-THRESHOLD
    OffsetCorrectionJumpThreshold                                          = 6221,
    /// OFFSET-CORRECTION-OUT
    OffsetCorrectionOut                                                    = 918,
    /// OFFSET-CORRECTION-START
    OffsetCorrectionStart                                                  = 4143,
    /// OFFSET-SI-TO-UNIT
    OffsetSiToUnit                                                         = 4693,
    /// OFFSET-TIME-DOMAIN-REF
    OffsetTimeDomainRef                                                    = 6088,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 5123,
    /// OFFSETS
    Offsets                                                                = 4024,
    /// OFNS-DATA-ID-LIST
    OfnsDataIdList                                                         = 1860,
    /// OFNS-DATA-ID-LISTS
    OfnsDataIdLists                                                        = 3489,
    /// OFS-DATA-ID-LIST
    OfsDataIdList                                                          = 68,
    /// OFS-DATA-ID-LISTS
    OfsDataIdLists                                                         = 5432,
    /// OFS-SUB-TLV
    OfsSubTlv                                                              = 5363,
    /// ON-BOARD-MONITOR-ID
    OnBoardMonitorId                                                       = 2897,
    /// ON-CHANGE-DATA-PROTOTYPES
    OnChangeDataPrototypes                                                 = 4911,
    /// ON-FAIL-PERMISSIVE-MODE
    OnFailPermissiveMode                                                   = 4918,
    /// ON-FAIL-PERMISSIVE-MODE-TIMEOUT
    OnFailPermissiveModeTimeout                                            = 5514,
    /// ON-TRANSITION-VALUE
    OnTransitionValue                                                      = 5803,
    /// ONE-EVERY-N
    OneEveryN                                                              = 3257,
    /// OP
    Op                                                                     = 1856,
    /// OPERATION-ARGUMENT-INSTANCE-IREF
    OperationArgumentInstanceIref                                          = 1324,
    /// OPERATION-CYCLE
    OperationCycle                                                         = 5442,
    /// OPERATION-CYCLE-AUTOMATIC-END
    OperationCycleAutomaticEnd                                             = 2312,
    /// OPERATION-CYCLE-AUTOSTART
    OperationCycleAutostart                                                = 4812,
    /// OPERATION-CYCLE-REF
    OperationCycleRef                                                      = 2747,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 1611,
    /// OPERATION-IREF
    OperationIref                                                          = 4124,
    /// OPERATION-IREFS
    OperationIrefs                                                         = 5361,
    /// OPERATION-MAPPINGS
    OperationMappings                                                      = 2449,
    /// OPERATION-MODE
    OperationMode                                                          = 4226,
    /// OPERATION-PROTOTYPE-IREF
    OperationPrototypeIref                                                 = 50,
    /// OPERATION-REF
    OperationRef                                                           = 1035,
    /// OPERATION-REQUEST-TRIGGERING-REF
    OperationRequestTriggeringRef                                          = 3776,
    /// OPERATION-RESPONSE-TRIGGERING-REF
    OperationResponseTriggeringRef                                         = 4606,
    /// OPERATIONS
    Operations                                                             = 3401,
    /// OPERATOR
    Operator                                                               = 4222,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF
    OptAppRecordElementInMethodIref                                        = 2967,
    /// OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS
    OptAppRecordElementInMethodIrefs                                       = 950,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    OptAppRecordElementInServiceInterfaceIref                              = 5762,
    /// OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS
    OptAppRecordElementInServiceInterfaceIrefs                             = 2635,
    /// OPT-ARGUMENT-REF
    OptArgumentRef                                                         = 5225,
    /// OPT-ARGUMENT-REFS
    OptArgumentRefs                                                        = 5269,
    /// OPT-IMPL-RECORD-ELEMENT
    OptImplRecordElement                                                   = 3434,
    /// OPT-IMPL-RECORD-ELEMENTS
    OptImplRecordElements                                                  = 5580,
    /// OPTION
    Option                                                                 = 1955,
    /// OPTION-ARGUMENT
    OptionArgument                                                         = 3216,
    /// OPTION-KIND
    OptionKind                                                             = 4703,
    /// OPTION-NAME
    OptionName                                                             = 1928,
    /// OPTIONAL
    Optional                                                               = 6411,
    /// OPTIONAL-ELEMENTS
    OptionalElements                                                       = 2966,
    /// OPTIONS
    Options                                                                = 5187,
    /// ORDERED-ELEMENTS
    OrderedElements                                                        = 2754,
    /// ORDERED-MASTER
    OrderedMaster                                                          = 3653,
    /// ORDERED-MASTER-LIST
    OrderedMasterList                                                      = 3613,
    /// ORIGIN
    Origin                                                                 = 2169,
    /// OS-ARTI-ADAPTER-LAUNCH-BEHAVIOR
    OsArtiAdapterLaunchBehavior                                            = 6191,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 4033,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 5159,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 4020,
    /// OS-TASK-PROXY-REF
    OsTaskProxyRef                                                         = 295,
    /// OUT-MEASUREMENT-SET
    OutMeasurementSet                                                      = 2094,
    /// OUT-MEASURMENT-SET
    OutMeasurmentSet                                                       = 2547,
    /// OUT-OF-RANGE-BITFIELD-ERRORS-ENABLED
    OutOfRangeBitfieldErrorsEnabled                                        = 4053,
    /// OUTER-PORT-REF
    OuterPortRef                                                           = 2873,
    /// OUTER-PORTS
    OuterPorts                                                             = 3545,
    /// OUTGOING-CALLBACKS
    OutgoingCallbacks                                                      = 382,
    /// OVERRIDE-ID
    OverrideId                                                             = 2878,
    /// OVERRIDE-INITIAL-STATE-IREF
    OverrideInitialStateIref                                               = 5431,
    /// OWNER-REF
    OwnerRef                                                               = 5565,
    /// OWNER-REFS
    OwnerRefs                                                              = 442,
    /// OWNERSHIP
    Ownership                                                              = 1904,
    /// OWNERSHIP-KIND
    OwnershipKind                                                          = 3663,
    /// OWNERSHIP-STRENGTH
    OwnershipStrength                                                      = 2524,
    /// P
    P                                                                      = 3560,
    /// P-2-MAX
    P2Max                                                                  = 4941,
    /// P-2-SERVER-MAX
    P2ServerMax                                                            = 1922,
    /// P-2-STAR-SERVER-MAX
    P2StarServerMax                                                        = 6438,
    /// P-2-TIMING
    P2Timing                                                               = 4412,
    /// P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    PModeGroupInAtomicSwcInstanceRef                                       = 711,
    /// P-PORT-IN-COMPOSITION-INSTANCE-REF
    PPortInCompositionInstanceRef                                          = 3583,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 2188,
    /// P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    PPortPrototypeInExecutableIref                                         = 1952,
    /// P-PORT-PROTOTYPE-IREF
    PPortPrototypeIref                                                     = 4615,
    /// P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF
    PTriggerInAtomicSwcTypeInstanceRef                                     = 779,
    /// PACKAGE-REF
    PackageRef                                                             = 1886,
    /// PACKAGER-ID
    PackagerId                                                             = 6123,
    /// PACKAGER-SIGNATURE-REF
    PackagerSignatureRef                                                   = 1532,
    /// PACKAGING-PIN-NAME
    PackagingPinName                                                       = 4294,
    /// PACKING-BYTE-ORDER
    PackingByteOrder                                                       = 4216,
    /// PADDING-ACTIVATION
    PaddingActivation                                                      = 5746,
    /// PADDING-VALUE
    PaddingValue                                                           = 4537,
    /// PARAMETER
    Parameter                                                              = 3408,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 2338,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REF
    ParameterAccessedForDebugRef                                           = 1614,
    /// PARAMETER-ACCESSED-FOR-DEBUG-REFS
    ParameterAccessedForDebugRefs                                          = 4775,
    /// PARAMETER-ACCESSS
    ParameterAccesss                                                       = 1103,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 515,
    /// PARAMETER-DATA-PROTOTYPE-IREF
    ParameterDataPrototypeIref                                             = 2599,
    /// PARAMETER-ELEMENT-ACCESS
    ParameterElementAccess                                                 = 4256,
    /// PARAMETER-ID
    ParameterId                                                            = 3608,
    /// PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT
    ParameterImplementationDataTypeElement                                 = 836,
    /// PARAMETER-INSTANCE
    ParameterInstance                                                      = 5550,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 4248,
    /// PARAMETER-POLICYS
    ParameterPolicys                                                       = 4251,
    /// PARAMETER-PORT-ANNOTATION
    ParameterPortAnnotation                                                = 3619,
    /// PARAMETER-PORT-ANNOTATIONS
    ParameterPortAnnotations                                               = 3540,
    /// PARAMETER-PROVIDE-COM-SPEC
    ParameterProvideComSpec                                                = 2467,
    /// PARAMETER-REF
    ParameterRef                                                           = 4428,
    /// PARAMETER-REQUIRE-COM-SPEC
    ParameterRequireComSpec                                                = 27,
    /// PARAMETER-SIZE
    ParameterSize                                                          = 5898,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 3787,
    /// PARAMETER-VALUES
    ParameterValues                                                        = 2380,
    /// PARAMETERS
    Parameters                                                             = 5900,
    /// PARENT-CATEGORY
    ParentCategory                                                         = 4289,
    /// PARENT-SHORT-LABEL
    ParentShortLabel                                                       = 6606,
    /// PARTIAL-NETWORK-REF
    PartialNetworkRef                                                      = 5854,
    /// PARTIAL-NETWORK-REFS
    PartialNetworkRefs                                                     = 4833,
    /// PARTICIPATING-J-1939-CLUSTER-REF
    ParticipatingJ1939ClusterRef                                           = 291,
    /// PARTICIPATING-J-1939-CLUSTER-REFS
    ParticipatingJ1939ClusterRefs                                          = 873,
    /// PARTITION-NAME
    PartitionName                                                          = 1600,
    /// PARTITION-REF
    PartitionRef                                                           = 4466,
    /// PARTITIONS
    Partitions                                                             = 4372,
    /// PARTS
    Parts                                                                  = 2525,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 5004,
    /// PATH
    Path                                                                   = 1938,
    /// PATH-MTU-DISCOVERY-ENABLED
    PathMtuDiscoveryEnabled                                                = 5489,
    /// PATH-MTU-ENABLED
    PathMtuEnabled                                                         = 434,
    /// PATH-MTU-TIMEOUT
    PathMtuTimeout                                                         = 4135,
    /// PATTERN
    Pattern                                                                = 5931,
    /// PATTERN-JITTER
    PatternJitter                                                          = 3398,
    /// PATTERN-LENGTH
    PatternLength                                                          = 4195,
    /// PATTERN-PERIOD
    PatternPeriod                                                          = 5028,
    /// PAYLOAD-BYTE-PATTERN-RULE
    PayloadBytePatternRule                                                 = 4613,
    /// PAYLOAD-BYTE-PATTERN-RULE-PART
    PayloadBytePatternRulePart                                             = 2345,
    /// PAYLOAD-BYTE-PATTERN-RULE-PARTS
    PayloadBytePatternRuleParts                                            = 3439,
    /// PAYLOAD-BYTE-PATTERN-RULES
    PayloadBytePatternRules                                                = 3734,
    /// PAYLOAD-LENGTH
    PayloadLength                                                          = 3861,
    /// PAYLOAD-LENGTH-STATIC
    PayloadLengthStatic                                                    = 5486,
    /// PAYLOAD-PREAMBLE-INDICATOR
    PayloadPreambleIndicator                                               = 4593,
    /// PAYLOAD-REF
    PayloadRef                                                             = 1189,
    /// PAYLOAD-TYPE
    PayloadType                                                            = 5447,
    /// PAYLOADS
    Payloads                                                               = 1314,
    /// PCM-BIT-DEPTH
    PcmBitDepth                                                            = 6131,
    /// PDELAY-LATENCY-THRESHOLD
    PdelayLatencyThreshold                                                 = 3991,
    /// PDELAY-REQUEST-PERIOD
    PdelayRequestPeriod                                                    = 1159,
    /// PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT
    PdelayRespAndRespFollowUpTimeout                                       = 2230,
    /// PDELAY-RESPONSE-ENABLED
    PdelayResponseEnabled                                                  = 2003,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 2419,
    /// PDU-ACTIVATION-ROUTING-GROUPS
    PduActivationRoutingGroups                                             = 2389,
    /// PDU-COLLECTION-MAX-BUFFER-SIZE
    PduCollectionMaxBufferSize                                             = 6282,
    /// PDU-COLLECTION-PDU-TIMEOUT
    PduCollectionPduTimeout                                                = 4074,
    /// PDU-COLLECTION-SEMANTICS
    PduCollectionSemantics                                                 = 5719,
    /// PDU-COLLECTION-TIMEOUT
    PduCollectionTimeout                                                   = 1245,
    /// PDU-COLLECTION-TRIGGER
    PduCollectionTrigger                                                   = 334,
    /// PDU-COUNTER-SIZE
    PduCounterSize                                                         = 2122,
    /// PDU-COUNTER-START-POSITION
    PduCounterStartPosition                                                = 1048,
    /// PDU-COUNTER-THRESHOLD
    PduCounterThreshold                                                    = 3519,
    /// PDU-COUNTERS
    PduCounters                                                            = 166,
    /// PDU-MAX-LENGTH
    PduMaxLength                                                           = 567,
    /// PDU-POOLS
    PduPools                                                               = 6171,
    /// PDU-REF
    PduRef                                                                 = 1847,
    /// PDU-REPLICATION-VOTING
    PduReplicationVoting                                                   = 3694,
    /// PDU-REPLICATIONS
    PduReplications                                                        = 4432,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 1147,
    /// PDU-TO-FRAME-MAPPINGS
    PduToFrameMappings                                                     = 2204,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 1992,
    /// PDU-TRIGGERING-REF
    PduTriggeringRef                                                       = 3383,
    /// PDU-TRIGGERING-REF-CONDITIONAL
    PduTriggeringRefConditional                                            = 965,
    /// PDU-TRIGGERING-REFS
    PduTriggeringRefs                                                      = 2998,
    /// PDU-TRIGGERINGS
    PduTriggerings                                                         = 3189,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 6351,
    /// PDUR-TP-CHUNK-SIZE
    PdurTpChunkSize                                                        = 6279,
    /// PDUS
    Pdus                                                                   = 3741,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 5503,
    /// PER-INSTANCE-MEMORY-REF
    PerInstanceMemoryRef                                                   = 2436,
    /// PER-INSTANCE-MEMORY-SIZE
    PerInstanceMemorySize                                                  = 5384,
    /// PER-INSTANCE-MEMORY-SIZES
    PerInstanceMemorySizes                                                 = 5183,
    /// PER-INSTANCE-MEMORYS
    PerInstanceMemorys                                                     = 4774,
    /// PER-INSTANCE-PARAMETER-REF
    PerInstanceParameterRef                                                = 5735,
    /// PER-INSTANCE-PARAMETERS
    PerInstanceParameters                                                  = 951,
    /// PER-STATE-TIMEOUT
    PerStateTimeout                                                        = 3705,
    /// PER-STATE-TIMEOUTS
    PerStateTimeouts                                                       = 3054,
    /// PERIOD
    Period                                                                 = 4768,
    /// PERIOD-BEGIN
    PeriodBegin                                                            = 1260,
    /// PERIOD-END
    PeriodEnd                                                              = 3268,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 4290,
    /// PERIODIC-RATE-CATEGORY
    PeriodicRateCategory                                                   = 133,
    /// PERIODIC-RATES
    PeriodicRates                                                          = 1765,
    /// PERIODIC-RESPONSE-UUDT-REF
    PeriodicResponseUudtRef                                                = 5230,
    /// PERIODIC-RESPONSE-UUDT-REFS
    PeriodicResponseUudtRefs                                               = 5612,
    /// PERMISSIBLE-SIGNAL-PATH
    PermissibleSignalPath                                                  = 4345,
    /// PERMISSION-CERTIFICATE-AUTHORITY-REF
    PermissionCertificateAuthorityRef                                      = 1424,
    /// PERMISSION-TO-CREATE-CHILD-PROCESS
    PermissionToCreateChildProcess                                         = 5373,
    /// PERMIT-MULTIPLE-REFERENCES-TO-EE
    PermitMultipleReferencesToEe                                           = 1436,
    /// PERSISTENCY-ACCESS
    PersistencyAccess                                                      = 5016,
    /// PERSISTENCY-CENTRAL-STORAGE-URI
    PersistencyCentralStorageUri                                           = 4339,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 4895,
    /// PERSISTENCY-DATA-PROVIDED-COM-SPEC
    PersistencyDataProvidedComSpec                                         = 1233,
    /// PERSISTENCY-DATA-REQUIRED-COM-SPEC
    PersistencyDataRequiredComSpec                                         = 1580,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-REF
    PersistencyDeploymentElementRef                                        = 5471,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 1134,
    /// PERSISTENCY-DEPLOYMENT-REF
    PersistencyDeploymentRef                                               = 2257,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 182,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 2246,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 2752,
    /// PERSISTENCY-DEPLOYMENT-URI
    PersistencyDeploymentUri                                               = 6637,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 3109,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 2963,
    /// PERSISTENCY-FILE-ARRAY-REF
    PersistencyFileArrayRef                                                = 5160,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 5272,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 1399,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 1452,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 2328,
    /// PERSISTENCY-FILE-REQUIRED-COM-SPEC
    PersistencyFileRequiredComSpec                                         = 4055,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 4075,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 3698,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 224,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 2438,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 5793,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 1964,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 1179,
    /// PERSISTENCY-PORT-PROTOTYPE-IREF
    PersistencyPortPrototypeIref                                           = 1007,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 3339,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 2918,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 5309,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 444,
    /// PERSISTENCY-PROVIDED-COM-SPEC
    PersistencyProvidedComSpec                                             = 6377,
    /// PERSISTENCY-REDUNDANCY-CRC
    PersistencyRedundancyCrc                                               = 3191,
    /// PERSISTENCY-REDUNDANCY-HASH
    PersistencyRedundancyHash                                              = 4839,
    /// PERSISTENCY-REDUNDANCY-M-OUT-OF-N
    PersistencyRedundancyMOutOfN                                           = 3236,
    /// PERSISTENT-STORAGE
    PersistentStorage                                                      = 4981,
    /// PGN
    Pgn                                                                    = 4707,
    /// PHM-ACTION
    PhmAction                                                              = 2543,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 5997,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 152,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 5603,
    /// PHM-CHECKPOINT-IREF
    PhmCheckpointIref                                                      = 4849,
    /// PHM-CONTRIBUTION-REF
    PhmContributionRef                                                     = 5346,
    /// PHM-CONTRIBUTION-REFS
    PhmContributionRefs                                                    = 5313,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 1544,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 5057,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 5086,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 4340,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 3840,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 80,
    /// PHM-RULE
    PhmRule                                                                = 2928,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 4647,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 2142,
    /// PHYS-CONSTRS
    PhysConstrs                                                            = 2287,
    /// PHYSICAL-CHANNEL-REF
    PhysicalChannelRef                                                     = 3524,
    /// PHYSICAL-CHANNEL-REF-CONDITIONAL
    PhysicalChannelRefConditional                                          = 4563,
    /// PHYSICAL-CHANNEL-REFS
    PhysicalChannelRefs                                                    = 3014,
    /// PHYSICAL-CHANNELS
    PhysicalChannels                                                       = 1037,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 5512,
    /// PHYSICAL-DIMENSION-MAPPING
    PhysicalDimensionMapping                                               = 5696,
    /// PHYSICAL-DIMENSION-MAPPING-REF
    PhysicalDimensionMappingRef                                            = 1006,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 4823,
    /// PHYSICAL-DIMENSION-MAPPINGS
    PhysicalDimensionMappings                                              = 599,
    /// PHYSICAL-DIMENSION-REF
    PhysicalDimensionRef                                                   = 5232,
    /// PHYSICAL-LAYER-TYPE
    PhysicalLayerType                                                      = 5529,
    /// PHYSICAL-PROPS
    PhysicalProps                                                          = 5304,
    /// PHYSICAL-REQUEST-REF
    PhysicalRequestRef                                                     = 4699,
    /// PID
    Pid                                                                    = 966,
    /// PID-REF
    PidRef                                                                 = 5982,
    /// PID-REFS
    PidRefs                                                                = 2502,
    /// PID-SIZE
    PidSize                                                                = 3404,
    /// PIN-NUMBER
    PinNumber                                                              = 164,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 634,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 4680,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 2927,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 5019,
    /// PLATFORM-TIME-BASES
    PlatformTimeBases                                                      = 369,
    /// PLCA-LOCAL-NODE-COUNT
    PlcaLocalNodeCount                                                     = 3304,
    /// PLCA-LOCAL-NODE-ID
    PlcaLocalNodeId                                                        = 4193,
    /// PLCA-MAX-BURST-COUNT
    PlcaMaxBurstCount                                                      = 3151,
    /// PLCA-MAX-BURST-TIMER
    PlcaMaxBurstTimer                                                      = 3047,
    /// PLCA-PROPS
    PlcaProps                                                              = 6547,
    /// PLCA-TRANSMIT-OPPORTUNITY-TIMER
    PlcaTransmitOpportunityTimer                                           = 5352,
    /// PN-RESET-TIME
    PnResetTime                                                            = 4246,
    /// PN-RESET-TIMER
    PnResetTimer                                                           = 3914,
    /// PNC-CLUSTER-VECTOR-LENGTH
    PncClusterVectorLength                                                 = 2645,
    /// PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS
    PncConsumedProvidedServiceInstanceGroups                               = 5436,
    /// PNC-FILTER-ARRAY-MASK
    PncFilterArrayMask                                                     = 4400,
    /// PNC-FILTER-ARRAY-MASKS
    PncFilterArrayMasks                                                    = 685,
    /// PNC-FILTER-DATA-MASK
    PncFilterDataMask                                                      = 5421,
    /// PNC-GATEWAY-TYPE
    PncGatewayType                                                         = 1846,
    /// PNC-GROUP-REF
    PncGroupRef                                                            = 2010,
    /// PNC-GROUP-REFS
    PncGroupRefs                                                           = 2011,
    /// PNC-IDENTIFIER
    PncIdentifier                                                          = 4564,
    /// PNC-MAPPING
    PncMapping                                                             = 5954,
    /// PNC-MAPPING-IDENT-REF
    PncMappingIdentRef                                                     = 2605,
    /// PNC-MAPPING-IDENT-REF-CONDITIONAL
    PncMappingIdentRefConditional                                          = 1716,
    /// PNC-MAPPING-REF
    PncMappingRef                                                          = 4416,
    /// PNC-MAPPING-REFS
    PncMappingRefs                                                         = 4890,
    /// PNC-MAPPINGS
    PncMappings                                                            = 5977,
    /// PNC-NM-REQUEST
    PncNmRequest                                                           = 225,
    /// PNC-PDUR-GROUP-REF
    PncPdurGroupRef                                                        = 4409,
    /// PNC-PDUR-GROUP-REFS
    PncPdurGroupRefs                                                       = 1916,
    /// PNC-PREPARE-SLEEP-TIMER
    PncPrepareSleepTimer                                                   = 6406,
    /// PNC-SYNCHRONOUS-WAKEUP
    PncSynchronousWakeup                                                   = 147,
    /// PNC-VECTOR-LENGTH
    PncVectorLength                                                        = 3131,
    /// PNC-VECTOR-OFFSET
    PncVectorOffset                                                        = 2385,
    /// PNC-WAKEUP-CAN-ID
    PncWakeupCanId                                                         = 5644,
    /// PNC-WAKEUP-CAN-ID-EXTENDED
    PncWakeupCanIdExtended                                                 = 5390,
    /// PNC-WAKEUP-CAN-ID-MASK
    PncWakeupCanIdMask                                                     = 2690,
    /// PNC-WAKEUP-DATA-MASK
    PncWakeupDataMask                                                      = 1789,
    /// PNC-WAKEUP-DLC
    PncWakeupDlc                                                           = 3658,
    /// PNC-WAKEUP-ENABLE
    PncWakeupEnable                                                        = 1792,
    /// POLICY
    Policy                                                                 = 699,
    /// POLICY-ACTION
    PolicyAction                                                           = 3078,
    /// PORT-ADDRESS
    PortAddress                                                            = 1783,
    /// PORT-API-OPTION
    PortApiOption                                                          = 1509,
    /// PORT-API-OPTIONS
    PortApiOptions                                                         = 6080,
    /// PORT-ARG-VALUES
    PortArgValues                                                          = 2778,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 1458,
    /// PORT-BLUEPRINT-INIT-VALUE
    PortBlueprintInitValue                                                 = 5964,
    /// PORT-BLUEPRINT-MAPPING
    PortBlueprintMapping                                                   = 5089,
    /// PORT-BLUEPRINT-MAPPINGS
    PortBlueprintMappings                                                  = 4308,
    /// PORT-BLUEPRINT-REF
    PortBlueprintRef                                                       = 5069,
    /// PORT-CONFIGS
    PortConfigs                                                            = 2608,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINT
    PortDefinedArgumentBlueprint                                           = 4708,
    /// PORT-DEFINED-ARGUMENT-BLUEPRINTS
    PortDefinedArgumentBlueprints                                          = 609,
    /// PORT-DEFINED-ARGUMENT-VALUE
    PortDefinedArgumentValue                                               = 713,
    /// PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS
    PortElementToComResourceMappings                                       = 4406,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 5551,
    /// PORT-GROUP
    PortGroup                                                              = 1316,
    /// PORT-GROUPS
    PortGroups                                                             = 5632,
    /// PORT-INSTANTIATION-BEHAVIOR
    PortInstantiationBehavior                                              = 6203,
    /// PORT-INTERFACE-BLUEPRINT-MAPPING
    PortInterfaceBlueprintMapping                                          = 1805,
    /// PORT-INTERFACE-BLUEPRINT-REF
    PortInterfaceBlueprintRef                                              = 5100,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 6004,
    /// PORT-INTERFACE-MAPPINGS
    PortInterfaceMappings                                                  = 6222,
    /// PORT-INTERFACE-REF
    PortInterfaceRef                                                       = 6540,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 3645,
    /// PORT-NUMBER
    PortNumber                                                             = 6164,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 4002,
    /// PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE
    PortPrototypeBlueprintInitValue                                        = 5712,
    /// PORT-PROTOTYPE-BLUEPRINT-MAPPING
    PortPrototypeBlueprintMapping                                          = 1001,
    /// PORT-PROTOTYPE-BLUEPRINT-REF
    PortPrototypeBlueprintRef                                              = 2221,
    /// PORT-PROTOTYPE-IREF
    PortPrototypeIref                                                      = 3794,
    /// PORT-PROTOTYPE-PROPS
    PortPrototypeProps                                                     = 5419,
    /// PORT-PROTOTYPE-REF
    PortPrototypeRef                                                       = 3962,
    /// PORT-PROTOTYPE-REF-CONDITIONAL
    PortPrototypeRefConditional                                            = 4531,
    /// PORT-PROTOTYPE-SLUG-FRAGMENT
    PortPrototypeSlugFragment                                              = 6512,
    /// PORT-REF
    PortRef                                                                = 701,
    /// PORT-SCHEDULER
    PortScheduler                                                          = 5711,
    /// PORTS
    Ports                                                                  = 1013,
    /// POSITION
    Position                                                               = 2399,
    /// POSITION-IN-TABLE
    PositionInTable                                                        = 5682,
    /// POSITIVE-INTEGER-VALUE-VARIATION-POINT
    PositiveIntegerValueVariationPoint                                     = 1763,
    /// POSSIBLE-AP-ERROR-REF
    PossibleApErrorRef                                                     = 3548,
    /// POSSIBLE-AP-ERROR-REFS
    PossibleApErrorRefs                                                    = 1298,
    /// POSSIBLE-AP-ERROR-SET-REF
    PossibleApErrorSetRef                                                  = 1603,
    /// POSSIBLE-AP-ERROR-SET-REFS
    PossibleApErrorSetRefs                                                 = 2944,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 79,
    /// POSSIBLE-ERROR-REACTIONS
    PossibleErrorReactions                                                 = 2030,
    /// POSSIBLE-ERROR-REF
    PossibleErrorRef                                                       = 6316,
    /// POSSIBLE-ERROR-REFS
    PossibleErrorRefs                                                      = 1098,
    /// POSSIBLE-ERRORS
    PossibleErrors                                                         = 297,
    /// POST-BUILD-CHANGEABLE
    PostBuildChangeable                                                    = 1777,
    /// POST-BUILD-VALUE-ACCESS-REF
    PostBuildValueAccessRef                                                = 4157,
    /// POST-BUILD-VARIANT-CONDITION
    PostBuildVariantCondition                                              = 5895,
    /// POST-BUILD-VARIANT-CONDITIONS
    PostBuildVariantConditions                                             = 6289,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 1924,
    /// POST-BUILD-VARIANT-CRITERION-VALUE
    PostBuildVariantCriterionValue                                         = 1484,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 3011,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF
    PostBuildVariantCriterionValueSetRef                                   = 4067,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS
    PostBuildVariantCriterionValueSetRefs                                  = 1948,
    /// POST-BUILD-VARIANT-CRITERION-VALUES
    PostBuildVariantCriterionValues                                        = 772,
    /// POST-BUILD-VARIANT-MULTIPLICITY
    PostBuildVariantMultiplicity                                           = 5207,
    /// POST-BUILD-VARIANT-SUPPORT
    PostBuildVariantSupport                                                = 1227,
    /// POST-BUILD-VARIANT-USED
    PostBuildVariantUsed                                                   = 1357,
    /// POST-BUILD-VARIANT-VALUE
    PostBuildVariantValue                                                  = 4900,
    /// POST-VERIFICATION-REBOOT
    PostVerificationReboot                                                 = 340,
    /// POSTS
    Posts                                                                  = 6104,
    /// POWER-DOWN-TIME
    PowerDownTime                                                          = 2760,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 6227,
    /// PRE-ACTIVATE-IREF
    PreActivateIref                                                        = 4945,
    /// PRE-ACTIVATE-IREFS
    PreActivateIrefs                                                       = 1637,
    /// PRE-ACTIVATE-REF
    PreActivateRef                                                         = 4757,
    /// PRE-ACTIVATE-REFS
    PreActivateRefs                                                        = 3811,
    /// PRE-ACTIVATION-REBOOT
    PreActivationReboot                                                    = 3138,
    /// PRE-MAPPING
    PreMapping                                                             = 2898,
    /// PRE-SHARED-KEY-REF
    PreSharedKeyRef                                                        = 1990,
    /// PRECONFIGURED-CONFIGURATION-REF
    PreconfiguredConfigurationRef                                          = 4167,
    /// PRECONFIGURED-CONFIGURATION-REFS
    PreconfiguredConfigurationRefs                                         = 4878,
    /// PREDECESSOR-ACTION-REF
    PredecessorActionRef                                                   = 420,
    /// PREDECESSOR-ACTION-REFS
    PredecessorActionRefs                                                  = 5344,
    /// PREDECESSOR-FIFO-REF
    PredecessorFifoRef                                                     = 659,
    /// PREDECESSOR-REF
    PredecessorRef                                                         = 5229,
    /// PREDECESSOR-REFS
    PredecessorRefs                                                        = 2780,
    /// PREDEFINED-TEXT
    PredefinedText                                                         = 3342,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 1434,
    /// PREEMPTABILITY
    Preemptability                                                         = 1840,
    /// PREFIX
    Prefix                                                                 = 2102,
    /// PREFIX-REF
    PrefixRef                                                              = 314,
    /// PREPARE-ROLLBACK
    PrepareRollback                                                        = 629,
    /// PREPARE-UPDATE
    PrepareUpdate                                                          = 2241,
    /// PRESTORAGE-FREEZE-FRAME
    PrestorageFreezeFrame                                                  = 220,
    /// PRESTORED-FREEZEFRAME-STORED-IN-NVM
    PrestoredFreezeframeStoredInNvm                                        = 188,
    /// PREVIOUS-CONTRACT-VERSION
    PreviousContractVersion                                                = 4935,
    /// PREVIOUS-DATA-TYPE-REF
    PreviousDataTypeRef                                                    = 5033,
    /// PREVIOUS-EXECUTABLE-VERSION
    PreviousExecutableVersion                                              = 243,
    /// PRIMARY-DOWNLOAD-URI
    PrimaryDownloadUri                                                     = 5758,
    /// PRIMITIVE
    Primitive                                                              = 784,
    /// PRIMITIVE-ATTRIBUTE-CONDITION
    PrimitiveAttributeCondition                                            = 5654,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 3899,
    /// PRIMITIVE-FAMILY
    PrimitiveFamily                                                        = 4704,
    /// PRIMITIVE-TYPE-MAPPINGS
    PrimitiveTypeMappings                                                  = 65,
    /// PRIORITY
    Priority                                                               = 6619,
    /// PRIORITY-ID
    PriorityId                                                             = 3629,
    /// PRIORITYS
    Prioritys                                                              = 4733,
    /// PRIVACY-LEVEL
    PrivacyLevel                                                           = 1774,
    /// PRM
    Prm                                                                    = 5317,
    /// PRM-CHAR
    PrmChar                                                                = 660,
    /// PRM-UNIT
    PrmUnit                                                                = 513,
    /// PRMS
    Prms                                                                   = 1020,
    /// PROCESS
    Process                                                                = 3854,
    /// PROCESS-ARGUMENT
    ProcessArgument                                                        = 5127,
    /// PROCESS-ARGUMENTS
    ProcessArguments                                                       = 4421,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 1354,
    /// PROCESS-DESIGN-REF
    ProcessDesignRef                                                       = 3497,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 926,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 964,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS
    ProcessDesignToMachineDesignMappings                                   = 4505,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 355,
    /// PROCESS-EXECUTION-ERROR-REF
    ProcessExecutionErrorRef                                               = 4741,
    /// PROCESS-MODE-IREF
    ProcessModeIref                                                        = 5775,
    /// PROCESS-MODE-MACHINE
    ProcessModeMachine                                                     = 3176,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 3772,
    /// PROCESS-REF
    ProcessRef                                                             = 3391,
    /// PROCESS-STATE-IREF
    ProcessStateIref                                                       = 2913,
    /// PROCESS-STATE-MACHINE
    ProcessStateMachine                                                    = 1508,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 5768,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 6111,
    /// PROCESS-TO-MACHINE-MAPPINGS
    ProcessToMachineMappings                                               = 3494,
    /// PROCESSING-KIND
    ProcessingKind                                                         = 272,
    /// PROCESSING-STYLE
    ProcessingStyle                                                        = 2791,
    /// PROCESSING-UNIT-REF
    ProcessingUnitRef                                                      = 6400,
    /// PROCESSOR
    Processor                                                              = 1376,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 2127,
    /// PROCESSOR-MODE
    ProcessorMode                                                          = 5853,
    /// PROCESSOR-SPEED
    ProcessorSpeed                                                         = 739,
    /// PROCESSORS
    Processors                                                             = 6288,
    /// PRODUCT-ID
    ProductId                                                              = 5097,
    /// PRODUCT-RELEASE
    ProductRelease                                                         = 3943,
    /// PROFILE-BEHAVIOR
    ProfileBehavior                                                        = 3896,
    /// PROFILE-NAME
    ProfileName                                                            = 5429,
    /// PROG-CODE
    ProgCode                                                               = 3805,
    /// PROGRAMMING-LANGUAGE
    ProgrammingLanguage                                                    = 3379,
    /// PROP-SEG
    PropSeg                                                                = 717,
    /// PROPABILITY
    Propability                                                            = 3470,
    /// PROPAGATION-DELAY
    PropagationDelay                                                       = 1483,
    /// PROPERTYS
    Propertys                                                              = 2043,
    /// PROPS
    Props                                                                  = 5441,
    /// PROTOCOL
    Protocol                                                               = 4835,
    /// PROTOCOL-KIND
    ProtocolKind                                                           = 2412,
    /// PROTOCOL-NAME
    ProtocolName                                                           = 5365,
    /// PROTOCOL-VERSION
    ProtocolVersion                                                        = 2025,
    /// PROVIDE
    Provide                                                                = 3857,
    /// PROVIDE-RESOURCES
    ProvideResources                                                       = 5725,
    /// PROVIDED-CLIENT-SERVER-ENTRYS
    ProvidedClientServerEntrys                                             = 1844,
    /// PROVIDED-COM-SPECS
    ProvidedComSpecs                                                       = 4089,
    /// PROVIDED-DATA-REF
    ProvidedDataRef                                                        = 1910,
    /// PROVIDED-DATAS
    ProvidedDatas                                                          = 5331,
    /// PROVIDED-DDS-EVENT-QOS-PROPS
    ProvidedDdsEventQosProps                                               = 2042,
    /// PROVIDED-DDS-OPERATIONS
    ProvidedDdsOperations                                                  = 1845,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 361,
    /// PROVIDED-DDS-SERVICE-INSTANCE-EVENTS
    ProvidedDdsServiceInstanceEvents                                       = 4770,
    /// PROVIDED-ENTRYS
    ProvidedEntrys                                                         = 998,
    /// PROVIDED-EVENT-GROUPS
    ProvidedEventGroups                                                    = 346,
    /// PROVIDED-EVENTS
    ProvidedEvents                                                         = 3324,
    /// PROVIDED-FIELDS
    ProvidedFields                                                         = 1658,
    /// PROVIDED-INTERFACE-TREF
    ProvidedInterfaceTref                                                  = 5868,
    /// PROVIDED-MEMORY-REF
    ProvidedMemoryRef                                                      = 5409,
    /// PROVIDED-MODE-GROUP-REF
    ProvidedModeGroupRef                                                   = 2477,
    /// PROVIDED-MODE-GROUPS
    ProvidedModeGroups                                                     = 4166,
    /// PROVIDED-OPERATIONS
    ProvidedOperations                                                     = 1726,
    /// PROVIDED-OUTER-PORT-REF
    ProvidedOuterPortRef                                                   = 1437,
    /// PROVIDED-PORT-PROTOTYPE-IREF
    ProvidedPortPrototypeIref                                              = 429,
    /// PROVIDED-REQUIRED-INTERFACE-TREF
    ProvidedRequiredInterfaceTref                                          = 1567,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 2541,
    /// PROVIDED-SERVICE-INSTANCE-ID
    ProvidedServiceInstanceId                                              = 1204,
    /// PROVIDED-SERVICE-INSTANCE-REF
    ProvidedServiceInstanceRef                                             = 5354,
    /// PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL
    ProvidedServiceInstanceRefConditional                                  = 2440,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 5656,
    /// PROVIDED-SERVICE-INSTANCES
    ProvidedServiceInstances                                               = 5482,
    /// PROVIDED-SERVICE-PORT-IREF
    ProvidedServicePortIref                                                = 3307,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 3895,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 3946,
    /// PROVIDER-IREF
    ProviderIref                                                           = 5498,
    /// PROVIDER-RATE-DEVIATION-MAX
    ProviderRateDeviationMax                                               = 5502,
    /// PROVIDER-REF
    ProviderRef                                                            = 4235,
    /// PROVIDER-SOFTWARE-CLUSTER-REF
    ProviderSoftwareClusterRef                                             = 525,
    /// PROVIEDE-DATA-REF
    ProviedeDataRef                                                        = 4325,
    /// PROXIMITY-PROOF-REQUIRED
    ProximityProofRequired                                                 = 4786,
    /// PSK-IDENTITY
    PskIdentity                                                            = 4001,
    /// PSK-IDENTITY-HINT
    PskIdentityHint                                                        = 4419,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 3948,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS
    PskIdentityToKeySlotMappings                                           = 510,
    /// PUBLISHER
    Publisher                                                              = 1474,
    /// PULSE-TEST
    PulseTest                                                              = 298,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 4608,
    /// PURPOSE-OF-UPDATE-REF
    PurposeOfUpdateRef                                                     = 1563,
    /// PUTS
    Puts                                                                   = 2693,
    /// PWM-L
    PwmL                                                                   = 6280,
    /// PWM-O
    PwmO                                                                   = 1443,
    /// PWM-S
    PwmS                                                                   = 359,
    /// QOS-PROFILE
    QosProfile                                                             = 1875,
    /// QUEUE-LENGTH
    QueueLength                                                            = 2321,
    /// QUEUE-SIZE
    QueueSize                                                              = 3030,
    /// QUEUED-RECEIVER-COM-SPEC
    QueuedReceiverComSpec                                                  = 4076,
    /// QUEUED-RECEIVER-COM-SPEC-PROPS
    QueuedReceiverComSpecProps                                             = 1883,
    /// QUEUED-RECEIVER-COM-SPEC-PROPSS
    QueuedReceiverComSpecPropss                                            = 303,
    /// QUEUED-SENDER-COM-SPEC
    QueuedSenderComSpec                                                    = 2320,
    /// R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF
    RModeGroupInAtomicSwcInstanceRef                                       = 1698,
    /// R-PORT-IN-COMPOSITION-INSTANCE-REF
    RPortInCompositionInstanceRef                                          = 214,
    /// R-PORT-IREF
    RPortIref                                                              = 1012,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 2839,
    /// R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF
    RPortPrototypeInExecutableIref                                         = 1160,
    /// R-PORT-PROTOTYPE-IREF
    RPortPrototypeIref                                                     = 1676,
    /// R-PORT-PROTOTYPE-PROPS
    RPortPrototypeProps                                                    = 6634,
    /// R-PORT-PROTOTYPE-REF
    RPortPrototypeRef                                                      = 5481,
    /// R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF
    RTriggerInAtomicSwcInstanceRef                                         = 1079,
    /// RAM-BLOCK
    RamBlock                                                               = 2715,
    /// RAM-BLOCK-INIT-VALUE
    RamBlockInitValue                                                      = 2310,
    /// RAM-BLOCK-STATUS-CONTROL
    RamBlockStatusControl                                                  = 6440,
    /// RAM-LOCATION-REF
    RamLocationRef                                                         = 2348,
    /// RANGE-MAX
    RangeMax                                                               = 1491,
    /// RANGE-MIN
    RangeMin                                                               = 2146,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 189,
    /// RATE-BASED-MONITORED-EVENT-REF
    RateBasedMonitoredEventRef                                             = 5182,
    /// RATE-CORRECTION-MEASUREMENT-DURATION
    RateCorrectionMeasurementDuration                                      = 4106,
    /// RATE-CORRECTION-OUT
    RateCorrectionOut                                                      = 2562,
    /// RATE-CORRECTION-THRESHOLD
    RateCorrectionThreshold                                                = 3894,
    /// RATE-CORRECTIONS-PER-MEASUREMENT-DURATION
    RateCorrectionsPerMeasurementDuration                                  = 1828,
    /// RATE-DEVIATION-MEASUREMENT-DURATION
    RateDeviationMeasurementDuration                                       = 3817,
    /// RATE-LIMITATION-FILTERS
    RateLimitationFilters                                                  = 3850,
    /// RATE-POLICYS
    RatePolicys                                                            = 2339,
    /// RATIO-KIND
    RatioKind                                                              = 3660,
    /// RATIONALE
    Rationale                                                              = 6462,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 87,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 5609,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 4025,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 5284,
    /// RAW-DATA-STREAM-INTERFACE-REF
    RawDataStreamInterfaceRef                                              = 2077,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 4706,
    /// RAW-DATA-STREAM-REF
    RawDataStreamRef                                                       = 5944,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 1457,
    /// REACTION-CODE
    ReactionCode                                                           = 2885,
    /// READ
    Read                                                                   = 4295,
    /// READ-CLASS-REF
    ReadClassRef                                                           = 6068,
    /// READ-DATA-CLASS-REF
    ReadDataClassRef                                                       = 847,
    /// READ-DTC-INFORMATION-CLASS-REF
    ReadDtcInformationClassRef                                             = 427,
    /// READ-LOCAL-VARIABLES
    ReadLocalVariables                                                     = 5739,
    /// READ-NV-DATA
    ReadNvData                                                             = 2413,
    /// READ-SCALING-DATA-CLASS-REF
    ReadScalingDataClassRef                                                = 4437,
    /// READ-STATUS
    ReadStatus                                                             = 3965,
    /// READER-ENTITY-ID
    ReaderEntityId                                                         = 4662,
    /// READONLY
    Readonly                                                               = 3085,
    /// REASON
    Reason                                                                 = 3336,
    /// RECEIVE-ACTIVITY
    ReceiveActivity                                                        = 6211,
    /// RECEIVE-WINDOW-MIN
    ReceiveWindowMin                                                       = 2095,
    /// RECEIVED-DATA-REF
    ReceivedDataRef                                                        = 6305,
    /// RECEIVER-ANNOTATION
    ReceiverAnnotation                                                     = 3367,
    /// RECEIVER-CAPABILITY
    ReceiverCapability                                                     = 1438,
    /// RECEIVER-INTENT
    ReceiverIntent                                                         = 3414,
    /// RECEIVER-IREF
    ReceiverIref                                                           = 3872,
    /// RECEIVER-IREFS
    ReceiverIrefs                                                          = 6480,
    /// RECEIVER-REF
    ReceiverRef                                                            = 2255,
    /// RECEIVER-REFS
    ReceiverRefs                                                           = 3902,
    /// RECEPTION-DEFAULT-VALUES
    ReceptionDefaultValues                                                 = 1310,
    /// RECEPTION-POLICYS
    ReceptionPolicys                                                       = 5383,
    /// RECEPTION-PROPS
    ReceptionProps                                                         = 652,
    /// RECOMMENDED-CONFIGURATION-REF
    RecommendedConfigurationRef                                            = 3465,
    /// RECOMMENDED-CONFIGURATION-REFS
    RecommendedConfigurationRefs                                           = 6115,
    /// RECORD-ELEMENT-MAPPINGS
    RecordElementMappings                                                  = 285,
    /// RECORD-ELEMENT-REF
    RecordElementRef                                                       = 3159,
    /// RECORD-ELEMENTS
    RecordElements                                                         = 931,
    /// RECORD-NUMBER
    RecordNumber                                                           = 2571,
    /// RECORD-VALUE-SPECIFICATION
    RecordValueSpecification                                               = 6433,
    /// RECOVERABLE-IN-SAME-OPERATION-CYCLE
    RecoverableInSameOperationCycle                                        = 2842,
    /// RECOVERY
    Recovery                                                               = 3587,
    /// RECOVERY-ACTION-IREF
    RecoveryActionIref                                                     = 5154,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 3871,
    /// RECOVERY-NOTIFICATION-REF
    RecoveryNotificationRef                                                = 3565,
    /// RECOVERY-NOTIFICATION-REFS
    RecoveryNotificationRefs                                               = 2409,
    /// RECOVERY-NOTIFICATION-RETRY
    RecoveryNotificationRetry                                              = 1320,
    /// RECOVERY-NOTIFICATION-TIMEOUT
    RecoveryNotificationTimeout                                            = 5583,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 2401,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 6564,
    /// RECOVERY-VIA-APPLICATION-ACTION-REF
    RecoveryViaApplicationActionRef                                        = 4398,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 841,
    /// REDUNDANCY
    Redundancy                                                             = 6030,
    /// REDUNDANCY-HANDLINGS
    RedundancyHandlings                                                    = 6580,
    /// REENTRANCY-LEVEL
    ReentrancyLevel                                                        = 990,
    /// REF
    Ref                                                                    = 5638,
    /// REF-CALPRM-SET
    RefCalprmSet                                                           = 5397,
    /// REF-MEASUREMENT-SET
    RefMeasurementSet                                                      = 2604,
    /// REFERENCE-BASE
    ReferenceBase                                                          = 4780,
    /// REFERENCE-BASES
    ReferenceBases                                                         = 260,
    /// REFERENCE-CONDITION
    ReferenceCondition                                                     = 3036,
    /// REFERENCE-REF
    ReferenceRef                                                           = 6364,
    /// REFERENCE-TABLE-REF
    ReferenceTableRef                                                      = 5399,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 2035,
    /// REFERENCE-VALUE-REF
    ReferenceValueRef                                                      = 2491,
    /// REFERENCE-VALUE-SPECIFICATION
    ReferenceValueSpecification                                            = 2347,
    /// REFERENCE-VALUES
    ReferenceValues                                                        = 137,
    /// REFERENCED-BASELINE
    ReferencedBaseline                                                     = 3312,
    /// REFERENCED-TD-EVENT-SWC-REF
    ReferencedTdEventSwcRef                                                = 3505,
    /// REFERENCED-TD-EVENT-VFB-REF
    ReferencedTdEventVfbRef                                                = 4068,
    /// REFERENCES
    References                                                             = 4846,
    /// REFERRABLE-REF
    ReferrableRef                                                          = 3675,
    /// REFILL-AMOUNT
    RefillAmount                                                           = 3752,
    /// REFINED-EVENT-IREF
    RefinedEventIref                                                       = 4602,
    /// REFINED-MODULE-DEF-REF
    RefinedModuleDefRef                                                    = 6630,
    /// REG-DOES-NOT-REQUIRE-STABILITYS
    RegDoesNotRequireStabilitys                                            = 4006,
    /// REG-REQUIRES-STABILITYS
    RegRequiresStabilitys                                                  = 5052,
    /// REGENERATED-PRIORITY
    RegeneratedPriority                                                    = 3487,
    /// REGULAR-EXPRESSION
    RegularExpression                                                      = 258,
    /// REKEY-INTERVAL
    RekeyInterval                                                          = 3504,
    /// RELATED-TRACE-ITEM-REF
    RelatedTraceItemRef                                                    = 6354,
    /// RELATIONS
    Relations                                                              = 5926,
    /// RELATIVE
    Relative                                                               = 2243,
    /// RELATIVE-REPRESENTATION-TIME
    RelativeRepresentationTime                                             = 4084,
    /// RELATIVE-TOLERANCE
    RelativeTolerance                                                      = 1203,
    /// RELEASE-NOTES-REF
    ReleaseNotesRef                                                        = 4589,
    /// RELEASED-TRIGGER-POLICYS
    ReleasedTriggerPolicys                                                 = 4797,
    /// RELEASED-TRIGGER-REF
    ReleasedTriggerRef                                                     = 3154,
    /// RELEASED-TRIGGERS
    ReleasedTriggers                                                       = 1750,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF
    RelevantForDynamicPncMappingRef                                        = 5435,
    /// RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS
    RelevantForDynamicPncMappingRefs                                       = 6144,
    /// RELIABILITY
    Reliability                                                            = 418,
    /// RELIABILITY-KIND
    ReliabilityKind                                                        = 433,
    /// RELIABILITY-MAX-BLOCKING-TIME
    ReliabilityMaxBlockingTime                                             = 979,
    /// REMAPPED-CAN-ID
    RemappedCanId                                                          = 2452,
    /// REMARK
    Remark                                                                 = 1578,
    /// REMOTE-ACCESS-CONTROL-ENABLED
    RemoteAccessControlEnabled                                             = 2393,
    /// REMOTE-ADDRESSS
    RemoteAddresss                                                         = 682,
    /// REMOTE-CERTIFICATE-REF
    RemoteCertificateRef                                                   = 4300,
    /// REMOTE-CERTIFICATE-REFS
    RemoteCertificateRefs                                                  = 4561,
    /// REMOTE-CLIENT-CONFIG
    RemoteClientConfig                                                     = 139,
    /// REMOTE-CLIENT-QUEUE-LENGTH-CONTRIBUTION
    RemoteClientQueueLengthContribution                                    = 1114,
    /// REMOTE-CONFIGS
    RemoteConfigs                                                          = 234,
    /// REMOTE-ENDPOINT-CONFIGURATION
    RemoteEndpointConfiguration                                            = 6126,
    /// REMOTE-ID
    RemoteId                                                               = 4031,
    /// REMOTE-IP-ADDRESS-REF
    RemoteIpAddressRef                                                     = 4855,
    /// REMOTE-IP-ADDRESS-REFS
    RemoteIpAddressRefs                                                    = 3486,
    /// REMOTE-MODE-SENDER-QUEUE-LENGTH-CONTRIBUTION
    RemoteModeSenderQueueLengthContribution                                = 5110,
    /// REMOTE-MULTICAST-CONFIG-REF
    RemoteMulticastConfigRef                                               = 656,
    /// REMOTE-MULTICAST-CONFIG-REFS
    RemoteMulticastConfigRefs                                              = 5280,
    /// REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS
    RemoteMulticastSubscriptionAddresss                                    = 2850,
    /// REMOTE-NETWORK-ENDPOINT-REF
    RemoteNetworkEndpointRef                                               = 4397,
    /// REMOTE-PORT-RANGE-END
    RemotePortRangeEnd                                                     = 4048,
    /// REMOTE-PORT-RANGE-START
    RemotePortRangeStart                                                   = 5563,
    /// REMOTE-PORT-REF
    RemotePortRef                                                          = 5461,
    /// REMOTE-SENDER-QUEUE-LENGTH-CONTRIBUTION
    RemoteSenderQueueLengthContribution                                    = 5953,
    /// REMOTE-SERVER-CONFIG
    RemoteServerConfig                                                     = 3368,
    /// REMOTE-SUBJECT-REF
    RemoteSubjectRef                                                       = 1738,
    /// REMOTE-SUBJECT-REFS
    RemoteSubjectRefs                                                      = 3661,
    /// REMOTE-UNICAST-ADDRESSS
    RemoteUnicastAddresss                                                  = 4904,
    /// REMOTE-UNICAST-CONFIG-REF
    RemoteUnicastConfigRef                                                 = 402,
    /// REMOTE-UNICAST-CONFIG-REFS
    RemoteUnicastConfigRefs                                                = 2952,
    /// REMOTING-TECHNOLOGY
    RemotingTechnology                                                     = 674,
    /// REPETITION-PERIOD
    RepetitionPeriod                                                       = 3473,
    /// REPLACE-WITH
    ReplaceWith                                                            = 995,
    /// REPLAY-PROTECTION
    ReplayProtection                                                       = 6028,
    /// REPLAY-PROTECTION-WINDOW
    ReplayProtectionWindow                                                 = 2458,
    /// REPLICA-PDUS-REF
    ReplicaPdusRef                                                         = 3863,
    /// REPLICA-PDUS-REFS
    ReplicaPdusRefs                                                        = 589,
    /// REPLY-TOPIC-NAME
    ReplyTopicName                                                         = 3380,
    /// REPORT-BEHAVIOR
    ReportBehavior                                                         = 3334,
    /// REPORTABLE-SECURITY-EVENT-REF
    ReportableSecurityEventRef                                             = 2714,
    /// REPORTABLE-SECURITY-EVENT-REFS
    ReportableSecurityEventRefs                                            = 3534,
    /// REPORTED-SECURITY-EVENT-IREF
    ReportedSecurityEventIref                                              = 5165,
    /// REPORTING-BEHAVIOR
    ReportingBehavior                                                      = 805,
    /// REPORTING-PORT-PROTOTYPE-IREF
    ReportingPortPrototypeIref                                             = 755,
    /// REPOSITORY
    Repository                                                             = 3822,
    /// REPRESENTED-MODEL-ELEMENT-REF
    RepresentedModelElementRef                                             = 24,
    /// REPRESENTED-PORT-GROUP-REF
    RepresentedPortGroupRef                                                = 4524,
    /// REPRESENTS-VIN
    RepresentsVin                                                          = 6202,
    /// REQUEST-2-SUPPORT
    Request2Support                                                        = 2992,
    /// REQUEST-ARGUMENT-REF
    RequestArgumentRef                                                     = 6592,
    /// REQUEST-CONFIGURATION-DESIGNS
    RequestConfigurationDesigns                                            = 5050,
    /// REQUEST-CONFIGURATIONS
    RequestConfigurations                                                  = 4740,
    /// REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF
    RequestControlOfOnBoardDeviceClassRef                                  = 4123,
    /// REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF
    RequestCurrentPowertrainDiagnosticDataClassRef                         = 5005,
    /// REQUEST-DATA-SIZE
    RequestDataSize                                                        = 974,
    /// REQUEST-DOWNLOAD-CLASS-REF
    RequestDownloadClassRef                                                = 6201,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF
    RequestEmissionRelatedDtcClassPermanentStatusRef                       = 6561,
    /// REQUEST-EMISSION-RELATED-DTC-CLASS-REF
    RequestEmissionRelatedDtcClassRef                                      = 2334,
    /// REQUEST-FILE-TRANSFER-CLASS-REF
    RequestFileTransferClassRef                                            = 3993,
    /// REQUEST-GROUP-REF
    RequestGroupRef                                                        = 1364,
    /// REQUEST-METHOD
    RequestMethod                                                          = 2522,
    /// REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF
    RequestOnBoardMonitoringTestResultsClassRef                            = 4185,
    /// REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF
    RequestPowertrainFreezeFrameDataRef                                    = 6208,
    /// REQUEST-RESPONSE-DELAY
    RequestResponseDelay                                                   = 4920,
    /// REQUEST-RESULT
    RequestResult                                                          = 5214,
    /// REQUEST-SEED-ID
    RequestSeedId                                                          = 3796,
    /// REQUEST-TOPIC-NAME
    RequestTopicName                                                       = 5129,
    /// REQUEST-TYPE
    RequestType                                                            = 5130,
    /// REQUEST-UPLOAD-CLASS-REF
    RequestUploadClassRef                                                  = 489,
    /// REQUEST-VEHICLE-INFORMATION-CLASS-REF
    RequestVehicleInformationClassRef                                      = 4813,
    /// REQUESTABLE
    Requestable                                                            = 257,
    /// REQUESTED-MODE-IREF
    RequestedModeIref                                                      = 2031,
    /// REQUESTER-IREF
    RequesterIref                                                          = 2852,
    /// REQUESTER-REF
    RequesterRef                                                           = 3924,
    /// REQUESTER-REFS
    RequesterRefs                                                          = 3735,
    /// REQUESTER-SOFTWARE-CLUSTER-REF
    RequesterSoftwareClusterRef                                            = 469,
    /// REQUESTOR-REF
    RequestorRef                                                           = 4513,
    /// REQUESTOR-REFS
    RequestorRefs                                                          = 3400,
    /// REQUESTS
    Requests                                                               = 4922,
    /// REQUIRE-RESOURCES
    RequireResources                                                       = 1815,
    /// REQUIRED-AR-ELEMENT-REF
    RequiredArElementRef                                                   = 3642,
    /// REQUIRED-AR-ELEMENT-REFS
    RequiredArElementRefs                                                  = 1915,
    /// REQUIRED-ARTIFACTS
    RequiredArtifacts                                                      = 4032,
    /// REQUIRED-CLIENT-SERVER-ENTRY-REF
    RequiredClientServerEntryRef                                           = 4897,
    /// REQUIRED-CLIENT-SERVER-ENTRYS
    RequiredClientServerEntrys                                             = 2687,
    /// REQUIRED-COM-SPECS
    RequiredComSpecs                                                       = 649,
    /// REQUIRED-DATAS
    RequiredDatas                                                          = 2809,
    /// REQUIRED-DDS-EVENT-QOS-PROPS
    RequiredDdsEventQosProps                                               = 5651,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 5465,
    /// REQUIRED-DESIGN-ELEMENT-REF
    RequiredDesignElementRef                                               = 797,
    /// REQUIRED-DESIGN-ELEMENT-REFS
    RequiredDesignElementRefs                                              = 5894,
    /// REQUIRED-ENTRYS
    RequiredEntrys                                                         = 3801,
    /// REQUIRED-ENVIRONMENT-REF
    RequiredEnvironmentRef                                                 = 3775,
    /// REQUIRED-EVENT-GROUPS
    RequiredEventGroups                                                    = 6298,
    /// REQUIRED-FIBEX-ELEMENT-REF
    RequiredFibexElementRef                                                = 4098,
    /// REQUIRED-FIBEX-ELEMENT-REFS
    RequiredFibexElementRefs                                               = 857,
    /// REQUIRED-GENERATOR-TOOLS
    RequiredGeneratorTools                                                 = 3337,
    /// REQUIRED-INTERFACE-TREF
    RequiredInterfaceTref                                                  = 5968,
    /// REQUIRED-MINOR-VERSION
    RequiredMinorVersion                                                   = 5865,
    /// REQUIRED-MODE-GROUP-REF
    RequiredModeGroupRef                                                   = 4688,
    /// REQUIRED-MODE-GROUPS
    RequiredModeGroups                                                     = 1504,
    /// REQUIRED-OUTER-PORT-REF
    RequiredOuterPortRef                                                   = 4250,
    /// REQUIRED-PACKAGE-ELEMENT-REF
    RequiredPackageElementRef                                              = 3819,
    /// REQUIRED-PACKAGE-ELEMENT-REFS
    RequiredPackageElementRefs                                             = 3690,
    /// REQUIRED-PORT-PROTOTYPE-IREF
    RequiredPortPrototypeIref                                              = 445,
    /// REQUIRED-RTE-VENDOR
    RequiredRteVendor                                                      = 1893,
    /// REQUIRED-SERVICE-INSTANCE-ID
    RequiredServiceInstanceId                                              = 348,
    /// REQUIRED-SERVICE-INSTANCE-REF
    RequiredServiceInstanceRef                                             = 5764,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 36,
    /// REQUIRED-SERVICE-PORT-IREF
    RequiredServicePortIref                                                = 1581,
    /// REQUIRED-SERVICE-VERSION
    RequiredServiceVersion                                                 = 3358,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 757,
    /// REQUIRED-TRIGGER-IREF
    RequiredTriggerIref                                                    = 2637,
    /// REQUIRED-TRIGGERS
    RequiredTriggers                                                       = 286,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 5747,
    /// REQUIREMENT-REF
    RequirementRef                                                         = 4807,
    /// REQUIREMENT-REFS
    RequirementRefs                                                        = 1717,
    /// REQUIRES-INDEX
    RequiresIndex                                                          = 3921,
    /// REQUIRES-SYMBOLIC-NAME-VALUE
    RequiresSymbolicNameValue                                              = 4574,
    /// RESET-CONFIRMED-BIT-ON-OVERFLOW
    ResetConfirmedBitOnOverflow                                            = 2481,
    /// RESET-PENDING-BIT-ON-OVERFLOW
    ResetPendingBitOnOverflow                                              = 5901,
    /// RESET-TO-DEFAULT
    ResetToDefault                                                         = 2284,
    /// RESET-TO-DEFAULT-SUPPORTED
    ResetToDefaultSupported                                                = 695,
    /// RESISTANT-TO-CHANGED-SW
    ResistantToChangedSw                                                   = 4650,
    /// RESOLUTION
    Resolution                                                             = 3576,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 4086,
    /// RESOURCE-DEFINITION-REF
    ResourceDefinitionRef                                                  = 3314,
    /// RESOURCE-DEFINITIONS
    ResourceDefinitions                                                    = 4102,
    /// RESOURCE-ESTIMATIONS
    ResourceEstimations                                                    = 774,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 3753,
    /// RESOURCE-GROUP-REF
    ResourceGroupRef                                                       = 4592,
    /// RESOURCE-GROUP-REFS
    ResourceGroupRefs                                                      = 3643,
    /// RESOURCE-GROUPS
    ResourceGroups                                                         = 893,
    /// RESOURCE-GUARD-VALUE
    ResourceGuardValue                                                     = 4418,
    /// RESOURCE-IDENTIFIER-TYPE
    ResourceIdentifierType                                                 = 6526,
    /// RESOURCE-LIMITS
    ResourceLimits                                                         = 4569,
    /// RESOURCE-NEEDS-REF
    ResourceNeedsRef                                                       = 4758,
    /// RESOURCE-NEEDS-REFS
    ResourceNeedsRefs                                                      = 4538,
    /// RESOURCE-REF
    ResourceRef                                                            = 5800,
    /// RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS
    ResourceToApplicationPartitionMappings                                 = 6050,
    /// RESOURCES
    Resources                                                              = 1651,
    /// RESPOND-TO-RESET
    RespondToReset                                                         = 6320,
    /// RESPONSE-ARGUMENT-REF
    ResponseArgumentRef                                                    = 1404,
    /// RESPONSE-DATA-SIZE
    ResponseDataSize                                                       = 5542,
    /// RESPONSE-ERROR-POSITION
    ResponseErrorPosition                                                  = 6566,
    /// RESPONSE-ERROR-REF
    ResponseErrorRef                                                       = 3981,
    /// RESPONSE-GROUP-REF
    ResponseGroupRef                                                       = 134,
    /// RESPONSE-ON-ALL-REQUEST-SIDS
    ResponseOnAllRequestSids                                               = 2074,
    /// RESPONSE-ON-EVENT-ACTION
    ResponseOnEventAction                                                  = 5056,
    /// RESPONSE-ON-EVENT-CLASS-REF
    ResponseOnEventClassRef                                                = 6384,
    /// RESPONSE-ON-EVENT-REF
    ResponseOnEventRef                                                     = 1622,
    /// RESPONSE-ON-EVENT-SCHEDULER-RATE
    ResponseOnEventSchedulerRate                                           = 330,
    /// RESPONSE-ON-SECOND-DECLINED-REQUEST
    ResponseOnSecondDeclinedRequest                                        = 3298,
    /// RESPONSE-REF
    ResponseRef                                                            = 3198,
    /// RESPONSES
    Responses                                                              = 4252,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 1070,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 3537,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 4854,
    /// REST-ENDPOINT-ARGUMENT
    RestEndpointArgument                                                   = 2887,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 2893,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 1957,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 4280,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 1969,
    /// REST-ENUMERATOR-DEF
    RestEnumeratorDef                                                      = 4241,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 2523,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 1442,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 1762,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 3737,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 5577,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 4059,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 6627,
    /// REST-SYSTEM-TRIGGERED-EVENT
    RestSystemTriggeredEvent                                               = 2242,
    /// RESTORE-AT-START
    RestoreAtStart                                                         = 1505,
    /// RESTRICT-UPDATE
    RestrictUpdate                                                         = 2406,
    /// RESTRICTION
    Restriction                                                            = 3693,
    /// RESTRICTIONS
    Restrictions                                                           = 689,
    /// RESULTING-PROPERTIES
    ResultingProperties                                                    = 2860,
    /// RESULTING-RPT-SW-PROTOTYPING-ACCESS
    ResultingRptSwPrototypingAccess                                        = 2428,
    /// RESUME-POSITION
    ResumePosition                                                         = 637,
    /// RETRY
    Retry                                                                  = 1280,
    /// RETRY-INTERVAL-TIME
    RetryIntervalTime                                                      = 3155,
    /// RETURN-CODE
    ReturnCode                                                             = 1169,
    /// RETURN-NO-NEW-DATA-ENABLED
    ReturnNoNewDataEnabled                                                 = 6339,
    /// RETURN-SIGNAL-REF
    ReturnSignalRef                                                        = 4702,
    /// RETURN-SIGNAL-TRIGGERING-REF
    ReturnSignalTriggeringRef                                              = 3845,
    /// RETURN-TYPE
    ReturnType                                                             = 1705,
    /// RETURN-VALUE-PROVISION
    ReturnValueProvision                                                   = 508,
    /// REVERSED-TP-SDU-REF
    ReversedTpSduRef                                                       = 5522,
    /// REVISION-LABEL
    RevisionLabel                                                          = 3977,
    /// REVISION-LABEL-P-1
    RevisionLabelP1                                                        = 2055,
    /// REVISION-LABEL-P-2
    RevisionLabelP2                                                        = 2178,
    /// REVISION-LABELS
    RevisionLabels                                                         = 154,
    /// RID-NUMBER
    RidNumber                                                              = 2639,
    /// ROLE
    Role                                                                   = 2590,
    /// ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT
    RoleBasedBswModuleEntryAssignment                                      = 1654,
    /// ROLE-BASED-DATA-ASSIGNMENT
    RoleBasedDataAssignment                                                = 909,
    /// ROLE-BASED-DATA-TYPE-ASSIGNMENT
    RoleBasedDataTypeAssignment                                            = 3092,
    /// ROLE-BASED-MC-DATA-ASSIGNMENT
    RoleBasedMcDataAssignment                                              = 1796,
    /// ROLE-BASED-PORT-ASSIGNMENT
    RoleBasedPortAssignment                                                = 1173,
    /// ROLE-BASED-RESOURCE-DEPENDENCY
    RoleBasedResourceDependency                                            = 6026,
    /// ROLLOUT-QUALIFICATIONS
    RolloutQualifications                                                  = 833,
    /// ROM-BLOCK
    RomBlock                                                               = 6046,
    /// ROM-BLOCK-INIT-VALUE
    RomBlockInitValue                                                      = 5420,
    /// ROM-LOCATION-REF
    RomLocationRef                                                         = 157,
    /// ROOT-ARGUMENT-DATA-PROTOTYPE-REF
    RootArgumentDataPrototypeRef                                           = 3961,
    /// ROOT-AUTOSAR-DATA-PROTOTYPE-REF
    RootAutosarDataPrototypeRef                                            = 5440,
    /// ROOT-COMPOSITION
    RootComposition                                                        = 4152,
    /// ROOT-CONTEXT-REF
    RootContextRef                                                         = 1871,
    /// ROOT-DATA-PROTOTYPE-IN-CS-REF
    RootDataPrototypeInCsRef                                               = 2220,
    /// ROOT-DATA-PROTOTYPE-IN-SR-REF
    RootDataPrototypeInSrRef                                               = 1453,
    /// ROOT-DATA-PROTOTYPE-REF
    RootDataPrototypeRef                                                   = 2814,
    /// ROOT-PARAMETER-DATA-PROTOTYPE-REF
    RootParameterDataPrototypeRef                                          = 2670,
    /// ROOT-REF
    RootRef                                                                = 4926,
    /// ROOT-SOFTWARE-COMPOSITIONS
    RootSoftwareCompositions                                               = 1527,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1571,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 6417,
    /// ROOT-VARIABLE-DATA-PROTOTYPE-REF
    RootVariableDataPrototypeRef                                           = 2851,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 1711,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 2498,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 5430,
    /// ROUTINE-CONTROL-CLASS-REF
    RoutineControlClassRef                                                 = 2207,
    /// ROUTINE-INFO
    RoutineInfo                                                            = 891,
    /// ROUTINE-REF
    RoutineRef                                                             = 1939,
    /// ROUTINE-SUBFUNCTION-REF
    RoutineSubfunctionRef                                                  = 2925,
    /// ROUTING-ACTIVATION-TYPE
    RoutingActivationType                                                  = 1752,
    /// ROUTING-GROUP-REF
    RoutingGroupRef                                                        = 138,
    /// ROUTING-GROUP-REFS
    RoutingGroupRefs                                                       = 4208,
    /// ROW
    Row                                                                    = 2072,
    /// RP-IMPL-POLICY
    RpImplPolicy                                                           = 945,
    /// RPT-AR-HOOK-IREF
    RptArHookIref                                                          = 5026,
    /// RPT-COMPONENT
    RptComponent                                                           = 2098,
    /// RPT-COMPONENTS
    RptComponents                                                          = 6226,
    /// RPT-CONTAINER
    RptContainer                                                           = 2092,
    /// RPT-CONTAINERS
    RptContainers                                                          = 2951,
    /// RPT-ENABLER-IMPL-TYPE
    RptEnablerImplType                                                     = 596,
    /// RPT-EVENT-ID
    RptEventId                                                             = 3781,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 3928,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 895,
    /// RPT-EXECUTABLE-ENTITY-EVENTS
    RptExecutableEntityEvents                                              = 1526,
    /// RPT-EXECUTABLE-ENTITY-PROPERTIES
    RptExecutableEntityProperties                                          = 2140,
    /// RPT-EXECUTABLE-ENTITYS
    RptExecutableEntitys                                                   = 4495,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 248,
    /// RPT-EXECUTION-CONTROL
    RptExecutionControl                                                    = 1396,
    /// RPT-HOOK
    RptHook                                                                = 6455,
    /// RPT-HOOK-ACCESS
    RptHookAccess                                                          = 4756,
    /// RPT-HOOKS
    RptHooks                                                               = 1566,
    /// RPT-IMPL-POLICY
    RptImplPolicy                                                          = 3480,
    /// RPT-PREPARATION-LEVEL
    RptPreparationLevel                                                    = 3023,
    /// RPT-PROFILE
    RptProfile                                                             = 6488,
    /// RPT-PROFILES
    RptProfiles                                                            = 5714,
    /// RPT-READ-ACCESS
    RptReadAccess                                                          = 74,
    /// RPT-READS
    RptReads                                                               = 5017,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 5817,
    /// RPT-SERVICE-POINT-POST-REF
    RptServicePointPostRef                                                 = 2574,
    /// RPT-SERVICE-POINT-POST-REFS
    RptServicePointPostRefs                                                = 3600,
    /// RPT-SERVICE-POINT-PRE-REF
    RptServicePointPreRef                                                  = 1615,
    /// RPT-SERVICE-POINT-PRE-REFS
    RptServicePointPreRefs                                                 = 26,
    /// RPT-SERVICE-POINTS
    RptServicePoints                                                       = 3306,
    /// RPT-SUPPORT-DATA
    RptSupportData                                                         = 3031,
    /// RPT-SW-PROTOTYPING-ACCESS
    RptSwPrototypingAccess                                                 = 5024,
    /// RPT-SYSTEM-REF
    RptSystemRef                                                           = 5343,
    /// RPT-WRITE-ACCESS
    RptWriteAccess                                                         = 5406,
    /// RPT-WRITES
    RptWrites                                                              = 4013,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 5527,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 5576,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 2132,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 3644,
    /// RTE-EVENT-IREF
    RteEventIref                                                           = 4476,
    /// RTE-EVENT-IREFS
    RteEventIrefs                                                          = 4190,
    /// RTE-EVENT-SEPARATIONS
    RteEventSeparations                                                    = 2894,
    /// RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS
    RteEventToOsTaskProxyMappings                                          = 2889,
    /// RTE-PLUGIN-PROPS
    RtePluginProps                                                         = 3572,
    /// RTE-RESOURCE-ESTIMATION
    RteResourceEstimation                                                  = 1787,
    /// RTP-TP
    RtpTp                                                                  = 6183,
    /// RTPS-PROTECTION-KIND
    RtpsProtectionKind                                                     = 4542,
    /// RULE
    Rule                                                                   = 4968,
    /// RULE-ARGUMENTS
    RuleArguments                                                          = 2904,
    /// RULE-BASED-AXIS-CONT
    RuleBasedAxisCont                                                      = 3409,
    /// RULE-BASED-VALUES
    RuleBasedValues                                                        = 4217,
    /// RULE-INIT-STATE
    RuleInitState                                                          = 3256,
    /// RULES
    Rules                                                                  = 3197,
    /// RUN-MODE
    RunMode                                                                = 1024,
    /// RUN-RESOURCE
    RunResource                                                            = 2023,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 5689,
    /// RUNNABLE-ENTITY-ARGUMENT
    RunnableEntityArgument                                                 = 1997,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 553,
    /// RUNNABLE-ENTITY-GROUP-IREF
    RunnableEntityGroupIref                                                = 2160,
    /// RUNNABLE-ENTITY-GROUP-IREFS
    RunnableEntityGroupIrefs                                               = 3597,
    /// RUNNABLE-ENTITY-IREF
    RunnableEntityIref                                                     = 1002,
    /// RUNNABLE-ENTITY-IREFS
    RunnableEntityIrefs                                                    = 238,
    /// RUNNABLE-MAPPINGS
    RunnableMappings                                                       = 4069,
    /// RUNNABLE-REF
    RunnableRef                                                            = 519,
    /// RUNNABLES
    Runnables                                                              = 2651,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REF
    RunsInsideExclusiveAreaRef                                             = 4597,
    /// RUNS-INSIDE-EXCLUSIVE-AREA-REFS
    RunsInsideExclusiveAreaRefs                                            = 5120,
    /// RUNS-INSIDES
    RunsInsides                                                            = 6645,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 2218,
    /// RUNTIME-IP-ADDRESS-CONFIGURATION
    RuntimeIpAddressConfiguration                                          = 2392,
    /// RUNTIME-PORT-CONFIGURATION
    RuntimePortConfiguration                                               = 3815,
    /// RVF-ACTIVE-PIXELS
    RvfActivePixels                                                        = 1459,
    /// RVF-COLOR-SPACE
    RvfColorSpace                                                          = 1060,
    /// RVF-EVENT-DEFAULT
    RvfEventDefault                                                        = 2110,
    /// RVF-FRAME-RATE
    RvfFrameRate                                                           = 3562,
    /// RVF-INTERLACED
    RvfInterlaced                                                          = 1235,
    /// RVF-PIXEL-DEPTH
    RvfPixelDepth                                                          = 4763,
    /// RVF-PIXEL-FORMAT
    RvfPixelFormat                                                         = 6058,
    /// RVF-TOTAL-LINES
    RvfTotalLines                                                          = 3009,
    /// RX-ACCEPT-CONTAINED-I-PDU
    RxAcceptContainedIPdu                                                  = 3533,
    /// RX-CYCLE-PROCESS-TIME
    RxCycleProcessTime                                                     = 229,
    /// RX-IDENTIFIER-RANGE
    RxIdentifierRange                                                      = 5985,
    /// RX-MASK
    RxMask                                                                 = 4659,
    /// RX-NM-PDU-REF
    RxNmPduRef                                                             = 5871,
    /// RX-NM-PDU-REFS
    RxNmPduRefs                                                            = 5135,
    /// RX-PDU-POOL-REF
    RxPduPoolRef                                                           = 6247,
    /// RX-PDU-TRIGGERING-REF
    RxPduTriggeringRef                                                     = 1064,
    /// RX-SECURITY-VERIFICATION
    RxSecurityVerification                                                 = 2278,
    /// RX-TIMEOUT-TIME
    RxTimeoutTime                                                          = 3477,
    /// S-3-SERVER-TIMEOUT
    S3ServerTimeout                                                        = 750,
    /// SA-OVER-TIME
    SaOverTime                                                             = 1284,
    /// SA-RAND-TIME
    SaRandTime                                                             = 6133,
    /// SA-REKEY-TIME
    SaRekeyTime                                                            = 3315,
    /// SAFE-TRANSLATION
    SafeTranslation                                                        = 3295,
    /// SAFETY-CONDITION
    SafetyCondition                                                        = 978,
    /// SAFETY-CONDITIONS
    SafetyConditions                                                       = 461,
    /// SAFETY-MARGIN
    SafetyMargin                                                           = 6380,
    /// SAFETY-POLICY
    SafetyPolicy                                                           = 5723,
    /// SAK-REF
    SakRef                                                                 = 1595,
    /// SAK-REKEY-TIME-SPAN
    SakRekeyTimeSpan                                                       = 4370,
    /// SAMPLE-CLOCK-PERIOD
    SampleClockPeriod                                                      = 6647,
    /// SAMPLES-PER-MICROTICK
    SamplesPerMicrotick                                                    = 6277,
    /// SAVE-CONFIGURATION
    SaveConfiguration                                                      = 2112,
    /// SAVE-CONFIGURATION-ENTRY
    SaveConfigurationEntry                                                 = 398,
    /// SCALE-CONSTR
    ScaleConstr                                                            = 3744,
    /// SCALE-CONSTRS
    ScaleConstrs                                                           = 5558,
    /// SCALING-INFO-SIZE
    ScalingInfoSize                                                        = 4527,
    /// SCHEDULE-CHANGE-NEXT-TIME-BASE
    ScheduleChangeNextTimeBase                                             = 5351,
    /// SCHEDULE-TABLES
    ScheduleTables                                                         = 665,
    /// SCHEDULER-MAX-NUMBER
    SchedulerMaxNumber                                                     = 3378,
    /// SCHEDULER-NAME-PREFIX-REF
    SchedulerNamePrefixRef                                                 = 5824,
    /// SCHEDULER-NAME-PREFIXS
    SchedulerNamePrefixs                                                   = 5622,
    /// SCHEDULING-POLICY
    SchedulingPolicy                                                       = 4796,
    /// SCHEDULING-PRIORITY
    SchedulingPriority                                                     = 5819,
    /// SCOPE
    Scope                                                                  = 4162,
    /// SCOPE-EVENT-REF
    ScopeEventRef                                                          = 1989,
    /// SCOPE-EVENT-REFS
    ScopeEventRefs                                                         = 1518,
    /// SCOPE-REF
    ScopeRef                                                               = 969,
    /// SCOPE-REFS
    ScopeRefs                                                              = 1557,
    /// SD
    Sd                                                                     = 3836,
    /// SD-CLIENT-CONFIG
    SdClientConfig                                                         = 4845,
    /// SD-CLIENT-CONFIG-REF
    SdClientConfigRef                                                      = 4705,
    /// SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SdClientEventGroupTimingConfigRef                                      = 4072,
    /// SD-CLIENT-EVENT-TIMING-CONFIG
    SdClientEventTimingConfig                                              = 4109,
    /// SD-CLIENT-TIMER-CONFIGS
    SdClientTimerConfigs                                                   = 3911,
    /// SD-SERVER-CONFIG
    SdServerConfig                                                         = 2636,
    /// SD-SERVER-CONFIG-REF
    SdServerConfigRef                                                      = 5474,
    /// SD-SERVER-EG-TIMING-CONFIGS
    SdServerEgTimingConfigs                                                = 2767,
    /// SD-SERVER-EVENT-CONFIG
    SdServerEventConfig                                                    = 4825,
    /// SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SdServerEventGroupTimingConfigRef                                      = 3848,
    /// SD-SERVER-TIMER-CONFIGS
    SdServerTimerConfigs                                                   = 11,
    /// SDF
    Sdf                                                                    = 2813,
    /// SDG
    Sdg                                                                    = 364,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 2943,
    /// SDG-CAPTION
    SdgCaption                                                             = 1329,
    /// SDG-CAPTION-REF
    SdgCaptionRef                                                          = 922,
    /// SDG-CLASS
    SdgClass                                                               = 5662,
    /// SDG-CLASS-REF
    SdgClassRef                                                            = 2304,
    /// SDG-CLASSES
    SdgClasses                                                             = 3623,
    /// SDG-CONSTRAINT-REF
    SdgConstraintRef                                                       = 558,
    /// SDG-CONSTRAINT-REFS
    SdgConstraintRefs                                                      = 2299,
    /// SDG-DEF
    SdgDef                                                                 = 2527,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 3276,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 813,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 4967,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 281,
    /// SDG-REFERENCE
    SdgReference                                                           = 6589,
    /// SDG-TAILORING
    SdgTailoring                                                           = 5683,
    /// SDG-TAILORINGS
    SdgTailorings                                                          = 5286,
    /// SDGS
    Sdgs                                                                   = 5989,
    /// SDU-REF
    SduRef                                                                 = 3264,
    /// SDU-REFS
    SduRefs                                                                = 6184,
    /// SDU-TYPE
    SduType                                                                = 2088,
    /// SDX-REF
    SdxRef                                                                 = 3239,
    /// SDXF
    Sdxf                                                                   = 1892,
    /// SEARCH-BEHAVIOR
    SearchBehavior                                                         = 6495,
    /// SEARCH-INTENTION
    SearchIntention                                                        = 3809,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REF
    SecOcComPropsForMulticastRef                                           = 6114,
    /// SEC-OC-COM-PROPS-FOR-MULTICAST-REFS
    SecOcComPropsForMulticastRefs                                          = 3800,
    /// SEC-OC-CRYPTO-MAPPING-REF
    SecOcCryptoMappingRef                                                  = 3844,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 2151,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING-REF
    SecOcCryptoServiceMappingRef                                           = 52,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING-REF-CONDITIONAL
    SecOcCryptoServiceMappingRefConditional                                = 2235,
    /// SEC-OC-CRYPTO-SERVICE-MAPPINGS
    SecOcCryptoServiceMappings                                             = 5329,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 4388,
    /// SEC-OC-IGNORE-VERIFICATION-RESULT
    SecOcIgnoreVerificationResult                                          = 2209,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 5201,
    /// SEC-OC-JOB-MAPPINGS
    SecOcJobMappings                                                       = 5835,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 5078,
    /// SEC-OC-JOB-REQUIREMENT-REF
    SecOcJobRequirementRef                                                 = 5149,
    /// SEC-OC-JOB-SEMANTIC
    SecOcJobSemantic                                                       = 639,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 6490,
    /// SEC-OC-SECURE-COM-PROPS-REF
    SecOcSecureComPropsRef                                                 = 102,
    /// SEC-OC-SECURE-COM-PROPS-REF-CONDITIONAL
    SecOcSecureComPropsRefConditional                                      = 1055,
    /// SECOND-APPLICATION-ERROR-REF
    SecondApplicationErrorRef                                              = 59,
    /// SECOND-DATA-PROTOTYPE-REF
    SecondDataPrototypeRef                                                 = 1855,
    /// SECOND-ELEMENT-INSTANCE-IREF
    SecondElementInstanceIref                                              = 5391,
    /// SECOND-ELEMENT-INSTANCE-IREFS
    SecondElementInstanceIrefs                                             = 4997,
    /// SECOND-ELEMENT-REF
    SecondElementRef                                                       = 547,
    /// SECOND-ELEMENT-REFS
    SecondElementRefs                                                      = 541,
    /// SECOND-ELEMENTS
    SecondElements                                                         = 2744,
    /// SECOND-KEY-SLOT-ID
    SecondKeySlotId                                                        = 1788,
    /// SECOND-MODE-GROUP-REF
    SecondModeGroupRef                                                     = 5006,
    /// SECOND-MODE-REF
    SecondModeRef                                                          = 4441,
    /// SECOND-OPERATION-REF
    SecondOperationRef                                                     = 6330,
    /// SECOND-PHYSICAL-DIMENSION-REF
    SecondPhysicalDimensionRef                                             = 2532,
    /// SECOND-PORT-REF
    SecondPortRef                                                          = 1906,
    /// SECOND-TO-FIRST-DATA-TRANSFORMATION-REF
    SecondToFirstDataTransformationRef                                     = 97,
    /// SECOND-TRIGGER-REF
    SecondTriggerRef                                                       = 1831,
    /// SECOND-VALUE
    SecondValue                                                            = 3384,
    /// SECONDARY-DOWNLOAD-URIS
    SecondaryDownloadUris                                                  = 3835,
    /// SECONDARY-FRESHNESS-VALUE-ID
    SecondaryFreshnessValueId                                              = 2660,
    /// SECTION-INITIALIZATION-POLICY
    SectionInitializationPolicy                                            = 6106,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 4326,
    /// SECTION-NAME-PREFIXS
    SectionNamePrefixs                                                     = 851,
    /// SECTION-TYPE
    SectionType                                                            = 6216,
    /// SECURE-COM-CONFIG-REF
    SecureComConfigRef                                                     = 5525,
    /// SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING
    SecureComConfigToCryptoKeySlotMapping                                  = 333,
    /// SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS
    SecureComConfigToKeySlotMappings                                       = 4540,
    /// SECURE-COM-CONFIGS
    SecureComConfigs                                                       = 4039,
    /// SECURE-COM-PROPS-FOR-DDS-REF
    SecureComPropsForDdsRef                                                = 4875,
    /// SECURE-COM-PROPS-FOR-TCP-REF
    SecureComPropsForTcpRef                                                = 6582,
    /// SECURE-COM-PROPS-FOR-TCP-REFS
    SecureComPropsForTcpRefs                                               = 405,
    /// SECURE-COM-PROPS-FOR-UDP-REF
    SecureComPropsForUdpRef                                                = 4923,
    /// SECURE-COM-PROPS-FOR-UDP-REFS
    SecureComPropsForUdpRefs                                               = 1973,
    /// SECURE-COM-PROPS-REF
    SecureComPropsRef                                                      = 2054,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 2282,
    /// SECURE-COM-PROPSS
    SecureComPropss                                                        = 4714,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 2669,
    /// SECURE-COMMUNICATION-DEPLOYMENTS
    SecureCommunicationDeployments                                         = 3253,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 6379,
    /// SECURE-COMMUNICATION-PROPS
    SecureCommunicationProps                                               = 625,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 1412,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 3732,
    /// SECURE-ONBOARD-COM-ERROR-REF
    SecureOnboardComErrorRef                                               = 3459,
    /// SECURE-PROPS-FOR-EXTERNAL-COMM-REF
    SecurePropsForExternalCommRef                                          = 5640,
    /// SECURE-PROPS-FOR-TCP-REF
    SecurePropsForTcpRef                                                   = 6153,
    /// SECURE-PROPS-FOR-UDP-REF
    SecurePropsForUdpRef                                                   = 4891,
    /// SECURE-TRANSLATION
    SecureTranslation                                                      = 1289,
    /// SECURED-AREA-LENGTH
    SecuredAreaLength                                                      = 5889,
    /// SECURED-AREA-OFFSET
    SecuredAreaOffset                                                      = 2128,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 3456,
    /// SECURED-RX-VERIFICATION
    SecuredRxVerification                                                  = 4759,
    /// SECURITY-ACCESS-CLASS-REF
    SecurityAccessClassRef                                                 = 783,
    /// SECURITY-ACCESS-LEVEL
    SecurityAccessLevel                                                    = 4043,
    /// SECURITY-DELAY-TIME
    SecurityDelayTime                                                      = 4163,
    /// SECURITY-DELAY-TIME-ON-BOOT
    SecurityDelayTimeOnBoot                                                = 5647,
    /// SECURITY-EVENT-CONTEXT-DATA
    SecurityEventContextData                                               = 4847,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION
    SecurityEventContextDataDefinition                                     = 1926,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF
    SecurityEventContextDataDefinitionRef                                  = 1367,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF-CONDITIONAL
    SecurityEventContextDataDefinitionRefConditional                       = 5468,
    /// SECURITY-EVENT-CONTEXT-DATA-DEFINITIONS
    SecurityEventContextDataDefinitions                                    = 3028,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT
    SecurityEventContextDataElement                                        = 3145,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF
    SecurityEventContextDataElementRef                                     = 2123,
    /// SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF-CONDITIONAL
    SecurityEventContextDataElementRefConditional                          = 3882,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 104,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 4202,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 3874,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 4425,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 4158,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 5081,
    /// SECURITY-EVENT-DEFINITION-REF
    SecurityEventDefinitionRef                                             = 3039,
    /// SECURITY-EVENT-DEFINITION-REF-CONDITIONAL
    SecurityEventDefinitionRefConditional                                  = 3674,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 4030,
    /// SECURITY-EVENT-FILTER-CHAIN-REF
    SecurityEventFilterChainRef                                            = 4725,
    /// SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL
    SecurityEventFilterChainRefConditional                                 = 2912,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 3169,
    /// SECURITY-EVENT-PROPS-REF
    SecurityEventPropsRef                                                  = 1128,
    /// SECURITY-EVENT-REF
    SecurityEventRef                                                       = 6276,
    /// SECURITY-EVENT-REFS
    SecurityEventRefs                                                      = 5709,
    /// SECURITY-EVENT-REPORT-INSTANCE-DEFINITION
    SecurityEventReportInstanceDefinition                                  = 240,
    /// SECURITY-EVENT-REPORT-INSTANCE-DEFINITION-REF
    SecurityEventReportInstanceDefinitionRef                               = 5891,
    /// SECURITY-EVENT-REPORT-INSTANCE-VALUE
    SecurityEventReportInstanceValue                                       = 2016,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 2795,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 2289,
    /// SECURITY-EVENTS
    SecurityEvents                                                         = 306,
    /// SECURITY-LEVEL-REF
    SecurityLevelRef                                                       = 1956,
    /// SECURITY-LEVEL-REFS
    SecurityLevelRefs                                                      = 3002,
    /// SEED-SIZE
    SeedSize                                                               = 1112,
    /// SEGMENT-BYTE-ORDER
    SegmentByteOrder                                                       = 4214,
    /// SEGMENT-LENGTH
    SegmentLength                                                          = 2544,
    /// SEGMENT-POSITION
    SegmentPosition                                                        = 437,
    /// SEGMENT-POSITIONS
    SegmentPositions                                                       = 5367,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME
    SegmentReceptionTimeoutTime                                            = 5353,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME-REQUEST
    SegmentReceptionTimeoutTimeRequest                                     = 4652,
    /// SEGMENT-RECEPTION-TIMEOUT-TIME-RESPONSE
    SegmentReceptionTimeoutTimeResponse                                    = 1188,
    /// SEGMENT-REF
    SegmentRef                                                             = 5228,
    /// SEGMENT-REFS
    SegmentRefs                                                            = 581,
    /// SEGMENTATION-SUPPORTED
    SegmentationSupported                                                  = 5279,
    /// SELECT-BLOCK-FOR-FIRST-INIT-ALL
    SelectBlockForFirstInitAll                                             = 6022,
    /// SELECTIONS
    Selections                                                             = 4463,
    /// SELECTOR-FIELD-BYTE-ORDER
    SelectorFieldByteOrder                                                 = 1401,
    /// SELECTOR-FIELD-CODE
    SelectorFieldCode                                                      = 4577,
    /// SELECTOR-FIELD-LENGTH
    SelectorFieldLength                                                    = 980,
    /// SELECTOR-FIELD-START-POSITION
    SelectorFieldStartPosition                                             = 2195,
    /// SEND-ACTIVITY
    SendActivity                                                           = 2508,
    /// SEND-INDICATION
    SendIndication                                                         = 5049,
    /// SEND-POLICYS
    SendPolicys                                                            = 5119,
    /// SEND-RESP-PEND-ON-TRANS-TO-BOOT
    SendRespPendOnTransToBoot                                              = 2782,
    /// SENDER-ANNOTATION
    SenderAnnotation                                                       = 615,
    /// SENDER-CAPABILITY
    SenderCapability                                                       = 31,
    /// SENDER-INTENT
    SenderIntent                                                           = 3748,
    /// SENDER-IREF
    SenderIref                                                             = 1687,
    /// SENDER-REC-ARRAY-ELEMENT-MAPPING
    SenderRecArrayElementMapping                                           = 3824,
    /// SENDER-REC-ARRAY-TYPE-MAPPING
    SenderRecArrayTypeMapping                                              = 7,
    /// SENDER-REC-RECORD-ELEMENT-MAPPING
    SenderRecRecordElementMapping                                          = 1009,
    /// SENDER-REC-RECORD-TYPE-MAPPING
    SenderRecRecordTypeMapping                                             = 3713,
    /// SENDER-RECEIVER-ANNOTATIONS
    SenderReceiverAnnotations                                              = 1250,
    /// SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING
    SenderReceiverCompositeElementToSignalMapping                          = 4435,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 670,
    /// SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING
    SenderReceiverToSignalGroupMapping                                     = 708,
    /// SENDER-RECEIVER-TO-SIGNAL-MAPPING
    SenderReceiverToSignalMapping                                          = 5908,
    /// SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING
    SenderToSignalTextTableMapping                                         = 4315,
    /// SENDING-NODE-REF
    SendingNodeRef                                                         = 4955,
    /// SENDING-NODE-REFS
    SendingNodeRefs                                                        = 4787,
    /// SENSOR-ACTUATOR-REF
    SensorActuatorRef                                                      = 4607,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 2753,
    /// SENSOR-INSTANCE-ID
    SensorInstanceId                                                       = 3769,
    /// SEPARATE-SIGNAL-PATH
    SeparateSignalPath                                                     = 6060,
    /// SEPARATED-COMPONENT-IREF
    SeparatedComponentIref                                                 = 6399,
    /// SEPARATED-COMPONENT-IREFS
    SeparatedComponentIrefs                                                = 795,
    /// SEPARATION-CYCLE-EXPONENT
    SeparationCycleExponent                                                = 2341,
    /// SEPARATION-TIME
    SeparationTime                                                         = 5556,
    /// SEPARATION-TIME-REQUEST
    SeparationTimeRequest                                                  = 4679,
    /// SEPARATION-TIME-RESPONSE
    SeparationTimeResponse                                                 = 3686,
    /// SEQUENCE-COUNTER
    SequenceCounter                                                        = 2629,
    /// SEQUENCE-COUNTER-JUMP-WIDTH
    SequenceCounterJumpWidth                                               = 2985,
    /// SEQUENCE-OFFSET
    SequenceOffset                                                         = 3001,
    /// SEQUENTIAL-INSTRUCTIONS-BEGIN
    SequentialInstructionsBegin                                            = 3185,
    /// SEQUENTIAL-INSTRUCTIONS-END
    SequentialInstructionsEnd                                              = 216,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 5186,
    /// SERIALIZATION-TECHNOLOGY-REF
    SerializationTechnologyRef                                             = 5642,
    /// SERIALIZER
    Serializer                                                             = 75,
    /// SERIALIZER-REF
    SerializerRef                                                          = 3099,
    /// SERVER-ARGUMENT-IMPL-POLICY
    ServerArgumentImplPolicy                                               = 3376,
    /// SERVER-CALL-POINTS
    ServerCallPoints                                                       = 3709,
    /// SERVER-COM-SPEC
    ServerComSpec                                                          = 1743,
    /// SERVER-COM-SPEC-PROPS
    ServerComSpecProps                                                     = 1083,
    /// SERVER-COM-SPEC-PROPSS
    ServerComSpecPropss                                                    = 5023,
    /// SERVER-NAME-IDENTIFICATION
    ServerNameIdentification                                               = 6469,
    /// SERVER-PORT-REF
    ServerPortRef                                                          = 407,
    /// SERVER-SERVICE-MAJOR-VERSION
    ServerServiceMajorVersion                                              = 2418,
    /// SERVER-SERVICE-MINOR-VERSION
    ServerServiceMinorVersion                                              = 5101,
    /// SERVICE-BUSY
    ServiceBusy                                                            = 2101,
    /// SERVICE-CONTROL
    ServiceControl                                                         = 2184,
    /// SERVICE-DEPENDENCYS
    ServiceDependencys                                                     = 1655,
    /// SERVICE-DEPLOYMENT-REF
    ServiceDeploymentRef                                                   = 4701,
    /// SERVICE-DISCOVER-CONFIGS
    ServiceDiscoverConfigs                                                 = 687,
    /// SERVICE-DISCOVERY-CONFIGS
    ServiceDiscoveryConfigs                                                = 3903,
    /// SERVICE-ELEMENT-MAPPING-REF
    ServiceElementMappingRef                                               = 5665,
    /// SERVICE-ELEMENT-MAPPING-REFS
    ServiceElementMappingRefs                                              = 1943,
    /// SERVICE-ELEMENT-SECURE-COM-CONFIG-REF
    ServiceElementSecureComConfigRef                                       = 5339,
    /// SERVICE-FIND-TIME-TO-LIVE
    ServiceFindTimeToLive                                                  = 1347,
    /// SERVICE-ID
    ServiceId                                                              = 459,
    /// SERVICE-IDENTIFIER
    ServiceIdentifier                                                      = 1761,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 3280,
    /// SERVICE-INSTANCE-ID
    ServiceInstanceId                                                      = 2306,
    /// SERVICE-INSTANCE-PORT-CONFIG
    ServiceInstancePortConfig                                              = 4371,
    /// SERVICE-INSTANCE-REF
    ServiceInstanceRef                                                     = 5483,
    /// SERVICE-INSTANCE-REFS
    ServiceInstanceRefs                                                    = 1455,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 474,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 5597,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF
    ServiceInstanceToPortPrototypeMappingRef                               = 2957,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 3996,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 5952,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS
    ServiceInstanceToSignalMappings                                        = 1170,
    /// SERVICE-INSTANCES
    ServiceInstances                                                       = 1718,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 5628,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 1639,
    /// SERVICE-INTERFACE-DEPLOYMENT-REF
    ServiceInterfaceDeploymentRef                                          = 1556,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REF
    ServiceInterfaceElementMappingRef                                      = 3440,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING-REFS
    ServiceInterfaceElementMappingRefs                                     = 1545,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 4140,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 810,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 3906,
    /// SERVICE-INTERFACE-ID
    ServiceInterfaceId                                                     = 6359,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 6613,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 6604,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 4070,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 1243,
    /// SERVICE-INTERFACE-REF
    ServiceInterfaceRef                                                    = 1296,
    /// SERVICE-INTERFACE-REFS
    ServiceInterfaceRefs                                                   = 4565,
    /// SERVICE-INTERFACE-SUB-ELEMENT
    ServiceInterfaceSubElement                                             = 5546,
    /// SERVICE-INTERFACE-TREF
    ServiceInterfaceTref                                                   = 3266,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 897,
    /// SERVICE-INTERFACE-VERSION
    ServiceInterfaceVersion                                                = 1460,
    /// SERVICE-ITEMS
    ServiceItems                                                           = 3876,
    /// SERVICE-KIND
    ServiceKind                                                            = 1891,
    /// SERVICE-MINOR-VERSION
    ServiceMinorVersion                                                    = 4806,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 961,
    /// SERVICE-OFFER-TIME-TO-LIVE
    ServiceOfferTimeToLive                                                 = 5424,
    /// SERVICE-POINT-SYMBOL-POST
    ServicePointSymbolPost                                                 = 5168,
    /// SERVICE-POINT-SYMBOL-PRE
    ServicePointSymbolPre                                                  = 3598,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 4328,
    /// SERVICE-REQUEST-CALLBACK-TYPE
    ServiceRequestCallbackType                                             = 3088,
    /// SERVICE-RESOURCE-REF
    ServiceResourceRef                                                     = 6019,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 3259,
    /// SERVICE-TABLES
    ServiceTables                                                          = 4333,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 3308,
    /// SESSION-CONTROL-CLASS-REF
    SessionControlClassRef                                                 = 2964,
    /// SESSION-HANDLING
    SessionHandling                                                        = 4932,
    /// SESSION-HANDLING-SR
    SessionHandlingSr                                                      = 4637,
    /// SESSION-ID
    SessionId                                                              = 1820,
    /// SESSION-ID-SUPPORT
    SessionIdSupport                                                       = 514,
    /// SET
    Set                                                                    = 1165,
    /// SET-FUNCTION-GROUP-STATE-IREF
    SetFunctionGroupStateIref                                              = 1082,
    /// SETTER-CALL-REF
    SetterCallRef                                                          = 5458,
    /// SETTER-CALL-SIGNAL-REF
    SetterCallSignalRef                                                    = 4518,
    /// SETTER-OPERATION-REF
    SetterOperationRef                                                     = 462,
    /// SETTER-REF
    SetterRef                                                              = 868,
    /// SETTER-RETURN-REF
    SetterReturnRef                                                        = 6246,
    /// SETTER-RETURN-SIGNAL-REF
    SetterReturnSignalRef                                                  = 6441,
    /// SEVERITY
    Severity                                                               = 3214,
    /// SHALL-NOT-RUN-ON-REF
    ShallNotRunOnRef                                                       = 2756,
    /// SHALL-NOT-RUN-ON-REFS
    ShallNotRunOnRefs                                                      = 4648,
    /// SHALL-RUN-ON-REF
    ShallRunOnRef                                                          = 6110,
    /// SHALL-RUN-ON-REFS
    ShallRunOnRefs                                                         = 5681,
    /// SHAPER
    Shaper                                                                 = 6038,
    /// SHARED
    Shared                                                                 = 5466,
    /// SHARED-AXIS-TYPE-REF
    SharedAxisTypeRef                                                      = 2350,
    /// SHARED-PARAMETERS
    SharedParameters                                                       = 4581,
    /// SHARED-TIMER
    SharedTimer                                                            = 4816,
    /// SHORT-LABEL
    ShortLabel                                                             = 3730,
    /// SHORT-LABEL-PATTERN
    ShortLabelPattern                                                      = 63,
    /// SHORT-NAME
    ShortName                                                              = 2107,
    /// SHORT-NAME-FRAGMENT
    ShortNameFragment                                                      = 4395,
    /// SHORT-NAME-FRAGMENTS
    ShortNameFragments                                                     = 4254,
    /// SHORT-NAME-PATTERN
    ShortNamePattern                                                       = 3544,
    /// SHORT-TERM-ADJUSTMENT
    ShortTermAdjustment                                                    = 3550,
    /// SHORT-TERM-ADJUSTMENT-SUPPORTED
    ShortTermAdjustmentSupported                                           = 435,
    /// SHUTDOWN
    Shutdown                                                               = 5715,
    /// SIGNAL-AGE
    SignalAge                                                              = 4549,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 924,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 1211,
    /// SIGNAL-BASED-EVENT-REF
    SignalBasedEventRef                                                    = 6228,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 6142,
    /// SIGNAL-BASED-FIELD-REF
    SignalBasedFieldRef                                                    = 6107,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 249,
    /// SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFireAndForgetMethodToISignalTriggeringMapping               = 5961,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 4219,
    /// SIGNAL-BASED-METHOD-REF
    SignalBasedMethodRef                                                   = 3672,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 1631,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 3442,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 2404,
    /// SIGNAL-FAN
    SignalFan                                                              = 6005,
    /// SIGNAL-GROUP-REF
    SignalGroupRef                                                         = 647,
    /// SIGNAL-I-PDU-COUNTER
    SignalIPduCounter                                                      = 5319,
    /// SIGNAL-I-PDU-REPLICATION
    SignalIPduReplication                                                  = 2226,
    /// SIGNAL-MAPPINGS
    SignalMappings                                                         = 1216,
    /// SIGNAL-PATH-CONSTRAINTS
    SignalPathConstraints                                                  = 4866,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 5088,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 5220,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS
    SignalServiceTranslationEventPropss                                    = 2929,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 2880,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 3892,
    /// SIGNAL-SERVICE-TRANSLATION-PROPSS
    SignalServiceTranslationPropss                                         = 6254,
    /// SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING
    SignalToReceiverTextTableMapping                                       = 6072,
    /// SIGNALS
    Signals                                                                = 2410,
    /// SIGNATURE-SCHEME-ID
    SignatureSchemeId                                                      = 3119,
    /// SIGNATURE-SCHEME-REF
    SignatureSchemeRef                                                     = 738,
    /// SIGNATURE-SCHEME-REFS
    SignatureSchemeRefs                                                    = 2286,
    /// SIGNATURE-SUPPORT-AP
    SignatureSupportAp                                                     = 756,
    /// SIGNATURE-SUPPORT-CP
    SignatureSupportCp                                                     = 4389,
    /// SIGNIFICANCE
    Significance                                                           = 1498,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 1744,
    /// SINGLE-ELEMENT-SERIALIZATION
    SingleElementSerialization                                             = 6053,
    /// SIZE
    Size                                                                   = 5678,
    /// SIZE-OF-ARRAY-LENGTH-FIELD
    SizeOfArrayLengthField                                                 = 875,
    /// SIZE-OF-ARRAY-LENGTH-FIELDS
    SizeOfArrayLengthFields                                                = 1449,
    /// SIZE-OF-STRING-LENGTH-FIELD
    SizeOfStringLengthField                                                = 3631,
    /// SIZE-OF-STRING-LENGTH-FIELDS
    SizeOfStringLengthFields                                               = 3107,
    /// SIZE-OF-STRUCT-LENGTH-FIELD
    SizeOfStructLengthField                                                = 864,
    /// SIZE-OF-STRUCT-LENGTH-FIELDS
    SizeOfStructLengthFields                                               = 1558,
    /// SIZE-OF-UNION-LENGTH-FIELD
    SizeOfUnionLengthField                                                 = 1678,
    /// SIZE-OF-UNION-LENGTH-FIELDS
    SizeOfUnionLengthFields                                                = 2266,
    /// SIZE-OF-UNION-TYPE-SELECTOR-FIELD
    SizeOfUnionTypeSelectorField                                           = 2111,
    /// SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE
    SlaveActAsPassiveCommunicationSlave                                    = 3221,
    /// SLAVE-EVENT-REF
    SlaveEventRef                                                          = 5493,
    /// SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME
    SlaveQualifiedUnexpectedLinkDownTime                                   = 1590,
    /// SLAVE-REF
    SlaveRef                                                               = 5007,
    /// SLAVES
    Slaves                                                                 = 718,
    /// SLEEP-MODE-EXECUTION-DELAY
    SleepModeExecutionDelay                                                = 3150,
    /// SLEEP-MODE-SUPPORTED
    SleepModeSupported                                                     = 1668,
    /// SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST
    SleepRepetitionDelayOfSleepRequest                                     = 1032,
    /// SLEEP-REPETITIONS-OF-SLEEP-REQUEST
    SleepRepetitionsOfSleepRequest                                         = 6435,
    /// SLEEP-TIME
    SleepTime                                                              = 3364,
    /// SLOT-CAPACITY
    SlotCapacity                                                           = 5643,
    /// SLOT-ID
    SlotId                                                                 = 4993,
    /// SLOT-TYPE
    SlotType                                                               = 153,
    /// SM-INTERACTS-WITH-NM-MAPPING
    SmInteractsWithNmMapping                                               = 517,
    /// SNAPSHOT-RECORD-CONTENTS
    SnapshotRecordContents                                                 = 5076,
    /// SO-AD-CONFIG
    SoAdConfig                                                             = 213,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 6200,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 5480,
    /// SO-CON-I-PDU-IDENTIFIER-REF
    SoConIPduIdentifierRef                                                 = 5315,
    /// SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL
    SoConIPduIdentifierRefConditional                                      = 5980,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 2994,
    /// SOCKET-ADDRESS-REF
    SocketAddressRef                                                       = 6479,
    /// SOCKET-ADDRESS-REF-CONDITIONAL
    SocketAddressRefConditional                                            = 4456,
    /// SOCKET-ADDRESSS
    SocketAddresss                                                         = 4595,
    /// SOCKET-CONNECTION
    SocketConnection                                                       = 6492,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 4045,
    /// SOCKET-CONNECTION-BUNDLE-REF
    SocketConnectionBundleRef                                              = 5094,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER
    SocketConnectionIpduIdentifier                                         = 5034,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 1482,
    /// SOCKET-CONNECTION-REF
    SocketConnectionRef                                                    = 736,
    /// SOCKET-CONNECTION-REFS
    SocketConnectionRefs                                                   = 5105,
    /// SOCKET-OPTION
    SocketOption                                                           = 569,
    /// SOCKET-OPTIONS
    SocketOptions                                                          = 5701,
    /// SOCKET-PROTOCOL
    SocketProtocol                                                         = 2276,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION
    SoftwareActivationDependencyCompareCondition                           = 3526,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA
    SoftwareActivationDependencyFormula                                    = 775,
    /// SOFTWARE-ACTIVATION-DEPENDENCY-REF
    SoftwareActivationDependencyRef                                        = 2265,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 5231,
    /// SOFTWARE-CLUSTER-COM-RESOURCE-REF
    SoftwareClusterComResourceRef                                          = 6372,
    /// SOFTWARE-CLUSTER-DEPENDENCY
    SoftwareClusterDependency                                              = 5227,
    /// SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION
    SoftwareClusterDependencyCompareCondition                              = 6451,
    /// SOFTWARE-CLUSTER-DEPENDENCY-FORMULA
    SoftwareClusterDependencyFormula                                       = 5009,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 5412,
    /// SOFTWARE-CLUSTER-DESIGN-DEPENDENCY
    SoftwareClusterDesignDependency                                        = 1294,
    /// SOFTWARE-CLUSTER-DESIGN-REF
    SoftwareClusterDesignRef                                               = 3707,
    /// SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS
    SoftwareClusterDiagnosticDeploymentProps                               = 2739,
    /// SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS
    SoftwareClusterDoipDiagnosticAddress                                   = 321,
    /// SOFTWARE-CLUSTER-ID
    SoftwareClusterId                                                      = 4750,
    /// SOFTWARE-CLUSTER-REF
    SoftwareClusterRef                                                     = 2702,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 2996,
    /// SOFTWARE-CLUSTER-SOVD-ADDRESS
    SoftwareClusterSovdAddress                                             = 855,
    /// SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING
    SoftwareClusterToApplicationPartitionMapping                           = 4273,
    /// SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPINGS
    SoftwareClusterToApplicationPartitionMappings                          = 4683,
    /// SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS
    SoftwareClusterToResourceMappings                                      = 3247,
    /// SOFTWARE-CLUSTER-UDS-DIAGNOSTIC-ADDRESS
    SoftwareClusterUdsDiagnosticAddress                                    = 4746,
    /// SOFTWARE-COMPOSITION-TREF
    SoftwareCompositionTref                                                = 706,
    /// SOFTWARE-CONTEXT
    SoftwareContext                                                        = 3830,
    /// SOFTWARE-MEMORY-SECTION-REF
    SoftwareMemorySectionRef                                               = 1299,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 3186,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 2759,
    /// SOFTWARE-PACKAGE-STEPS
    SoftwarePackageSteps                                                   = 2085,
    /// SOFTWARE-PACKAGE-STORING
    SoftwarePackageStoring                                                 = 5774,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 4359,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 3695,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 3098,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 1745,
    /// SOMEIP-EVENT-PROPS
    SomeipEventProps                                                       = 6002,
    /// SOMEIP-FIELD
    SomeipField                                                            = 3846,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 4376,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 2371,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 1072,
    /// SOMEIP-METHOD-PROPS
    SomeipMethodProps                                                      = 1701,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 6000,
    /// SOMEIP-REMOTE-MULTICAST-CONFIG
    SomeipRemoteMulticastConfig                                            = 184,
    /// SOMEIP-REMOTE-UNICAST-CONFIG
    SomeipRemoteUnicastConfig                                              = 1568,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 1149,
    /// SOMEIP-RULE
    SomeipRule                                                             = 4319,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 2073,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdClientEventGroupTimingConfigRef                                = 5320,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdClientEventGroupTimingConfigRefConditional                     = 1754,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 802,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdClientServiceInstanceConfigRef                                 = 6476,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdClientServiceInstanceConfigRefConditional                      = 2029,
    /// SOMEIP-SD-RULE
    SomeipSdRule                                                           = 2342,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 1440,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF
    SomeipSdServerEventGroupTimingConfigRef                                = 2677,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL
    SomeipSdServerEventGroupTimingConfigRefConditional                     = 6386,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 3452,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF
    SomeipSdServerServiceInstanceConfigRef                                 = 6632,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL
    SomeipSdServerServiceInstanceConfigRefConditional                      = 5760,
    /// SOMEIP-SERVICE-DISCOVERY
    SomeipServiceDiscovery                                                 = 2244,
    /// SOMEIP-SERVICE-DISCOVERY-PORT
    SomeipServiceDiscoveryPort                                             = 5588,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 1747,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 5221,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 5463,
    /// SOMEIP-SERVICE-VERSION
    SomeipServiceVersion                                                   = 4263,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 88,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 38,
    /// SOMEIP-TP-CONNECTION
    SomeipTpConnection                                                     = 375,
    /// SOMEIP-TRANSFORMATION-DESCRIPTION
    SomeipTransformationDescription                                        = 4244,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS
    SomeipTransformationISignalProps                                       = 6624,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    SomeipTransformationISignalPropsConditional                            = 3765,
    /// SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    SomeipTransformationISignalPropsVariants                               = 3,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 6292,
    /// SOMEIP-TRANSFORMATION-PROPS-REF
    SomeipTransformationPropsRef                                           = 3128,
    /// SOUCE-CAN-ID-REF
    SouceCanIdRef                                                          = 2949,
    /// SOURCE-ADDRESS-CODE
    SourceAddressCode                                                      = 5657,
    /// SOURCE-ADDRESS-MASK
    SourceAddressMask                                                      = 4776,
    /// SOURCE-ADDRESS-RANGE-END
    SourceAddressRangeEnd                                                  = 846,
    /// SOURCE-ADDRESS-RANGE-START
    SourceAddressRangeStart                                                = 1674,
    /// SOURCE-CAN-ID-CODE
    SourceCanIdCode                                                        = 3285,
    /// SOURCE-CAN-ID-MASK
    SourceCanIdMask                                                        = 5040,
    /// SOURCE-CAN-ID-REF
    SourceCanIdRef                                                         = 264,
    /// SOURCE-CHANNEL
    SourceChannel                                                          = 1899,
    /// SOURCE-EEC-REF
    SourceEecRef                                                           = 2143,
    /// SOURCE-EEC-REFS
    SourceEecRefs                                                          = 812,
    /// SOURCE-ELEMENT-REF
    SourceElementRef                                                       = 3333,
    /// SOURCE-ELEMENT-REFS
    SourceElementRefs                                                      = 2625,
    /// SOURCE-EVENT-REF
    SourceEventRef                                                         = 1984,
    /// SOURCE-EVENT-REFS
    SourceEventRefs                                                        = 5509,
    /// SOURCE-FIELD-REF
    SourceFieldRef                                                         = 2573,
    /// SOURCE-FRAME-REF
    SourceFrameRef                                                         = 4187,
    /// SOURCE-I-PDU-REF
    SourceIPduRef                                                          = 4799,
    /// SOURCE-ID
    SourceId                                                               = 3004,
    /// SOURCE-INSTANCE-IREF
    SourceInstanceIref                                                     = 5791,
    /// SOURCE-INSTANCE-IREFS
    SourceInstanceIrefs                                                    = 5021,
    /// SOURCE-IP-ADDRESS
    SourceIpAddress                                                        = 1,
    /// SOURCE-IPV-4-ADDRESS
    SourceIpv4Address                                                      = 4017,
    /// SOURCE-IPV-6-ADDRESS
    SourceIpv6Address                                                      = 397,
    /// SOURCE-LIN-PID-REF
    SourceLinPidRef                                                        = 3217,
    /// SOURCE-MAC-ADDRESS
    SourceMacAddress                                                       = 6296,
    /// SOURCE-MAC-ADDRESS-MASK
    SourceMacAddressMask                                                   = 1880,
    /// SOURCE-MAX-ADDRESS
    SourceMaxAddress                                                       = 320,
    /// SOURCE-METHOD-REF
    SourceMethodRef                                                        = 4414,
    /// SOURCE-MIN-ADDRESS
    SourceMinAddress                                                       = 2567,
    /// SOURCE-NETWORK-MASK
    SourceNetworkMask                                                      = 143,
    /// SOURCE-PORTS
    SourcePorts                                                            = 3021,
    /// SOURCE-POSSIBLE-ERROR-REF
    SourcePossibleErrorRef                                                 = 3123,
    /// SOURCE-REF
    SourceRef                                                              = 3226,
    /// SOURCE-SERVICE-INTERFACE-REF
    SourceServiceInterfaceRef                                              = 4499,
    /// SOURCE-SERVICE-INTERFACE-REFS
    SourceServiceInterfaceRefs                                             = 4200,
    /// SOURCE-SIGNAL-REF
    SourceSignalRef                                                        = 4625,
    /// SOURCE-TRIGGER-REF
    SourceTriggerRef                                                       = 2699,
    /// SOVD-CONTENT-REF
    SovdContentRef                                                         = 4654,
    /// SOVD-CONTENT-REFS
    SovdContentRefs                                                        = 3999,
    /// SOVD-DATA-CATEGORY-REF
    SovdDataCategoryRef                                                    = 6054,
    /// SOVD-GATEWAY-INSTANTIATION
    SovdGatewayInstantiation                                               = 1781,
    /// SOVD-GROUP-REF
    SovdGroupRef                                                           = 471,
    /// SOVD-GROUP-REFS
    SovdGroupRefs                                                          = 6230,
    /// SOVD-LOCK-REF
    SovdLockRef                                                            = 5274,
    /// SOVD-OPERATION-REF
    SovdOperationRef                                                       = 3723,
    /// SOVD-PROPERTIES
    SovdProperties                                                         = 2219,
    /// SOVD-SERVER-INSTANTIATION
    SovdServerInstantiation                                                = 4373,
    /// SOVD-SNAPSHOT-CONTENT-REF
    SovdSnapshotContentRef                                                 = 3826,
    /// SOVD-VALIDATION-ORDER-REF
    SovdValidationOrderRef                                                 = 3177,
    /// SOVD-VALIDATION-ORDER-REFS
    SovdValidationOrderRefs                                                = 2245,
    /// SPARSE-TIMESTAMP-ENABLED
    SparseTimestampEnabled                                                 = 2130,
    /// SPECIFIC-CHANNEL-REF
    SpecificChannelRef                                                     = 1540,
    /// SPECIFIC-CHANNELS
    SpecificChannels                                                       = 6206,
    /// SPECIFIC-PHYSICAL-CHANNEL-REF
    SpecificPhysicalChannelRef                                             = 5807,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 403,
    /// SPECIFICATION-DOCUMENT-SCOPES
    SpecificationDocumentScopes                                            = 406,
    /// SPECIFICATION-SCOPE
    SpecificationScope                                                     = 4407,
    /// SPEED
    Speed                                                                  = 2070,
    /// SPEEDUP
    Speedup                                                                = 956,
    /// SPN
    Spn                                                                    = 4649,
    /// SPN-REF
    SpnRef                                                                 = 3879,
    /// SPN-REFS
    SpnRefs                                                                = 1627,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 5789,
    /// SRVT
    Srvt                                                                   = 4133,
    /// SSP-OFFSET
    SspOffset                                                              = 6124,
    /// SSRC
    Ssrc                                                                   = 4959,
    /// ST-MIN
    StMin                                                                  = 4590,
    /// STACK-USAGES
    StackUsages                                                            = 2303,
    /// STANDARD
    Standard                                                               = 3301,
    /// STANDARD-REVISION
    StandardRevision                                                       = 1534,
    /// STANDARD-REVISIONS
    StandardRevisions                                                      = 4257,
    /// START
    Start                                                                  = 3287,
    /// START-ACTION-REF
    StartActionRef                                                         = 4856,
    /// START-ACTION-REFS
    StartActionRefs                                                        = 1648,
    /// START-ADDRESS
    StartAddress                                                           = 3916,
    /// START-AGENT-ERROR-REF
    StartAgentErrorRef                                                     = 6214,
    /// START-AGENT-REF
    StartAgentRef                                                          = 2139,
    /// START-INDEX
    StartIndex                                                             = 159,
    /// START-IREF
    StartIref                                                              = 2298,
    /// START-ON-EVENT-REF
    StartOnEventRef                                                        = 73,
    /// START-POSITION
    StartPosition                                                          = 5200,
    /// START-STATE-MACHINE-REF
    StartStateMachineRef                                                   = 5820,
    /// STARTS-ON-EVENT-REF
    StartsOnEventRef                                                       = 5175,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 1237,
    /// STARTUP-CONFIG-REF
    StartupConfigRef                                                       = 6418,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 5484,
    /// STARTUP-CONFIGS
    StartupConfigs                                                         = 4598,
    /// STARTUP-OPTION
    StartupOption                                                          = 2162,
    /// STARTUP-OPTIONS
    StartupOptions                                                         = 4381,
    /// STATE
    State                                                                  = 3422,
    /// STATE-CLIENT-INTERFACE
    StateClientInterface                                                   = 4112,
    /// STATE-DEP-FIREWALL-REF
    StateDepFirewallRef                                                    = 3412,
    /// STATE-DEP-FIREWALL-REFS
    StateDepFirewallRefs                                                   = 928,
    /// STATE-DEPENDENT-FIREWALL
    StateDependentFirewall                                                 = 4596,
    /// STATE-DEPENDENT-FIREWALL-REF
    StateDependentFirewallRef                                              = 5417,
    /// STATE-DEPENDENT-STARTUP-CONFIG
    StateDependentStartupConfig                                            = 4481,
    /// STATE-DEPENDENT-STARTUP-CONFIGS
    StateDependentStartupConfigs                                           = 3161,
    /// STATE-MACHINE
    StateMachine                                                           = 5189,
    /// STATE-MANAGEMEN-PHM-ERROR-INTERFACE
    StateManagemenPhmErrorInterface                                        = 2790,
    /// STATE-MANAGEMENT-ACTION-LIST
    StateManagementActionList                                              = 679,
    /// STATE-MANAGEMENT-BASED-ON-TCP-FLAGS
    StateManagementBasedOnTcpFlags                                         = 1988,
    /// STATE-MANAGEMENT-COMPARE-FORMULA
    StateManagementCompareFormula                                          = 6240,
    /// STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE
    StateManagementDiagTriggerInterface                                    = 2941,
    /// STATE-MANAGEMENT-EM-ERROR-INTERFACE
    StateManagementEmErrorInterface                                        = 6194,
    /// STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-ACTION-ITEM
    StateManagementEnterSuspendToRamActionItem                             = 3069,
    /// STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-OS-ACTION-ITEM
    StateManagementEnterSuspendToRamOsActionItem                           = 5082,
    /// STATE-MANAGEMENT-ERROR-COMPARE-RULE
    StateManagementErrorCompareRule                                        = 4818,
    /// STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE
    StateManagementFunctionGroupSwitchNotificationInterface                = 2513,
    /// STATE-MANAGEMENT-LEAVE-SUSPEND-TO-RAM-ACTION-ITEM
    StateManagementLeaveSuspendToRamActionItem                             = 604,
    /// STATE-MANAGEMENT-MODULE-INSTANTIATION
    StateManagementModuleInstantiation                                     = 121,
    /// STATE-MANAGEMENT-NM-ACTION-ITEM
    StateManagementNmActionItem                                            = 37,
    /// STATE-MANAGEMENT-PHM-ERROR-INTERFACE
    StateManagementPhmErrorInterface                                       = 5410,
    /// STATE-MANAGEMENT-REQUEST-ERROR
    StateManagementRequestError                                            = 4798,
    /// STATE-MANAGEMENT-REQUEST-RULE
    StateManagementRequestRule                                             = 5599,
    /// STATE-MANAGEMENT-REQUEST-TRIGGER
    StateManagementRequestTrigger                                          = 4461,
    /// STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM
    StateManagementSetFunctionGroupStateActionItem                         = 2262,
    /// STATE-MANAGEMENT-SLEEP-ACTION-ITEM
    StateManagementSleepActionItem                                         = 4789,
    /// STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM
    StateManagementStateMachineActionItem                                  = 2642,
    /// STATE-MANAGEMENT-STATE-NOTIFICATION
    StateManagementStateNotification                                       = 4876,
    /// STATE-MANAGEMENT-SYNC-ACTION-ITEM
    StateManagementSyncActionItem                                          = 1803,
    /// STATE-MANAGEMENT-TRIGGER-COMPARE-RULE
    StateManagementTriggerCompareRule                                      = 4909,
    /// STATE-REF
    StateRef                                                               = 30,
    /// STATE-REFERENCES
    StateReferences                                                        = 6145,
    /// STATE-REQUEST-PORT-IREF
    StateRequestPortIref                                                   = 507,
    /// STATE-REQUEST-REF
    StateRequestRef                                                        = 5702,
    /// STATIC-MEMORYS
    StaticMemorys                                                          = 3316,
    /// STATIC-PART
    StaticPart                                                             = 4843,
    /// STATIC-PARTS
    StaticParts                                                            = 5761,
    /// STATIC-REMOTE-MULTICAST-ADDRESSES
    StaticRemoteMulticastAddresses                                         = 1034,
    /// STATIC-REMOTE-UNICAST-ADDRESSES
    StaticRemoteUnicastAddresses                                           = 4090,
    /// STATIC-SLOT-DURATION
    StaticSlotDuration                                                     = 4056,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 3108,
    /// STATIC-SOCKET-CONNECTION-REF
    StaticSocketConnectionRef                                              = 1487,
    /// STATIC-SOCKET-CONNECTIONS
    StaticSocketConnections                                                = 5456,
    /// STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR
    StatusBitHandlingTestFailedSinceLastClear                              = 3758,
    /// STATUS-BIT-STORAGE-TEST-FAILED
    StatusBitStorageTestFailed                                             = 206,
    /// STATUS-ID
    StatusId                                                               = 4992,
    /// STATUS-IREF
    StatusIref                                                             = 575,
    /// STATUS-REF
    StatusRef                                                              = 4442,
    /// STATUS-SUB-TLV
    StatusSubTlv                                                           = 3116,
    /// STATUSS
    Statuss                                                                = 5446,
    /// STD
    Std                                                                    = 5936,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 1757,
    /// STD-REF
    StdRef                                                                 = 571,
    /// STD-REFS
    StdRefs                                                                = 6571,
    /// STEP-SIZE
    StepSize                                                               = 2530,
    /// STIM-ENABLER
    StimEnabler                                                            = 1451,
    /// STIMULUS-REF
    StimulusRef                                                            = 4277,
    /// STOP
    Stop                                                                   = 3992,
    /// STOP-AGENT-ERROR-REF
    StopAgentErrorRef                                                      = 5445,
    /// STOP-AGENT-REF
    StopAgentRef                                                           = 2357,
    /// STOP-IREF
    StopIref                                                               = 807,
    /// STOP-STATE-MACHINE-REF
    StopStateMachineRef                                                    = 3225,
    /// STORAGE-CONDITION-GROUP-REF
    StorageConditionGroupRef                                               = 2238,
    /// STORAGE-CONDITIONS
    StorageConditions                                                      = 6500,
    /// STORAGE-STATE-EVALUATION
    StorageStateEvaluation                                                 = 530,
    /// STORE-AT-SHUTDOWN
    StoreAtShutdown                                                        = 4375,
    /// STORE-CYCLIC
    StoreCyclic                                                            = 5148,
    /// STORE-EMERGENCY
    StoreEmergency                                                         = 520,
    /// STORE-EVENT-ENABLED
    StoreEventEnabled                                                      = 4269,
    /// STORE-EVENT-SUPPORT
    StoreEventSupport                                                      = 1776,
    /// STORE-IMMEDIATE
    StoreImmediate                                                         = 2352,
    /// STORE-ON-CHANGE
    StoreOnChange                                                          = 1732,
    /// STORING
    Storing                                                                = 6474,
    /// STREAM-FILTER-PORT-RANGE
    StreamFilterPortRange                                                  = 501,
    /// STREAM-FILTER-RULE
    StreamFilterRule                                                       = 4732,
    /// STREAM-FILTERS
    StreamFilters                                                          = 5783,
    /// STREAM-GATE-REF
    StreamGateRef                                                          = 6315,
    /// STREAM-GATES
    StreamGates                                                            = 2828,
    /// STREAM-ID
    StreamId                                                               = 1030,
    /// STREAM-IDENTIFICATION-HANDLE-REF
    StreamIdentificationHandleRef                                          = 5879,
    /// STREAM-IDENTIFICATION-HANDLE-REFS
    StreamIdentificationHandleRefs                                         = 2684,
    /// STREAM-IDENTIFICATION-WILDCARD
    StreamIdentificationWildcard                                           = 5386,
    /// STREAM-IDENTIFIER
    StreamIdentifier                                                       = 2346,
    /// STREAMS-PER-FRAME
    StreamsPerFrame                                                        = 3617,
    /// STRING-ENCODING
    StringEncoding                                                         = 1587,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 3064,
    /// SUB
    Sub                                                                    = 5685,
    /// SUB-ATTRIBUTE-TAILORINGS
    SubAttributeTailorings                                                 = 5796,
    /// SUB-CONTAINERS
    SubContainers                                                          = 1850,
    /// SUB-DOMAIN-REF
    SubDomainRef                                                           = 2611,
    /// SUB-DOMAIN-REFS
    SubDomainRefs                                                          = 2129,
    /// SUB-ELEMENT-MAPPING
    SubElementMapping                                                      = 5112,
    /// SUB-ELEMENT-MAPPINGS
    SubElementMappings                                                     = 1276,
    /// SUB-ELEMENTS
    SubElements                                                            = 1244,
    /// SUB-FUNCTION-REF
    SubFunctionRef                                                         = 5658,
    /// SUB-FUNCTION-REFS
    SubFunctionRefs                                                        = 422,
    /// SUB-GROUP-REF
    SubGroupRef                                                            = 5206,
    /// SUB-GROUP-REFS
    SubGroupRefs                                                           = 5102,
    /// SUB-NODE-CHANNEL-REF
    SubNodeChannelRef                                                      = 5450,
    /// SUB-NODE-CHANNELS
    SubNodeChannels                                                        = 767,
    /// SUB-NODE-NUMBER
    SubNodeNumber                                                          = 3729,
    /// SUB-NODE-PHYSICAL-CHANNEL-REF
    SubNodePhysicalChannelRef                                              = 1641,
    /// SUB-SDG-REF
    SubSdgRef                                                              = 4525,
    /// SUB-SOFTWARE-CLUSTER-REF
    SubSoftwareClusterRef                                                  = 1555,
    /// SUB-SOFTWARE-CLUSTER-REFS
    SubSoftwareClusterRefs                                                 = 1338,
    /// SUB-TLV-CONFIG
    SubTlvConfig                                                           = 2563,
    /// SUB-TYPE
    SubType                                                                = 4779,
    /// SUBFUNCTION
    Subfunction                                                            = 2323,
    /// SUBFUNCTIONS
    Subfunctions                                                           = 4141,
    /// SUBMESSAGE-TYPE
    SubmessageType                                                         = 3103,
    /// SUBNET-NUMBER
    SubnetNumber                                                           = 4632,
    /// SUBSCRIBE-EVENTGROUP-RETRY-DELAY
    SubscribeEventgroupRetryDelay                                          = 2955,
    /// SUBSCRIBE-EVENTGROUP-RETRY-MAX
    SubscribeEventgroupRetryMax                                            = 564,
    /// SUBSTITUTED-FRAME-REF
    SubstitutedFrameRef                                                    = 5540,
    /// SUBSTITUTED-FRAME-REFS
    SubstitutedFrameRefs                                                   = 6415,
    /// SUBTITLE
    Subtitle                                                               = 2104,
    /// SUCCESSOR-REF
    SuccessorRef                                                           = 3521,
    /// SUCCESSOR-REFS
    SuccessorRefs                                                          = 1230,
    /// SUP
    Sup                                                                    = 1349,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 2439,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF
    SupervisedEntityCheckpointNeedsRef                                     = 1729,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL
    SupervisedEntityCheckpointNeedsRefConditional                          = 3498,
    /// SUPERVISED-ENTITY-ID
    SupervisedEntityId                                                     = 4582,
    /// SUPERVISED-ENTITY-IREF
    SupervisedEntityIref                                                   = 5543,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 5010,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 1153,
    /// SUPERVISION-CONDITION
    SupervisionCondition                                                   = 1814,
    /// SUPERVISION-CYCLE
    SupervisionCycle                                                       = 3821,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 487,
    /// SUPERVISION-ENTITYS
    SupervisionEntitys                                                     = 5449,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 6212,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 2569,
    /// SUPERVISION-MODE-CONDITIONS
    SupervisionModeConditions                                              = 2115,
    /// SUPERVISION-MODES
    SupervisionModes                                                       = 5916,
    /// SUPERVISION-REF
    SupervisionRef                                                         = 1829,
    /// SUPPLIER-ID
    SupplierId                                                             = 2930,
    /// SUPPLIER-VALIDATION-ORDER-REF
    SupplierValidationOrderRef                                             = 730,
    /// SUPPLIER-VALIDATION-ORDER-REFS
    SupplierValidationOrderRefs                                            = 2653,
    /// SUPPORT-BUFFER-LOCKING
    SupportBufferLocking                                                   = 4528,
    /// SUPPORT-DIRTY-FLAG
    SupportDirtyFlag                                                       = 5190,
    /// SUPPORT-INFO
    SupportInfo                                                            = 5581,
    /// SUPPORT-INFO-BIT
    SupportInfoBit                                                         = 2672,
    /// SUPPORT-INFO-BYTE
    SupportInfoByte                                                        = 3330,
    /// SUPPORTED-CIPHER-SUITES
    SupportedCipherSuites                                                  = 776,
    /// SUPPORTED-CONFIG-VARIANT
    SupportedConfigVariant                                                 = 5724,
    /// SUPPORTED-CONFIG-VARIANTS
    SupportedConfigVariants                                                = 4151,
    /// SUPPORTED-FEATURES
    SupportedFeatures                                                      = 132,
    /// SUPPORTED-TIMER-GRANULARITY
    SupportedTimerGranularity                                              = 623,
    /// SUPPORTING-MATERIAL
    SupportingMaterial                                                     = 3261,
    /// SUPPORTS-ASYNCHRONOUS-MODE-SWITCH
    SupportsAsynchronousModeSwitch                                         = 1798,
    /// SUPPORTS-MULTIPLE-INSTANTIATION
    SupportsMultipleInstantiation                                          = 2833,
    /// SUPPORTS-MULTIPLE-NOTIFIER-SETS
    SupportsMultipleNotifierSets                                           = 5873,
    /// SUSPEND-TO-RAM-AWARENESS
    SuspendToRamAwareness                                                  = 1199,
    /// SUSPEND-TO-RAM-HUB-INTERFACE
    SuspendToRamHubInterface                                               = 3482,
    /// SUSPEND-TO-RAM-HUB-MAPPING
    SuspendToRamHubMapping                                                 = 6119,
    /// SUSPEND-TO-RAM-MODULE-INSTANTIATION
    SuspendToRamModuleInstantiation                                        = 3501,
    /// SUSPEND-TO-RAM-SATELLITE-INTERFACE
    SuspendToRamSatelliteInterface                                         = 5265,
    /// SUSPEND-TO-RAM-SATELLITE-MAPPING
    SuspendToRamSatelliteMapping                                           = 4225,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 2052,
    /// SW-ADDR-METHOD-REF
    SwAddrMethodRef                                                        = 832,
    /// SW-ADDRMETHOD-REF
    SwAddrmethodRef                                                        = 592,
    /// SW-ALIGNMENT
    SwAlignment                                                            = 2548,
    /// SW-ARRAYSIZE
    SwArraysize                                                            = 6383,
    /// SW-AXIS-CONT
    SwAxisCont                                                             = 2386,
    /// SW-AXIS-CONTS
    SwAxisConts                                                            = 4262,
    /// SW-AXIS-GENERIC
    SwAxisGeneric                                                          = 5947,
    /// SW-AXIS-GROUPED
    SwAxisGrouped                                                          = 568,
    /// SW-AXIS-INDEX
    SwAxisIndex                                                            = 3738,
    /// SW-AXIS-INDIVIDUAL
    SwAxisIndividual                                                       = 3684,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 3130,
    /// SW-AXIS-TYPE-REF
    SwAxisTypeRef                                                          = 5237,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 2558,
    /// SW-BIT-REPRESENTATION
    SwBitRepresentation                                                    = 4667,
    /// SW-CALIBRATION-ACCESS
    SwCalibrationAccess                                                    = 6489,
    /// SW-CALIBRATION-NOTES
    SwCalibrationNotes                                                     = 5392,
    /// SW-CALIBRATION-NOTESS
    SwCalibrationNotess                                                    = 1954,
    /// SW-CALPRM-AXIS
    SwCalprmAxis                                                           = 1833,
    /// SW-CALPRM-AXIS-SET
    SwCalprmAxisSet                                                        = 5096,
    /// SW-CALPRM-IMPL
    SwCalprmImpl                                                           = 3195,
    /// SW-CALPRM-IMPLS
    SwCalprmImpls                                                          = 383,
    /// SW-CALPRM-PROTOTYPE-REF
    SwCalprmPrototypeRef                                                   = 1843,
    /// SW-CARB-DOC
    SwCarbDoc                                                              = 5044,
    /// SW-CARB-DOCS
    SwCarbDocs                                                             = 2100,
    /// SW-CLASS-ATTR-INSTANCE-IMPL
    SwClassAttrInstanceImpl                                                = 5636,
    /// SW-CLASS-IMPL
    SwClassImpl                                                            = 597,
    /// SW-CLASS-IMPLS
    SwClassImpls                                                           = 4954,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 1510,
    /// SW-CLASS-PROTOTYPE-REF
    SwClassPrototypeRef                                                    = 5910,
    /// SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF
    SwClusterDesignProvidedPortIref                                        = 3528,
    /// SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF
    SwClusterDesignRequiredPortIref                                        = 5717,
    /// SW-CLUSTER-MAPPINGS
    SwClusterMappings                                                      = 5382,
    /// SW-CLUSTER-REF
    SwClusterRef                                                           = 5674,
    /// SW-CLUSTER-REFS
    SwClusterRefs                                                          = 5077,
    /// SW-CLUSTERS
    SwClusters                                                             = 1591,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 506,
    /// SW-COMP-TO-ECU-MAPPING-REF
    SwCompToEcuMappingRef                                                  = 99,
    /// SW-COMP-TO-ECU-MAPPING-REFS
    SwCompToEcuMappingRefs                                                 = 1766,
    /// SW-COMPARISON-VARIABLES
    SwComparisonVariables                                                  = 6102,
    /// SW-COMPONENT-ASSIGNMENTS
    SwComponentAssignments                                                 = 5797,
    /// SW-COMPONENT-DOCUMENTATION
    SwComponentDocumentation                                               = 1613,
    /// SW-COMPONENT-DOCUMENTATIONS
    SwComponentDocumentations                                              = 5198,
    /// SW-COMPONENT-IREF
    SwComponentIref                                                        = 2680,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 175,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 2314,
    /// SW-COMPONENT-PROTOTYPE-ASSIGNMENT
    SwComponentPrototypeAssignment                                         = 352,
    /// SW-COMPONENT-PROTOTYPE-IREF
    SwComponentPrototypeIref                                               = 2420,
    /// SW-COMPONENT-PROTOTYPE-REF
    SwComponentPrototypeRef                                                = 1857,
    /// SW-COMPOSITIONS
    SwCompositions                                                         = 414,
    /// SW-DATA-DEF-PROPS
    SwDataDefProps                                                         = 4146,
    /// SW-DATA-DEF-PROPS-CONDITIONAL
    SwDataDefPropsConditional                                              = 5878,
    /// SW-DATA-DEF-PROPS-VARIANTS
    SwDataDefPropsVariants                                                 = 3779,
    /// SW-DATA-DEPENDENCY
    SwDataDependency                                                       = 2654,
    /// SW-DATA-DEPENDENCY-ARGS
    SwDataDependencyArgs                                                   = 2688,
    /// SW-DATA-DEPENDENCY-FORMULA
    SwDataDependencyFormula                                                = 6336,
    /// SW-DIAGNOSTICS-NOTES
    SwDiagnosticsNotes                                                     = 2836,
    /// SW-DIAGNOSTICS-NOTESS
    SwDiagnosticsNotess                                                    = 3904,
    /// SW-FEATURE-DEF
    SwFeatureDef                                                           = 2024,
    /// SW-FEATURE-DEFS
    SwFeatureDefs                                                          = 601,
    /// SW-FEATURE-DESC
    SwFeatureDesc                                                          = 441,
    /// SW-FEATURE-DESCS
    SwFeatureDescs                                                         = 1115,
    /// SW-FILL-CHARACTER
    SwFillCharacter                                                        = 1190,
    /// SW-GENERIC-AXIS-DESC
    SwGenericAxisDesc                                                      = 2618,
    /// SW-GENERIC-AXIS-PARAM
    SwGenericAxisParam                                                     = 3286,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 6302,
    /// SW-GENERIC-AXIS-PARAM-TYPE-REF
    SwGenericAxisParamTypeRef                                              = 5765,
    /// SW-GENERIC-AXIS-PARAM-TYPES
    SwGenericAxisParamTypes                                                = 502,
    /// SW-GENERIC-AXIS-PARAMS
    SwGenericAxisParams                                                    = 4122,
    /// SW-HOST-VARIABLE
    SwHostVariable                                                         = 4960,
    /// SW-IMPL-MAPPINGS
    SwImplMappings                                                         = 4980,
    /// SW-IMPL-POLICY
    SwImplPolicy                                                           = 6025,
    /// SW-INTENDED-RESOLUTION
    SwIntendedResolution                                                   = 5268,
    /// SW-INTERPOLATION-METHOD
    SwInterpolationMethod                                                  = 6570,
    /// SW-IS-VIRTUAL
    SwIsVirtual                                                            = 5371,
    /// SW-MAINTENANCE-NOTES
    SwMaintenanceNotes                                                     = 4351,
    /// SW-MAINTENANCE-NOTESS
    SwMaintenanceNotess                                                    = 1520,
    /// SW-MAPPINGS
    SwMappings                                                             = 2501,
    /// SW-MAX-AXIS-POINTS
    SwMaxAxisPoints                                                        = 5721,
    /// SW-MAX-TEXT-SIZE
    SwMaxTextSize                                                          = 1308,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 5103,
    /// SW-MIN-AXIS-POINTS
    SwMinAxisPoints                                                        = 5091,
    /// SW-NUMBER-OF-AXIS-POINTS
    SwNumberOfAxisPoints                                                   = 3218,
    /// SW-POINTER-TARGET-PROPS
    SwPointerTargetProps                                                   = 3229,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 4107,
    /// SW-RECORD-LAYOUT-COMPONENT
    SwRecordLayoutComponent                                                = 1958,
    /// SW-RECORD-LAYOUT-GROUP
    SwRecordLayoutGroup                                                    = 925,
    /// SW-RECORD-LAYOUT-GROUP-AXIS
    SwRecordLayoutGroupAxis                                                = 831,
    /// SW-RECORD-LAYOUT-GROUP-FROM
    SwRecordLayoutGroupFrom                                                = 3594,
    /// SW-RECORD-LAYOUT-GROUP-INDEX
    SwRecordLayoutGroupIndex                                               = 6617,
    /// SW-RECORD-LAYOUT-GROUP-STEP
    SwRecordLayoutGroupStep                                                = 4402,
    /// SW-RECORD-LAYOUT-GROUP-TO
    SwRecordLayoutGroupTo                                                  = 5233,
    /// SW-RECORD-LAYOUT-REF
    SwRecordLayoutRef                                                      = 1807,
    /// SW-RECORD-LAYOUT-V
    SwRecordLayoutV                                                        = 4082,
    /// SW-RECORD-LAYOUT-V-AXIS
    SwRecordLayoutVAxis                                                    = 1242,
    /// SW-RECORD-LAYOUT-V-FIX-VALUE
    SwRecordLayoutVFixValue                                                = 1311,
    /// SW-RECORD-LAYOUT-V-INDEX
    SwRecordLayoutVIndex                                                   = 2796,
    /// SW-RECORD-LAYOUT-V-PROP
    SwRecordLayoutVProp                                                    = 2731,
    /// SW-REFRESH-TIMING
    SwRefreshTiming                                                        = 6436,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 6217,
    /// SW-SERVICE-IMPL-POLICY
    SwServiceImplPolicy                                                    = 5771,
    /// SW-SYSCOND
    SwSyscond                                                              = 5500,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 668,
    /// SW-SYSTEMCONST-REF
    SwSystemconstRef                                                       = 5828,
    /// SW-SYSTEMCONST-VALUE
    SwSystemconstValue                                                     = 1120,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 1933,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REF
    SwSystemconstantValueSetRef                                            = 4995,
    /// SW-SYSTEMCONSTANT-VALUE-SET-REFS
    SwSystemconstantValueSetRefs                                           = 1530,
    /// SW-SYSTEMCONSTANT-VALUES
    SwSystemconstantValues                                                 = 194,
    /// SW-TEST-DESC
    SwTestDesc                                                             = 5372,
    /// SW-TEST-DESCS
    SwTestDescs                                                            = 5499,
    /// SW-TEXT-PROPS
    SwTextProps                                                            = 2155,
    /// SW-VALUE-BLOCK-SIZE
    SwValueBlockSize                                                       = 3245,
    /// SW-VALUE-BLOCK-SIZE-MULTS
    SwValueBlockSizeMults                                                  = 5675,
    /// SW-VALUE-CONT
    SwValueCont                                                            = 4851,
    /// SW-VALUES-PHYS
    SwValuesPhys                                                           = 2737,
    /// SW-VARIABLE-IMPL
    SwVariableImpl                                                         = 476,
    /// SW-VARIABLE-IMPLS
    SwVariableImpls                                                        = 1229,
    /// SW-VARIABLE-PROTOTYPE-REF
    SwVariablePrototypeRef                                                 = 3402,
    /// SW-VARIABLE-REFS
    SwVariableRefs                                                         = 3158,
    /// SW-VERSION
    SwVersion                                                              = 3654,
    /// SWC-BEHAVIOR-REF
    SwcBehaviorRef                                                         = 6579,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 4873,
    /// SWC-BSW-MAPPING-REF
    SwcBswMappingRef                                                       = 3851,
    /// SWC-BSW-RUNNABLE-MAPPING
    SwcBswRunnableMapping                                                  = 1901,
    /// SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE
    SwcBswSynchronizedModeGroupPrototype                                   = 4296,
    /// SWC-BSW-SYNCHRONIZED-TRIGGER
    SwcBswSynchronizedTrigger                                              = 5032,
    /// SWC-EXCLUSIVE-AREA-POLICY
    SwcExclusiveAreaPolicy                                                 = 917,
    /// SWC-FLAT-SERVICE-DEPENDENCY-REF
    SwcFlatServiceDependencyRef                                            = 4624,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 4267,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 5408,
    /// SWC-MAPPING-CONSTRAINT-REF
    SwcMappingConstraintRef                                                = 5623,
    /// SWC-MAPPING-CONSTRAINT-REFS
    SwcMappingConstraintRefs                                               = 4440,
    /// SWC-MODE-GROUP-IREF
    SwcModeGroupIref                                                       = 4631,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 4306,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 3710,
    /// SWC-MODE-SWITCH-EVENT-REF
    SwcModeSwitchEventRef                                                  = 3733,
    /// SWC-RUNNABLE-REF
    SwcRunnableRef                                                         = 1107,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 3431,
    /// SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF
    SwcServiceDependencyInExecutableIref                                   = 1740,
    /// SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF
    SwcServiceDependencyInSystemIref                                       = 5650,
    /// SWC-SERVICE-DEPENDENCY-IREF
    SwcServiceDependencyIref                                               = 912,
    /// SWC-TIMING
    SwcTiming                                                              = 3492,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 4417,
    /// SWC-TO-APPLICATION-PARTITION-MAPPINGS
    SwcToApplicationPartitionMappings                                      = 2794,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 2227,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT
    SwcToEcuMappingConstraint                                              = 5275,
    /// SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE
    SwcToEcuMappingConstraintType                                          = 3297,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 2841,
    /// SWC-TO-SWC-OPERATION-ARGUMENTS
    SwcToSwcOperationArguments                                             = 769,
    /// SWC-TO-SWC-SIGNAL
    SwcToSwcSignal                                                         = 2398,
    /// SWC-TRIGGER-IREF
    SwcTriggerIref                                                         = 1004,
    /// SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY
    SwitchAsynchronousTrafficShaperGroupEntry                              = 5095,
    /// SWITCH-ATS-INSTANCE-ENTRY
    SwitchAtsInstanceEntry                                                 = 3743,
    /// SWITCH-FLOW-METERING-ENTRY
    SwitchFlowMeteringEntry                                                = 1827,
    /// SWITCH-MAC-ADDRESS-LEARNING-MODE
    SwitchMacAddressLearningMode                                           = 3751,
    /// SWITCH-STREAM-FILTER-ENTRY
    SwitchStreamFilterEntry                                                = 5063,
    /// SWITCH-STREAM-GATE-ENTRY
    SwitchStreamGateEntry                                                  = 4358,
    /// SWITCH-STREAM-IDENTIFICATION
    SwitchStreamIdentification                                             = 6278,
    /// SWITCH-STREAM-IDENTIFICATIONS
    SwitchStreamIdentifications                                            = 6370,
    /// SYMBOL
    Symbol                                                                 = 798,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 5554,
    /// SYMBOL-WINDOW
    SymbolWindow                                                           = 1177,
    /// SYMBOL-WINDOW-ACTION-POINT-OFFSET
    SymbolWindowActionPointOffset                                          = 2700,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 1359,
    /// SYMBOLIC-NAME-VALUE
    SymbolicNameValue                                                      = 4429,
    /// SYNC-CONFIRMATION-TIMEOUT
    SyncConfirmationTimeout                                                = 5590,
    /// SYNC-COUNTER-INIT
    SyncCounterInit                                                        = 1507,
    /// SYNC-DATA-ID-LIST
    SyncDataIdList                                                         = 2484,
    /// SYNC-DATA-ID-LISTS
    SyncDataIdLists                                                        = 3749,
    /// SYNC-FRAME-ID-COUNT-MAX
    SyncFrameIdCountMax                                                    = 2080,
    /// SYNC-INTERVAL
    SyncInterval                                                           = 1392,
    /// SYNC-JUMP-WIDTH
    SyncJumpWidth                                                          = 5995,
    /// SYNC-LOSS-THRESHOLD
    SyncLossThreshold                                                      = 42,
    /// SYNC-LOSS-TIMEOUT
    SyncLossTimeout                                                        = 3040,
    /// SYNC-NODE-MAX
    SyncNodeMax                                                            = 1918,
    /// SYNC-PERIOD
    SyncPeriod                                                             = 2090,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 3005,
    /// SYNCHRONIZATION-CONSTRAINT-TYPE
    SynchronizationConstraintType                                          = 6509,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 2781,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 4426,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 595,
    /// SYNCHRONIZED-MODE-GROUPS
    SynchronizedModeGroups                                                 = 3015,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 5316,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 1073,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 650,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 3912,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 2542,
    /// SYNCHRONIZED-TRIGGERS
    SynchronizedTriggers                                                   = 3679,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 6371,
    /// SYSC-REF
    SyscRef                                                                = 1400,
    /// SYSC-STRING-REF
    SyscStringRef                                                          = 5635,
    /// SYSTEM
    System                                                                 = 1522,
    /// SYSTEM-COM-SPEC-DEFINITION-REF
    SystemComSpecDefinitionRef                                             = 2769,
    /// SYSTEM-COM-SPEC-DEFINITION-REFS
    SystemComSpecDefinitionRefs                                            = 3823,
    /// SYSTEM-COM-SPEC-DEFINITION-SET
    SystemComSpecDefinitionSet                                             = 3905,
    /// SYSTEM-DOCUMENTATIONS
    SystemDocumentations                                                   = 5617,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 2319,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 4755,
    /// SYSTEM-MEMORY-USAGES
    SystemMemoryUsages                                                     = 5566,
    /// SYSTEM-REF
    SystemRef                                                              = 6141,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 1635,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 1707,
    /// SYSTEM-SIGNAL-GROUP-REF
    SystemSignalGroupRef                                                   = 4465,
    /// SYSTEM-SIGNAL-GROUP-TO-COM-RESOURCE-MAPPINGS
    SystemSignalGroupToComResourceMappings                                 = 4618,
    /// SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalGroupToCommunicationResourceMapping                        = 878,
    /// SYSTEM-SIGNAL-REF
    SystemSignalRef                                                        = 6095,
    /// SYSTEM-SIGNAL-REFS
    SystemSignalRefs                                                       = 6224,
    /// SYSTEM-SIGNAL-TO-COM-RESOURCE-MAPPINGS
    SystemSignalToComResourceMappings                                      = 3847,
    /// SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING
    SystemSignalToCommunicationResourceMapping                             = 4621,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 606,
    /// SYSTEM-TRIGGERED-EVENTS
    SystemTriggeredEvents                                                  = 5851,
    /// SYSTEM-VERSION
    SystemVersion                                                          = 2890,
    /// TA-TYPE
    TaType                                                                 = 3034,
    /// TABLE
    Table                                                                  = 680,
    /// TABLE-CAPTION
    TableCaption                                                           = 3335,
    /// TABLE-ENTRYS
    TableEntrys                                                            = 3866,
    /// TAG-ID
    TagId                                                                  = 6586,
    /// TAG-WITH-OPTIONAL-VALUE
    TagWithOptionalValue                                                   = 3237,
    /// TAILORING-REF
    TailoringRef                                                           = 940,
    /// TAILORING-REFS
    TailoringRefs                                                          = 2248,
    /// TARGET-APPLICATION-RECORD-ELEMENT-REF
    TargetApplicationRecordElementRef                                      = 2971,
    /// TARGET-CATEGORY
    TargetCategory                                                         = 3579,
    /// TARGET-CHANNEL
    TargetChannel                                                          = 32,
    /// TARGET-COMPONENT-REF
    TargetComponentRef                                                     = 6069,
    /// TARGET-DATA-ELEMENT-REF
    TargetDataElementRef                                                   = 122,
    /// TARGET-DATA-PROTOTYPE-GROUP-REF
    TargetDataPrototypeGroupRef                                            = 3630,
    /// TARGET-DATA-PROTOTYPE-IN-CS-REF
    TargetDataPrototypeInCsRef                                             = 5318,
    /// TARGET-DATA-PROTOTYPE-IN-SR-REF
    TargetDataPrototypeInSrRef                                             = 5325,
    /// TARGET-DATA-PROTOTYPE-REF
    TargetDataPrototypeRef                                                 = 1742,
    /// TARGET-DATA-PROTOYPE-REF
    TargetDataProtoypeRef                                                  = 1101,
    /// TARGET-EEC-REF
    TargetEecRef                                                           = 2942,
    /// TARGET-EEC-REFS
    TargetEecRefs                                                          = 1121,
    /// TARGET-ELEMENT-REF
    TargetElementRef                                                       = 1725,
    /// TARGET-EVENT-REF
    TargetEventRef                                                         = 5147,
    /// TARGET-EVENT-REFS
    TargetEventRefs                                                        = 3386,
    /// TARGET-FIELD-REF
    TargetFieldRef                                                         = 1065,
    /// TARGET-FRAME-REF
    TargetFrameRef                                                         = 1191,
    /// TARGET-HEALTH-CHANNEL-REF
    TargetHealthChannelRef                                                 = 3947,
    /// TARGET-I-PDU
    TargetIPdu                                                             = 5098,
    /// TARGET-I-PDU-REF
    TargetIPduRef                                                          = 5060,
    /// TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TargetImplementationDataTypeElementRef                                 = 2176,
    /// TARGET-METHOD-REF
    TargetMethodRef                                                        = 5664,
    /// TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF
    TargetModeDeclarationGroupPrototypeRef                                 = 4181,
    /// TARGET-MODE-DECLARATION-REF
    TargetModeDeclarationRef                                               = 1309,
    /// TARGET-MODE-GROUP-REF
    TargetModeGroupRef                                                     = 5925,
    /// TARGET-MODE-REF
    TargetModeRef                                                          = 6009,
    /// TARGET-MODULE-ID
    TargetModuleId                                                         = 3530,
    /// TARGET-MODULE-REFS
    TargetModuleRefs                                                       = 3581,
    /// TARGET-OPERATION-REF
    TargetOperationRef                                                     = 5743,
    /// TARGET-P-PORT-PROTOTYPE-REF
    TargetPPortPrototypeRef                                                = 2044,
    /// TARGET-P-PORT-REF
    TargetPPortRef                                                         = 6165,
    /// TARGET-PARAMETER-DATA-PROTOTYPE-REF
    TargetParameterDataPrototypeRef                                        = 5142,
    /// TARGET-PDU-TRIGGERINGS
    TargetPduTriggerings                                                   = 1197,
    /// TARGET-PHM-CHECKPOINT-REF
    TargetPhmCheckpointRef                                                 = 470,
    /// TARGET-PHM-SUPERVISED-ENTITY-IREF
    TargetPhmSupervisedEntityIref                                          = 4544,
    /// TARGET-PORT-PROTOTYPE-REF
    TargetPortPrototypeRef                                                 = 4766,
    /// TARGET-POSSIBLE-ERROR-REF
    TargetPossibleErrorRef                                                 = 6151,
    /// TARGET-PROVIDED-OPERATION-REF
    TargetProvidedOperationRef                                             = 2379,
    /// TARGET-R-PORT-PROTOTYPE-REF
    TargetRPortPrototypeRef                                                = 4734,
    /// TARGET-R-PORT-REF
    TargetRPortRef                                                         = 2789,
    /// TARGET-REF
    TargetRef                                                              = 6078,
    /// TARGET-REFS
    TargetRefs                                                             = 5830,
    /// TARGET-REQUIRED-OPERATION-REF
    TargetRequiredOperationRef                                             = 1790,
    /// TARGET-RTE-EVENT-REF
    TargetRteEventRef                                                      = 3926,
    /// TARGET-RUNNABLE-ENTITY-GROUP-REF
    TargetRunnableEntityGroupRef                                           = 2561,
    /// TARGET-RUNNABLE-ENTITY-REF
    TargetRunnableEntityRef                                                = 6342,
    /// TARGET-SIGNAL-REF
    TargetSignalRef                                                        = 4795,
    /// TARGET-STATUS-REF
    TargetStatusRef                                                        = 1406,
    /// TARGET-SWC-SERVICE-DEPENDENCY-REF
    TargetSwcServiceDependencyRef                                          = 3115,
    /// TARGET-TRIGGER-REF
    TargetTriggerRef                                                       = 1176,
    /// TARGET-TYPE
    TargetType                                                             = 3989,
    /// TARGET-VARIABLE-ACCESS-REF
    TargetVariableAccessRef                                                = 4684,
    /// TARGET-VARIABLE-DATA-PROTOTYPE-REF
    TargetVariableDataPrototypeRef                                         = 5798,
    /// TASK
    Task                                                                   = 4128,
    /// TBODY
    Tbody                                                                  = 2906,
    /// TCP-ALIVE-CHECK-RESPONSE-TIMEOUT
    TcpAliveCheckResponseTimeout                                           = 1172,
    /// TCP-CONGESTION-AVOIDANCE-ENABLED
    TcpCongestionAvoidanceEnabled                                          = 4253,
    /// TCP-CONNECT-TIMEOUT
    TcpConnectTimeout                                                      = 6458,
    /// TCP-DELAYED-ACK-TIMEOUT
    TcpDelayedAckTimeout                                                   = 3731,
    /// TCP-FAST-RECOVERY-ENABLED
    TcpFastRecoveryEnabled                                                 = 3551,
    /// TCP-FAST-RETRANSMIT-ENABLED
    TcpFastRetransmitEnabled                                               = 4449,
    /// TCP-FIN-WAIT-2-TIMEOUT
    TcpFinWait2Timeout                                                     = 2432,
    /// TCP-GENERAL-INACTIVITY-TIME
    TcpGeneralInactivityTime                                               = 1258,
    /// TCP-INITIAL-INACTIVITY-TIME
    TcpInitialInactivityTime                                               = 1779,
    /// TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP
    TcpIpArpNumGratuitousArpOnStartup                                      = 5848,
    /// TCP-IP-ARP-PACKET-QUEUE-ENABLED
    TcpIpArpPacketQueueEnabled                                             = 6235,
    /// TCP-IP-ARP-REQUEST-TIMEOUT
    TcpIpArpRequestTimeout                                                 = 3035,
    /// TCP-IP-ARP-TABLE-ENTRY-TIMEOUT
    TcpIpArpTableEntryTimeout                                              = 5920,
    /// TCP-IP-AUTO-IP-INIT-TIMEOUT
    TcpIpAutoIpInitTimeout                                                 = 1972,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MAX
    TcpIpDhcpV6CnfDelayMax                                                 = 3232,
    /// TCP-IP-DHCP-V-6-CNF-DELAY-MIN
    TcpIpDhcpV6CnfDelayMin                                                 = 2064,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MAX
    TcpIpDhcpV6InfDelayMax                                                 = 579,
    /// TCP-IP-DHCP-V-6-INF-DELAY-MIN
    TcpIpDhcpV6InfDelayMin                                                 = 5075,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MAX
    TcpIpDhcpV6SolDelayMax                                                 = 3770,
    /// TCP-IP-DHCP-V-6-SOL-DELAY-MIN
    TcpIpDhcpV6SolDelayMin                                                 = 1816,
    /// TCP-IP-ICMP-PROPS-REF
    TcpIpIcmpPropsRef                                                      = 2704,
    /// TCP-IP-ICMP-PROPS-REFS
    TcpIpIcmpPropsRefs                                                     = 4314,
    /// TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED
    TcpIpIcmpV4EchoReplyEnabled                                            = 6439,
    /// TCP-IP-ICMP-V-4-TTL
    TcpIpIcmpV4Ttl                                                         = 5533,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION
    TcpIpIcmpV6EchoReplyAvoidFragmentation                                 = 5360,
    /// TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED
    TcpIpIcmpV6EchoReplyEnabled                                            = 3782,
    /// TCP-IP-ICMP-V-6-HOP-LIMIT
    TcpIpIcmpV6HopLimit                                                    = 2240,
    /// TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED
    TcpIpIcmpV6MsgDestinationUnreachableEnabled                            = 4539,
    /// TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED
    TcpIpIcmpV6MsgParameterProblemEnabled                                  = 6071,
    /// TCP-IP-IP-FRAGMENTATION-RX-ENABLED
    TcpIpIpFragmentationRxEnabled                                          = 4736,
    /// TCP-IP-IP-NUM-FRAGMENTS
    TcpIpIpNumFragments                                                    = 3106,
    /// TCP-IP-IP-NUM-REASS-DGRAMS
    TcpIpIpNumReassDgrams                                                  = 3018,
    /// TCP-IP-IP-REASS-TIMEOUT
    TcpIpIpReassTimeout                                                    = 1377,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-COUNT
    TcpIpIpReassemblyBufferCount                                           = 4260,
    /// TCP-IP-IP-REASSEMBLY-BUFFER-SIZE
    TcpIpIpReassemblyBufferSize                                            = 5114,
    /// TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT
    TcpIpIpReassemblySegmentCount                                          = 4027,
    /// TCP-IP-IP-REASSEMBLY-TIMEOUT
    TcpIpIpReassemblyTimeout                                               = 3503,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT
    TcpIpIpTxFragmentBufferCount                                           = 5680,
    /// TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE
    TcpIpIpTxFragmentBufferSize                                            = 4383,
    /// TCP-IP-NDP-DEFAULT-REACHABLE-TIME
    TcpIpNdpDefaultReachableTime                                           = 1653,
    /// TCP-IP-NDP-DEFAULT-RETRANS-TIMER
    TcpIpNdpDefaultRetransTimer                                            = 1547,
    /// TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE
    TcpIpNdpDefaultRouterListSize                                          = 203,
    /// TCP-IP-NDP-DEFENSIVE-PROCESSING
    TcpIpNdpDefensiveProcessing                                            = 2147,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME
    TcpIpNdpDelayFirstProbeTime                                            = 1005,
    /// TCP-IP-NDP-DELAY-FIRST-PROBE-TIME-VALUE
    TcpIpNdpDelayFirstProbeTimeValue                                       = 100,
    /// TCP-IP-NDP-DESTINATION-CACHE-SIZE
    TcpIpNdpDestinationCacheSize                                           = 3621,
    /// TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED
    TcpIpNdpDynamicHopLimitEnabled                                         = 1255,
    /// TCP-IP-NDP-DYNAMIC-MTU-ENABLED
    TcpIpNdpDynamicMtuEnabled                                              = 1213,
    /// TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED
    TcpIpNdpDynamicReachableTimeEnabled                                    = 2097,
    /// TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED
    TcpIpNdpDynamicRetransTimeEnabled                                      = 2091,
    /// TCP-IP-NDP-MAX-RANDOM-FACTOR
    TcpIpNdpMaxRandomFactor                                                = 6332,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY
    TcpIpNdpMaxRtrSolicitationDelay                                        = 2697,
    /// TCP-IP-NDP-MAX-RTR-SOLICITATIONS
    TcpIpNdpMaxRtrSolicitations                                            = 251,
    /// TCP-IP-NDP-MIN-RANDOM-FACTOR
    TcpIpNdpMinRandomFactor                                                = 901,
    /// TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED
    TcpIpNdpNeighborUnreachabilityDetectionEnabled                         = 4559,
    /// TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS
    TcpIpNdpNumMulticastSolicitations                                      = 3858,
    /// TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS
    TcpIpNdpNumUnicastSolicitations                                        = 6649,
    /// TCP-IP-NDP-PACKET-QUEUE-ENABLED
    TcpIpNdpPacketQueueEnabled                                             = 2785,
    /// TCP-IP-NDP-PREFIX-LIST-SIZE
    TcpIpNdpPrefixListSize                                                 = 4126,
    /// TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED
    TcpIpNdpRandomReachableTimeEnabled                                     = 3605,
    /// TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED
    TcpIpNdpRndRtrSolicitationDelayEnabled                                 = 3224,
    /// TCP-IP-NDP-RTR-SOLICITATION-INTERVAL
    TcpIpNdpRtrSolicitationInterval                                        = 3022,
    /// TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS
    TcpIpNdpSlaacDadNumberOfTransmissions                                  = 4348,
    /// TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY
    TcpIpNdpSlaacDadRetransmissionDelay                                    = 5276,
    /// TCP-IP-NDP-SLAAC-DELAY-ENABLED
    TcpIpNdpSlaacDelayEnabled                                              = 5084,
    /// TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED
    TcpIpNdpSlaacOptimisticDadEnabled                                      = 1598,
    /// TCP-IP-PROPS-REF
    TcpIpPropsRef                                                          = 2249,
    /// TCP-IP-PROPS-REFS
    TcpIpPropsRefs                                                         = 3016,
    /// TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC
    TcpIpTlsUseSecurityExtensionForceEncryptThenMac                        = 3929,
    /// TCP-KEEP-ALIVE-ENABLED
    TcpKeepAliveEnabled                                                    = 826,
    /// TCP-KEEP-ALIVE-INTERVAL
    TcpKeepAliveInterval                                                   = 6241,
    /// TCP-KEEP-ALIVE-PROBES-MAX
    TcpKeepAliveProbesMax                                                  = 440,
    /// TCP-KEEP-ALIVE-TIME
    TcpKeepAliveTime                                                       = 4785,
    /// TCP-LOCAL-PORT
    TcpLocalPort                                                           = 2799,
    /// TCP-MAX-RTX
    TcpMaxRtx                                                              = 1987,
    /// TCP-MSL
    TcpMsl                                                                 = 5145,
    /// TCP-NAGLE-ENABLED
    TcpNagleEnabled                                                        = 3596,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 3516,
    /// TCP-OPTION-FILTER-LISTS
    TcpOptionFilterLists                                                   = 4237,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 2486,
    /// TCP-PORT
    TcpPort                                                                = 905,
    /// TCP-PORT-REF
    TcpPortRef                                                             = 4057,
    /// TCP-PROPS
    TcpProps                                                               = 308,
    /// TCP-RECEIVE-WINDOW-MAX
    TcpReceiveWindowMax                                                    = 1966,
    /// TCP-REMOTE-PORT
    TcpRemotePort                                                          = 6420,
    /// TCP-RETRANSMISSION-TIMEOUT
    TcpRetransmissionTimeout                                               = 5003,
    /// TCP-ROLE
    TcpRole                                                                = 5769,
    /// TCP-RULE
    TcpRule                                                                = 6391,
    /// TCP-SLOW-START-ENABLED
    TcpSlowStartEnabled                                                    = 6503,
    /// TCP-SYN-MAX-RTX
    TcpSynMaxRtx                                                           = 1813,
    /// TCP-SYN-RECEIVED-TIMEOUT
    TcpSynReceivedTimeout                                                  = 6155,
    /// TCP-TP
    TcpTp                                                                  = 2681,
    /// TCP-TP-CONFIG
    TcpTpConfig                                                            = 1135,
    /// TCP-TP-PORT
    TcpTpPort                                                              = 3739,
    /// TCP-TTL
    TcpTtl                                                                 = 1640,
    /// TCP-UDP-CONFIG
    TcpUdpConfig                                                           = 2987,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 3960,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 5195,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 4749,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS
    TdCpSoftwareClusterResourceToTdMappings                                = 2417,
    /// TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS
    TdCpSoftwareClusterToTdMappings                                        = 3586,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 853,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE
    TdEventBswInternalBehaviorType                                         = 5402,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 1599,
    /// TD-EVENT-BSW-MODE-DECLARATION-TYPE
    TdEventBswModeDeclarationType                                          = 2118,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 6310,
    /// TD-EVENT-BSW-MODULE-TYPE
    TdEventBswModuleType                                                   = 1858,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 3988,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 1919,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 5965,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 3716,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 5398,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 163,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 3144,
    /// TD-EVENT-MODE-DECLARATION-TYPE
    TdEventModeDeclarationType                                             = 1961,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 4504,
    /// TD-EVENT-OPERATION-TYPE
    TdEventOperationType                                                   = 417,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 4872,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE
    TdEventServiceInstanceDiscoveryType                                    = 5234,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 1909,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE
    TdEventServiceInstanceEventType                                        = 1291,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 2641,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE
    TdEventServiceInstanceFieldType                                        = 6633,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 3491,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE
    TdEventServiceInstanceMethodType                                       = 5350,
    /// TD-EVENT-SLLET-PORT
    TdEventSlletPort                                                       = 1272,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 5047,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 1014,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE
    TdEventSwcInternalBehaviorType                                         = 2674,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 4837,
    /// TD-EVENT-TRIGGER-TYPE
    TdEventTriggerType                                                     = 6325,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 3627,
    /// TD-EVENT-TYPE
    TdEventType                                                            = 5012,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 1163,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE
    TdEventVariableDataPrototypeType                                       = 1946,
    /// TD-EVENT-VFB-PORT-GROUP
    TdEventVfbPortGroup                                                    = 373,
    /// TD-EVENT-VFB-PORT-REF
    TdEventVfbPortRef                                                      = 5618,
    /// TD-EVENT-VFB-PORT-REFS
    TdEventVfbPortRefs                                                     = 4860,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 2397,
    /// TD-HEADER-ID-FILTERS
    TdHeaderIdFilters                                                      = 780,
    /// TD-HEADER-ID-RANGE
    TdHeaderIdRange                                                        = 5323,
    /// TD-PDU-TRIGGERING-FILTER-REF
    TdPduTriggeringFilterRef                                               = 5051,
    /// TD-PDU-TRIGGERING-FILTER-REFS
    TdPduTriggeringFilterRefs                                              = 3124,
    /// TDLET-ZONE-CLOCK
    TdletZoneClock                                                         = 1182,
    /// TEAR-DOWN-ACTION-REF
    TearDownActionRef                                                      = 5125,
    /// TEAR-DOWN-ACTION-REFS
    TearDownActionRefs                                                     = 4660,
    /// TEMPERATURE-EXP
    TemperatureExp                                                         = 4828,
    /// TEMPLATE-ARGUMENTS
    TemplateArguments                                                      = 4379,
    /// TEMPLATE-TYPE-REF
    TemplateTypeRef                                                        = 6537,
    /// TERMINATE
    Terminate                                                              = 6513,
    /// TERMINATING-CHECKPOINT-REF
    TerminatingCheckpointRef                                               = 1703,
    /// TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION
    TerminatingCheckpointTimeoutUntilTermination                           = 2165,
    /// TERMINATION-BEHAVIOR
    TerminationBehavior                                                    = 4771,
    /// TEST-ID
    TestId                                                                 = 4689,
    /// TEST-ID-REF
    TestIdRef                                                              = 1602,
    /// TEST-IDENTIFIER
    TestIdentifier                                                         = 250,
    /// TEST-PATTERN
    TestPattern                                                            = 3468,
    /// TEST-RESULT-REF
    TestResultRef                                                          = 3448,
    /// TESTED-ITEM-REF
    TestedItemRef                                                          = 5205,
    /// TESTED-ITEM-REFS
    TestedItemRefs                                                         = 4081,
    /// TEX-MATH
    TexMath                                                                = 2187,
    /// TEXT
    Text                                                                   = 4530,
    /// TEXT-TABLE-MAPPING
    TextTableMapping                                                       = 5496,
    /// TEXT-TABLE-MAPPINGS
    TextTableMappings                                                      = 130,
    /// TEXT-TABLE-VALUE-PAIR
    TextTableValuePair                                                     = 1143,
    /// TEXT-VALUE-SPECIFICATION
    TextValueSpecification                                                 = 2819,
    /// TEXTUAL-CONDITION
    TextualCondition                                                       = 5988,
    /// TFOOT
    Tfoot                                                                  = 1802,
    /// TGROUP
    Tgroup                                                                 = 5606,
    /// THEAD
    Thead                                                                  = 4777,
    /// THRESHOLD
    Threshold                                                              = 485,
    /// THRESHOLD-NUMBER
    ThresholdNumber                                                        = 636,
    /// THRESHOLD-SIZE
    ThresholdSize                                                          = 5615,
    /// TIME-BASE
    TimeBase                                                               = 2932,
    /// TIME-BASE-CONSUMER-REF
    TimeBaseConsumerRef                                                    = 5767,
    /// TIME-BASE-JITTER
    TimeBaseJitter                                                         = 2539,
    /// TIME-BASE-KIND
    TimeBaseKind                                                           = 2757,
    /// TIME-BASE-PROVIDER-REF
    TimeBaseProviderRef                                                    = 3507,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 528,
    /// TIME-BASE-RESOURCE-REF
    TimeBaseResourceRef                                                    = 1853,
    /// TIME-BASE-RESOURCE-REF-CONDITIONAL
    TimeBaseResourceRefConditional                                         = 3082,
    /// TIME-BASE-RESOURCE-REFS
    TimeBaseResourceRefs                                                   = 3338,
    /// TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE
    TimeBasedFdcThresholdStorageValue                                      = 399,
    /// TIME-BASES
    TimeBases                                                              = 4989,
    /// TIME-BR
    TimeBr                                                                 = 5786,
    /// TIME-BUFFER
    TimeBuffer                                                             = 1501,
    /// TIME-CS
    TimeCs                                                                 = 791,
    /// TIME-EXP
    TimeExp                                                                = 2268,
    /// TIME-FAILED-THRESHOLD
    TimeFailedThreshold                                                    = 3687,
    /// TIME-FR-IF
    TimeFrIf                                                               = 6428,
    /// TIME-HARDWARE-CORRECTION-THRESHOLD
    TimeHardwareCorrectionThreshold                                        = 5955,
    /// TIME-INTERVAL
    TimeInterval                                                           = 6140,
    /// TIME-LEAP-FUTURE-THRESHOLD
    TimeLeapFutureThreshold                                                = 984,
    /// TIME-LEAP-HEALING-COUNTER
    TimeLeapHealingCounter                                                 = 2093,
    /// TIME-LEAP-PAST-THRESHOLD
    TimeLeapPastThreshold                                                  = 1018,
    /// TIME-MARK
    TimeMark                                                               = 4826,
    /// TIME-MASTER-PRIORITY
    TimeMasterPriority                                                     = 1279,
    /// TIME-OFFSET
    TimeOffset                                                             = 3885,
    /// TIME-PASSED-THRESHOLD
    TimePassedThreshold                                                    = 2353,
    /// TIME-PERIOD
    TimePeriod                                                             = 3614,
    /// TIME-SEG-1
    TimeSeg1                                                               = 328,
    /// TIME-SEG-2
    TimeSeg2                                                               = 6546,
    /// TIME-SUB-TLV
    TimeSubTlv                                                             = 198,
    /// TIME-SYNC-CLIENT
    TimeSyncClient                                                         = 4355,
    /// TIME-SYNC-CORRECTION
    TimeSyncCorrection                                                     = 1051,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 2624,
    /// TIME-SYNC-P-PORT-PROTOTYPE-IREF
    TimeSyncPPortPrototypeIref                                             = 253,
    /// TIME-SYNC-PORT-PROTOTYPE-IREF
    TimeSyncPortPrototypeIref                                              = 4739,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 3699,
    /// TIME-SYNC-R-PORT-PROTOTYPE-IREF
    TimeSyncRPortPrototypeIref                                             = 2609,
    /// TIME-SYNC-SERVER
    TimeSyncServer                                                         = 5438,
    /// TIME-SYNC-SERVER-IDENTIFIER
    TimeSyncServerIdentifier                                               = 1435,
    /// TIME-SYNC-SERVER-REF
    TimeSyncServerRef                                                      = 60,
    /// TIME-SYNC-TECHNOLOGY
    TimeSyncTechnology                                                     = 1069,
    /// TIME-SYNCHRONIZATION
    TimeSynchronization                                                    = 425,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 2989,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 4099,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 1277,
    /// TIME-TO-LIVE
    TimeToLive                                                             = 468,
    /// TIME-TRIGGERED-CAN-LEVEL
    TimeTriggeredCanLevel                                                  = 3759,
    /// TIME-VALUE
    TimeValue                                                              = 4411,
    /// TIME-VALUE-VALUE-VARIATION-POINT
    TimeValueValueVariationPoint                                           = 860,
    /// TIMEOUT
    Timeout                                                                = 5035,
    /// TIMEOUT-AR
    TimeoutAr                                                              = 946,
    /// TIMEOUT-AS
    TimeoutAs                                                              = 2774,
    /// TIMEOUT-BR
    TimeoutBr                                                              = 5816,
    /// TIMEOUT-BS
    TimeoutBs                                                              = 3967,
    /// TIMEOUT-CHECK
    TimeoutCheck                                                           = 6160,
    /// TIMEOUT-CR
    TimeoutCr                                                              = 6232,
    /// TIMEOUT-CS
    TimeoutCs                                                              = 48,
    /// TIMEOUT-SUBSTITUTION-VALUE
    TimeoutSubstitutionValue                                               = 4320,
    /// TIMESTAMP-FORMAT
    TimestampFormat                                                        = 254,
    /// TIMESTAMP-INTERVAL
    TimestampInterval                                                      = 3795,
    /// TIMESTAMP-RX-ACCEPTANCE-WINDOW
    TimestampRxAcceptanceWindow                                            = 3612,
    /// TIMESTAMP-SUPPORT
    TimestampSupport                                                       = 1801,
    /// TIMING-ARGUMENT-REF
    TimingArgumentRef                                                      = 6329,
    /// TIMING-ARGUMENTS
    TimingArguments                                                        = 1551,
    /// TIMING-CLOCK-SYNC-ACCURACY
    TimingClockSyncAccuracy                                                = 5832,
    /// TIMING-CLOCK-SYNC-ACCURACYS
    TimingClockSyncAccuracys                                               = 1728,
    /// TIMING-CLOCKS
    TimingClocks                                                           = 170,
    /// TIMING-CONDITION
    TimingCondition                                                        = 3577,
    /// TIMING-CONDITION-FORMULA
    TimingConditionFormula                                                 = 1171,
    /// TIMING-CONDITION-REF
    TimingConditionRef                                                     = 115,
    /// TIMING-CONDITIONS
    TimingConditions                                                       = 25,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 2934,
    /// TIMING-DESCRIPTION-REF
    TimingDescriptionRef                                                   = 3584,
    /// TIMING-DESCRIPTIONS
    TimingDescriptions                                                     = 6085,
    /// TIMING-EVENT
    TimingEvent                                                            = 4079,
    /// TIMING-EVENT-REF
    TimingEventRef                                                         = 3121,
    /// TIMING-GUARANTEES
    TimingGuarantees                                                       = 4422,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 2610,
    /// TIMING-MODE-REF
    TimingModeRef                                                          = 4221,
    /// TIMING-MODES
    TimingModes                                                            = 3160,
    /// TIMING-PROPS
    TimingProps                                                            = 628,
    /// TIMING-REQUIREMENTS
    TimingRequirements                                                     = 477,
    /// TIMING-RESOURCE
    TimingResource                                                         = 6357,
    /// TIMING-VARIABLE-REF
    TimingVariableRef                                                      = 1131,
    /// TIMING-VARIABLES
    TimingVariables                                                        = 3536,
    /// TLS-CIPHER-SUITE
    TlsCipherSuite                                                         = 1344,
    /// TLS-CIPHER-SUITES
    TlsCipherSuites                                                        = 5904,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 1978,
    /// TLS-CONNECTION-REF
    TlsConnectionRef                                                       = 1282,
    /// TLS-CONNECTION-REFS
    TlsConnectionRefs                                                      = 3671,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 4040,
    /// TLS-CRYPTO-MAPPING-REF
    TlsCryptoMappingRef                                                    = 3625,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 2919,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 3566,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 1817,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 967,
    /// TLS-JOB-MAPPINGS
    TlsJobMappings                                                         = 2526,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 2619,
    /// TLS-JOB-REQUIREMENT-REF
    TlsJobRequirementRef                                                   = 244,
    /// TLS-JOB-SEMANTIC
    TlsJobSemantic                                                         = 4586,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 6629,
    /// TLS-SECURE-COM-PROPS-REF
    TlsSecureComPropsRef                                                   = 2822,
    /// TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF
    TlvAppRecordElementInMethodIref                                        = 4831,
    /// TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF
    TlvAppRecordElementInServiceInterfaceIref                              = 2995,
    /// TLV-ARGUMENT-REF
    TlvArgumentRef                                                         = 2519,
    /// TLV-DATA-ID
    TlvDataId                                                              = 5136,
    /// TLV-DATA-ID-0-REF
    TlvDataId0Ref                                                          = 927,
    /// TLV-DATA-ID-0-REFS
    TlvDataId0Refs                                                         = 788,
    /// TLV-DATA-ID-DEFINITION
    TlvDataIdDefinition                                                    = 2017,
    /// TLV-DATA-ID-DEFINITION-REF
    TlvDataIdDefinitionRef                                                 = 5217,
    /// TLV-DATA-ID-DEFINITION-REFS
    TlvDataIdDefinitionRefs                                                = 465,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 2736,
    /// TLV-DATA-ID-DEFINITIONS
    TlvDataIdDefinitions                                                   = 5843,
    /// TLV-DATA-IDS
    TlvDataIds                                                             = 5322,
    /// TLV-IMPL-RECORD-ELEMENT
    TlvImplRecordElement                                                   = 5570,
    /// TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF
    TlvImplementationDataTypeElementRef                                    = 1936,
    /// TLV-RECORD-ELEMENT-REF
    TlvRecordElementRef                                                    = 4950,
    /// TLV-SUB-ELEMENT-REF
    TlvSubElementRef                                                       = 5594,
    /// TO-REF
    ToRef                                                                  = 935,
    /// TOL
    Tol                                                                    = 5428,
    /// TOLERANCE
    Tolerance                                                              = 1043,
    /// TOLERATED-FAILED-CYCLES
    ToleratedFailedCycles                                                  = 4730,
    /// TOOL
    Tool                                                                   = 3742,
    /// TOOL-VERSION
    ToolVersion                                                            = 1649,
    /// TOPIC-1
    Topic1                                                                 = 867,
    /// TOPIC-DATA
    TopicData                                                              = 326,
    /// TOPIC-NAME
    TopicName                                                              = 6293,
    /// TP-ADDRESS
    TpAddress                                                              = 3407,
    /// TP-ADDRESS-EXTENSION-VALUE
    TpAddressExtensionValue                                                = 3633,
    /// TP-ADDRESS-REF
    TpAddressRef                                                           = 2279,
    /// TP-ADDRESS-REFS
    TpAddressRefs                                                          = 2149,
    /// TP-ADDRESSS
    TpAddresss                                                             = 3804,
    /// TP-CHANNEL-REF
    TpChannelRef                                                           = 6210,
    /// TP-CHANNELS
    TpChannels                                                             = 2359,
    /// TP-CONCURRENT-PROCESSING-SDUS
    TpConcurrentProcessingSdus                                             = 2009,
    /// TP-CONFIGURATION
    TpConfiguration                                                        = 488,
    /// TP-CONNECTION-CONTROL-REF
    TpConnectionControlRef                                                 = 4249,
    /// TP-CONNECTION-CONTROLS
    TpConnectionControls                                                   = 2005,
    /// TP-CONNECTION-REF
    TpConnectionRef                                                        = 5506,
    /// TP-CONNECTION-REFS
    TpConnectionRefs                                                       = 4365,
    /// TP-CONNECTIONS
    TpConnections                                                          = 561,
    /// TP-ECU
    TpEcu                                                                  = 4236,
    /// TP-ECUS
    TpEcus                                                                 = 1908,
    /// TP-NODES
    TpNodes                                                                = 2644,
    /// TP-PGS
    TpPgs                                                                  = 5065,
    /// TP-PROTOCOL-TYPE
    TpProtocolType                                                         = 4711,
    /// TP-SDU-REF
    TpSduRef                                                               = 3137,
    /// TP-SDU-REFS
    TpSduRefs                                                              = 3655,
    /// TP-TECHNOLOGY
    TpTechnology                                                           = 2658,
    /// TRACE
    Trace                                                                  = 3785,
    /// TRACE-MESSAGE-REF
    TraceMessageRef                                                        = 3350,
    /// TRACE-REF
    TraceRef                                                               = 1562,
    /// TRACE-REFS
    TraceRefs                                                              = 5967,
    /// TRACE-SWITCH
    TraceSwitch                                                            = 4338,
    /// TRACE-SWITCH-CONFIG
    TraceSwitchConfig                                                      = 335,
    /// TRACE-SWITCH-CONFIGURATION
    TraceSwitchConfiguration                                               = 196,
    /// TRACE-SWITCH-CONFIGURATIONS
    TraceSwitchConfigurations                                              = 6312,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 274,
    /// TRACED-FAILURES
    TracedFailures                                                         = 2113,
    /// TRAFFIC-CLASS
    TrafficClass                                                           = 3829,
    /// TRAFFIC-CLASS-PREEMPTION-SUPPORT
    TrafficClassPreemptionSupport                                          = 3508,
    /// TRAFFIC-LIMITATION-FILTERS
    TrafficLimitationFilters                                               = 4278,
    /// TRAFFIC-SHAPER-GROUP-REF
    TrafficShaperGroupRef                                                  = 5619,
    /// TRAFFIC-SHAPER-GROUPS
    TrafficShaperGroups                                                    = 862,
    /// TRANCEIVER-STANDBY-DELAY
    TranceiverStandbyDelay                                                 = 6583,
    /// TRANSFER-EXIT-CLASS-REF
    TransferExitClassRef                                                   = 3139,
    /// TRANSFER-PROPERTY
    TransferProperty                                                       = 1323,
    /// TRANSFER-REF
    TransferRef                                                            = 4940,
    /// TRANSFER-REFS
    TransferRefs                                                           = 1697,
    /// TRANSFERS
    Transfers                                                              = 1164,
    /// TRANSFORMATION-COM-SPEC-PROPSS
    TransformationComSpecPropss                                            = 3868,
    /// TRANSFORMATION-DESCRIPTIONS
    TransformationDescriptions                                             = 2307,
    /// TRANSFORMATION-I-SIGNAL-PROPSS
    TransformationISignalPropss                                            = 3689,
    /// TRANSFORMATION-PROPS-MAPPING-SET-REF
    TransformationPropsMappingSetRef                                       = 2311,
    /// TRANSFORMATION-PROPS-REF
    TransformationPropsRef                                                 = 2656,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 771,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 6237,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 2447,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING
    TransformationPropsToServiceInterfaceMapping                           = 3360,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 1161,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF
    TransformationPropsToServiceInterfaceMappingSetRef                     = 1224,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS
    TransformationPropsToServiceInterfaceMappings                          = 2661,
    /// TRANSFORMATION-PROPSS
    TransformationPropss                                                   = 5116,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 2457,
    /// TRANSFORMATION-TECHNOLOGYS
    TransformationTechnologys                                              = 2915,
    /// TRANSFORMER-CHAIN-REF
    TransformerChainRef                                                    = 6366,
    /// TRANSFORMER-CHAIN-REFS
    TransformerChainRefs                                                   = 5792,
    /// TRANSFORMER-CLASS
    TransformerClass                                                       = 3791,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 2816,
    /// TRANSFORMER-REF
    TransformerRef                                                         = 2742,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 5984,
    /// TRANSFORMING-SYSTEM-SIGNAL-REF
    TransformingSystemSignalRef                                            = 793,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 6374,
    /// TRANSIT-TO-INVALID-EXTENDED
    TransitToInvalidExtended                                               = 5104,
    /// TRANSITION-REF
    TransitionRef                                                          = 4969,
    /// TRANSITION-REFS
    TransitionRefs                                                         = 3127,
    /// TRANSITIONS
    Transitions                                                            = 2924,
    /// TRANSLATED-VLAN-ID
    TranslatedVlanId                                                       = 1288,
    /// TRANSLATION-TARGET-IREF
    TranslationTargetIref                                                  = 3363,
    /// TRANSMISSION-ACKNOWLEDGE
    TransmissionAcknowledge                                                = 4772,
    /// TRANSMISSION-DEADLINE
    TransmissionDeadline                                                   = 5605,
    /// TRANSMISSION-MODE
    TransmissionMode                                                       = 4963,
    /// TRANSMISSION-MODE-CONDITION
    TransmissionModeCondition                                              = 2476,
    /// TRANSMISSION-MODE-CONDITIONS
    TransmissionModeConditions                                             = 3204,
    /// TRANSMISSION-MODE-DECLARATION
    TransmissionModeDeclaration                                            = 5124,
    /// TRANSMISSION-MODE-FALSE-TIMING
    TransmissionModeFalseTiming                                            = 1963,
    /// TRANSMISSION-MODE-TRUE-TIMING
    TransmissionModeTrueTiming                                             = 2161,
    /// TRANSMISSION-PROPS
    TransmissionProps                                                      = 277,
    /// TRANSMISSION-START-SEQUENCE-DURATION
    TransmissionStartSequenceDuration                                      = 165,
    /// TRANSMISSION-TRIGGER
    TransmissionTrigger                                                    = 6463,
    /// TRANSMIT-CANCELLATION
    TransmitCancellation                                                   = 1089,
    /// TRANSMIT-PDU-REF
    TransmitPduRef                                                         = 2533,
    /// TRANSMIT-PDU-REFS
    TransmitPduRefs                                                        = 3415,
    /// TRANSMITTER-REF
    TransmitterRef                                                         = 2466,
    /// TRANSPORT-ERROR-COUNT-ENABLED
    TransportErrorCountEnabled                                             = 844,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 4305,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 3424,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF
    TransportLayerIndependentInstanceIdRef                                 = 4036,
    /// TRANSPORT-LAYER-RULE
    TransportLayerRule                                                     = 704,
    /// TRANSPORT-PDU-REF
    TransportPduRef                                                        = 2655,
    /// TRANSPORT-PLUGIN
    TransportPlugin                                                        = 6394,
    /// TRANSPORT-PLUGINS
    TransportPlugins                                                       = 5333,
    /// TRANSPORT-PRIORITY
    TransportPriority                                                      = 3068,
    /// TRANSPORT-PROTOCOL
    TransportProtocol                                                      = 6555,
    /// TRANSPORT-PROTOCOLS
    TransportProtocols                                                     = 2977,
    /// TRCV-DELAY-COMPENSATION-OFFSET
    TrcvDelayCompensationOffset                                            = 3558,
    /// TRCV-PWM-MODE-ENABLED
    TrcvPwmModeEnabled                                                     = 3852,
    /// TRIGGER
    Trigger                                                                = 4439,
    /// TRIGGER-DIRECT-IMPLEMENTATIONS
    TriggerDirectImplementations                                           = 4626,
    /// TRIGGER-I-PDU-SEND-CONDITION
    TriggerIPduSendCondition                                               = 2316,
    /// TRIGGER-I-PDU-SEND-CONDITIONS
    TriggerIPduSendConditions                                              = 3172,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 6348,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 3024,
    /// TRIGGER-IREF
    TriggerIref                                                            = 5172,
    /// TRIGGER-MAPPING
    TriggerMapping                                                         = 3925,
    /// TRIGGER-MAPPINGS
    TriggerMappings                                                        = 5247,
    /// TRIGGER-MODE
    TriggerMode                                                            = 5194,
    /// TRIGGER-PERIOD
    TriggerPeriod                                                          = 3281,
    /// TRIGGER-PORT-ANNOTATION
    TriggerPortAnnotation                                                  = 5038,
    /// TRIGGER-PORT-ANNOTATIONS
    TriggerPortAnnotations                                                 = 4723,
    /// TRIGGER-REF
    TriggerRef                                                             = 3311,
    /// TRIGGER-REF-CONDITIONAL
    TriggerRefConditional                                                  = 3305,
    /// TRIGGER-REFS
    TriggerRefs                                                            = 3003,
    /// TRIGGER-TO-SIGNAL-MAPPING
    TriggerToSignalMapping                                                 = 6506,
    /// TRIGGERING-EVENT-REF
    TriggeringEventRef                                                     = 5818,
    /// TRIGGERS
    Triggers                                                               = 4423,
    /// TRIGGERS-RECOVERY-NOTIFICATION
    TriggersRecoveryNotification                                           = 3260,
    /// TROUBLE-CODE-J-1939-REF
    TroubleCodeJ1939Ref                                                    = 4936,
    /// TROUBLE-CODE-OBD-REF
    TroubleCodeObdRef                                                      = 5684,
    /// TROUBLE-CODE-PROPSS
    TroubleCodePropss                                                      = 5857,
    /// TROUBLE-CODE-UDS-REF
    TroubleCodeUdsRef                                                      = 5362,
    /// TRUE-ACTION-LIST-REF
    TrueActionListRef                                                      = 2959,
    /// TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR
    TrustedPlatformExecutableLaunchBehavior                                = 3050,
    /// TT
    Tt                                                                     = 246,
    /// TT-CAN-CLUSTER-REF
    TtCanClusterRef                                                        = 4901,
    /// TTCAN-ABSOLUTELY-SCHEDULED-TIMING
    TtcanAbsolutelyScheduledTiming                                         = 3162,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 4510,
    /// TTCAN-CLUSTER-CONDITIONAL
    TtcanClusterConditional                                                = 4037,
    /// TTCAN-CLUSTER-VARIANTS
    TtcanClusterVariants                                                   = 1542,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 503,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 2021,
    /// TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL
    TtcanCommunicationControllerConditional                                = 4943,
    /// TTCAN-COMMUNICATION-CONTROLLER-VARIANTS
    TtcanCommunicationControllerVariants                                   = 1913,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 3331,
    /// TTL
    Ttl                                                                    = 3332,
    /// TTL-MAX
    TtlMax                                                                 = 2546,
    /// TTL-MIN
    TtlMin                                                                 = 4274,
    /// TWO-KEY-SLOT-MODE
    TwoKeySlotMode                                                         = 2433,
    /// TX-BIT-RATE-SWITCH
    TxBitRateSwitch                                                        = 6300,
    /// TX-DIRECT-PDU-PRIORITY
    TxDirectPduPriority                                                    = 6245,
    /// TX-ENABLE-WINDOW-LENGTH
    TxEnableWindowLength                                                   = 2721,
    /// TX-MASK
    TxMask                                                                 = 5841,
    /// TX-NM-PDU-REF
    TxNmPduRef                                                             = 12,
    /// TX-NM-PDU-REFS
    TxNmPduRefs                                                            = 1560,
    /// TX-PDU-POOL-REF
    TxPduPoolRef                                                           = 2881,
    /// TX-PDU-PRIORITY
    TxPduPriority                                                          = 2745,
    /// TX-PDU-TRIGGERING-REF
    TxPduTriggeringRef                                                     = 1427,
    /// TYP
    Typ                                                                    = 1941,
    /// TYPE
    Type                                                                   = 4392,
    /// TYPE-APPROVAL
    TypeApproval                                                           = 6135,
    /// TYPE-BLUEPRINTS
    TypeBlueprints                                                         = 5574,
    /// TYPE-DEFINITION
    TypeDefinition                                                         = 3228,
    /// TYPE-EMITTER
    TypeEmitter                                                            = 3205,
    /// TYPE-MAPPING
    TypeMapping                                                            = 452,
    /// TYPE-OF-DTC-SUPPORTED
    TypeOfDtcSupported                                                     = 1661,
    /// TYPE-OF-EVENT-COMBINATION-SUPPORTED
    TypeOfEventCombinationSupported                                        = 6560,
    /// TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION
    TypeOfFreezeFrameRecordNumeration                                      = 1536,
    /// TYPE-REFERENCE
    TypeReference                                                          = 5031,
    /// TYPE-REFERENCE-REF
    TypeReferenceRef                                                       = 904,
    /// TYPE-TAILORINGS
    TypeTailorings                                                         = 5513,
    /// TYPE-TREF
    TypeTref                                                               = 1944,
    /// UAS-ID
    UasId                                                                  = 2351,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 5448,
    /// UCM-MASTER-FALLBACK-REF
    UcmMasterFallbackRef                                                   = 1398,
    /// UCM-MASTER-FALLBACK-REFS
    UcmMasterFallbackRefs                                                  = 1618,
    /// UCM-MASTER-MODULE-INSTANTIATION
    UcmMasterModuleInstantiation                                           = 2869,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 1091,
    /// UCM-MODULE-INSTANTIATION-REF
    UcmModuleInstantiationRef                                              = 5079,
    /// UCM-NOT-AVAILABLE-ON-THE-NETWORK
    UcmNotAvailableOnTheNetwork                                            = 1133,
    /// UCM-PROCESSINGS
    UcmProcessings                                                         = 3249,
    /// UCM-REF
    UcmRef                                                                 = 1076,
    /// UCM-SOFTWARE-VERSION
    UcmSoftwareVersion                                                     = 2194,
    /// UCM-STEP
    UcmStep                                                                = 1283,
    /// UCM-SUBORDINATE-MODULE-INSTANTIATION
    UcmSubordinateModuleInstantiation                                      = 3828,
    /// UCM-TO-TIME-BASE-RESOURCE-MAPPING
    UcmToTimeBaseResourceMapping                                           = 347,
    /// UCMS
    Ucms                                                                   = 4046,
    /// UDP-CHECKSUM-HANDLING
    UdpChecksumHandling                                                    = 4853,
    /// UDP-COLLECTION-BUFFER-SIZE-THRESHOLD
    UdpCollectionBufferSizeThreshold                                       = 6139,
    /// UDP-COLLECTION-BUFFER-TIMEOUT
    UdpCollectionBufferTimeout                                             = 1706,
    /// UDP-COLLECTION-TRIGGER
    UdpCollectionTrigger                                                   = 1148,
    /// UDP-LOCAL-PORT
    UdpLocalPort                                                           = 3284,
    /// UDP-MIN-TX-BUFFER-SIZE
    UdpMinTxBufferSize                                                     = 1261,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 2141,
    /// UDP-NM-CLUSTER-COUPLING
    UdpNmClusterCoupling                                                   = 3250,
    /// UDP-NM-CLUSTER-REF
    UdpNmClusterRef                                                        = 3282,
    /// UDP-NM-ECU
    UdpNmEcu                                                               = 1422,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 2247,
    /// UDP-PORT
    UdpPort                                                                = 5639,
    /// UDP-PORT-REF
    UdpPortRef                                                             = 228,
    /// UDP-PROPS
    UdpProps                                                               = 2855,
    /// UDP-REMOTE-PORT
    UdpRemotePort                                                          = 241,
    /// UDP-RULE
    UdpRule                                                                = 2623,
    /// UDP-TP
    UdpTp                                                                  = 4447,
    /// UDP-TP-PORT
    UdpTpPort                                                              = 4473,
    /// UDP-TTL
    UdpTtl                                                                 = 5755,
    /// UDS-DTC-NUMBER
    UdsDtcNumber                                                           = 2598,
    /// UDS-DTC-VALUE
    UdsDtcValue                                                            = 4304,
    /// UDS-SERVICE
    UdsService                                                             = 545,
    /// UNASSIGN-FRAME-ID
    UnassignFrameId                                                        = 2114,
    /// UNASSIGNED-FRAME-TRIGGERING-REF
    UnassignedFrameTriggeringRef                                           = 217,
    /// UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE
    UncompressedSoftwareClusterSize                                        = 2683,
    /// UNICAST-CREDENTIALS
    UnicastCredentials                                                     = 2863,
    /// UNICAST-NETWORK-ENDPOINT-REF
    UnicastNetworkEndpointRef                                              = 4921,
    /// UNICAST-NETWORK-ENDPOINT-REFS
    UnicastNetworkEndpointRefs                                             = 232,
    /// UNICAST-SECURE-COM-PROPS-REF
    UnicastSecureComPropsRef                                               = 3167,
    /// UNICAST-SECURE-COM-PROPS-REFS
    UnicastSecureComPropsRefs                                              = 2772,
    /// UNICAST-UDP-CREDENTIALS
    UnicastUdpCredentials                                                  = 6074,
    /// UNIQUE-STREAM-ID
    UniqueStreamId                                                         = 4239,
    /// UNIT
    Unit                                                                   = 91,
    /// UNIT-AND-SCALING-ID
    UnitAndScalingId                                                       = 4617,
    /// UNIT-DISPLAY-NAME
    UnitDisplayName                                                        = 1046,
    /// UNIT-GROUP
    UnitGroup                                                              = 5348,
    /// UNIT-GROUP-REF
    UnitGroupRef                                                           = 3046,
    /// UNIT-GROUP-REFS
    UnitGroupRefs                                                          = 1371,
    /// UNIT-REF
    UnitRef                                                                = 2022,
    /// UNIT-REFS
    UnitRefs                                                               = 2917,
    /// UNLIMITED-INTEGER-VALUE-VARIATION-POINT
    UnlimitedIntegerValueVariationPoint                                    = 5992,
    /// UNRESOLVED-REFERENCE-RESTRICTION
    UnresolvedReferenceRestriction                                         = 1446,
    /// UNUSED-BIT-PATTERN
    UnusedBitPattern                                                       = 830,
    /// UPDATE
    Update                                                                 = 6346,
    /// UPDATE-INDICATION-BIT-POSITION
    UpdateIndicationBitPosition                                            = 2474,
    /// UPDATE-KIND
    UpdateKind                                                             = 245,
    /// UPDATE-SESSION-REJECTED
    UpdateSessionRejected                                                  = 6409,
    /// UPDATE-STRATEGY
    UpdateStrategy                                                         = 5462,
    /// UPPER-BOUND
    UpperBound                                                             = 2879,
    /// UPPER-BOUNDARY
    UpperBoundary                                                          = 1619,
    /// UPPER-CAN-ID
    UpperCanId                                                             = 2844,
    /// UPPER-HEADER-BITS-TO-SHIFT
    UpperHeaderBitsToShift                                                 = 4753,
    /// UPPER-LIMIT
    UpperLimit                                                             = 6453,
    /// UPPER-MULTIPLICITY
    UpperMultiplicity                                                      = 5045,
    /// UPPER-MULTIPLICITY-INFINITE
    UpperMultiplicityInfinite                                              = 411,
    /// UPPER-MULTIPLICITY-IS-INFINITE
    UpperMultiplicityIsInfinite                                            = 6231,
    /// UPPER-REF
    UpperRef                                                               = 6045,
    /// UPSTREAM-REFERENCE-IREF
    UpstreamReferenceIref                                                  = 3483,
    /// URI
    Uri                                                                    = 4947,
    /// URI-DESCRIPTION
    UriDescription                                                         = 5030,
    /// URL
    Url                                                                    = 5338,
    /// USAGE
    Usage                                                                  = 539,
    /// USAGES
    Usages                                                                 = 1985,
    /// USE-AS-CRYPTOGRAPHIC-I-PDU
    UseAsCryptographicIPdu                                                 = 3724,
    /// USE-AUTH-DATA-FRESHNESS
    UseAuthDataFreshness                                                   = 4168,
    /// USE-AUTO-VALIDATION-AT-SHUT-DOWN
    UseAutoValidationAtShutDown                                            = 770,
    /// USE-CASE
    UseCase                                                                = 4240,
    /// USE-CLIENT-AUTHENTICATION-REQUEST
    UseClientAuthenticationRequest                                         = 3135,
    /// USE-CRC-COMP-MECHANISM
    UseCrcCompMechanism                                                    = 6385,
    /// USE-FRESHNESS-TIMESTAMP
    UseFreshnessTimestamp                                                  = 5418,
    /// USE-INSTEAD-REF
    UseInsteadRef                                                          = 2402,
    /// USE-INSTEAD-REFS
    UseInsteadRefs                                                         = 3114,
    /// USE-MAC-ADDRESS-FOR-IDENTIFICATION
    UseMacAddressForIdentification                                         = 5291,
    /// USE-SECURED-PDU-HEADER
    UseSecuredPduHeader                                                    = 5993,
    /// USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT
    UseSecurityExtensionRecordSizeLimit                                    = 2506,
    /// USE-SMART-SENSOR-API
    UseSmartSensorApi                                                      = 492,
    /// USE-VARIABLE-DA
    UseVariableDa                                                          = 49,
    /// USE-VARIABLE-SA
    UseVariableSa                                                          = 1373,
    /// USE-VEHICLE-IDENTIFICATION-SYNC-STATUS
    UseVehicleIdentificationSyncStatus                                     = 5426,
    /// USED-CODE-GENERATOR
    UsedCodeGenerator                                                      = 4729,
    /// USED-DATA-ELEMENT
    UsedDataElement                                                        = 2613,
    /// USED-FID-REF
    UsedFidRef                                                             = 1125,
    /// USED-IMPLEMENTATION-DATA-TYPE-REF
    UsedImplementationDataTypeRef                                          = 1849,
    /// USED-IMPOSITION-TIME-DEFINITION-GROUP-REF
    UsedImpositionTimeDefinitionGroupRef                                   = 4028,
    /// USED-LANGUAGES
    UsedLanguages                                                          = 1445,
    /// USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF
    UsedLifeCycleStateDefinitionGroupRef                                   = 1533,
    /// USED-PARAMETER-ELEMENT
    UsedParameterElement                                                   = 4087,
    /// USED-PIM-REF
    UsedPimRef                                                             = 5602,
    /// USED-SECONDARY-FID-REF
    UsedSecondaryFidRef                                                    = 3240,
    /// USED-SECONDARY-FID-REFS
    UsedSecondaryFidRefs                                                   = 6256,
    /// USER-DATA-SUB-TLV
    UserDataSubTlv                                                         = 1702,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 173,
    /// USER-DEFINED-CLUSTER-CONDITIONAL
    UserDefinedClusterConditional                                          = 5972,
    /// USER-DEFINED-CLUSTER-VARIANTS
    UserDefinedClusterVariants                                             = 499,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 3359,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 2426,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL
    UserDefinedCommunicationControllerConditional                          = 1887,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS
    UserDefinedCommunicationControllerVariants                             = 555,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 45,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 4114,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 473,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 5882,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 4951,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 5253,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 1081,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 5883,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 3582,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 2340,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 4118,
    /// USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS
    UserDefinedTransformationComSpecProps                                  = 3522,
    /// USER-DEFINED-TRANSFORMATION-DESCRIPTION
    UserDefinedTransformationDescription                                   = 3637,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS
    UserDefinedTransformationISignalProps                                  = 516,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL
    UserDefinedTransformationISignalPropsConditional                       = 4634,
    /// USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS
    UserDefinedTransformationISignalPropsVariants                          = 110,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 970,
    /// USER-REF
    UserRef                                                                = 997,
    /// USER-REFS
    UserRefs                                                               = 3670,
    /// USES-ADDRESS-ARBITRATION
    UsesAddressArbitration                                                 = 1368,
    /// USES-DYNAMIC-ADDRESSING
    UsesDynamicAddressing                                                  = 329,
    /// USES-END-TO-END-PROTECTION
    UsesEndToEndProtection                                                 = 2829,
    /// USES-MONITOR-DATA
    UsesMonitorData                                                        = 2846,
    /// USES-TIME-BASE-RESOURCE
    UsesTimeBaseResource                                                   = 4942,
    /// V
    V                                                                      = 4761,
    /// V-2-X-DATA-MANAGER-NEEDS
    V2XDataManagerNeeds                                                    = 4408,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 1236,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 2020,
    /// V-2-X-SUPPORTED
    V2XSupported                                                           = 3564,
    /// V-LAN-REF
    VLanRef                                                                = 3910,
    /// V-LAN-REFS
    VLanRefs                                                               = 3451,
    /// VALID-BINDING-TIME
    ValidBindingTime                                                       = 1628,
    /// VALID-BINDING-TIMES
    ValidBindingTimes                                                      = 5975,
    /// VALIDATION-CONFIGURATION
    ValidationConfiguration                                                = 3318,
    /// VALIDATION-FORMULA
    ValidationFormula                                                      = 5890,
    /// VALIDATION-REF
    ValidationRef                                                          = 4283,
    /// VALIDATION-ROOT
    ValidationRoot                                                         = 4862,
    /// VALUE
    Value                                                                  = 486,
    /// VALUE-ACCESS
    ValueAccess                                                            = 4630,
    /// VALUE-AXIS-DATA-TYPE-REF
    ValueAxisDataTypeRef                                                   = 3458,
    /// VALUE-CONFIG-CLASSES
    ValueConfigClasses                                                     = 6196,
    /// VALUE-DATA-TYPE-REF
    ValueDataTypeRef                                                       = 3012,
    /// VALUE-ERROR-COUNT-ENABLED
    ValueErrorCountEnabled                                                 = 1559,
    /// VALUE-IREF
    ValueIref                                                              = 4,
    /// VALUE-PAIRS
    ValuePairs                                                             = 5906,
    /// VALUE-REF
    ValueRef                                                               = 5990,
    /// VALUE-RESTRICTION
    ValueRestriction                                                       = 870,
    /// VALUE-SPEC
    ValueSpec                                                              = 6551,
    /// VALUE-TYPE-REF
    ValueTypeRef                                                           = 4994,
    /// VALUE-TYPE-TREF
    ValueTypeTref                                                          = 4094,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 661,
    /// VARIABLE-ACCESS-IREF
    VariableAccessIref                                                     = 5342,
    /// VARIABLE-ACCESS-IREFS
    VariableAccessIrefs                                                    = 1839,
    /// VARIABLE-ACCESS-REF
    VariableAccessRef                                                      = 1479,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REF
    VariableAccessedForDebugRef                                            = 4446,
    /// VARIABLE-ACCESSED-FOR-DEBUG-REFS
    VariableAccessedForDebugRefs                                           = 3317,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 6337,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 4108,
    /// VARIABLE-DATA-PROTOTYPE-IREF
    VariableDataPrototypeIref                                              = 6448,
    /// VARIABLE-DATA-PROTOTYPE-IREFS
    VariableDataPrototypeIrefs                                             = 4270,
    /// VARIABLE-INSTANCE
    VariableInstance                                                       = 1735,
    /// VARIABLE-INSTANCE-IREF
    VariableInstanceIref                                                   = 4489,
    /// VARIABLE-LENGTH
    VariableLength                                                         = 4800,
    /// VARIABLE-REF
    VariableRef                                                            = 2478,
    /// VARIABLES
    Variables                                                              = 436,
    /// VARIANT-CRITERION-REF
    VariantCriterionRef                                                    = 3708,
    /// VARIANT-ID
    VariantId                                                              = 2595,
    /// VARIATION
    Variation                                                              = 4870,
    /// VARIATION-POINT
    VariationPoint                                                         = 4227,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 5869,
    /// VARIATION-POINT-PROXYS
    VariationPointProxys                                                   = 651,
    /// VARIATION-RESTRICTION
    VariationRestriction                                                   = 219,
    /// VCID
    Vcid                                                                   = 6514,
    /// VEHICLE-ANNOUNCEMENT-COUNT
    VehicleAnnouncementCount                                               = 4064,
    /// VEHICLE-ANNOUNCEMENT-INTERVAL
    VehicleAnnouncementInterval                                            = 2557,
    /// VEHICLE-DESCRIPTION-REF
    VehicleDescriptionRef                                                  = 4174,
    /// VEHICLE-DRIVER-NOTIFICATION
    VehicleDriverNotification                                              = 5501,
    /// VEHICLE-IDENTIFICATION-SYNC-STATUS
    VehicleIdentificationSyncStatus                                        = 1331,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 5720,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 1953,
    /// VEHICLE-SYSTEM
    VehicleSystem                                                          = 4243,
    /// VEHICLE-SYSTEM-INSTANCE
    VehicleSystemInstance                                                  = 2444,
    /// VENDOR
    Vendor                                                                 = 2224,
    /// VENDOR-API-INFIX
    VendorApiInfix                                                         = 823,
    /// VENDOR-ID
    VendorId                                                               = 5921,
    /// VENDOR-SIGNATURE-REF
    VendorSignatureRef                                                     = 5167,
    /// VENDOR-SPECIFIC-MODULE-DEF-REF
    VendorSpecificModuleDefRef                                             = 424,
    /// VENDOR-SPECIFIC-MODULE-DEF-REFS
    VendorSpecificModuleDefRefs                                            = 3037,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 2726,
    /// VERBATIM
    Verbatim                                                               = 3938,
    /// VERIFICATION-HASH
    VerificationHash                                                       = 4357,
    /// VERIFICATION-METHOD
    VerificationMethod                                                     = 5544,
    /// VERIFICATION-STATUS-INDICATION-MODE
    VerificationStatusIndicationMode                                       = 233,
    /// VERIFY-IREF
    VerifyIref                                                             = 822,
    /// VERIFY-IREFS
    VerifyIrefs                                                            = 1848,
    /// VERIFY-REF
    VerifyRef                                                              = 2797,
    /// VERIFY-REFS
    VerifyRefs                                                             = 205,
    /// VERIFY-UPDATE
    VerifyUpdate                                                           = 4751,
    /// VERSION
    Version                                                                = 6635,
    /// VERSION-DRIVEN-FIND-BEHAVIOR
    VersionDrivenFindBehavior                                              = 1907,
    /// VF
    Vf                                                                     = 3252,
    /// VFB-TIMING
    VfbTiming                                                              = 4386,
    /// VFC-IREF
    VfcIref                                                                = 5404,
    /// VFC-IREFS
    VfcIrefs                                                               = 5837,
    /// VG
    Vg                                                                     = 3288,
    /// VIEW-MAP
    ViewMap                                                                = 3656,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 3649,
    /// VIEW-MAPS
    ViewMaps                                                               = 2369,
    /// VIN-INVALIDITY-PATTERN
    VinInvalidityPattern                                                   = 1494,
    /// VIOLATED-SAFETY-CONDITION-BEHAVIOR
    ViolatedSafetyConditionBehavior                                        = 3296,
    /// VLAN
    Vlan                                                                   = 2360,
    /// VLAN-ID
    VlanId                                                                 = 6083,
    /// VLAN-IDENTIFIER
    VlanIdentifier                                                         = 5037,
    /// VLAN-MEMBERSHIP
    VlanMembership                                                         = 1881,
    /// VLAN-MEMBERSHIPS
    VlanMemberships                                                        = 4925,
    /// VLAN-MODIFIER-REF
    VlanModifierRef                                                        = 4433,
    /// VLAN-PRIORITY
    VlanPriority                                                           = 227,
    /// VLAN-REF
    VlanRef                                                                = 125,
    /// VLAN-REFS
    VlanRefs                                                               = 1879,
    /// VLAN-TRANSLATION-TABLES
    VlanTranslationTables                                                  = 1565,
    /// VT
    Vt                                                                     = 2554,
    /// VTF
    Vtf                                                                    = 4014,
    /// WAIT-POINT
    WaitPoint                                                              = 2705,
    /// WAIT-POINTS
    WaitPoints                                                             = 2667,
    /// WAKE-UP-BY-CONTROLLER-SUPPORTED
    WakeUpByControllerSupported                                            = 3667,
    /// WAKE-UP-CHANNEL
    WakeUpChannel                                                          = 548,
    /// WAKE-UP-OVER-BUS-SUPPORTED
    WakeUpOverBusSupported                                                 = 2189,
    /// WAKE-UP-PATTERN
    WakeUpPattern                                                          = 3097,
    /// WAKEUP-FORWARD-LOCAL-ENABLED
    WakeupForwardLocalEnabled                                              = 5011,
    /// WAKEUP-FORWARD-REMOTE-ENABLED
    WakeupForwardRemoteEnabled                                             = 5870,
    /// WAKEUP-FRAME-REF
    WakeupFrameRef                                                         = 5750,
    /// WAKEUP-FRAME-REFS
    WakeupFrameRefs                                                        = 5729,
    /// WAKEUP-LOCAL-DETECTION-TIME
    WakeupLocalDetectionTime                                               = 5141,
    /// WAKEUP-LOCAL-DURATION-TIME
    WakeupLocalDurationTime                                                = 5938,
    /// WAKEUP-LOCAL-ENABLED
    WakeupLocalEnabled                                                     = 1506,
    /// WAKEUP-REMOTE-ENABLED
    WakeupRemoteEnabled                                                    = 1215,
    /// WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST
    WakeupRepetitionDelayOfWakeupRequest                                   = 4697,
    /// WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST
    WakeupRepetitionsOfWakeupRequest                                       = 1884,
    /// WAKEUP-RX-IDLE
    WakeupRxIdle                                                           = 4092,
    /// WAKEUP-RX-LOW
    WakeupRxLow                                                            = 3636,
    /// WAKEUP-RX-WINDOW
    WakeupRxWindow                                                         = 585,
    /// WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF
    WakeupSleepOnDatalineConfigRef                                         = 2579,
    /// WAKEUP-SYMBOL-RX-WINDOW
    WakeupSymbolRxWindow                                                   = 4317,
    /// WAKEUP-TX-ACTIVE
    WakeupTxActive                                                         = 643,
    /// WAKEUP-TX-IDLE
    WakeupTxIdle                                                           = 457,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 1499,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 4480,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 4492,
    /// WINDOW-SIZE
    WindowSize                                                             = 942,
    /// WINDOW-SIZE-INIT
    WindowSizeInit                                                         = 2647,
    /// WINDOW-SIZE-INVALID
    WindowSizeInvalid                                                      = 4605,
    /// WINDOW-SIZE-VALID
    WindowSizeValid                                                        = 1736,
    /// WITH-AUTO
    WithAuto                                                               = 2335,
    /// WORST-CASE-EXECUTION-TIME
    WorstCaseExecutionTime                                                 = 6174,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 1870,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 6646,
    /// WRITE
    Write                                                                  = 4385,
    /// WRITE-ACCESS
    WriteAccess                                                            = 148,
    /// WRITE-CLASS-REF
    WriteClassRef                                                          = 5571,
    /// WRITE-ONLY-ONCE
    WriteOnlyOnce                                                          = 4548,
    /// WRITE-VERIFICATION
    WriteVerification                                                      = 4360,
    /// WRITER-ENTITY-ID
    WriterEntityId                                                         = 1983,
    /// WRITING-FREQUENCY
    WritingFrequency                                                       = 6601,
    /// WRITING-PRIORITY
    WritingPriority                                                        = 3279,
    /// WRITING-STRATEGY-ROLE
    WritingStrategyRole                                                    = 5151,
    /// WRITING-STRATEGYS
    WritingStrategys                                                       = 2344,
    /// WRITTEN-LOCAL-VARIABLES
    WrittenLocalVariables                                                  = 2657,
    /// WRITTEN-NV-DATA
    WrittenNvData                                                          = 3142,
    /// WRITTEN-READ-NV-DATA
    WrittenReadNvData                                                      = 3935,
    /// WWH-OBD-DTC-CLASS
    WwhObdDtcClass                                                         = 1467,
    /// X
    X                                                                      = 2324,
    /// XCP-PDU
    XcpPdu                                                                 = 3319,
    /// XDOC
    Xdoc                                                                   = 4861,
    /// XFILE
    Xfile                                                                  = 430,
    /// XREF
    Xref                                                                   = 484,
    /// XREF-TARGET
    XrefTarget                                                             = 6116,
}

impl ElementName {
    #[rustfmt::skip]
    const STRING_TABLE: [&'static str; 6651] = ["IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "SOURCE-IP-ADDRESS", "HW-TYPE", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "VALUE-IREF", "LIFE-CYCLE-INFOS", "DELEGATED-PORT-ANNOTATION", "SENDER-REC-ARRAY-TYPE-MAPPING", "CAN-FD-BAUDRATE", "APMC-NUMERICAL-PARAM-VALUE", "E-2-E-PROFILE-CONFIGURATION-SET", "SD-SERVER-TIMER-CONFIGS", "TX-NM-PDU-REF", "ECUC-MODULE-CONFIGURATION-VALUES", "I-PDU-IDENTIFIER-TCP-REF", "ETH-GLOBAL-TIME-DOMAIN-PROPS", "LT-MESSAGE-COLLECTION-TO-PORT-PROTOTYPE-MAPPING", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "CHANNEL-REF", "CLIENT-INTENT", "MACRO-PER-CYCLE", "LICENSE-REF", "CAN-FRAME-TRIGGERING", "ACCEPTANCE-FIELD", "REPRESENTED-MODEL-ELEMENT-REF", "TIMING-CONDITIONS", "RPT-SERVICE-POINT-PRE-REFS", "PARAMETER-REQUIRE-COM-SPEC", "EXPIRED-SUPERVISION-CYCLES-TOLERANCE", "MODULE-REFS", "STATE-REF", "SENDER-CAPABILITY", "TARGET-CHANNEL", "END-TO-END-PROTECTIONS", "DIAGNOSTIC-SOVD-GROUP", "CRYPTO-SERVICE-KEY", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "STATE-MANAGEMENT-NM-ACTION-ITEM", "SOMEIP-TP-CONFIG", "ECUC-BOOLEAN-PARAM-DEF", "FIRST-MODE-REF", "DERIVED-OBJECT-REF", "SYNC-LOSS-THRESHOLD", "FIGURE-CAPTION", "DIAGNOSTIC-SERVICE-TABLE-REF-CONDITIONAL", "USER-DEFINED-ETHERNET-FRAME", "NM-VOTING-CYCLE", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "TIMEOUT-CS", "USE-VARIABLE-DA", "OPERATION-PROTOTYPE-IREF", "LIN-IDENTIFIER", "SEC-OC-CRYPTO-SERVICE-MAPPING-REF", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "EVENT-READINESS-GROUPS", "MCD-IDENTIFIER", "CONFIG-ELEMENT-DEF-GLOBAL-REF", "DIAGNOSTIC-SERVICE-INSTANCE-REF-CONDITIONAL", "INITIAL-REF-OFFSET", "SECOND-APPLICATION-ERROR-REF", "TIME-SYNC-SERVER-REF", "LET-DATA-EXCHANGE-PARADIGM", "LIN-SLAVE-VARIANTS", "SHORT-LABEL-PATTERN", "DDS-FIELD-REPLY-TOPIC-REF", "PRIMITIVE-TYPE-MAPPINGS", "FIREWALL-RULE", "ACL-SCOPE", "OFS-DATA-ID-LIST", "ECU-PARTITION-TO-CORE-MAPPINGS", "IDSM-INSTANCES", "ALIGNMENT", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NODE-MAPPING", "START-ON-EVENT-REF", "RPT-READ-ACCESS", "SERIALIZER", "CREATE-ECU-WAKEUP-SOURCE", "DYNAMIC-ARRAY-SIZE-PROFILE", "CYCLIC-BACKUP-INTERVAL", "POSSIBLE-ERROR-REACTION", "PHM-RECOVERY-ACTION-INTERFACE", "DELETES", "DOMAIN-IDS", "COMPOUND-PRIMITIVE-ARGUMENTS", "DDS-EVENT-QOS-PROPS", "FLEXRAY-FIFO-RANGE", "CONDITION-FORMULA", "RAW-DATA-STREAM-CLIENT-INTERFACE", "SOMEIP-TP-CHANNEL", "EXECUTION-TIME-TYPE", "FUNCTION-GROUP-SET", "UNIT", "CLIENT-SERVER-ARRAY-TYPE-MAPPING", "CONTEXT-COMPONENT-REF", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "CRC-DOMAIN-NUMBER", "FINAL-CHECKPOINT-REF", "SECOND-TO-FIRST-DATA-TRANSFORMATION-REF", "ENVIRONMENT-VARIABLES", "SW-COMP-TO-ECU-MAPPING-REF", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME-VALUE", "ACK-TYPE", "SEC-OC-SECURE-COM-PROPS-REF", "IP-SEC-CONFIG-PROPS-REF", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "APPLICATION-TYPE-REF", "DIAGNOSTIC-EXTENDED-DATA-RECORD-CLIENT-PORT-MAPPING", "I-SIGNAL-I-PDU-REF", "NM-REPEAT-MSG-INDICATION-ENABLED", "COUNTER-DECREMENT-STEP-SIZE", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "MASTER", "COMM-CONNECTORS", "HARDWARE-ELEMENT-REF", "COMPONENT-SEPARATION", "TIMING-CONDITION-REF", "ALGORITHM-SECONDARY-FAMILY", "MODE-DECLARATION-GROUP-PROTOTYPE-REF", "DATA-CONSISTENCY-POLICY", "NEXT-STATE-IREF", "INDUSTRY-GROUP", "STATE-MANAGEMENT-MODULE-INSTANTIATION", "TARGET-DATA-ELEMENT-REF", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "FM-FEATURE-MAP-CONDITION", "VLAN-REF", "DATA-PROTOTYPE-REF", "DIAGNOSTIC-RELEVANCE", "MULTICAST-THRESHOLD", "CHECKPOINTS", "TEXT-TABLE-MAPPINGS", "CLIENT-COM-SPEC", "SUPPORTED-FEATURES", "PERIODIC-RATE-CATEGORY", "RESPONSE-GROUP-REF", "MEASURED-HEAP-USAGE", "IMPLICIT-DATA-ACCESS-IREFS", "REFERENCE-VALUES", "ROUTING-GROUP-REF", "REMOTE-CLIENT-CONFIG", "NM-CHANNEL-ACTIVE", "CUSTOM-DOCUMENTATION-REF", "ECU-TASK-PROXY-REF", "SOURCE-NETWORK-MASK", "DELTA-PACKAGE-APPLICABLE-VERSION", "DEPENDS-ON-REFS", "ACL-OBJECT-REFS", "PNC-SYNCHRONOUS-WAKEUP", "WRITE-ACCESS", "CHANNEL-SYNCHRONOUS-WAKEUP", "ALLOCATOR", "FM-ATTRIBUTE-VALUE", "PHM-ARBITRATION", "SLOT-TYPE", "REVISION-LABELS", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "CONFIDENCE-INTERVAL", "ROM-LOCATION-REF", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-INTERFACE", "START-INDEX", "ECUC-STRING-PARAM-DEF-VARIANTS", "APPLICATION-PARTITION", "OBJECT-DEFINTION-REF", "TD-EVENT-I-SIGNAL", "PIN-NUMBER", "TRANSMISSION-START-SEQUENCE-DURATION", "PDU-COUNTERS", "NM-NODE-REF", "FRAME-REF", "CRYPTO-SIGNATURE-SCHEME", "TIMING-CLOCKS", "NM-INTERACTS-WITH-SM-MAPPING", "CYCLE-REPETITION", "USER-DEFINED-CLUSTER", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-PORT-MAPPING", "SW-COMPONENT-MAPPING-CONSTRAINTS", "DEFAULT-TRACE-STATE", "EVENT-OCCURRENCE-KIND", "CAK-REF", "MIN-MAJOR-VERSION", "COM-CONFIGURATION-GW-TIME-BASE", "INITIAL-INACTIVITY-TIME", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "CLIENT-SERVER-RECORD-TYPE-MAPPING", "SOMEIP-REMOTE-MULTICAST-CONFIG", "NM-SYNCHRONIZATION-POINT-ENABLED", "FLEXRAY-AR-TP-CHANNEL", "DDS-METHOD-QOS-PROPS", "PRESTORED-FREEZEFRAME-STORED-IN-NVM", "RAPID-PROTOTYPING-SCENARIO", "MATCHING-RULE-REFS", "HW-ELEMENT-REFS", "ETHERNET-TRAFFIC-CLASS-ASSIGNMENTS", "ALLOW-HALT-DUE-TO-CLOCK", "SW-SYSTEMCONSTANT-VALUES", "ASSIGN-FRAME-ID-RANGE", "TRACE-SWITCH-CONFIGURATION", "NO-CHECKPOINT-SUPERVISIONS", "TIME-SUB-TLV", "COM-PROPS", "NM-NODE-DETECTION-ENABLED", "FUNCTION-POINTER-SIGNATURE-REF", "COUPLING-PORT-ENHANCED-TRAFFIC-SHAPER", "TCP-IP-NDP-DEFAULT-ROUTER-LIST-SIZE", "ACCESS-CONTROL", "VERIFY-REFS", "STATUS-BIT-STORAGE-TEST-FAILED", "MODE-INTERFACE-MAPPING", "MINIMUM-TIMER-GRANULARITY", "DDS-SECURE-GOVERNANCE", "HANDLE-TERMINATION-AND-RESTART", "DEFAULT-PRIORITY", "DIAGNOSTIC-COMMON-ELEMENT-REF", "SO-AD-CONFIG", "R-PORT-IN-COMPOSITION-INSTANCE-REF", "MAX-EVENTS-IN-INTERVAL", "SEQUENTIAL-INSTRUCTIONS-END", "UNASSIGNED-FRAME-TRIGGERING-REF", "ISSUED-TRIGGERS", "VARIATION-RESTRICTION", "PRESTORAGE-FREEZE-FRAME", "INITIAL-CHECKPOINT-REFS", "CAN-ENTER-EXCLUSIVE-AREA-REFS", "CLIENT-CAPABILITY", "PERSISTENCY-KEY-VALUE-DATABASE", "PNC-NM-REQUEST", "ECU-MAPPING", "VLAN-PRIORITY", "UDP-PORT-REF", "RX-CYCLE-PROCESS-TIME", "CONTEXT-COMPONENT-REFS", "ACCESS-COUNT-SETS", "UNICAST-NETWORK-ENDPOINT-REFS", "VERIFICATION-STATUS-INDICATION-MODE", "REMOTE-CONFIGS", "DOIP-INTERFACES", "APPLICATION-PARTITION-REF", "LIN-SLAVE-CONDITIONAL", "RUNNABLE-ENTITY-IREFS", "DDS-DOMAINS", "SECURITY-EVENT-REPORT-INSTANCE-DEFINITION", "UDP-REMOTE-PORT", "GENERIC-ETHERNET-FRAME", "PREVIOUS-EXECUTABLE-VERSION", "TLS-JOB-REQUIREMENT-REF", "UPDATE-KIND", "TT", "EXECUTABLE-GROUP", "RPT-EXECUTION-CONTEXT", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "TEST-IDENTIFIER", "TCP-IP-NDP-MAX-RTR-SOLICITATIONS", "GID-INVALIDITY-PATTERN", "TIME-SYNC-P-PORT-PROTOTYPE-IREF", "TIMESTAMP-FORMAT", "NODE-PORTS", "BUFFER-PROPERTIES", "REQUESTABLE", "REGULAR-EXPRESSION", "EXECUTION-TIMES", "REFERENCE-BASES", "IS-OBD-RELEVANT", "DATA-FILTER", "FUNCTION-GROUPS", "SOURCE-CAN-ID-REF", "DEFAULT-PATTERN", "BITFIELD-TEXT-TABLE-MASK-FIRST", "FIRST-TO-SECOND-DATA-TRANSFORMATION-REF", "MULTICAST-SECURE-COM-PROPS-REF", "ALIVE-REFERENCE-CYCLE", "ACL-ROLE-REF", "FM-FEATURE-MAP-ELEMENT", "PROCESSING-KIND", "ASSUMED-CURRENT-STATE-IREF", "TRACEABLE-TABLE", "NUMERICAL-VALUE-VARIATION-POINT", "MAX-ERROR-STATE-VALID", "TRANSMISSION-PROPS", "CALLBACK-HEADER-REF", "EVENT-GROUP-CONTROL-TYPE", "MC-FUNCTION", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "DCM-I-PDU", "LOCAL-VARIABLE-REF", "RECORD-ELEMENT-MAPPINGS", "REQUIRED-TRIGGERS", "DEPENDENT-RESOURCES", "MAX-NUMBER-OF-EVENT-ENTRIES", "ENTERED-MODE-REF", "IMPLEMENTS-LEGACY-STRING-SERIALIZATION", "PARTICIPATING-J-1939-CLUSTER-REF", "HW-ATTRIBUTE-LITERALS", "DDS-REQUIRED-SERVICE-INSTANCE", "BUILD-ACTIONS", "OS-TASK-PROXY-REF", "ACL-PERMISSION", "POSSIBLE-ERRORS", "PULSE-TEST", "LATENCY-CONSTRAINT-TYPE", "CLUSTERED-COMPONENT-IREFS", "CAN-CONTROLLER-CONFIGURATION-REQUIREMENTS", "DERIVED-FROM-BLUEPRINT-REFS", "QUEUED-RECEIVER-COM-SPEC-PROPSS", "MINIMUM-NUMBER-OF-REQUESTS", "ECUC-COND", "SECURITY-EVENTS", "APPLICATION-ASSOC-MAP-VALUE-SPECIFICATION", "TCP-PROPS", "CUSTOM-SDG-DEF-REF", "IP-PROTOCOL", "FILES", "DO-IP-LOGIC-ADDRESS", "IS-PIPELINING-PERMITTED", "PREFIX-REF", "DEPLOYMENT-REF", "COMPONENT-IREFS", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "DIAG-ROUTINE-TYPE", "MODE-DECLARATION-GROUP-PROTOTYPE-REF-CONDITIONAL", "SOURCE-MAX-ADDRESS", "SOFTWARE-CLUSTER-DOIP-DIAGNOSTIC-ADDRESS", "CRYPTO-SERVICE-JOB-NEEDS", "IP-SEC-RULES", "IS-VARIADIC-TEMPLATE", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "TOPIC-DATA", "HW-ELEMENT-CONNECTOR", "TIME-SEG-1", "USES-DYNAMIC-ADDRESSING", "RESPONSE-ON-EVENT-SCHEDULER-RATE", "BASE-REF", "IMPLEMENTATION-ARRAY-ELEMENT-REF", "SECURE-COM-CONFIG-TO-CRYPTO-KEY-SLOT-MAPPING", "PDU-COLLECTION-TRIGGER", "TRACE-SWITCH-CONFIG", "FLOW-CONTROL-PDU-REFS", "ADAPTIVE-AUTOSAR-APPLICATION", "J-1939-NODES", "IPV-4-PROPS", "POST-VERIFICATION-REBOOT", "I-SIGNAL-TRIGGERING-REF-CONDITIONAL", "DO-IP-TARGET-ADDRESS-REF", "DESTINATION-PHYSICAL-ADDRESS", "DESTINATION-MAC-ADDRESS", "DEBOUNCE-TIME", "PROVIDED-EVENT-GROUPS", "UCM-TO-TIME-BASE-RESOURCE-MAPPING", "REQUIRED-SERVICE-INSTANCE-ID", "ETH-TP-CONNECTION", "HEALTH-CHANNEL-EXTERNAL-REPORTED-STATUS", "DIAGNOSTIC-PERIODIC-RATE", "SW-COMPONENT-PROTOTYPE-ASSIGNMENT", "MINOR-VERSION", "ISSUED-BY", "PROCESS-EXECUTION-ERROR", "EXPLICIT-RPT-PROFILE-SELECTION-REFS", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "NUMBER-OF-STATIC-SLOTS", "PWM-S", "INDEX-DATA-TYPE-REF", "PROVIDED-DDS-SERVICE-INSTANCE", "DIAGNOSTIC-EXTENDED-DATA-RECORD-NEEDS", "AGGREGATION-CONDITION", "SDG", "CKN-REF", "LOG-TRACE-PORT-ID", "LOCAL-PORT-REF", "BITFIELD-TEXT-TABLE-MASK-PORT-PROTOTYPE", "PLATFORM-TIME-BASES", "ALIAS-NAMES", "CRC-PRECISE-ORIGIN-TIMESTAMP", "ALGORITHM-FAMILY", "TD-EVENT-VFB-PORT-GROUP", "BASE-TYPE-SIZE", "SOMEIP-TP-CONNECTION", "DIAGNOSTIC-SOVD-BULK-DATA", "LENGTH", "CONSUMED-OPERATIONS", "MODE-PORT-ANNOTATION", "IMPLEMENTED-ENTRY-REF", "CSE-CODE", "OUTGOING-CALLBACKS", "SW-CALPRM-IMPLS", "NAMESPACES", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REF", "IPV-4-ADDRESS-MASK", "ELEMENT-POSITION", "ENABLE-CONDITION-REF", "MAX-SERVICE-POINT-ID", "AUTHENTICATION-ENABLED", "FLEXRAY-CLUSTER-VARIANTS", "EXECUTABLE-REFS", "METHOD-REF", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "FR-CLUSTER-REF", "I-PDU-IDENTIFIERS", "SOURCE-IPV-6-ADDRESS", "SAVE-CONFIGURATION-ENTRY", "TIME-BASED-FDC-THRESHOLD-STORAGE-VALUE", "IPV-6-RULE", "HARDWARE-ELEMENT-REFS", "REMOTE-UNICAST-CONFIG-REF", "SPECIFICATION-DOCUMENT-SCOPE", "MIN-NUMBER-OF-TIME-QUANTA-PER-BIT", "SECURE-COM-PROPS-FOR-TCP-REFS", "SPECIFICATION-DOCUMENT-SCOPES", "SERVER-PORT-REF", "COM-TRIGGER-GRANT-DESIGN", "CRYPTO-KEY-SLOT-REFS", "BSW-SCHEDULABLE-ENTITY", "UPPER-MULTIPLICITY-INFINITE", "ICMP-RULE", "NM-CAR-WAKE-UP-FILTER-NODE-ID", "SW-COMPOSITIONS", "APMC-STRONG-REVISION-LABEL-PARAM-DEF", "CHECKSUM-VERIFICATION", "TD-EVENT-OPERATION-TYPE", "RELIABILITY", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "PREDECESSOR-ACTION-REF", "CONSTRAINT-TAILORING", "SUB-FUNCTION-REFS", "LIN-COMMUNICATION-CONNECTOR", "VENDOR-SPECIFIC-MODULE-DEF-REF", "TIME-SYNCHRONIZATION", "AFFECTED-BY-CONTROL-DTC-SETTING", "READ-DTC-INFORMATION-CLASS-REF", "INTERFACE-BLUEPRINT-REF", "PROVIDED-PORT-PROTOTYPE-IREF", "XFILE", "END-2-END-METHOD-PROTECTION-PROPS", "HW-ELEMENT", "RELIABILITY-KIND", "PATH-MTU-ENABLED", "SHORT-TERM-ADJUSTMENT-SUPPORTED", "VARIABLES", "SEGMENT-POSITION", "BSW-INTERNAL-TRIGGERING-POINT", "HW-PIN-CONNECTOR", "TCP-KEEP-ALIVE-PROBES-MAX", "SW-FEATURE-DESC", "OWNER-REFS", "DRIVER-PRIMITIVE-REF", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "REQUIRED-PORT-PROTOTYPE-IREF", "MAX-WITHOUT-CLOCK-CORRECTION-FATAL", "EVENT-GROUP-ID", "IMPLEMENTATION-DATA-TYPE", "CLIENT-SERVER-INTERFACE", "IS-FUNC-CLUSTER-ENDPOINT", "FIRST-OPERATION-REF", "TYPE-MAPPING", "IS-EVENT", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "FDC-THRESHOLD-STORAGE-VALUE", "ATTRIBUTE-DEFS", "WAKEUP-TX-IDLE", "BLUEPRINT-POLICY-SINGLE", "SERVICE-ID", "DLT-APPLICATION-TO-PROCESS-MAPPING", "SAFETY-CONDITIONS", "SETTER-OPERATION-REF", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "COUPLING-PORT-FIFO", "TLV-DATA-ID-DEFINITION-REFS", "CONNECT", "MINIMUM-TX-CONTAINER-QUEUE-SIZE", "TIME-TO-LIVE", "REQUESTER-SOFTWARE-CLUSTER-REF", "TARGET-PHM-CHECKPOINT-REF", "SOVD-GROUP-REF", "IIDC-TAG", "USER-DEFINED-FIELD-DEPLOYMENT", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "SW-VARIABLE-IMPL", "TIMING-REQUIREMENTS", "HOST-SYSTEM-REF", "DATA-LINK-LAYER-RULE", "DATA-ELEMENT-IREF", "COMPONENT-IREF", "OBJECT-REF", "CODE-DESCRIPTORS", "XREF", "THRESHOLD", "VALUE", "SUPERVISION-ENTITY", "TP-CONFIGURATION", "REQUEST-UPLOAD-CLASS-REF", "DRIVER-NOTIFICATIONS", "COUPLING-PORT", "USE-SMART-SENSOR-API", "COMMON-PROPERTIES", "MAX-MINOR-VERSION", "MIN-RPT-EVENT-ID", "CONSTANT-SPECIFICATION-MAPPING", "CLEAR-CONDITION-REFS", "APPLICATION-ASSOC-MAP-ELEMENT-VALUE-SPECIFICATION", "USER-DEFINED-CLUSTER-VARIANTS", "MAPPED-BSW-SERVICE-DEPENDENCY-REF", "STREAM-FILTER-PORT-RANGE", "SW-GENERIC-AXIS-PARAM-TYPES", "TTCAN-COMMUNICATION-CONNECTOR", "DOCUMENT-ELEMENT-SCOPES", "FUNCTION-CLUSTER-AFFILIATION", "SW-CODE-SYNTAX", "STATE-REQUEST-PORT-IREF", "RETURN-VALUE-PROVISION", "ACTION-ITEM-REF", "PSK-IDENTITY-TO-KEY-SLOT-MAPPINGS", "CONTEXT-LIMITATION-REFS", "APMC-IP-V6-ADDRESS-PARAM-DEF", "PRM-UNIT", "SESSION-ID-SUPPORT", "PARAMETER-DATA-PROTOTYPE", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS", "SM-INTERACTS-WITH-NM-MAPPING", "DIAGNOSTIC-SOVD-CONTENT", "RUNNABLE-REF", "STORE-EMERGENCY", "COM-BASED-SIGNAL-GROUP-TRANSFORMATIONS", "LOG-TRACE-LOG-MODES", "IDENTITIY-NUMBER", "COLLECTION-REF", "PROVIDER-SOFTWARE-CLUSTER-REF", "APMC-DEFINITION-COLLECTION", "DETERMINISTIC-CLIENT", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "DATA-READ-ACCESSS", "STORAGE-STATE-EVALUATION", "MAIN-FUNCTION-PERIOD", "MODE-CONDITION-REF", "IMPLIED-OPERATION-REFS", "OBJECT-REFS", "END-TO-END-PROTECTION-SET", "GENERAL-INACTIVITY-TIME", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "DTC-STATUS-AVAILABILITY-MASK", "USAGE", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "SECOND-ELEMENT-REFS", "IP-IAM-AUTHENTIC-CONNECTION-PROPS", "ALLOWED-TCP-OPTIONS-REF", "CURRENT-VALUE-REF", "UDS-SERVICE", "FM-FEATURE-DECOMPOSITION", "SECOND-ELEMENT-REF", "WAKE-UP-CHANNEL", "COM-OFFER-SERVICE-GRANT", "CANCELLATION", "MKA-PARTICIPANTS", "HARDWARE-CONFIGURATION", "RUNNABLE-ENTITY-GROUP", "ALLOW-PROVIDER-RATE-CORRECTION", "USER-DEFINED-COMMUNICATION-CONTROLLER-VARIANTS", "APPLICATION-MODE-MACHINE", "FINAL-CHECKPOINT-REFS", "SDG-CONSTRAINT-REF", "ABS", "FILE-PROXYS", "TP-CONNECTIONS", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "BSW-MODULE-DEPENDENCY", "SUBSCRIBE-EVENTGROUP-RETRY-MAX", "IMPOSITION-TIME-DEFINITION-GROUP", "DATA-MAPPINGS", "PDU-MAX-LENGTH", "SW-AXIS-GROUPED", "SOCKET-OPTION", "IKE-REKEY-TIME", "STD-REF", "DIAGNOSTIC-INDICATOR-INTERFACE", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "LIN-ORDERED-CONFIGURABLE-FRAMES", "STATUS-IREF", "DLT-USER-NEEDS", "DIAGNOSTIC-MULTIPLE-MONITOR-PORT-MAPPING", "C-CODE", "TCP-IP-DHCP-V-6-INF-DELAY-MAX", "NETWORK-ID", "SEGMENT-REFS", "COUPLING-PORT-ASYNCHRONOUS-TRAFFIC-SHAPER", "CONSTANT-MEMORYS", "DIAGNOSTIC-SOVD-CONFIGURATION-DATA-IDENTIFIER-MAPPING", "WAKEUP-RX-WINDOW", "DIAGNOSTIC-DATA-IDENTIFIER", "CONTAINED-PROCESS-REF", "GLOBAL-TIME-DOMAIN", "REPLICA-PDUS-REFS", "KEY-SLOT-REF", "ACCESS-PERMISSION-VALIDITY", "SW-ADDRMETHOD-REF", "MAXIMUM-DURATION-OF-CAMPAIGN", "GENERATED-ARTIFACTS", "SYNCHRONIZED-MASTER-TIME-BASE", "RPT-ENABLER-IMPL-TYPE", "SW-CLASS-IMPL", "MACHINE-TIMING", "PHYSICAL-DIMENSION-MAPPINGS", "I-SIGNAL-I-PDU-GROUP", "SW-FEATURE-DEFS", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-DEF", "MAX-NUMBER-OF-FILES", "STATE-MANAGEMENT-LEAVE-SUSPEND-TO-RAM-ACTION-ITEM", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "SYSTEM-TIMING", "IT-REF", "GLOBAL-TIME-PDU-REF", "PORT-DEFINED-ARGUMENT-BLUEPRINTS", "INITIAL-MODE-REF", "BINARY-MANIFEST-ITEM-DEFINITION", "INSTANCE-IDS", "INTERRUPT-SOURCE", "DATA-SEND-POINTS", "SENDER-ANNOTATION", "DIAGNOSTIC-TROUBLE-CODE-REF-CONDITIONAL", "COMPONENT-CLUSTERING", "DIAGNOSTIC-SOVD-DATA", "DIAGNOSTIC-CONNECTIONS", "E-2-E-TIMEOUT-ERROR-REF", "DDS-TOPIC-REF", "MAJOR-PROTOCOL-VERSION", "SUPPORTED-TIMER-GRANULARITY", "AUTO-START", "SECURE-COMMUNICATION-PROPS", "NUMBER-OF-CONNECTED-CLIENTS", "DIAGNOSTIC-SOVD-ARRAY-CONTENT-ELEMENT", "TIMING-PROPS", "PREPARE-ROLLBACK", "IPV-4-ADDRESS-SOURCE", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "IDSM-INSTANCE-REF", "AUTOSAR-VARIABLE", "PLATFORM-ACTION-ITEM", "BUILD-ACTION-MANIFEST-REF", "THRESHOLD-NUMBER", "RESUME-POSITION", "I-SIGNAL-I-PDU", "SEC-OC-JOB-SEMANTIC", "COUNTER-JUMP-UP", "EVENT-KIND", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "WAKEUP-TX-ACTIVE", "MAX-NUMBER-OF-OCCURRENCES", "MAX-NUMBER-OF-PARALLEL-TRANSFERS", "I-SIGNAL-GROUP", "SIGNAL-GROUP-REF", "FAILURE-MONITORING-REF", "REQUIRED-COM-SPECS", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "VARIATION-POINT-PROXYS", "RECEPTION-PROPS", "NM-CLUSTERS", "BSW-ENTRY-RELATIONSHIP-SET", "MAX-AR", "REMOTE-MULTICAST-CONFIG-REF", "CAN-PHYSICAL-CHANNEL", "IPV-4-DHCP-SERVER-CONFIGURATION", "PREDECESSOR-FIFO-REF", "PRM-CHAR", "VARIABLE-ACCESS", "IDSM-RATE-LIMITATION-REF-CONDITIONAL", "COMPONENT-REF", "ENGINEERING-OBJECT", "SCHEDULE-TABLES", "NM-PN-HANDLE-MULTIPLE-NETWORK-REQUESTS", "COMMUNICATION-CONNECTORS", "SW-SYSTEMCONST", "NOTIFICATION-PORT-IREF", "SENDER-RECEIVER-INTERFACE", "LOCAL-SUPERVISION-REFS", "ENABLE-TAKE-ADDRESS", "IDLE-SLOPE", "REMOTING-TECHNOLOGY", "MULTIPLEXED-I-PDU", "FREEZE-CURRENT-STATE", "IDS-COMMON-ELEMENT-REF", "DIAGNOSTIC-TROUBLE-CODE-OBD-PROPS", "STATE-MANAGEMENT-ACTION-LIST", "TABLE", "NM-STATE-CHANGE-IND-ENABLED", "REMOTE-ADDRESSS", "ENABLE-READ-ACCESS-CONTROL", "MIN-PWM-L", "PNC-FILTER-ARRAY-MASKS", "DIAGNOSTIC-EVENT", "SERVICE-DISCOVER-CONFIGS", "AP-APPLICATION-ERROR-REFS", "RESTRICTIONS", "NM-DATA-DISABLED", "IUMPR-GROUP", "DEPENDENT-SOFTWARE-CLUSTER-REF", "FUNCTION-GROUP-ERROR-MAPPING", "ALGORITHM-DESCRIPTIONS", "RESET-TO-DEFAULT-SUPPORTED", "DO-IP-LOGICAL-ADDRESS", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REF", "IDSM-RATE-LIMITATION", "POLICY", "DIAGNOSTIC-WWH-OBD-DTC-CLASS-ENUM-VALUE-VARIATION-POINT", "PORT-REF", "CYCLE-AUTOSTART", "CP-SOFTWARE-CLUSTER-REF-CONDITIONAL", "TRANSPORT-LAYER-RULE", "FUNCTIONAL-CLUSTER-DESIGNS", "SOFTWARE-COMPOSITION-TREF", "MAX-REQUEST-BYTES", "SENDER-RECEIVER-TO-SIGNAL-GROUP-MAPPING", "ETH-TP-CONFIG", "GLOBAL-TIME-CAN-SLAVE", "P-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "DLT-CONFIG", "PORT-DEFINED-ARGUMENT-VALUE", "J-1939-TP-PG", "DYNAMIC-RUNTIME-LENGTH-HANDLING", "NEEDS-ORIGINAL-DATA", "PROP-SEG", "SLAVES", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "CRYPTO-ALGORITHM-DESCRIPTION", "DLT-CONTEXT", "APPLICATION-ACTION-ITEM", "NETWORK-ENDPOINT-REF", "BSW-IMPLEMENTATION-REF", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "FIELD-DEPLOYMENTS", "LISTEN-NOISE", "MAC-SEC-PROPSS", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPING", "SUPPLIER-VALIDATION-ORDER-REF", "MC-FUNCTION-DATA-REF-SET-VARIANTS", "ECUC-ADD-INFO-PARAM-VALUE", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIGS", "AP-APPLICATION-ERROR", "SOCKET-CONNECTION-REF", "MSR-QUERY-RESULT-P-2", "SIGNATURE-SCHEME-REF", "PROCESSOR-SPEED", "CONSUMED-EVENT-GROUP", "COM-TRIGGER-GRANT", "BSW-SERVICE-DEPENDENCY-REF", "MODIFICATION", "NOTATION", "METHOD-DEPLOYMENTS", "FILTER-ACTION-VLAN-MODIFICATION", "CONTROLLED-FID-REF", "I-PV-6-EXT-HEADER-FILTER-LIST", "DIAGNOSTIC-OPERATION-CYCLE-REF", "S-3-SERVER-TIMEOUT", "N-ROM-BLOCKS", "INSTANCES", "DATA-CONSTR-RULE", "DATA-EXCHANGE-POINT", "REPORTING-PORT-PROTOTYPE-IREF", "SIGNATURE-SUPPORT-AP", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "INTERPOLATION-ROUTINE-MAPPING", "DIAGNOSTIC-J-1939-SW-MAPPING", "NM-PDU", "LIVENESS-KIND", "J-1939-NM-NODE-REF", "DIRECT-PDU-REF", "APMC-URI-PARAM-VALUE", "CRYPTO-KEY-SLOT-CLIENT-INTERFACE", "CLIENT-SERVER-TO-SIGNAL-MAPPING", "SUB-NODE-CHANNELS", "BINARY-MANIFEST-ITEM", "SWC-TO-SWC-OPERATION-ARGUMENTS", "USE-AUTO-VALIDATION-AT-SHUT-DOWN", "TRANSFORMATION-PROPS-SET", "POST-BUILD-VARIANT-CRITERION-VALUES", "CRYPTO-ALGO-CONFIG", "RESOURCE-ESTIMATIONS", "SOFTWARE-ACTIVATION-DEPENDENCY-FORMULA", "SUPPORTED-CIPHER-SUITES", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "DIAGNOSTIC-SOVD-PRIMITIVE-CONTENT-ELEMENT", "P-TRIGGER-IN-ATOMIC-SWC-TYPE-INSTANCE-REF", "TD-HEADER-ID-FILTERS", "DDS-SERVICE-INSTANCE-FIELD-CP", "DATA-PROTOTYPE-IREF", "SECURITY-ACCESS-CLASS-REF", "PRIMITIVE", "DATA-LENGTH-REQUEST", "ID", "DERIVED-FROM-BLUEPRINT-REF", "TLV-DATA-ID-0-REFS", "CONSTRAINT-REF", "MAXIMUM-ALLOWED-SIZE", "TIME-CS", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "TRANSFORMING-SYSTEM-SIGNAL-REF", "CHECK-STATIC-BLOCK-ID", "SEPARATED-COMPONENT-IREFS", "CONSIDER-PTO-STATUS", "REQUIRED-DESIGN-ELEMENT-REF", "SYMBOL", "GLOBAL-IN-PACKAGE-REFS", "DDS-TRANSFORMATION-I-SIGNAL-PROPS", "BASE-TYPE-ENCODING", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "IKE-RAND-TIME", "OCCURRENCE-COUNTER-PROCESSING", "REPORTING-BEHAVIOR", "CHANNEL-NAME", "STOP-IREF", "IPV-6-ADDRESS-SOURCE", "MAC-SEC-KAY-PARTICIPANT", "SERVICE-INTERFACE-EVENT-MAPPING", "NAME-TOKEN-VALUE-VARIATION-POINT", "SOURCE-EEC-REFS", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "ETHERNET-CLUSTER-VARIANTS", "BLUEPRINT-MAPPING-GUIDE", "IS-DELTA-PACKAGE", "L-1", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET-REF", "ACL-OPERATION-REF", "ALLOW-PLAIN-TEXT-IMPORT", "CONTEXT-OPERATION-REF", "VERIFY-IREF", "VENDOR-API-INFIX", "META-DATA-ITEM-SETS", "CRYPTO-KEY-SLOT-DESIGN-REF", "TCP-KEEP-ALIVE-ENABLED", "CONTEXT-MODE-DECLARATION-GROUP-REF", "ACL-OBJECT-REF", "FM-SYSCOND", "UNUSED-BIT-PATTERN", "SW-RECORD-LAYOUT-GROUP-AXIS", "SW-ADDR-METHOD-REF", "ROLLOUT-QUALIFICATIONS", "BSW-MODULE-DOCUMENTATIONS", "NM-NODE-ID-ENABLED", "PARAMETER-IMPLEMENTATION-DATA-TYPE-ELEMENT", "AR-TYPED-PER-INSTANCE-MEMORYS", "MODE-SWITCH-SENDER-COM-SPEC", "LONG-NAME", "AGGREGATION", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "CONTEXT-R-PORT-PROTOTYPE-REF", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF", "TRANSPORT-ERROR-COUNT-ENABLED", "NM-CAR-WAKE-UP-RX-ENABLED", "SOURCE-ADDRESS-RANGE-END", "READ-DATA-CLASS-REF", "DIAGNOSTIC-DATA-PORT-MAPPING", "DLT-MESSAGE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "SECTION-NAME-PREFIXS", "DDS-SECURE-COM-PROPS", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-PORT-MAPPING", "SOFTWARE-CLUSTER-SOVD-ADDRESS", "ELEMENT-REFS", "REQUIRED-FIBEX-ELEMENT-REFS", "FEATURE-MODEL-REF", "CHECKPOINTSS", "TIME-VALUE-VALUE-VARIATION-POINT", "DATA-ELEMENT-REF", "TRAFFIC-SHAPER-GROUPS", "IKE-REAUTH-TIME", "SIZE-OF-STRUCT-LENGTH-FIELD", "LIN-PHYSICAL-CHANNEL", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "TOPIC-1", "SETTER-REF", "INSTANCE-ID", "VALUE-RESTRICTION", "ARGUMENTSS", "CAN-XL-CONFIG", "PARTICIPATING-J-1939-CLUSTER-REFS", "ADDRESS", "SIZE-OF-ARRAY-LENGTH-FIELD", "ARBITRARY-EVENT-TRIGGERING", "LONG-NAME-1", "SYSTEM-SIGNAL-GROUP-TO-COMMUNICATION-RESOURCE-MAPPING", "CALIBRATION-PARAMETER-VALUE", "FIREWALL-STATE-IREFS", "COUNTER-JUMP-DOWN-VALUE", "MAX-VALUE", "BSW-DIRECT-CALL-POINT", "NUMBER-OF-ELEMENTS", "INIT-EVENT", "CONFIGURATION-CLASS-AFFECTION", "MAXIMUM-SEGMENT-LENGTH-RESPONSE", "LIN-SPORADIC-FRAME", "MIN-ALIVE-CYCLE", "BSW-RESOURCE-ESTIMATION", "ROUTINE-INFO", "DIAGNOSTIC-DATA-CHANGE-TRIGGER", "RESOURCE-GROUPS", "HTTP-ACCEPT-ENCODING", "RPT-EXECUTABLE-ENTITY-EVENT", "HAS-DYNAMIC-LENGTH", "SERVICE-INTERFACE-TRIGGER-MAPPING", "CAN-CONTROLLER-CONFIGURATION", "DESTINATION-IP-ADDRESS", "CLIENT-SERVER-RECORD-ELEMENT-MAPPING", "TCP-IP-NDP-MIN-RANDOM-FACTOR", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "DIAGNOSTIC-SOVD-OPERATION-PORT-MAPPING", "TYPE-REFERENCE-REF", "TCP-PORT", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-MAPPING", "CRYPTO-KEY-SLOT-TO-CLIENT-PORT-PROTOTYPE-MAPPING", "ROLE-BASED-DATA-ASSIGNMENT", "APPLIES-TO-DEPENDENCIES", "EVALUATED-VARIANT-REFS", "SWC-SERVICE-DEPENDENCY-IREF", "BEST-CASE-EXECUTION-TIME", "CRYPTO-KEY-SLOT-USAGE-DESIGN", "ENTITYS", "LIFE-CYCLE-STATE", "SWC-EXCLUSIVE-AREA-POLICY", "OFFSET-CORRECTION-OUT", "FLEXRAY-TP-CONNECTION", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINTS", "ARRAY-SIZE-SEMANTICS", "SDG-CAPTION-REF", "DEVELOPMENT-VALUE", "SIGNAL-BASED-EVENT-DEPLOYMENT", "SW-RECORD-LAYOUT-GROUP", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "TLV-DATA-ID-0-REF", "STATE-DEP-FIREWALL-REFS", "KEY", "NETWORK-TIME-PROVIDER-REF", "RECORD-ELEMENTS", "ACCEPT-VARIABLE-SA", "INTEGER-VALUE-VARIATION-POINT", "COM-PROCESSING-PERIOD", "TO-REF", "DATA-PROTOTYPE-TRANSFORMATION-PROPS", "DDS-FIELD-REQUEST-TOPIC-REF", "FIELD-SENDER-COM-SPEC", "EXECUTION-CONTEXTS", "TAILORING-REF", "BSW-CALLED-ENTITY", "WINDOW-SIZE", "HEALTH-CHANNEL-IREF", "DIAGNOSTIC-EDR-DATA-PROVIDER-MAPPING", "RP-IMPL-POLICY", "TIMEOUT-AR", "ASSIGNED-TRAFFIC-CLASS", "CAN-XL-PROPS-REF", "MAX-CONVERSATIONS", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREFS", "PER-INSTANCE-PARAMETERS", "COUPLING-PORT-SCHEDULER", "GLOBAL-TIME-FR-SLAVE", "DEFAULT-LEASE-TIME", "INTERNAL-CONSTRS", "SPEEDUP", "DIAGNOSTIC-PARAMETER-REF", "CLIENT-SERVER-OPERATION-BLUEPRINT-MAPPING", "GROUP-NUMBER", "INGRESS-PORT-REFS", "SERVICE-NEEDS", "CALIBRATION-PARAMETER-VALUE-SET-REFS", "MAXIMUM-LENGTH", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "PDU-TRIGGERING-REF-CONDITIONAL", "PID", "TLS-JOB-MAPPING", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "SCOPE-REF", "USER-DEFINED-TRANSFORMATION-PROPS", "MODE-IREFS", "DDS-CP-DOMAIN", "CONNECTION-BUNDLES", "REQUEST-DATA-SIZE", "AUTHENTICATION-TIMEOUT", "LOCAL-NETWORK-ENDPOINT-REF", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "SAFETY-CONDITION", "RELIABILITY-MAX-BLOCKING-TIME", "SELECTOR-FIELD-LENGTH", "CONSTANT-SPECIFICATION", "INDICATOR-FAILURE-CYCLE-COUNTER-THRESHOLD", "DESTINATION-REFS", "TIME-LEAP-FUTURE-THRESHOLD", "INFO-TYPE-REF", "CONFIGURED-NAD", "INCLUDE-REF", "DNS-SERVER-ADDRESS", "COM-CONTROL-CLASS-REF", "REENTRANCY-LEVEL", "IS-GLOBAL", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "CONTAINED-I-PDU-TRIGGERING-PROPSS", "CONNECTION-PRIO-PDUS", "REPLACE-WITH", "INTERRUPT-CATEGORY", "USER-REF", "PROVIDED-ENTRYS", "MAX-HEADER-ID", "EXPLICIT-WAKEUP-CHANNELS", "PORT-PROTOTYPE-BLUEPRINT-MAPPING", "RUNNABLE-ENTITY-IREF", "DO-IP-LOGIC-ADDRESS-PROPS", "SWC-TRIGGER-IREF", "TCP-IP-NDP-DELAY-FIRST-PROBE-TIME", "PHYSICAL-DIMENSION-MAPPING-REF", "PERSISTENCY-PORT-PROTOTYPE-IREF", "DIAGNOSTIC-SERVICE-INSTANCES", "SENDER-REC-RECORD-ELEMENT-MAPPING", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "MACHINE", "R-PORT-IREF", "PORTS", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REF", "DIAGNOSTIC-SOVD-CONTENT-PORT-MAPPING", "BSW-PER-INSTANCE-MEMORY-POLICY", "TIME-LEAP-PAST-THRESHOLD", "MASTER-EVENT-REF", "PRMS", "MESSAGE-ID", "NM-MAIN-FUNCTION-ACROSS-FR-CYCLE", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "RUN-MODE", "CONNECTED-INDICATORS", "CLIENT-PORT-FROM-CONNECTION-REQUEST", "BSW-MGR-NEEDS", "MIN-HEADER-ID", "CAN-FRAME-TX-BEHAVIOR", "STREAM-ID", "IS-REQUIRED", "SLEEP-REPETITION-DELAY-OF-SLEEP-REQUEST", "INITIAL-CHECKPOINT-REF", "STATIC-REMOTE-MULTICAST-ADDRESSES", "OPERATION-REF", "ELLIPTIC-CURVE-REF", "PHYSICAL-CHANNELS", "APMC-IP-V6-ADDRESS-PARAM-VALUE", "MAX-SDU-SIZE", "MEMORY-SECTION", "MINISLOT-ACTION-POINT-OFFSET", "CALLBACK-HEADER-REFS", "TOLERANCE", "LIN-SCHEDULE-TABLE", "MAX-NUMBER-OF-REQUEST-CORRECTLY-RECEIVED-RESPONSE-PENDING", "UNIT-DISPLAY-NAME", "DIAGNOSTIC-REQUEST-UPLOAD", "PDU-COUNTER-START-POSITION", "MEMORY-HIGH-ADDRESS", "ALLOW-CONTENT-TYPE-CHANGE", "TIME-SYNC-CORRECTION", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "CORES", "ETHER-TYPE", "SEC-OC-SECURE-COM-PROPS-REF-CONDITIONAL", "CIPHER-SUITE", "BYTE-VALUES", "IDS-TIME-BASE-REF", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "RVF-COLOR-SPACE", "IEEE-1722-TP-ACF-CONNECTION", "DO-IP-ROUTING-ACTIVATIONS", "HAS-SETTER", "RX-PDU-TRIGGERING-REF", "TARGET-FIELD-REF", "DIAGNOSTIC-TEST-RESULT-UPDATE-ENUM-VALUE-VARIATION-POINT", "INHIBITION-SOURCE-REF", "ACTIVATION-ACTION", "TIME-SYNC-TECHNOLOGY", "REST-ARRAY-PROPERTY-DEF", "EID", "SOMEIP-METHOD-DEPLOYMENT", "SYNCHRONIZED-TIME-BASE-CONSUMER", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF", "COMPOSITION-DATA-PROTOTYPE-REF", "UCM-REF", "BUS-IDLE-TIMEOUT-PERIOD", "DEPENDENCY", "R-TRIGGER-IN-ATOMIC-SWC-INSTANCE-REF", "DIAGNOSTIC-DTC-CHANGE-TRIGGER", "USER-DEFINED-METHOD-DEPLOYMENT", "SET-FUNCTION-GROUP-STATE-IREF", "SERVER-COM-SPEC-PROPS", "BSW-MODULE-DEPENDENCYS", "DIAGNOSTIC-TROUBLE-CODE-PROPS-REF", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "BSW-MODULE-ENTRY-REF", "CAN-TP-ADDRESS-REFS", "TRANSMIT-CANCELLATION", "EXCLUSIVE-AREAS", "UCM-MODULE-INSTANTIATION", "CAPABILITY-RECORDS", "IEE-E-1722-TP-CONNECTION-REF", "DDS-FIELD-DEPLOYMENT", "FM-FEATURE-RELATION", "J-1939-DCM-DM-19-SUPPORT", "DATA-DUMP-ENTRY", "POSSIBLE-ERROR-REFS", "DTC-FORMAT-TYPE", "DIAGNOSTIC-MULTIPLE-EVENT-INTERFACE", "TARGET-DATA-PROTOYPE-REF", "LC-STATE-REF", "PARAMETER-ACCESSS", "IMPLEMENTATION-REFS", "KEEP-ALIVES", "APMC-URI-FOREIGN-REFERENCE-VALUE", "SWC-RUNNABLE-REF", "FIELD-NOTIFIER-QOS-PROPSS", "LOC-MEASUREMENT-SET", "DIAGNOSTIC-DO-IP-POWER-MODE-PORT-MAPPING", "NUMBER", "SEED-SIZE", "NM-COORD-SYNC-SUPPORT", "REMOTE-CLIENT-QUEUE-LENGTH-CONTRIBUTION", "SW-FEATURE-DESCS", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "EXPLICIT-INTER-RUNNABLE-VARIABLES", "MAX-NUMBER-OF-ALLOWED-UPDATES", "DIAGNOSTIC-SOVD-DATA-CATEGORY", "SW-SYSTEMCONST-VALUE", "TARGET-EEC-REFS", "DATA-ELEMENTS", "DO-IP-TP-CONFIG", "COUNTER-JUMP-DOWN", "USED-FID-REF", "APPLICATION-RECORD-ELEMENT", "NV-DATA-PORT-ANNOTATIONS", "SECURITY-EVENT-PROPS-REF", "EXCLUSIVE-AREA-POLICYS", "ELEMENT-PROPSS", "TIMING-VARIABLE-REF", "DIAGNOSTIC-TRANSMIT-CERTIFICATE-INTERFACE", "UCM-NOT-AVAILABLE-ON-THE-NETWORK", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "TCP-TP-CONFIG", "MODE-INSTANCE", "ACTION-ITEM-LISTS", "DIAGNOSTIC-ENV-DATA-ELEMENT-CONDITION", "DENOMINATOR-CONDITION", "EXECUTABLE", "IDSM-TRAFFIC-LIMITATION", "MIN-TRCV-DELAY-COMPENSATION-OFFSET", "TEXT-TABLE-VALUE-PAIR", "CRYPTO-PROVIDER-DOCUMENTATION-REF", "CRYPTO-CERTIFICATE-GROUP", "HAS-NOTIFIER", "PDU-TO-FRAME-MAPPING", "UDP-COLLECTION-TRIGGER", "SOMEIP-REQUIRED-EVENT-GROUP", "HEADER-ID-SHORT-HEADER", "APMC-INSTANCE-REFERENCE-VALUE", "DIAGNOSTIC-CONTRIBUTION-SET", "SUPERVISION-CHECKPOINT", "FUNCTION-PROTOTYPE-EMITTER", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "APPLIED-STANDARDS", "J-1939-DCM-I-PDU", "PDELAY-REQUEST-PERIOD", "R-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "ALGORITHM-MODE", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "TRANSFERS", "SET", "DIAGNOSTIC-COMMON-PROPS-VARIANTS", "CUSTOM-DOCUMENT-ELEMENT-REF", "ECUC-MODULE-DEF", "RETURN-CODE", "SERVICE-INSTANCE-TO-SIGNAL-MAPPINGS", "TIMING-CONDITION-FORMULA", "TCP-ALIVE-CHECK-RESPONSE-TIMEOUT", "ROLE-BASED-PORT-ASSIGNMENT", "ASSOCIATED-COM-I-PDU-GROUP-REF", "IO-CONTROL-CLASS-REF", "TARGET-TRIGGER-REF", "SYMBOL-WINDOW", "COM-MGR-USER-NEEDS", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "ACF-PARTS", "DIAGNOSTIC-FREEZE-FRAME", "TDLET-ZONE-CLOCK", "K-RUN-RESOURCE", "DEFAULT-TRAFFIC-CLASS", "MEMORY-SECTION-LOCATIONS", "L-GRAPHIC", "NUMERICAL-RULE-BASED-VALUE-SPECIFICATION", "SEGMENT-RECEPTION-TIMEOUT-TIME-RESPONSE", "PAYLOAD-REF", "SW-FILL-CHARACTER", "TARGET-FRAME-REF", "CHOICES", "ACL-OBJECT-CLASS", "DIAGNOSTIC-EXTENDED-DATA-RECORD-ELEMENT", "BSW-DATA-RECEIVED-EVENT", "MESSAGE-SOURCE-FILE", "TARGET-PDU-TRIGGERINGS", "DO-IP-ACTIVATION-LINE-NEEDS", "SUSPEND-TO-RAM-AWARENESS", "DIAGNOSTIC-INDICATOR-TYPE-ENUM-VALUE-VARIATION-POINT", "LIN-SLAVE", "DIAGNOSTIC-FREEZE-FRAME-REF", "RELATIVE-TOLERANCE", "PROVIDED-SERVICE-INSTANCE-ID", "MODE-DECLARATION", "DATA-UPDATE-PERIOD", "DHCP-SERVER-CONFIGURATION", "CLASS-CONTENTS", "N-PDU-REF", "FLEXRAY-CLUSTER", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "KEY-SLOT-USED-FOR-SYNC", "TCP-IP-NDP-DYNAMIC-MTU-ENABLED", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "WAKEUP-REMOTE-ENABLED", "SIGNAL-MAPPINGS", "ACCEPT-ENCODING", "FIRE-AND-FORGET-METHOD-MAPPING", "INTERNAL-BEHAVIOR", "ECU-EXTRACT-VERSION", "FIRST-ELEMENTS", "FILE-NAME", "DATA-PROTOTYPE-MAPPING", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET-REF", "DATA-TRANSFORMATION-KIND", "MODE-CONDITION", "POST-BUILD-VARIANT-SUPPORT", "CONSTRAINT-TAILORINGS", "SW-VARIABLE-IMPLS", "SUCCESSOR-REFS", "IS-UNUSED", "DIAGNOSTIC-IO-CONTROL-NEEDS", "PERSISTENCY-DATA-PROVIDED-COM-SPEC", "JUMP-TO-BOOT-LOADER", "RVF-INTERLACED", "V-2-X-FAC-USER-NEEDS", "STARTUP-CONFIG", "DIAG-ARG-INTEGRITY", "DIAGNOSTIC-UDS-SEVERITY-ENUM-VALUE-VARIATION-POINT", "COMPOSITE-INTERFACE", "AGING-REF", "SW-RECORD-LAYOUT-V-AXIS", "SERVICE-INTERFACE-PEDIGREE", "SUB-ELEMENTS", "PDU-COLLECTION-TIMEOUT", "INITIAL-NAD", "ASSIGNED-DATA-TYPES", "AUTOSAR-ENGINEERING-OBJECT", "DIAGNOSTIC-IUMPR-GROUP-IDENTIFIER", "SENDER-RECEIVER-ANNOTATIONS", "BLUEPRINT-POLICYS", "NOTIFIER-REF", "DIAGNOSTIC-SESSION-REFS", "IGNORE-ORDER-ALLOWED", "TCP-IP-NDP-DYNAMIC-HOP-LIMIT-ENABLED", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "TCP-GENERAL-INACTIVITY-TIME", "BSW-MODE-SENDER-POLICY", "PERIOD-BEGIN", "UDP-MIN-TX-BUFFER-SIZE", "ECUC-STRING-PARAM-DEF-CONDITIONAL", "APMC-STRING-PARAM-DEF", "KEY-GENERATION", "ELEMENT-IN-IMPL-DATATYPE", "NM-COORDINATOR", "LC-STATES", "AUTH-ALGORITHM", "DYNAMIC-SLOT-IDLE-PHASE", "ASSOCIATED-RTE-PLUGIN-REF", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "TD-EVENT-SLLET-PORT", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-PORT-MAPPING", "I-PDU-MAPPING", "GLOBAL-TIME-ETH-MASTER", "SUB-ELEMENT-MAPPINGS", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "OBD-RATIO-SERVICE-NEEDS", "TIME-MASTER-PRIORITY", "RETRY", "CONSISTENCY-NEEDSS", "TLS-CONNECTION-REF", "UCM-STEP", "SA-OVER-TIME", "INCLUDED-VARIANT-REFS", "CALIBRATION-PARAMETER-VALUES", "COLLECTION-THRESHOLD", "TRANSLATED-VLAN-ID", "SECURE-TRANSLATION", "APPLICATION-MODE-IREF", "TD-EVENT-SERVICE-INSTANCE-EVENT-TYPE", "CONTENT-OBJECT", "COUPLING-PORT-CONNECTIONS", "SOFTWARE-CLUSTER-DESIGN-DEPENDENCY", "MODE-ACCESS-POINTS", "SERVICE-INTERFACE-REF", "DIAGNOSTIC-ENABLE-CONDITION", "POSSIBLE-AP-ERROR-REFS", "SOFTWARE-MEMORY-SECTION-REF", "HOP-COUNT", "AUTO-REQUIRE", "EVENT-REFS", "DIAGNOSTIC-SOVD-CONTENT-INTERFACE", "INSTALLATION-BEHAVIOR", "DESTINATION-IPV-4-ADDRESS", "EXPLICIT-RPT-PROFILE-SELECTION-REF", "FIRST-VALUE", "SW-MAX-TEXT-SIZE", "TARGET-MODE-DECLARATION-REF", "RECEPTION-DEFAULT-VALUES", "SW-RECORD-LAYOUT-V-FIX-VALUE", "MANUFACTURER-CODE", "CRYPTO-KEY-SLOTS", "PAYLOADS", "DEADLINE-SUPERVISION", "PORT-GROUP", "DELAY-COMPENSATION-A", "CONDITIONS", "EXECUTABLE-REF", "RECOVERY-NOTIFICATION-RETRY", "IEEE-1722-TP-AAF-CONNECTION", "GLOBAL-KAY-PROPS-REF", "TRANSFER-PROPERTY", "OPERATION-ARGUMENT-INSTANCE-IREF", "HEADER-LENGTH", "DELIVERY-ARTIFACTS", "NM-VOTE-INFORMATION", "IS-OPTIONAL", "SDG-CAPTION", "DATA-TRANSFORMATION-REF-CONDITIONAL", "VEHICLE-IDENTIFICATION-SYNC-STATUS", "BASE-COMPOSITION-REF", "NM-MSG-REDUCED-TIME", "IMPLEMENTATION-CONFIG-VARIANT", "ECU-SCOPE-REF", "MIRROR-SOURCE-LIN-TO-CAN-RANGE-BASE-ID", "IIDC-SOURCE-PACKET-HEADER", "SUB-SOFTWARE-CLUSTER-REFS", "MIN-DATA-LENGTH", "CONTENT-REF", "MAXIMUM-MESSAGE-LENGTH", "ECUC-QUERY-EXPRESSION", "ASSOCIATED-PDUR-I-PDU-GROUP-REF", "TLS-CIPHER-SUITE", "BLUEPRINT-MAPPING-SET", "IDS-COMMON-ELEMENT-REF-CONDITIONAL", "SERVICE-FIND-TIME-TO-LIVE", "ACTION-LIST-ITEM-REFS", "SUP", "ALLOWED-KEYSLOT-USAGE", "FLEXRAY-COMMUNICATION-CONTROLLER-VARIANTS", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "AR-TYPED-PER-INSTANCE-MEMORY-REF", "PROCESS-DESIGN", "EVENT-NEEDS-REF", "MAX-FR-IF", "POST-BUILD-VARIANT-USED", "EXPECTED-ALIVE-INDICATIONS-IN-SUPERVISION-CYCLE", "SYMBOLIC-NAME-PROPS", "J-1939-CONTROLLER-APPLICATION", "AUTH-ROLE-REF", "INTERVAL-LENGTH", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "REQUEST-GROUP-REF", "HARDWARE-PLATFORM", "FUNCTION-GROUP-MODE-IREF", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF", "USES-ADDRESS-ARBITRATION", "EXECUTION-ERROR", "CLIENT-ID-RANGE", "UNIT-GROUP-REFS", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "USE-VARIABLE-SA", "DIAGNOSTIC-IO-CONTROL", "DPG-DOES-NOT-REQUIRE-COHERENCYS", "PROCESSOR", "TCP-IP-IP-REASS-TIMEOUT", "COLLECTION-TIMEOUT", "ETH-IP-PROPS-REFS", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "IS-SYNCHRONOUS", "MODE-IN-SWC-INSTANCE-REF", "CONTEXT-P-PORT-REF", "INCLUDED-CERTIFICATE-REF", "COM-TIMEOUT-ERROR-REF", "J-1939-SHARED-ADDRESS-CLUSTERS", "CONSUMED-SERVICE-INSTANCES", "IP-SEC-CONFIG-PROPS", "METHOD-RESPONSE-PROPSS", "HW-PIN-GROUP-REFS", "DIAGNOSTIC-DATA-ELEMENT-REF", "SYNC-INTERVAL", "CHAPTERS", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITY", "GLOBAL-TIME-PORT-ROLE", "RPT-EXECUTION-CONTROL", "CAPABILITY", "UCM-MASTER-FALLBACK-REF", "PERSISTENCY-FILE-PROXY", "SYSC-REF", "SELECTOR-FIELD-BYTE-ORDER", "CONSUMED-EVENTS", "DIAGNOSTIC-SERVICE-TABLE-REF", "RESPONSE-ARGUMENT-REF", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "TARGET-STATUS-REF", "ACTIVATION-REASONS", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "FUNCTIONAL-CLUSTER-TO-SECURITY-EVENT-DEFINITION-MAPPING", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "MOLAR-AMOUNT-EXP", "SECURE-COMMUNICATION-PROPS-SET", "NM-CYCLETIME-MAIN-FUNCTION", "DESTINATION-ORDER-KIND", "APMC-FUNCTIONAL-CLUSTER-DEF", "DIAGNOSTIC-CONNECTION-REF-CONDITIONAL", "COMMUNICATION-RESOURCE-PROPS", "ELEMENT", "DYNAMIC-PART", "CONTEXT-ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE-REF", "CONFIDENCE-INTERVALS", "UDP-NM-ECU", "MKA-PARTICIPANT-REFS", "PERMISSION-CERTIFICATE-AUTHORITY-REF", "APPLIES-TO", "ALLOCATOR-REF", "TX-PDU-TRIGGERING-REF", "CLIENT-SERVER-OPERATION-COM-PROPS", "AFFECTED-APPLICATION", "ETHERNET-FRAME", "NETWORK-LAYER-RULE", "DIAGNOSTIC-SOVD-OPERATION-INTERFACE", "ACL-OBJECT-SET", "PREDEFINED-VARIANT", "TIME-SYNC-SERVER-IDENTIFIER", "PERMIT-MULTIPLE-REFERENCES-TO-EE", "PROVIDED-OUTER-PORT-REF", "RECEIVER-CAPABILITY", "IDSM-QUALIFIED-EVENT-RECEIVER-INTERFACE", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "MODE-SWITCHED-ACK", "REST-INTEGER-PROPERTY-DEF", "PWM-O", "DIRECTION", "USED-LANGUAGES", "UNRESOLVED-REFERENCE-RESTRICTION", "DESTINATION-MAX-ADDRESS", "CALL-POINTS", "SIZE-OF-ARRAY-LENGTH-FIELDS", "CLIENT-SERVICE-MINOR-VERSION", "STIM-ENABLER", "PERSISTENCY-FILE-PROXY-INTERFACE", "ROOT-DATA-PROTOTYPE-IN-SR-REF", "COUPLING-TYPE", "SERVICE-INSTANCE-REFS", "ENVIRONMENTAL-CONDITION-REF", "RAW-DATA-STREAM-SERVER-INTERFACE", "PORT-BLUEPRINT", "RVF-ACTIVE-PIXELS", "SERVICE-INTERFACE-VERSION", "CRYPTO-ALG-ID", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "MAXIMUM-RESIDENCE-TIME", "INTERPOLATION-ROUTINES", "DESIGN-REFS", "FUNCTION-GROUP-IREFS", "WWH-OBD-DTC-CLASS", "ERROR-TRACER-NEEDS", "MAX-WITHOUT-CLOCK-CORRECTION-PASSIVE", "MAXIMUM-SUPPORTED-UCM-VERSION", "FIELD-TOPICS-ACCESS-RULE-REF", "CONTAINED-PACKAGE-ELEMENT-REFS", "DISABLED-IN-MODE-IREFS", "PUBLISHER", "ENABLE-JOIN-ACCESS-CONTROL", "ECUC-IMPLEMENTATION-CONFIGURATION-CLASS", "ETHERNET-PRIORITY-REGENERATION", "IP-TP-RULE", "VARIABLE-ACCESS-REF", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "INCLUDED-CERTIFICATE-REFS", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "PROPAGATION-DELAY", "POST-BUILD-VARIANT-CRITERION-VALUE", "NAGLES-ALGORITHM", "COUPLING-PORT-ROLE", "STATIC-SOCKET-CONNECTION-REF", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "CLOCK-REFERENCE-REF", "MAPPING-DIRECTION", "RANGE-MAX", "KEEP-ALIVE-PROBES-MAX", "E-2-E-PROFILE-CONFIGURATIONS", "VIN-INVALIDITY-PATTERN", "ECUC-FUNCTION-NAME-DEF-CONDITIONAL", "APPLICATION-PRIMITIVE-DATA-TYPE", "BOR-TIME-TX-ENSURED", "SIGNIFICANCE", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "COM-CONFIGURATION-ID", "TIME-BUFFER", "DATA-IREF", "INTERFACE-MAPPING", "REQUIRED-MODE-GROUPS", "RESTORE-AT-START", "WAKEUP-LOCAL-ENABLED", "SYNC-COUNTER-INIT", "PROCESS-STATE-MACHINE", "PORT-API-OPTION", "SW-CLASS-INSTANCE", "NM-IMMEDIATE-NM-TRANSMISSIONS", "LIN-CONFIGURABLE-FRAMES", "FIRE-AND-FORGET-MAPPING", "BUS-MIRROR-CAN-ID-RANGE-MAPPING", "ASSOCIATED-COM-I-PDU-GROUP-REFS", "ACCESS-MODE", "NM-VECTOR-EARLY-UPDATE", "SCOPE-EVENT-REFS", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "SW-MAINTENANCE-NOTESS", "DATA-PROVIDER", "SYSTEM", "CONTROL-OPTION-RECORD-PRESENT", "FIRST-PORT-REF", "BYTE-VALUE", "RPT-EXECUTABLE-ENTITY-EVENTS", "ROOT-SOFTWARE-COMPOSITIONS", "ALIVE-TIMEOUT", "MIN-DESTINATION-PORT-NUMBER", "SW-SYSTEMCONSTANT-VALUE-SET-REFS", "COM-EVENT-GRANT", "PACKAGER-SIGNATURE-REF", "USED-LIFE-CYCLE-STATE-DEFINITION-GROUP-REF", "STANDARD-REVISION", "KEY-SLOT-USAGE", "TYPE-OF-FREEZE-FRAME-RECORD-NUMERATION", "DATA-RECEIVED-EVENT", "MAX-DIFF", "COUPLING-PORT-REF-CONDITIONAL", "SPECIFIC-CHANNEL-REF", "MINIMUM-DISTANCES", "TTCAN-CLUSTER-VARIANTS", "CONDITIONAL-CHANGE-NAD", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "SERVICE-INTERFACE-ELEMENT-MAPPING-REFS", "FREEZE-FRAME-REF", "TCP-IP-NDP-DEFAULT-RETRANS-TIMER", "ASYNCHRONOUS-SERVER-CALL-POINT", "ETHERNET-RAW-DATA-STREAM-MAPPING", "DEFAULT-VALUE-ELEMENTS", "TIMING-ARGUMENTS", "BSW-EXCLUSIVE-AREA-POLICY", "ACL-OBJECT-CLASSS", "ECUC-MULTIPLICITY-CONFIGURATION-CLASS", "SUB-SOFTWARE-CLUSTER-REF", "SERVICE-INTERFACE-DEPLOYMENT-REF", "SCOPE-REFS", "SIZE-OF-STRUCT-LENGTH-FIELDS", "VALUE-ERROR-COUNT-ENABLED", "TX-NM-PDU-REFS", "FORMULA", "TRACE-REF", "PURPOSE-OF-UPDATE-REF", "CONTEXT-R-PORT-REF", "VLAN-TRANSLATION-TABLES", "RPT-HOOKS", "PROVIDED-REQUIRED-INTERFACE-TREF", "SOMEIP-REMOTE-UNICAST-CONFIG", "MAX-TESTER-CONNECTIONS", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "ROOT-SW-COMPONENT-PROTOTYPE", "CPP-IMPLEMENTATION-DATA-TYPE", "GETTER-RETURN-SIGNAL-REF", "MODE-DECLARATION-REFS", "IEEE-1722-TP-CONFIG", "NETWORK-INTERFACES", "DDS-I-SIGNAL-TO-TOPIC-MAPPINGS", "REMARK", "APMC-FUNCTIONAL-CLUSTER-VALUE", "PERSISTENCY-DATA-REQUIRED-COM-SPEC", "REQUIRED-SERVICE-PORT-IREF", "CHANNELS", "MC-VARIABLE-INSTANCES", "INSTANCE-ID-PROPS", "KEY-SERVER-PRIORITY", "MAC-ADDRESS-MASK", "STRING-ENCODING", "DIAGNOSTIC-EXTRACT-REF", "NESTED-ELEMENT-REF", "SLAVE-QUALIFIED-UNEXPECTED-LINK-DOWN-TIME", "SW-CLUSTERS", "LISTEN-TIMEOUT", "CONSISTENCY-NEEDS", "BSW-TRIGGER-REF", "SAK-REF", "DOMAIN-ID", "DIAGNOSTIC-CONNECTION-REF", "TCP-IP-NDP-SLAAC-OPTIMISTIC-DAD-ENABLED", "TD-EVENT-BSW-MODE-DECLARATION", "PARTITION-NAME", "DEFAULT-PERIOD-BEGIN", "TEST-ID-REF", "POSSIBLE-AP-ERROR-SET-REF", "CONTEXT-TYPE-LIST", "ADDRESSING-FORMAT", "BUILD-ACTION-ENVIRONMENTS", "ADAPTIVE-FIREWALL-TO-PORT-PROTOTYPE-MAPPING", "ECUC-TEXTUAL-PARAM-VALUE", "COMPOSITION-SW-COMPONENT-TYPE-REF-CONDITIONAL", "ALLOWED-I-PV-6-EXT-HEADERS", "OPERATION-INVOKED-EVENT", "END-TO-END-PROTECTION", "SW-COMPONENT-DOCUMENTATION", "PARAMETER-ACCESSED-FOR-DEBUG-REF", "RPT-SERVICE-POINT-PRE-REF", "NAMED-CURVE-ID", "COLSPEC", "UCM-MASTER-FALLBACK-REFS", "UPPER-BOUNDARY", "I-SIGNAL-TO-PDU-MAPPINGS", "EVENT-MULTICAST-ADDRESSS", "RESPONSE-ON-EVENT-REF", "INCLUDED-MODE-DECLARATION-GROUP-SETS", "COUPLING-FLAG", "ARBITRATIONS", "DIAGNOSTIC-SOVD-LOCK", "SPN-REFS", "VALID-BINDING-TIME", "CYCLE-COUNTER", "LOGIC-ADDRESS", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "IEEE-1722-TP-CRF-CONNECTION", "NV-PROVIDE-COM-SPEC", "SYSTEM-SIGNAL", "BSW-QUEUED-DATA-RECEPTION-POLICY", "PRE-ACTIVATE-IREFS", "MSR-QUERY-CHAPTER", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "TCP-TTL", "SUB-NODE-PHYSICAL-CHANNEL-REF", "NESTED-CONTEXT-DATAS", "DDS-EVENT-REF", "DATA-TRANSFORMATION-SET", "DLT-MESSAGE-REF-CONDITIONAL", "MAX-SAMPLE-POINT", "MAX-NUM-CHANGE-OF-DATA-IDENTFIER-EVENTS", "START-ACTION-REFS", "TOOL-VERSION", "DYNAMIC-BS", "RESOURCES", "FUNCTION-NAME", "TCP-IP-NDP-DEFAULT-REACHABLE-TIME", "ROLE-BASED-BSW-MODULE-ENTRY-ASSIGNMENT", "SERVICE-DEPENDENCYS", "APPLICATION-SW-COMPONENT-TYPE", "IMPOSITION-TIMES", "PROVIDED-FIELDS", "IDENTIFIER", "DIAGNOSTIC-AUTHENTICATION-CLASS", "TYPE-OF-DTC-SUPPORTED", "MINIMUM-SEND-INTERVAL", "BULK-NV-DATA-DESCRIPTORS", "DDS-PARTITION-REF", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPING", "HEALTH-CHANNEL-ARGUMENT-REFS", "COMMUNICATION-CONNECTOR-REF", "SLEEP-MODE-SUPPORTED", "MAX-SLOTS", "OBD-DTC-VALUE-3-BYTE", "MINIMUM-FIFO-LENGTH", "BSW-BEHAVIOR-REF", "APMC-URI-FOREIGN-REFERENCE-DEF", "SOURCE-ADDRESS-RANGE-START", "MSG-ID-MASK", "R-PORT-PROTOTYPE-IREF", "KEY-VALUE-STORAGE-REF", "SIZE-OF-UNION-LENGTH-FIELD", "EXTERNAL-TRIGGERING-POINTS", "CHECKPOINT-INITIAL-REF", "LOGICAL-SUPERVISION", "EXECUTION-ORDER-CONSTRAINT-TYPE", "LINKER", "COM-FIELD-GRANT-DESIGN", "AUTHENTICATION-PROPS-REF", "COM-CONFIGURATION-TX-TIME-BASE", "SENDER-IREF", "ECUC-REF", "ESTIMATED-EXECUTION-TIME", "CLASS-TAILORINGS", "LOG-SINK-REFS", "MASK", "INVERSE-PROTOCOL-VERSION", "END-TO-END-TRANSFORMATION-DESCRIPTION", "NM-VARIANT", "ENABLES-LOG-TRACE", "TRANSFER-REFS", "R-MODE-GROUP-IN-ATOMIC-SWC-INSTANCE-REF", "BINARY-MANIFEST-RESOURCE-DEFINITION", "NM-MESSAGE-TIMEOUT-TIME", "SOMEIP-METHOD-PROPS", "USER-DATA-SUB-TLV", "TERMINATING-CHECKPOINT-REF", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "RETURN-TYPE", "UDP-COLLECTION-BUFFER-TIMEOUT", "SYSTEM-SIGNAL-GROUP", "DDS-EVENT-QOS-PROFILE-REF", "CLIENT-SERVER-APPLICATION-ERROR-MAPPING", "FUNCTION-GROUP-MODE-IREFS", "ROUGH-ESTIMATE-HEAP-USAGE", "HW-ATTRIBUTE-LITERAL-DEF", "OBJECT-DEFINITION-REFS", "MAPPING-CONSTRAINTS", "CAN-BE-INVOKED-CONCURRENTLY", "PNC-MAPPING-IDENT-REF-CONDITIONAL", "REQUIREMENT-REFS", "SERVICE-INSTANCES", "AFFECTED-STATE-IREF", "E", "LOWER-REF", "ALLOWED-I-PV-6-EXT-HEADERS-REF", "DDS-SERVICE-QOS-PROFILE-REF", "OBD-RELEVANCE", "TARGET-ELEMENT-REF", "PROVIDED-OPERATIONS", "NM-NETWORK-HANDLE-REF", "TIMING-CLOCK-SYNC-ACCURACYS", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF", "EVENTS", "DESTINATION-BASE-ID", "STORE-ON-CHANGE", "EXECUTABLE-PROVIDED-PORT-IREF", "MEMORY-SECTIONS", "VARIABLE-INSTANCE", "WINDOW-SIZE-VALID", "EVENT-MULTICAST-SUBSCRIPTION-ADDRESSS", "REMOTE-SUBJECT-REF", "MODES", "SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "IS-EXTERNAL", "TARGET-DATA-PROTOTYPE-REF", "SERVER-COM-SPEC", "SIMULATED-EXECUTION-TIME", "SOMEIP-EVENT-GROUP", "ASSIGNED-ENTRY-ROLES", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "DDS-RPC-SERVICES", "ICV-FRESHNESS-VALUE-ID", "RELEASED-TRIGGERS", "HW-ATTRIBUTE-VALUE", "ROUTING-ACTIVATION-TYPE", "MIN-OK-STATE-INIT", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "BSW-SERVICE-DEPENDENCY", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-AUTOSAR-DATA-PROTOTYPE-REF", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "BACKGROUND-EVENT", "MAX-MAJOR-VERSION", "IDSM-MODULE-INSTANTIATION-REF", "SERVICE-IDENTIFIER", "REST-NUMBER-PROPERTY-DEF", "POSITIVE-INTEGER-VALUE-VARIATION-POINT", "HANDLE-INVALID", "PERIODIC-RATES", "SW-COMP-TO-ECU-MAPPING-REFS", "LET-INTERVAL-REF", "ETHERNET-CLUSTER-CONDITIONAL", "COM-ENABLE-MDT-FOR-CYCLIC-TRANSMISSION", "MINIMUM-START-INTERVAL", "MC-FUNCTION-DATA-REF-SET-CONDITIONAL", "HANDLE-OUT-OF-RANGE", "EXCESS-BURST-SIZE", "PRIVACY-LEVEL", "MIN-NUMBER-OF-ELEMENTS", "STORE-EVENT-SUPPORT", "POST-BUILD-CHANGEABLE", "FIELD-REF", "TCP-INITIAL-INACTIVITY-TIME", "BLOCK-INCONSISTENT", "SOVD-GATEWAY-INSTANTIATION", "FIXED-LENGTH", "PORT-ADDRESS", "CHANNEL-MODE", "END-2-END-EVENT-PROTECTION-PROPS", "CAN-XL-BAUDRATE", "RTE-RESOURCE-ESTIMATION", "SECOND-KEY-SLOT-ID", "PNC-WAKEUP-DATA-MASK", "TARGET-REQUIRED-OPERATION-REF", "DIAGNOSTIC-SOVD-CONFIGURATION-BULK-DATA", "PNC-WAKEUP-ENABLE", "MAX-SOURCE-ELEMENT", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "FLOW-CONTROL-REF", "ROLE-BASED-MC-DATA-ASSIGNMENT", "CONSUMED-SERVICE-INSTANCE-REF", "SUPPORTS-ASYNCHRONOUS-MODE-SWITCH", "COM-MANAGEMENT-GROUP-REFS", "IPV-4-ADDRESS", "TIMESTAMP-SUPPORT", "TFOOT", "STATE-MANAGEMENT-SYNC-ACTION-ITEM", "APMC-INTEGER-PARAM-DEF", "PORT-INTERFACE-BLUEPRINT-MAPPING", "NOT-AVAILABLE-VALUE-SPECIFICATION", "SW-RECORD-LAYOUT-REF", "ECU-INSTANCE-REF-CONDITIONAL", "INNER-GROUP-IREF", "FRESHNESS-PROPS-REF", "DIAGNOSTIC-INFO-TYPE", "IP-V-6-PATH-MTU-TIMEOUT", "TCP-SYN-MAX-RTX", "SUPERVISION-CONDITION", "REQUIRE-RESOURCES", "TCP-IP-DHCP-V-6-SOL-DELAY-MIN", "TLS-IAM-REMOTE-SUBJECT", "HW-PIN-GROUP-CONNECTOR", "DATA-PROTOTYPE-IN-PORT-INTERFACE-REF", "SESSION-ID", "BLUEPRINT-MAPPING", "INDEX", "IMPLEMENTED-ENTRYS", "HOLD-OVER-TIME", "IPV-6-PROPS", "CAN-TP-CONFIG", "SWITCH-FLOW-METERING-ENTRY", "RATE-CORRECTIONS-PER-MEASUREMENT-DURATION", "SUPERVISION-REF", "FEATURE-MODEL-REFS", "SECOND-TRIGGER-REF", "CRYPTO-SERVICE-CERTIFICATE", "SW-CALPRM-AXIS", "DEFAULT-ACTION", "IPV-6-CONFIGURATION", "E-2-E-NO-NEW-DATA-EXTENSION-FACTOR", "ECUC-VALUE-COLLECTION", "NRC-VALUE", "VARIABLE-ACCESS-IREFS", "PREEMPTABILITY", "NUMERICAL-VALUE", "NO-CHECKPOINT-SUPERVISION", "SW-CALPRM-PROTOTYPE-REF", "PROVIDED-CLIENT-SERVER-ENTRYS", "PROVIDED-DDS-OPERATIONS", "PNC-GATEWAY-TYPE", "PDU-REF", "VERIFY-IREFS", "USED-IMPLEMENTATION-DATA-TYPE-REF", "SUB-CONTAINERS", "NETWORK-CONFIGURATION-REF", "COMPOSITE-TYPE-MAPPINGS", "TIME-BASE-RESOURCE-REF", "ICV-SECURED", "SECOND-DATA-PROTOTYPE-REF", "OP", "SW-COMPONENT-PROTOTYPE-REF", "TD-EVENT-BSW-MODULE-TYPE", "EVENT-CLEAR-ALLOWED", "OFNS-DATA-ID-LIST", "EVENT-MAPPINGS", "LOWER-BOUNDARY", "DIAGNOSTIC-DEBOUNCE-BEHAVIOR-ENUM-VALUE-VARIATION-POINT", "EVENT-FAILURE-CYCLE-COUNTER-THRESHOLD", "FRAME-MAPPING", "DIAGNOSTIC-SECURE-CODING-MAPPING", "COMMUNICATION-CONTROLLER-MAPPING", "MINIMUM-SEPARATION-TIME", "DIAGNOSTIC-DE-AUTHENTICATION", "WORST-CASE-HEAP-USAGE", "ROOT-CONTEXT-REF", "MAY-BE-UNCONNECTED", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "DEST-META-CLASS", "QOS-PROFILE", "CALCULATION-FORMULA", "HW-PIN-GROUP-REF", "DIAGNOSTIC-MULTIPLE-CONDITION-PORT-MAPPING", "VLAN-REFS", "SOURCE-MAC-ADDRESS-MASK", "VLAN-MEMBERSHIP", "IPV-4-RULE", "QUEUED-RECEIVER-COM-SPEC-PROPS", "WAKEUP-REPETITIONS-OF-WAKEUP-REQUEST", "GATEWAYS", "PACKAGE-REF", "USER-DEFINED-COMMUNICATION-CONTROLLER-CONDITIONAL", "CONTAINED-AR-ELEMENT-REFS", "COUPLING-PORTS", "CAN-TP-CHANNEL-REF", "SERVICE-KIND", "SDXF", "REQUIRED-RTE-VENDOR", "DIAGNOSTIC-IUMPR-GROUP", "CONTEXT-PORT-REF", "CLIENT-SERVER-PRIMITIVE-TYPE-MAPPING", "COMMUNICATION-CONNECTOR-REFS", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "SOURCE-CHANNEL", "MICRO-PER-CYCLE", "SWC-BSW-RUNNABLE-MAPPING", "ACCEPTED-STARTUP-RANGE", "DIAGNOSTIC-ROUTINE", "OWNERSHIP", "COUPLING-PORT-STRUCTURAL-ELEMENTS", "SECOND-PORT-REF", "VERSION-DRIVEN-FIND-BEHAVIOR", "TP-ECUS", "TD-EVENT-SERVICE-INSTANCE-EVENT", "PROVIDED-DATA-REF", "MEMORY-SECTION-LOCATION", "IPV-4-MULTICAST-IP-ADDRESS", "TTCAN-COMMUNICATION-CONTROLLER-VARIANTS", "HW-PIN-GROUP-CONTENT", "REQUIRED-AR-ELEMENT-REFS", "PNC-PDUR-GROUP-REFS", "DEADLINE-MAX", "SYNC-NODE-MAX", "TD-EVENT-FR-CLUSTER-CYCLE-START", "FRAME-PORT-REF", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "P-2-SERVER-MAX", "GROUP-REF", "POST-BUILD-VARIANT-CRITERION", "DROP-NOT-REQUESTED-NAD", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION", "FALSE-ACTION-LIST-REF", "OPTION-NAME", "MAXIMUM-NUMBER-OF-RETRIES", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "MACRO-INITIAL-OFFSET-B", "NETWORK-CONFIGURATION-REFS", "SW-SYSTEMCONSTANT-VALUE-SET", "CRYPTO-DRIVER", "E-2-E-METHOD-PROTECTION-PROPSS", "TLV-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "N-PDU", "PATH", "ROUTINE-REF", "CONTAINED-PACKAGE-ELEMENT-REF", "TYP", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-PERSISTENCY-DEPLOYMENT-MAPPING", "SERVICE-ELEMENT-MAPPING-REFS", "TYPE-TREF", "DATA-TYPE-MAPPING-SET-REFS", "TD-EVENT-VARIABLE-DATA-PROTOTYPE-TYPE", "EVENT-MAPPING", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REFS", "LIN-TP-CONNECTION", "DIAGNOSTIC-MONITOR-INTERFACE", "EOC-EXECUTABLE-ENTITY-REF", "P-PORT-PROTOTYPE-IN-EXECUTABLE-IREF", "VEHICLE-ROLLOUT-STEP", "SW-CALIBRATION-NOTESS", "OPTION", "SECURITY-LEVEL-REF", "REST-ENDPOINT-GET", "SW-RECORD-LAYOUT-COMPONENT", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "FIREWALL-STATE-MACHINES", "TD-EVENT-MODE-DECLARATION-TYPE", "ETH-TCP-IP-PROPS", "TRANSMISSION-MODE-FALSE-TIMING", "PERSISTENCY-KEY-VALUE-STORAGE", "DIAGNOSTIC-ROUTINE-INTERFACE", "TCP-RECEIVE-WINDOW-MAX", "DERIVED-INTERFACE-REF", "CONSUMED-EVENT-GROUP-REFS", "REST-ENDPOINT-PUT", "J-1939-NM-ECU", "DATA-PROTOTOTYPE-IN-PORT-INTERFACE-REF", "TCP-IP-AUTO-IP-INIT-TIMEOUT", "SECURE-COM-PROPS-FOR-UDP-REFS", "CONFIGURATION-HANDLING", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "DIAGNOSTIC-SOVD-PROXIMITY-CHALLENGE-INTERFACE", "BYTE-ORDER", "TLS-CONNECTION-GROUP", "COMMUNICATION-CONTROLLERS", "DETERMINISTIC-SYNC-M-OUT-OF-N", "DDS-RULE", "CONTAINED-I-PDU-PROPS", "WRITER-ENTITY-ID", "SOURCE-EVENT-REF", "USAGES", "GOVERNANCE-REF", "TCP-MAX-RTX", "STATE-MANAGEMENT-BASED-ON-TCP-FLAGS", "SCOPE-EVENT-REF", "PRE-SHARED-KEY-REF", "DIAGNOSTIC-ROUTINE-CONTROL", "PDU-TRIGGERING", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REFS", "DATA-PDU-REF", "KEY-IDENTITY-REF", "AH-CIPHER-SUITE-NAMES", "RUNNABLE-ENTITY-ARGUMENT", "DLT-LOG-CHANNEL-DESIGN-REF", "CAN-NM-NODE", "MC-FUNCTION-REFS", "FLEXRAY-COMMUNICATION-CONTROLLER-CONDITIONAL", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "PDELAY-RESPONSE-ENABLED", "EXTERN-RATE-CORRECTION", "TP-CONNECTION-CONTROLS", "I-SIGNAL-TYPE", "BROADCAST", "MINIMUM", "TP-CONCURRENT-PROCESSING-SDUS", "PNC-GROUP-REF", "PNC-GROUP-REFS", "BIT-OFFSET", "DIAGNOSTIC-CONTROL-ENABLE-MASK-BIT", "HW-ATTRIBUTE-DEF", "EVENT-HANDLERS", "SECURITY-EVENT-REPORT-INSTANCE-VALUE", "TLV-DATA-ID-DEFINITION", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "CONTENT-ELEMENT-REF", "V-2-X-M-USER-NEEDS", "TTCAN-COMMUNICATION-CONTROLLER", "UNIT-REF", "RUN-RESOURCE", "SW-FEATURE-DEF", "PROTOCOL-VERSION", "FULLY-QUALIFIED-DOMAIN-NAME", "MEMORY-USAGES", "DIAGNOSTIC-DEPLOYMENT-PROPS-REF", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "POSSIBLE-ERROR-REACTIONS", "REQUESTED-MODE-IREF", "COMPU-SCALE", "CAN-TP-NODE", "ARRAY-SIZE-HANDLING", "REFERENCE-TAILORING", "MAX-BUFFER-REQUEST", "DIAGNOSTIC-EVENT-NEEDS", "COMM-CONTROLLER-REF", "MATCHING-EGRESS-RULE-REF", "ARRAY-ELEMENT-MAPPINGS", "LOCAL-CERTIFICATE-REFS", "PROVIDED-DDS-EVENT-QOS-PROPS", "PROPERTYS", "TARGET-P-PORT-PROTOTYPE-REF", "CONTEXT-DATAS", "DIAGNOSTIC-PARAMETER", "MODE-RECEIVER-POLICYS", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "BR", "FLEXRAY-ABSOLUTELY-SCHEDULED-TIMING", "DIAGNOSTIC-COM-CONTROL-SPECIFIC-CHANNEL", "SW-ADDR-METHOD", "HOST-ID", "SECURE-COM-PROPS-REF", "REVISION-LABEL-P-1", "DIAGNOSTIC-ROUTINE-NEEDS", "META-DATA-FIELDS", "DLT-LOG-CHANNEL-REF", "KEY-SLOTS", "CONFIRMATION-THRESHOLD", "MSR-QUERY-RESULT-CHAPTER", "J-1939-NM-NODE", "MAXIMUM-DISTANCES", "TCP-IP-DHCP-V-6-CNF-DELAY-MIN", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "CAN-ENTER-EXCLUSIVE-AREA-REF", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "NUMERICAL-VALUE-SPECIFICATION", "KEEP-ALIVE-TIME", "SPEED", "CONTEXT-SWC-PROTOTYPE-REF", "ROW", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "RESPONSE-ON-ALL-REQUEST-SIDS", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "NETWORK-HANDLES", "RAW-DATA-STREAM-INTERFACE-REF", "BASE-IS-THIS-PACKAGE", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "SYNC-FRAME-ID-COUNT-MAX", "FREEZE-CURRENT-STATE-SUPPORTED", "FRAME-PORT", "CHANGE", "DIAGNOSTIC-EDR-SENDER-PORT-MAPPING", "SOFTWARE-PACKAGE-STEPS", "MODE-SWITCH-POINTS", "FILTER-ACTION-DROP-FRAME", "SDU-TYPE", "APPLICABILITY-INFOS", "SYNC-PERIOD", "TCP-IP-NDP-DYNAMIC-RETRANS-TIME-ENABLED", "RPT-CONTAINER", "TIME-LEAP-HEALING-COUNTER", "OUT-MEASUREMENT-SET", "RECEIVE-WINDOW-MIN", "ACTIVATION", "TCP-IP-NDP-DYNAMIC-REACHABLE-TIME-ENABLED", "RPT-COMPONENT", "INTERFACE-BLUEPRINT-MAPPING", "SW-CARB-DOCS", "SERVICE-BUSY", "PREFIX", "ADD-INFO-5", "SUBTITLE", "AFFECTION-KIND", "FOLLOW-UP-ACTION-REFS", "SHORT-NAME", "FOREIGN-MODEL-REFERENCE", "COMPOSITE-RULE-BASED-VALUE-SPECIFICATION", "RVF-EVENT-DEFAULT", "SIZE-OF-UNION-TYPE-SELECTOR-FIELD", "SAVE-CONFIGURATION", "TRACED-FAILURES", "UNASSIGN-FRAME-ID", "SUPERVISION-MODE-CONDITIONS", "LOG-SINK-REF", "APPLICATION-VALUE-SPECIFICATION", "TD-EVENT-BSW-MODE-DECLARATION-TYPE", "DESTINATION-URI-NESTING-CONTRACT", "MACHINE-DESIGN", "MEMORY-CONSUMPTION", "PDU-COUNTER-SIZE", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF", "APPLICABILITY-INFO-SET", "DEFERRING-FID-REFS", "DATA-TRANSFORMATION", "PROCESSOR-CORE", "SECURED-AREA-OFFSET", "SUB-DOMAIN-REFS", "SPARSE-TIMESTAMP-ENABLED", "MANAGED-COUPLING-PORTS", "RTE-EVENT-IN-SYSTEM-SEPARATION", "BSW-INTERNAL-BEHAVIOR", "MAC-SEC-CIPHER-SUITE-CONFIG", "ACTIVATION-SWITCH", "ECUC-DEFINITION-COLLECTION", "CRYPTO-NEED-REF", "CAN-CONTROLLER-ATTRIBUTES", "START-AGENT-REF", "RPT-EXECUTABLE-ENTITY-PROPERTIES", "UDP-NM-CLUSTER", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "SOURCE-EEC-REF", "ARTIFACT-DESCRIPTORS", "IMMEDIATE-NV-DATA-STORAGE", "RANGE-MIN", "TCP-IP-NDP-DEFENSIVE-PROCESSING", "NM-INSTANCE-ID", "TP-ADDRESS-REFS", "CRYPTO-ALGORITHM-ID", "SEC-OC-CRYPTO-SERVICE-MAPPING", "FOLLOW-UP-ACTION-REF", "COM-FIELD-GRANT", "I-SIGNAL-REFS", "SW-TEXT-PROPS", "CAN-NM-CLUSTER-COUPLING", "CONSTANT-MAPPING-REFS", "MAX-NUMBER-OF-CONNECTIONS", "DOIP-CONNECTION-REF", "RUNNABLE-ENTITY-GROUP-IREF", "TRANSMISSION-MODE-TRUE-TIMING", "STARTUP-OPTION", "DIAGNOSTIC-ENV-CONDITION-FORMULA", "BLUEPRINT-REF", "TERMINATING-CHECKPOINT-TIMEOUT-UNTIL-TERMINATION", "COUNTER-PASSED-THRESHOLD", "MODE-REF", "DIAGNOSTIC-TRANSFER-EXIT", "ORIGIN", "ETHERNET-COMMUNICATION-CONTROLLER-VARIANTS", "APMC-MAC-ADDRESS-PARAM-VALUE", "DID-NUMBER", "CAN-CONTROLLER-FD-ATTRIBUTES", "BUFFER-OUTPUT", "DIAGNOSTIC-CONTRIBUTION-REFS", "TARGET-IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "MIN-SOURCE-PORT-NUMBER", "REVISION-LABEL-P-2", "MAX-ERROR-STATE-INVALID", "CRYPTO-PROVIDER-REF", "NOTIFICATION-TIME", "CAN-NM-ECU", "CRYPTO-SERVICE-NEEDS", "SERVICE-CONTROL", "DIAGNOSTIC-PROTOCOL", "ENABLE-WRITE-ACCESS-CONTROL", "TEX-MATH", "P-PORT-PROTOTYPE", "WAKE-UP-OVER-BUS-SUPPORTED", "FUP-DATA-ID-LISTS", "DIAGNOSTIC-DATA-IDENTIFIER-SET-REF-CONDITIONAL", "LOCAL-PARAMETER-REF", "J-1939-TP-CONNECTION", "UCM-SOFTWARE-VERSION", "SELECTOR-FIELD-START-POSITION", "ECU-INSTANCE-REF", "MESSAGE-COMPLIANCE", "LENGTH-CLIENT-ID", "APPLICATION-ENDPOINT", "CRYPTO-CERTIFICATE-GROUP-INTERFACE", "GLOBAL-RESOURCE-ID", "IMPLEMENTS-SOMEIP-STRING-HANDLING", "LIN-CLUSTER-VARIANTS", "PDU-TO-FRAME-MAPPINGS", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "CONTROL-CONSUMED-EVENT-GROUP-REFS", "ROUTINE-CONTROL-CLASS-REF", "IGNORE-AFTER-TX", "SEC-OC-IGNORE-VERIFICATION-RESULT", "ECU-TASK-PROXY-REFS", "LOCAL-COMM-CONNECTOR-REF", "IS-ACTIVATION-LINE-DEPENDENT", "COM-MANAGEMENT-GROUP-REF", "EVENT-SYMBOL-NAME", "MINISLOT-DURATION", "LIN-NM-CLUSTER", "APPLICATION-TYPE-TREF", "RUNTIME-ERROR", "SOVD-PROPERTIES", "ROOT-DATA-PROTOTYPE-IN-CS-REF", "PORT-PROTOTYPE-BLUEPRINT-REF", "DIAGNOSTIC-EVENT-PORT-MAPPING", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "VENDOR", "DIAGNOSTIC-CONTROL-NEEDS", "SIGNAL-I-PDU-REPLICATION", "SWC-TO-ECU-MAPPING", "MAPPINGS", "MINIMUM-SUPPORTED-UCM-VERSION", "PDELAY-RESP-AND-RESP-FOLLOW-UP-TIMEOUT", "EXECUTABLE-TIMING", "DIAGNOSTIC-J-1939-SPN-MAPPING", "CAN-NM-TX-CAN-ID", "ITEM-DEFINITIONS", "SEC-OC-CRYPTO-SERVICE-MAPPING-REF-CONDITIONAL", "MEMORY-DESTINATION-REF", "FIRST-ELEMENT-INSTANCE-IREF", "STORAGE-CONDITION-GROUP-REF", "IP-V-6-ADDRESS", "TCP-IP-ICMP-V-6-HOP-LIMIT", "PREPARE-UPDATE", "REST-SYSTEM-TRIGGERED-EVENT", "RELATIVE", "SOMEIP-SERVICE-DISCOVERY", "SOVD-VALIDATION-ORDER-REFS", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "UDP-NM-NODE", "TAILORING-REFS", "TCP-IP-PROPS-REF", "NM-WAIT-BUS-SLEEP-TIME", "MANDATORY", "COMMAND", "DTC-STATUS-MASK", "DATA-PROTOTYPES", "RECEIVER-REF", "ECUC-VALIDATION-CONDITION", "PERSISTENCY-DEPLOYMENT-REF", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "EXPRESSION", "COMPU-METHOD-REF", "FRAME-PREEMPTION-SUPPORT", "STATE-MANAGEMENT-SET-FUNCTION-GROUP-STATE-ACTION-ITEM", "FRESHNESS-VALUE-LENGTH", "DISCOVERY-TYPE", "SOFTWARE-ACTIVATION-DEPENDENCY-REF", "SIZE-OF-UNION-LENGTH-FIELDS", "BSW-ENTITY-REF", "TIME-EXP", "NUM-FAILED-SECURITY-ACCESS", "MC-SUPPORT", "APMC-STRONG-REVISION-LABEL-PARAM-VALUE", "I-SIGNAL-TO-I-PDU-MAPPING", "BSW-MODE-SWITCH-EVENT", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-VALUE", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "SOCKET-PROTOCOL", "DESCRIPTION", "RX-SECURITY-VERIFICATION", "TP-ADDRESS-REF", "LOG-TRACE-DEFAULT-LOG-LEVEL", "GETTER-REF", "SECURE-COM-PROPS-SET", "DIAGNOSTIC-AGING", "RESET-TO-DEFAULT", "ARTIFACT-CHECKSUM", "SIGNATURE-SCHEME-REFS", "PHYS-CONSTRS", "DO-IP-PROTOCOL-VERSION", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "CRYPTO-SERVICE-QUEUE", "BUILD-ACTION-IO-ELEMENT", "CRYPTO-R-PORT-COM-SPEC", "BLUEPRINT-POLICY-LIST", "CRYPTO-ALGORITHM-STRING", "COUPLING-ELEMENT-DETAILS", "COMMITTED-INFORMATION-RATE", "FULL-DUPLEX-ENABLED", "START-IREF", "SDG-CONSTRAINT-REFS", "COLLECTED-INSTANCE-IREF", "ARGUMENT", "LOCAL-ID", "STACK-USAGES", "SDG-CLASS-REF", "API-PRINCIPLE", "SERVICE-INSTANCE-ID", "TRANSFORMATION-DESCRIPTIONS", "MAPPING-SCOPE", "DECOMPOSITIONS", "RAM-BLOCK-INIT-VALUE", "TRANSFORMATION-PROPS-MAPPING-SET-REF", "OPERATION-CYCLE-AUTOMATIC-END", "NM-BUS-LOAD-REDUCTION-ENABLED", "SW-COMPONENT-PROTOTYPE", "COM-EVENT-GRANT-DESIGN", "TRIGGER-I-PDU-SEND-CONDITION", "LATENCY-BUDGET-DURATION", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "SYSTEM-MAPPING", "QUEUED-SENDER-COM-SPEC", "QUEUE-LENGTH", "CONTEXT-IMPLEMENTATION-DATA-ELEMENT-REFS", "SUBFUNCTION", "X", "MODE-SWITCH-SENDER-COM-SPEC-PROPSS", "ADDRESS-RANGE-UPPER-BOUND", "COMMUNICATION-CLUSTER-REFS", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "INVALID-VALUE-BITFIELD-ERRORS-ENABLED", "LOAD-BALANCING-PRIORITY", "AFFECTED-BSW-MODULE", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "ADDITIONAL-NATIVE-TYPE-QUALIFIER", "REQUEST-EMISSION-RELATED-DTC-CLASS-REF", "WITH-AUTO", "IKE-CIPHER-SUITE-NAME", "AUTHENTICATION-BUILD-ATTEMPTS", "PARAMETER-ACCESS", "RATE-POLICYS", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "SEPARATION-CYCLE-EXPONENT", "SOMEIP-SD-RULE", "I-SIGNAL-I-PDUS", "WRITING-STRATEGYS", "PAYLOAD-BYTE-PATTERN-RULE-PART", "STREAM-IDENTIFIER", "REFERENCE-VALUE-SPECIFICATION", "RAM-LOCATION-REF", "CRYPTO-KEY-TYPE", "SHARED-AXIS-TYPE-REF", "UAS-ID", "STORE-IMMEDIATE", "TIME-PASSED-THRESHOLD", "CERTIFICATE-REF", "FLEXRAY-AR-TP-NODE", "BUILD-ACTION-MANIFESTS", "STOP-AGENT-REF", "DETECT-NIT-ERROR", "TP-CHANNELS", "VLAN", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "GLOBAL-ELEMENTS", "LATENCY-BUDGET", "AUDIENCE", "NETWORK-SEGMENT-ID", "NETWORK-REPRESENTATION", "DATA-LENGTH", "MESSAGE-TYPE", "VIEW-MAPS", "FRESHNESS-TIMESTAMP-TIME-PERIOD-FACTOR", "SOMEIP-METHOD", "DATA-CONSTR-REF", "HW-ATTRIBUTE-DEF-REF", "ACTION-LIST-PROCESSING-FAILED-ERROR-REF", "I-PDU-TRIGGERING", "CRYPTO-JOBS", "ALLOW-PASSIVE-TO-ACTIVE", "INCLUDED-DATA-TYPE-SETS", "TARGET-PROVIDED-OPERATION-REF", "PARAMETER-VALUES", "DLT-MESSAGE-REFS", "DLT-ARGUMENT-PROPSS", "FREEZE-FRAMES", "J-1939-DTC-VALUE", "PNC-VECTOR-OFFSET", "SW-AXIS-CONT", "AUTHENTICATION-KEY-REF", "IDENTICAL-MAPPING", "PDU-ACTIVATION-ROUTING-GROUPS", "IMPLICIT-INTER-RUNNABLE-VARIABLES", "BINARY-MANIFEST-ITEM-NUMERICAL-VALUE", "RUNTIME-IP-ADDRESS-CONFIGURATION", "REMOTE-ACCESS-CONTROL-ENABLED", "ABSOLUTE", "ENTRYS", "IP-IAM-REMOTE-SUBJECT", "TD-EVENT-VFB-REFERENCE", "SWC-TO-SWC-SIGNAL", "POSITION", "APPLIED-STANDARD", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "USE-INSTEAD-REF", "CERTIFICATE-EVALUATIONS", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "CUSTOM-SPECIFICATION-REF", "RESTRICT-UPDATE", "MINIMUM-INTER-ARRIVAL-TIME", "AUTOSAR-VARIABLE-IN-IMPL-DATATYPE", "RECOVERY-NOTIFICATION-REFS", "SIGNALS", "CONTEXT-SW-COMPONENT-REF", "PROTOCOL-KIND", "READ-NV-DATA", "COMMENT", "INITIAL-EVENT-STATUS", "FIELD-NOTIFIER-REF", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-TO-TD-MAPPINGS", "SERVER-SERVICE-MAJOR-VERSION", "PDU-ACTIVATION-ROUTING-GROUP", "SW-COMPONENT-PROTOTYPE-IREF", "EXPECTED-TX-TRIGGER", "ACTION-POINT-OFFSET", "CP-SOFTWARE-CLUSTER-RESOURCE-REF", "COMPOSITION-SW-COMPONENT-TYPE", "ENABLE-DEACTIVATION", "USER-DEFINED-COMMUNICATION-CONTROLLER", "INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "RESULTING-RPT-SW-PROTOTYPING-ACCESS", "DIAGNOSTIC-DATA-IDENTIFIER-REF-CONDITIONAL", "NM-IMMEDIATE-RESTART-ENABLED", "NM-CBV-POSITION", "TCP-FIN-WAIT-2-TIMEOUT", "TWO-KEY-SLOT-MODE", "ECUC-DESTINATION-URI-DEF", "IIDC-FRACTION-NUMBER", "PER-INSTANCE-MEMORY-REF", "MODE-DEPENDENT-STARTUP-CONFIG", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "PROVIDED-SERVICE-INSTANCE-REF-CONDITIONAL", "BLUEPRINT-DERIVATION-GUIDE", "BLOCK-IF-STATE-ACTIVE-CP-REF", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "VEHICLE-SYSTEM-INSTANCE", "MAX-NUMBER-OF-RESP-PENDING-FRAMES", "ACTION-ITEMS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "ELEMENT-CREATION", "OPERATION-MAPPINGS", "MINIMUM-RX-CONTAINER-QUEUE-SIZE", "CONTEXT-ROOT-SW-COMPOSITION-REF", "REMAPPED-CAN-ID", "CONNECTIVITY-KEY-NAME", "CAN-FD-FRAME-SUPPORT", "MAPPED-AP-DATA-ELEMENT-IREF", "IEEE-1722-RAW-DATA-STREAM-CONSUMER-MAPPING", "TRANSFORMATION-TECHNOLOGY", "REPLAY-PROTECTION-WINDOW", "FUNCTION-GROUP-REF", "ECU-INSTANCE-REFS", "DO-IP-INSTANTIATION", "DATE", "CRYPTO-KEY-REF", "MAX-CYCLE-REPETITIONS", "DIAGNOSTIC-EVENT-REFS", "TRANSMITTER-REF", "PARAMETER-PROVIDE-COM-SPEC", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "CHAPTER", "FUNCTION-INSTANCE", "CRYPTO-DRIVER-TO-CRYPTO-JOB-MAPPINGS", "GENERAL-PURPOSE-I-PDU", "MIN-MINOR-VERSION", "UPDATE-INDICATION-BIT-POSITION", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "TRANSMISSION-MODE-CONDITION", "PROVIDED-MODE-GROUP-REF", "VARIABLE-REF", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "DTC-SETTING-PARAMETER", "RESET-CONFIRMED-BIT-ON-OVERFLOW", "L-5", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "SYNC-DATA-ID-LIST", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "TCP-OPTION-FILTER-SET", "NM-BUS-SYNCHRONIZATION-ENABLED", "I-SIGNAL-IN-I-PDU-REF", "ACCESS-COUNT-SET", "APPLICATION-ENTRY", "REFERENCE-VALUE-REF", "EXCLUSIVE-AREA-REF-CONDITIONAL", "FRAME-PORT-REFS", "FRAME-PIDS", "NM-DATA-INFORMATION", "DLT-ECU", "OBD-PROPSS", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "CONTEXT-ATOMIC-COMPONENT-REF", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "SW-MAPPINGS", "PID-REFS", "FUNCTION-GROUP-PORT-MAPPING", "GLOBAL-TIME-TX-PERIOD", "MEM-USAGE", "USE-SECURITY-EXTENSION-RECORD-SIZE-LIMIT", "DEFINITION-REF", "SEND-ACTIVITY", "EXPIRED-SUPERVISION-CYCLE-TOL", "IS-DEFAULT", "AUTOSAR-PARAMETER-IREF", "INDIRECT-API", "STATE-MANAGEMENT-FUNCTION-GROUP-SWITCH-NOTIFICATION-INTERFACE", "DEFAULT-ENDIANNESS", "INTERPOLATION-ROUTINE-MAPPING-SET-REFS", "ACTION-LISTS", "MIN-SERVICE-POINT-ID", "COMPUTED", "TLV-ARGUMENT-REF", "MODE-SWITCH-SENDER-COM-SPEC-PROPS", "CONTAINED-PDU-TRIGGERING-REFS", "REQUEST-METHOD", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "OWNERSHIP-STRENGTH", "PARTS", "TLS-JOB-MAPPINGS", "SDG-DEF", "DIAGNOSTIC-EXTENDED-DATA-RECORD-INTERFACE", "AUTH-DATA-FRESHNESS-START-POSITION", "STEP-SIZE", "DO-IP-TESTER-ROUTING-ACTIVATION-REFS", "SECOND-PHYSICAL-DIMENSION-REF", "TRANSMIT-PDU-REF", "COMPU-INVERSE-VALUE", "DLT-APPLICATION-REF", "MODE-DECLARATION-GROUP-PROTOTYPE", "FIRST-TRIGGER-REF", "DATA-ELEMENT-REFS", "TIME-BASE-JITTER", "FIREWALL-RULE-REFS", "PROVIDED-SERVICE-INSTANCE", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "PHM-ACTION", "SEGMENT-LENGTH", "NUMBER-OF-INSTRUCTIONS", "TTL-MAX", "OUT-MEASURMENT-SET", "SW-ALIGNMENT", "INITIAL-DELAY-MAX-VALUE", "ASSIGNED-ENTRY-REF", "NV-BLOCK-NEEDS", "LUMINOUS-INTENSITY-EXP", "DDS-QOS-PROFILES", "VT", "N-PDU-REFS", "NOTE", "VEHICLE-ANNOUNCEMENT-INTERVAL", "SW-BASE-TYPE", "GLOBAL-TIME-DOMAIN-REF-CONDITIONAL", "DEFAULT-VALUE-HANDLING", "TARGET-RUNNABLE-ENTITY-GROUP-REF", "RATE-CORRECTION-OUT", "SUB-TLV-CONFIG", "EVENT-HANDLER-REFS", "APPLICATION-COMPOSITE-DATA-TYPE-SUB-ELEMENT-REF", "NETWORK-ENDPOINT-REF-CONDITIONAL", "SOURCE-MIN-ADDRESS", "DIAGNOSTIC-ADDRESSS", "SUPERVISION-MODE-CONDITION", "APMC-BOOLEAN-PARAM-DEF", "RECORD-NUMBER", "MINIMUM-INTERVAL-LENGTH", "SOURCE-FIELD-REF", "RPT-SERVICE-POINT-POST-REF", "CYCLE-TIME-VALUE", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "APMC-IP-V4-ADDRESS-PARAM-VALUE", "BSW-INTERNAL-TRIGGERING-POINT-REF-CONDITIONAL", "WAKEUP-SLEEP-ON-DATALINE-CONFIG-REF", "CONTENT-URI", "CRYPTO-KEY-SLOT-USER-DESIGN-MAPPING", "META-DATA-ITEM-SET", "ECUC-INSTANCE-REFERENCE-DEF", "CUSTOM-SDG-DEF-REFS", "FIRST-MODE-GROUP-REF", "LIN-PID-TO-CAN-ID-MAPPINGS", "BOR-COUNTER-L-1-TO-L-2", "COUPLING-PORT-DETAILS", "MAX-RETRIES", "ROLE", "DIAGNOSTIC-CLEAR-CONDITION", "CONSTANT-MAPPING-REF", "AP-APPLICATION-ERROR-SET", "MAX-SOURCE-PORT-NUMBER", "VARIANT-ID", "IMPL-CONSTANT-REF", "MODE-SWITCHED-ACK-EVENT", "UDS-DTC-NUMBER", "PARAMETER-DATA-PROTOTYPE-IREF", "CAN-TP-CHANNEL", "DESTINATION-REF", "NM-NID-POSITION", "MEMORY-HIGH-ADDRESS-LABEL", "REF-MEASUREMENT-SET", "PNC-MAPPING-IDENT-REF", "COUPLING-PORT-STARTUP-ACTIVE-TIME", "EXTENDS-META-CLASS", "PORT-CONFIGS", "TIME-SYNC-R-PORT-PROTOTYPE-IREF", "TIMING-MODE-INSTANCE", "SUB-DOMAIN-REF", "CRYPTO-MODULE-INSTANTIATION", "USED-DATA-ELEMENT", "BLACKLISTED-VERSIONS", "NV-REQUIRE-COM-SPEC", "MAX-GRADIENT", "ECU-PARTITION-REF", "SW-GENERIC-AXIS-DESC", "TLS-JOB-REQUIREMENT", "INDICATOR-STATUS-NEEDS", "MAX-DID-TO-READ", "CLEAR-RESET-EMISSION-RELATED-DIAGNOSTIC-INFO-CLASS-REF", "UDP-RULE", "TIME-SYNC-MODULE-INSTANTIATION", "SOURCE-ELEMENT-REFS", "AR-PACKAGE", "FILTER-ACTION-BLOCK-SOURCE", "MAX-BS", "SEQUENCE-COUNTER", "FIRST-DATA-PROTOTYPE-REF", "LENGTH-EXP", "NUMBER-OF-WORKERS", "DIAGNOSTIC-SOVD-AUTHORIZATION-INTERFACE", "DIAGNOSTIC-CONTROL-DTC-SETTING", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREFS", "SD-SERVER-CONFIG", "REQUIRED-TRIGGER-IREF", "CONSTR-LEVEL", "RID-NUMBER", "EXCLUSIVE-AREA-NESTING-ORDER", "TD-EVENT-SERVICE-INSTANCE-FIELD", "STATE-MANAGEMENT-STATE-MACHINE-ACTION-ITEM", "APPLICATION-ID", "TP-NODES", "PNC-CLUSTER-VECTOR-LENGTH", "LOG-TRACE-PROCESS-ID", "WINDOW-SIZE-INIT", "DIAGNOSTIC-VALUE-NEEDS", "IMMEDIATE-RESUME-TIME", "ENTRY-MODE-DECLARATION-REF", "RUNNABLES", "MODE-SWITCH-INTERFACE", "SUPPLIER-VALIDATION-ORDER-REFS", "SW-DATA-DEPENDENCY", "TRANSPORT-PDU-REF", "TRANSFORMATION-PROPS-REF", "WRITTEN-LOCAL-VARIABLES", "TP-TECHNOLOGY", "INDICATIONS", "SECONDARY-FRESHNESS-VALUE-ID", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPINGS", "ASSIGN-NAD", "ALIAS-NAME-ASSIGNMENT", "CORE-ID", "MULTIPLE-CONFIGURATION-CONTAINER", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE", "WAIT-POINTS", "DATA-ID", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "ROOT-PARAMETER-DATA-PROTOTYPE-REF", "LICENSE-REFS", "SUPPORT-INFO-BIT", "ESP-CIPHER-SUITE-NAME", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-TYPE", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "CUSTOM-TRIGGER", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "EXECUTION-CONTEXT-REFS", "INCLUDED-LIBRARY-REF", "SW-COMPONENT-IREF", "TCP-TP", "FALL-BACK-INTERNAL", "UNCOMPRESSED-SOFTWARE-CLUSTER-SIZE", "STREAM-IDENTIFICATION-HANDLE-REFS", "EXPECTED-ENTRYS", "ACTIONS", "REQUIRED-CLIENT-SERVER-ENTRYS", "SW-DATA-DEPENDENCY-ARGS", "END-TO-END-TRANSFORMATION-COM-SPEC-PROPS", "PNC-WAKEUP-CAN-ID-MASK", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "PUTS", "MODE-PORT-ANNOTATIONS", "DATA-TYPE-FOR-SERIALIZATION-REF", "NM-CONTROL-BIT-VECTOR-ACTIVE", "TCP-IP-NDP-MAX-RTR-SOLICITATION-DELAY", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "SOURCE-TRIGGER-REF", "SYMBOL-WINDOW-ACTION-POINT-OFFSET", "GLOBAL-SUPERVISION-ENTITY", "SOFTWARE-CLUSTER-REF", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "TCP-IP-ICMP-PROPS-REF", "WAIT-POINT", "DISABLE-END-TO-END-STATE-MACHINE", "MODIFICATIONS", "ACL-CONTEXT", "BIT", "DATA-TYPE-MAPPING-SET", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPINGS", "KEYWORD-SET", "ECUC-FUNCTION-NAME-DEF", "REPORTABLE-SECURITY-EVENT-REF", "RAM-BLOCK", "NM-USER-DATA-ENABLED", "CERTIFICATE-TO-KEY-SLOT-MAPPINGS", "COUNTER-FAILED-THRESHOLD", "INTERPOLATION-ROUTINE-MAPPINGS", "CONFLICTS-TO", "TX-ENABLE-WINDOW-LENGTH", "ENUMERATION-MAPPING-TABLE", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-INTERFACE", "CLIENT-SERVER-ANNOTATIONS", "DIAGNOSTIC-ADDRESS", "VENDOR-SPECIFIC-SERVICE-NEEDS", "CAN-IDENTIFIER-RANGE", "DIAGNOSTIC-AUTH-ROLE", "BSW-PER-INSTANCE-MEMORY-POLICYS", "MONOTONY", "SW-RECORD-LAYOUT-V-PROP", "NM-MAIN-FUNCTION-PERIOD", "ALLOWED-SERVICE-CONSUMERS", "DIAGNOSTIC-CONTRIBUTION-REF", "DDS-CP-CONSUMED-SERVICE-INSTANCE", "TLV-DATA-ID-DEFINITION-SET", "SW-VALUES-PHYS", "EVENT-COMBINATION-REPORTING-BEHAVIOR", "SOFTWARE-CLUSTER-DIAGNOSTIC-DEPLOYMENT-PROPS", "NM-COORD-CLUSTER", "ECU-STATE-MGR-USER-NEEDS", "TRANSFORMER-REF", "CRYPTO-KEY-SLOT", "SECOND-ELEMENTS", "TX-PDU-PRIORITY", "CAN-XL-FRAME-TRIGGERING-PROPS", "OPERATION-CYCLE-REF", "LOCAL-SUPERVISIONS", "J-1939-CONTROLLER-APPLICATION-REF", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "DATA-TRANSFER-CLASS-REF", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "ORDERED-ELEMENTS", "CPU-USAGE", "SHALL-NOT-RUN-ON-REF", "TIME-BASE-KIND", "MINIMUM-DELAY", "SOFTWARE-PACKAGE-STEP", "POWER-DOWN-TIME", "FOLLOW-UP-OFFSET", "FIELD-REPLY-TOPIC-NAME", "IDSM-REPORTING-MODE-PROVIDER-MAPPING", "CLIENT-SERVER-INTERFACE-MAPPING", "IEEE-1722-TP-RVF-CONNECTION", "LABELED-ITEM", "SD-SERVER-EG-TIMING-CONFIGS", "CHECKPOINT-INITIAL-REFS", "SYSTEM-COM-SPEC-DEFINITION-REF", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPES", "OBD-MID", "UNICAST-SECURE-COM-PROPS-REFS", "COMMUNICATION-MODE", "TIMEOUT-AS", "EVALUATED-VARIANT-REF", "CAN-CONTROLLER-FD-REQUIREMENTS", "DETERMINISTIC-SYNC-MASTER-REF", "PORT-ARG-VALUES", "CAN-TP-ADDRESS-REF", "PREDECESSOR-REFS", "SYNCHRONIZATION-POINT-CONSTRAINT", "SEND-RESP-PEND-ON-TRANS-TO-BOOT", "IDSM-PROPERTIES", "BSW-MODE-MANAGER-ERROR-EVENT", "TCP-IP-NDP-PACKET-QUEUE-ENABLED", "LOGICAL-EXPRESSION-ARGUMENT-REF", "DERIVATION", "GENERIC-TP", "TARGET-R-PORT-REF", "STATE-MANAGEMEN-PHM-ERROR-INTERFACE", "PROCESSING-STYLE", "LOG-AND-TRACE-INTERFACE", "IP-ADDRESS-KEEP-BEHAVIOR", "SWC-TO-APPLICATION-PARTITION-MAPPINGS", "SECURITY-EVENT-REPORT-INTERFACE", "SW-RECORD-LAYOUT-V-INDEX", "VERIFY-REF", "NETWORK-MANAGEMENT-PORT-INTERFACE", "TCP-LOCAL-PORT", "NODE-NAME", "MANAGED-PHYSICAL-CHANNEL-REFS", "NM-COM-CONTROL-ENABLED", "NAME", "COMMUNICATION-CONTROLLER-REF", "DEFAULT-VALUE", "COMPU-DEFAULT-VALUE", "DTC-KIND", "COMPOSITE-NETWORK-REPRESENTATION", "REQUIRED-DATAS", "EVALUATION-ID", "ENTRY-REF", "I-SIGNAL-TRIGGERINGS", "SDF", "ROOT-DATA-PROTOTYPE-REF", "FIELD-REFS", "TRANSFORMER-HARD-ERROR-EVENT", "MAX-AVAILABLE-PERSISTENCY-STORAGE-SPACE", "DIAGNOSTIC-SOVD-CONTENT-ELEMENT-INTERFACE", "TEXT-VALUE-SPECIFICATION", "AUTOSAR", "CONTAINED-FIBEX-ELEMENT-REFS", "TLS-SECURE-COM-PROPS-REF", "ASSIGNED-LIN-SLAVE-CONFIG-REF", "FUNCTION-GROUP-TYPE-REF", "LOG-TRACE-DEFAULT-LOG-THRESHOLD", "DATA-RECEIVE-POINT-BY-ARGUMENTS", "CONTEXT-DATA-SOURCE", "STREAM-GATES", "USES-END-TO-END-PROTECTION", "BSW-BACKGROUND-EVENT", "APP-TASK-PROXY-REF", "FAILED-SUPERVISION-CYCLES-TOLERANCE", "SUPPORTS-MULTIPLE-INSTANTIATION", "FIFO-DEPTH", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "SW-DIAGNOSTICS-NOTES", "LEGISLATED-FREEZE-FRAME-CONTENT-UDS-OBDS", "MODE-TRANSITIONS", "R-PORT-PROTOTYPE", "CLEAR-EVENT-BEHAVIOR", "SWC-TO-IMPL-MAPPING", "RECOVERABLE-IN-SAME-OPERATION-CYCLE", "INITIAL-REPETITIONS-BASE-DELAY", "UPPER-CAN-ID", "ENABLE-DISCOVERY-PROTECTION", "USES-MONITOR-DATA", "CRYPTO-CERTIFICATE", "DIAGNOSTIC-J-1939-NODE", "NETWORK-TIME-MASTER-REF", "REMOTE-MULTICAST-SUBSCRIPTION-ADDRESSS", "ROOT-VARIABLE-DATA-PROTOTYPE-REF", "REQUESTER-IREF", "EXIT-TIMEOUT-VALUE", "CHANNEL-ID", "UDP-PROPS", "DDS-RPC-SERVICE-DEPLOYMENT", "CRYPTO-SERVICE-CERTIFICATE-REF", "GETS", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "RESULTING-PROPERTIES", "DATA-RECEIVE-POINT-BY-VALUES", "DESTINATION-URI-POLICY", "UNICAST-CREDENTIALS", "NM-SCHEDULE-VARIANT", "DLT-LOG-SINK-REFS", "MODE-USER-ERROR-BEHAVIOR", "DYNAMIC-LENGTH", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "UCM-MASTER-MODULE-INSTANTIATION", "MC-DATA-INSTANCE-REFS", "FRAGMENTATION-PROPS", "CONTROL-ENABLE-MASK-BITS", "OUTER-PORT-REF", "CHARGE-MANAGER-NEEDS", "MC-DATA-INSTANCE-REF", "AUTHENTICATION-ROLE-REFS", "ECUC-MULTILINE-STRING-PARAM-DEF-VARIANTS", "OVERRIDE-ID", "UPPER-BOUND", "SIGNAL-SERVICE-TRANSLATION-PROPS", "TX-PDU-POOL-REF", "DEFAULT-ROUTER", "LEGISLATED-FREEZE-FRAME-CONTENT-WWH-OBDS", "CAN-FD-CONFIG", "REACTION-CODE", "DEBOUNCE-BEHAVIOR", "REST-ENDPOINT-ARGUMENT", "DLT-LOG-CHANNEL", "RTE-EVENT-TO-OS-TASK-PROXY-MAPPINGS", "SYSTEM-VERSION", "IEEE-1722-TP-ACF-LIN", "FM-FEATURE-RESTRICTION", "REST-ENDPOINT-DELETE", "RTE-EVENT-SEPARATIONS", "APPLICATION-CONTEXT-REFS", "NOTIFIER-DATA-ELEMENT-REF", "ON-BOARD-MONITOR-ID", "PRE-MAPPING", "MODE-DECLARATION-MAPPING-SET", "METHOD-CALL-REF", "DDS-CP-PARTITION", "CONCRETE-CLASS-TAILORING", "ALLOW-DYNAMIC-L-SDU-LENGTH", "RULE-ARGUMENTS", "CONTEXT-DATA-PROTOTYPE-REF", "TBODY", "DATA-LENGTH-RESPONSE", "I-PDU-PORT-REFS", "KIND", "DESTINATION-URI-DEFS", "FUNCTIONAL-REQUEST-REF", "SECURITY-EVENT-FILTER-CHAIN-REF-CONDITIONAL", "PROCESS-STATE-IREF", "NETWORK-INTERFACE-REF", "TRANSFORMATION-TECHNOLOGYS", "FIREWALL-STATE-MODE-DECLARATION-REF", "UNIT-REFS", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "TLS-CRYPTO-SERVICE-MAPPING", "LIN-FRAME-TRIGGERING", "GENERAL-PURPOSE-PDU", "NM-INSTANTIATION", "INSTANCE-IDENTIFIER", "TRANSITIONS", "ROUTINE-SUBFUNCTION-REF", "CONNECTOR-REF", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "PHM-RULE", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPSS", "SUPPLIER-ID", "ACTION", "TIME-BASE", "MAX-PWM-O", "TIMING-DESCRIPTION-EVENT-CHAIN", "ECUC-REFERENCE-DEF", "MODE-TRANSITION", "IS-REENTRANT", "ENABLE-ANYCAST", "LIN-ERROR-RESPONSE", "CLIENT-SERVER-TO-SIGNAL-GROUP-MAPPING", "STATE-MANAGEMENT-DIAG-TRIGGER-INTERFACE", "TARGET-EEC-REF", "SDG-AGGREGATION-WITH-VARIATION", "POSSIBLE-AP-ERROR-SET-REFS", "CURRENT-DATA-TYPE-REF", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "NM-DATA-ENABLED", "MODE-DECLARATION-MAPPING", "SOUCE-CAN-ID-REF", "AUTH-INFO-TX-LENGTH", "RPT-CONTAINERS", "REMOTE-UNICAST-CONFIG-REFS", "DDS-QOS-PROFILE-REF", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "SUBSCRIBE-EVENTGROUP-RETRY-DELAY", "FLEXRAY-COMMUNICATION-CONNECTOR", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING-REF", "IDSM-TIMESTAMP-PROVIDER-INTERFACE", "TRUE-ACTION-LIST-REF", "ATS-INSTANCE-ENTRY-REF", "AAF-NOMINAL-RATE", "CONTEXT-ROOT-SW-COMPONENT-PROTOTYPE-REF", "PERSISTENCY-FILE-ARRAY", "SESSION-CONTROL-CLASS-REF", "ECUC-LINKER-SYMBOL-DEF-VARIANTS", "OPTIONAL-ELEMENTS", "OPT-APP-RECORD-ELEMENT-IN-METHOD-IREF", "J-1939-NODE", "DIAGNOSTIC-ENABLE-CONDITION-REF", "EVENT-IREF", "TARGET-APPLICATION-RECORD-ELEMENT-REF", "APPLICATIONS", "EXITED-MODE-REF", "NM-CAR-WAKE-UP-BIT-POSITION", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "I-PDU-IDENTIFIER-UDP-REF", "TRANSPORT-PROTOCOLS", "DATA-WRITE-ACCESSS", "ETHERNET-VLAN-TRANSLATION-TABLE", "MODE-IREF", "DLT-ARGUMENT", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "FLAT-INSTANCE-DESCRIPTOR", "HW-ELEMENT-CONNECTIONS", "SEQUENCE-COUNTER-JUMP-WIDTH", "EXTENDED-ADDRESSING", "TCP-UDP-CONFIG", "NM-RANGE-CONFIG", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "MAX-NUMBER-OF-NPDU-PER-CYCLE", "FUNCTION-INHIBITION-NEEDS", "REQUEST-2-SUPPORT", "ARTIFACT-LOCATOR", "SOCKET-ADDRESS", "TLV-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "SOFTWARE-CLUSTER-REQUIREMENT", "DLT-LOG-SINK", "PDU-TRIGGERING-REFS", "DIAGNOSTIC-SOVD-CONFIGURATION-PARAMETER", "NM-CHANNEL-SLEEP-MASTER", "SEQUENCE-OFFSET", "SECURITY-LEVEL-REFS", "TRIGGER-REFS", "SOURCE-ID", "SYNC-TIME-BASE-MGR-USER-NEEDS", "MAC-ADDRESS-STREAM-ID", "FEATURE-REFS", "MAX-EXP-BS", "RVF-TOTAL-LINES", "CIPHER-SUITE-SHORT-LABEL", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "VALUE-DATA-TYPE-REF", "AES-3-DATA-TYPE-H", "PHYSICAL-CHANNEL-REFS", "SYNCHRONIZED-MODE-GROUPS", "TCP-IP-PROPS-REFS", "FEATURE-REF", "TCP-IP-IP-NUM-REASS-DGRAMS", "GATEWAY", "GLOBAL-SUPERVISION-NEEDS", "SOURCE-PORTS", "TCP-IP-NDP-RTR-SOLICITATION-INTERVAL", "RPT-PREPARATION-LEVEL", "TRIGGER-INTERFACE-MAPPING", "BSW-IMPLEMENTATION", "ML-GRAPHIC", "AUTOMATIC-END", "SECURITY-EVENT-CONTEXT-DATA-DEFINITIONS", "MAPPED-SWC-SERVICE-DEPENDENCY-IN-EXECUTABLE-IREF", "QUEUE-SIZE", "RPT-SUPPORT-DATA", "CONTROLLED-HW-ELEMENT-REF", "LIN-UNCONDITIONAL-FRAME-REFS", "TA-TYPE", "TCP-IP-ARP-REQUEST-TIMEOUT", "REFERENCE-CONDITION", "VENDOR-SPECIFIC-MODULE-DEF-REFS", "CHECKPOINT-FINAL-REF", "SECURITY-EVENT-DEFINITION-REF", "SYNC-LOSS-TIMEOUT", "ACL-ROLE", "CSE-CODE-FACTOR", "ACCESSED-VARIABLE-REF", "EXTERNAL-REPLACEMENT-REF", "CYCLE", "UNIT-GROUP-REF", "PLCA-MAX-BURST-TIMER", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "CHECKPOINT-REFS", "TRUSTED-PLATFORM-EXECUTABLE-LAUNCH-BEHAVIOR", "BSW-MODE-SWITCHED-ACK-EVENT", "LIN-TP-NODE", "CAN-TP-CONNECTION", "PER-STATE-TIMEOUTS", "AGE", "INPLACE", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "GLOBAL-TIME-FR-MASTER", "BLOCK-IF-STATE-ACTIVE-AP-IREFS", "MC-PARAMETER-INSTANCES", "INSTANCE-IN-MEMORY", "FUNCTIONAL-CLUSTER-REF", "MAC-MULTICAST-ADDRESS-REFS", "STRUCTURED-REQ", "EVALUATED-ELEMENT-REFS", "GENERATOR", "DATA-PROTOTYPE-IN-CLIENT-SERVER-INTERFACE-IREF", "TRANSPORT-PRIORITY", "STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-ACTION-ITEM", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "AREA", "HAS-GETTER", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "MAX-MARGIN", "CRYPTO-KEY-SLOT-REF", "HW-PIN-GROUPS", "POLICY-ACTION", "APPL-WATCHDOG-LIMIT", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "MODULE-INSTANTIATIONS", "TIME-BASE-RESOURCE-REF-CONDITIONAL", "ABBR-NAME", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "READONLY", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-REF", "MAX-NUMBER-OF-STORED-DTC-STATUS-CHANGED-EVENTS", "SERVICE-REQUEST-CALLBACK-TYPE", "ECUC-FUNCTION-NAME-DEF-VARIANTS", "AUXILIARY-FIELD-DEFINITIONS", "EXCLUSIVE-AREA", "ROLE-BASED-DATA-TYPE-ASSIGNMENT", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "FILE-INFO-COMMENT", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "CLUSTER-DRIFT-DAMPING", "WAKE-UP-PATTERN", "SOMEIP-EVENT-DEPLOYMENT", "SERIALIZER-REF", "EXPECTED-CALLBACKS", "ERROR-CONTEXT-REF", "DIAGNOSTIC-FREEZE-FRAME-REF-CONDITIONAL", "SUBMESSAGE-TYPE", "CALLED-ENTRYS", "INCLUDE-REFS", "TCP-IP-IP-NUM-FRAGMENTS", "SIZE-OF-STRING-LENGTH-FIELDS", "STATIC-SOCKET-CONNECTION", "PERSISTENCY-FILE", "L-4", "ESP-CIPHER-SUITE-NAMES", "BITFIELD-TEXT-TABLE-MASK-SECOND", "MULTICAST-UDP-PORT", "USE-INSTEAD-REFS", "TARGET-SWC-SERVICE-DEPENDENCY-REF", "STATUS-SUB-TLV", "BSW-MODULE-ENTRY-REF-CONDITIONAL", "COM-MANAGEMENT-MAPPING", "SIGNATURE-SCHEME-ID", "INVALIDATION-POLICY", "TIMING-EVENT-REF", "NM-HANDLE-REF", "SOURCE-POSSIBLE-ERROR-REF", "TD-PDU-TRIGGERING-FILTER-REFS", "DDS-TOPICS", "INVERT-CONDITION", "TRANSITION-REFS", "SOMEIP-TRANSFORMATION-PROPS-REF", "DATA-OFFSET", "SW-AXIS-TYPE", "PNC-VECTOR-LENGTH", "INTERFACE-MAPPING-SET", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "MESSAGE-LINK-POSITION", "USE-CLIENT-AUTHENTICATION-REQUEST", "ECUC-URI-REFERENCE-DEF", "TP-SDU-REF", "PRE-ACTIVATION-REBOOT", "TRANSFER-EXIT-CLASS-REF", "ACF-BUS-PART-REF", "HEADER-TYPE", "WRITTEN-NV-DATA", "DEFAULT-LC-STATE-REF", "TD-EVENT-MODE-DECLARATION", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT", "DIAGNOSTIC-PARAMETER-ELEMENT", "DEPENDS-ON", "FUNCTION-IDENTIFIER-REF", "APMC-VALUE-COLLECTION", "SLEEP-MODE-EXECUTION-DELAY", "PLCA-MAX-BURST-COUNT", "ALL-CHANNELS-REFS", "DATA-PROTOTYPE-IN-SENDER-RECEIVER-INTERFACE-IREF", "RELEASED-TRIGGER-REF", "RETRY-INTERVAL-TIME", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "DIAGNOSTIC-MULTIPLE-EVENT-PORT-MAPPING", "SW-VARIABLE-REFS", "RECORD-ELEMENT-REF", "TIMING-MODES", "STATE-DEPENDENT-STARTUP-CONFIGS", "TTCAN-ABSOLUTELY-SCHEDULED-TIMING", "MASTER-TO-SLAVE-EVENT-MAPPINGS", "DIAGNOSTIC-IO-CONTROL-CLASS", "CRYPTO-CERTIFICATE-TO-CRYPTO-KEY-SLOT-MAPPING", "IAM-MODULE-INSTANTIATION", "UNICAST-SECURE-COM-PROPS-REF", "MAXIMUM-KEY-LENGTH", "SECURITY-EVENT-MAPPING", "GLOBAL-TIME-PROPS", "FLAT-MAP-ENTRY-REF", "TRIGGER-I-PDU-SEND-CONDITIONS", "FRESHNESS-VALUE-TX-LENGTH", "FIREWALL-RULE-PROPSS", "MULTICAST-CONNECTOR-REF", "PROCESS-MODE-MACHINE", "SOVD-VALIDATION-ORDER-REF", "DURABILITY", "DO-IP-NETWORK-CONFIGURATION", "CAN-CLUSTER-CONDITIONAL", "ERROR-HANDLING", "BGCOLOR", "IEEE-1722-TP-CONNECTION-REF-CONDITIONAL", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "SEQUENTIAL-INSTRUCTIONS-BEGIN", "SOFTWARE-PACKAGE", "INTENDED-TARGET-MACHINE-REFS", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "PDU-TRIGGERINGS", "CALL-TIMEOUT", "PERSISTENCY-REDUNDANCY-CRC", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "CONTEXT-REF", "NTU", "SW-CALPRM-IMPL", "NV-BLOCK-DATA-MAPPING", "RULES", "RESPONSE-REF", "APPLICATION-DESCRIPTION", "HW-PIN-CONNECTIONS", "E-2-E-PROFILE-CONFIGURATION", "ETS-AVAILABLE-BANDWIDTH-IN-PERCENT", "AP-APPLICATION-ENDPOINTS", "TRANSMISSION-MODE-CONDITIONS", "TYPE-EMITTER", "NEXT-HIGHER-CERTIFICATE-REF", "CONCRETE-PATTERN-EVENT-TRIGGERING", "MAX-NUMBER-OF-ELEMENTS", "APPLICATION-PARTITION-REFS", "DIAGNOSTIC-SOVD-UPDATE-PORT-MAPPING", "FLEXRAY-NM-ECU", "CYCLIC-TIMING", "FLAT-MAP-ENTRY-REFS", "SEVERITY", "DDS-CP-PROVIDED-SERVICE-INSTANCE", "OPTION-ARGUMENT", "SOURCE-LIN-PID-REF", "SW-NUMBER-OF-AXIS-POINTS", "ECUC-ADD-INFO-PARAM-DEF", "DIAGNOSTIC-SECURITY-LEVEL", "SLAVE-ACT-AS-PASSIVE-COMMUNICATION-SLAVE", "DYNAMIC-PNC-MAPPING-PDU-GROUP-REF", "LIFE-CYCLE-INFO-SET", "TCP-IP-NDP-RND-RTR-SOLICITATION-DELAY-ENABLED", "STOP-STATE-MACHINE-REF", "SOURCE-REF", "CLIENT-SERVER-OPERATION-MAPPING", "TYPE-DEFINITION", "SW-POINTER-TARGET-PROPS", "FORMAL-BLUEPRINT-GENERATOR", "FORMULA-CAPTION", "TCP-IP-DHCP-V-6-CNF-DELAY-MAX", "DIAGNOSTIC-J-1939-SPN", "CONFIG-ELEMENT-DEF-LOCAL-REF", "CIPHER-SUITE-ID", "PERSISTENCY-REDUNDANCY-M-OUT-OF-N", "TAG-WITH-OPTIONAL-VALUE", "INGRESS-PRIORITY", "SDX-REF", "USED-SECONDARY-FID-REF", "INSTANTIATION-RTE-EVENT-PROPSS", "BSW-ENTRY-RELATIONSHIPS", "GETTER-OPERATION-REF", "EGRESS-PORT-REF", "SW-VALUE-BLOCK-SIZE", "DO-IP-TP-CONNECTION", "SOFTWARE-CLUSTER-TO-RESOURCE-MAPPINGS", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "UCM-PROCESSINGS", "UDP-NM-CLUSTER-COUPLING", "ECUC-MULTILINE-STRING-PARAM-DEF-CONDITIONAL", "VF", "SECURE-COMMUNICATION-DEPLOYMENTS", "ECU-CONFIGURATION-REF", "EXIT-MODE-DECLARATION-REF", "RULE-INIT-STATE", "ONE-EVERY-N", "IDENT", "SERVICE-SW-COMPONENT-TYPE", "TRIGGERS-RECOVERY-NOTIFICATION", "SUPPORTING-MATERIAL", "MAX-ALIVE-CYCLE", "I-PV-6-EXT-HEADER-FILTER-SET", "SDU-REF", "APPLICATION-COMPOSITE-ELEMENT-IREF", "SERVICE-INTERFACE-TREF", "DIAGNOSTIC-INDICATOR", "PERIOD-END", "COUPLING-PORT-SHAPER", "BIT-NUMBER", "KEY-SLOT-ID", "BSW-OS-TASK-EXECUTION-EVENT", "CONTRACT-VERSION", "FIELD-ELEMENTS", "DTCS", "SDG-FOREIGN-REFERENCE", "ETHERNET-RAW-DATA-STREAM-GRANT", "FILTERING-DEBOUNCING", "WRITING-PRIORITY", "SERVICE-INSTANCE-COLLECTION-SET", "TRIGGER-PERIOD", "UDP-NM-CLUSTER-REF", "DIAGNOSTIC-SOVD-BULK-DATA-INTERFACE", "UDP-LOCAL-PORT", "SOURCE-CAN-ID-CODE", "SW-GENERIC-AXIS-PARAM", "START", "VG", "EID-USE-MAC", "ENTITY-STATUS-MAX-BYTE-FIELD-USE", "LABELED-LIST", "MATCHING-INGRESS-RULE-REF", "MANAGED-PHYSICAL-CHANNEL-REF", "HEADER-ID", "SAFE-TRANSLATION", "VIOLATED-SAFETY-CONDITION-BEHAVIOR", "SWC-TO-ECU-MAPPING-CONSTRAINT-TYPE", "RESPONSE-ON-SECOND-DECLINED-REQUEST", "COMM-CONTROLLERS", "DIAGNOSTIC-SOVD-UPDATE", "STANDARD", "ETH-TCP-IP-ICMP-PROPS", "BYTE", "PLCA-LOCAL-NODE-COUNT", "TRIGGER-REF-CONDITIONAL", "RPT-SERVICE-POINTS", "PROVIDED-SERVICE-PORT-IREF", "SERVICE-TIMING", "CUSTOM-SERVICE-CLASS-REF", "DDS-PROVIDED-SERVICE-INSTANCE", "TRIGGER-REF", "REFERENCED-BASELINE", "METHOD-IREF", "RESOURCE-DEFINITION-REF", "SA-REKEY-TIME", "STATIC-MEMORYS", "VARIABLE-ACCESSED-FOR-DEBUG-REFS", "VALIDATION-CONFIGURATION", "XCP-PDU", "DO-NOT-FRAGMENT", "DATA-TYPE-REFS", "COM-MANAGEMENT-PORT-GROUP-IREFS", "DATA-TRANSFORMATIONS", "PROVIDED-EVENTS", "DATA-SEND-COMPLETED-EVENT", "MATCHING-INGRESS-RULE-REFS", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "DIAGNOSTIC-TROUBLE-CODE-OBD", "NV-DATAS", "SUPPORT-INFO-BYTE", "TTCAN-PHYSICAL-CHANNEL", "TTL", "SOURCE-ELEMENT-REF", "REPORT-BEHAVIOR", "TABLE-CAPTION", "REASON", "REQUIRED-GENERATOR-TOOLS", "TIME-BASE-RESOURCE-REFS", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "ALL-PHYSICAL-CHANNELS-REF", "ELEMENTS", "PREDEFINED-TEXT", "DIAGNOSTIC-COMMON-PROPS-CONDITIONAL", "ETH-IP-PROPS-REF", "LIMIT", "MODE-IN-BSW-INSTANCE-REF", "GENERAL-PURPOSE-CONNECTION", "DYNAMIC-PARTS", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "TRACE-MESSAGE-REF", "ECUC-DESTINATION-URI-DEF-SET", "CONTROL-PROVIDED-EVENT-GROUP-REF", "ATS-INSTANCE-ENTRYS", "DISABLED-IN-MODE-IREF", "IDSM-INSTANCE-REF-CONDITIONAL", "LOCAL-UNICAST-ADDRESSS", "MAC-MULTICAST-ADDRESS-REF", "REQUIRED-SERVICE-VERSION", "USER-DEFINED-COMMUNICATION-CONNECTOR", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING", "HW-ATTRIBUTE-VALUES", "EVENT-DEFAULT-VALUE", "TRANSLATION-TARGET-IREF", "SLEEP-TIME", "NETWORK-ENDPOINTS", "GLOBAL-TIME-PDU-TRIGGERING-REF", "RECEIVER-ANNOTATION", "REMOTE-SERVER-CONFIG", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "ECUC-QUERY-REF", "COMPLEX-TYPE-MAPPING", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "HOP-LIMIT", "EXCESS-INFORMATION-RATE", "MODE-ELEMENTS", "SERVER-ARGUMENT-IMPL-POLICY", "DIAGNOSTIC-DATA-IDENTIFIER-REF", "SCHEDULER-MAX-NUMBER", "PROGRAMMING-LANGUAGE", "REPLY-TOPIC-NAME", "ECUC-ENUMERATION-LITERAL-DEF", "BSW-INTERRUPT-EVENT", "PDU-TRIGGERING-REF", "SECOND-VALUE", "ALLOWED-TCP-OPTIONS", "TARGET-EVENT-REFS", "DIAGNOSTIC-COM-CONTROL-SUB-NODE-CHANNEL", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF-CONDITIONAL", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "FM-FEATURE-MODEL", "PROCESS-REF", "APPLICATION-DATA-TYPE-REF", "IMPORTANCE", "MIN-NUMBER-OF-OCCURRENCES", "MEMORY-DESTINATION-REFS", "FLAT-MAP", "CONTAINER-I-PDU-HEADER-BYTE-ORDER", "PATTERN-JITTER", "CRYPTO-KEY-SLOT-DESIGN-DESCRIPTION", "REQUESTOR-REFS", "OPERATIONS", "SW-VARIABLE-PROTOTYPE-REF", "CONTEXT-ROOT-COMPONENT-REF", "PID-SIZE", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "MIN-PWM-O", "TP-ADDRESS", "PARAMETER", "RULE-BASED-AXIS-CONT", "INNER-PORT-IREF", "ACTION-LIST-EXECUTION", "STATE-DEP-FIREWALL-REF", "DPD-DELAY", "RECEIVER-INTENT", "TRANSMIT-PDU-REFS", "KEY-ID", "EXTENDED-DATA-RECORD-ELEMENT-REF", "DEFAULT-MODE-REF", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "APPLICATION-DEFERRED-DATA-TYPE", "CAN-XL-PROPS-REFS", "STATE", "AUTO-AVAILABLE", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "BEHAVIOR", "FR-GLOBAL-TIME-DOMAIN-PROPS", "MAX-SIZE-TO-FILL", "FIBEX-ELEMENT-REF-CONDITIONAL", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "CAN-IDENTIFIER-MASK", "SWC-SERVICE-DEPENDENCY", "COUNT-PROFILE", "ETHERNET-COMMUNICATION-CONNECTOR", "OPT-IMPL-RECORD-ELEMENT", "APPLICABILITY-INFO", "IIDC-T-CODE", "IDS-DESIGN", "FLEXRAY-TP-NODE", "PAYLOAD-BYTE-PATTERN-RULE-PARTS", "SERVICE-INTERFACE-ELEMENT-MAPPING-REF", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "DATA-PROTOTYPE-IN-SERVICE-INTERFACE-IREF", "INTERPOLATION-ROUTINE-MAPPING-SET", "IN-PLACE", "BSW-MODULE-CLIENT-SERVER-ENTRY", "CAN-FRAME-RX-BEHAVIOR", "TEST-RESULT-REF", "IS-SYSTEM-WIDE-GLOBAL-TIME-MASTER", "MAX-RUN-CYCLE-LOOP", "V-LAN-REFS", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "EVENT-WINDOWS", "ECU-EXTRACT-REF", "FORBIDDEN-SIGNAL-PATH", "SECURED-I-PDU", "INCLUDED-DATA-TYPE-SET", "VALUE-AXIS-DATA-TYPE-REF", "SECURE-ONBOARD-COM-ERROR-REF", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "DEADLINE-MIN", "INITIAL-STATUS", "MC-SW-EMULATION-METHOD-SUPPORT", "HEALTH-CHANNELS", "RECOMMENDED-CONFIGURATION-REF", "ALIVE-SUPERVISIONS", "DEFERRING-FID-REF", "TEST-PATTERN", "MINIMUM-SELECTED-BINDING-TIME", "PROPABILITY", "INTERNET-HEADER-LENGTH", "INTENDED-FILENAME", "REPETITION-PERIOD", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-PORT-MAPPING", "FLOW-METERING-REF", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-IDENTIFICATION", "RX-TIMEOUT-TIME", "ICV-SECURE-COM-PROPSS", "COLLISION-RESOLVING-SCHEDULE-REF", "RPT-IMPL-POLICY", "BUFFER-RATIO", "SUSPEND-TO-RAM-HUB-INTERFACE", "UPSTREAM-REFERENCE-IREF", "MODE-REPORTER-PORT-IREF", "CRC-OFFSET", "REMOTE-IP-ADDRESS-REFS", "REGENERATED-PRIORITY", "ACCESSED-DATA-PROTOTYPE-IREF", "OFNS-DATA-ID-LISTS", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "TD-EVENT-SERVICE-INSTANCE-METHOD", "SWC-TIMING", "CAN-ADDRESSING-MODE", "PROCESS-TO-MACHINE-MAPPINGS", "DIAGNOSTIC-TROUBLE-CODE-PROPS-REF-CONDITIONAL", "INIT-RESOURCE", "PROCESS-DESIGN-REF", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS-REF-CONDITIONAL", "MEASURED-EXECUTION-TIME", "FIRST-APPLICATION-ERROR-REF", "SUSPEND-TO-RAM-MODULE-INSTANTIATION", "FM-FEATURE-MAP-ASSERTION", "TCP-IP-IP-REASSEMBLY-TIMEOUT", "REKEY-INTERVAL", "REFERENCED-TD-EVENT-SWC-REF", "DIAGNOSTIC-CONDITION-INTERFACE", "TIME-BASE-PROVIDER-REF", "TRAFFIC-CLASS-PREEMPTION-SUPPORT", "DIAGNOSTIC-EVENT-INTERFACE", "GENERIC-DIAGNOSTIC-TRANSPORT-INSTANTIATION", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "NM-IF-ECU-REF", "INTERPOLATION-ROUTINE", "CRYPTO-PROVIDER", "DETERMINISTIC-SYNC-MASTER", "TCP-OPTION-FILTER-LIST", "ABSTRACT-CLASS-TAILORING", "ENTRY", "PDU-COUNTER-THRESHOLD", "HW-PIN-REF", "SUCCESSOR-REF", "USER-DEFINED-TRANSFORMATION-COM-SPEC-PROPS", "MAC-MULTICAST-ADDRESS", "PHYSICAL-CHANNEL-REF", "COLLECTION-PROPS", "SOFTWARE-ACTIVATION-DEPENDENCY-COMPARE-CONDITION", "I-PDU-SIGNAL-PROCESSING", "SW-CLUSTER-DESIGN-PROVIDED-PORT-IREF", "BUS-MIRROR-CAN-ID-TO-CAN-ID-MAPPING", "TARGET-MODULE-ID", "EID-RETRIEVAL", "GLOBAL-TIME-DOMAIN-PROPS", "RX-ACCEPT-CONTAINED-I-PDU", "REPORTABLE-SECURITY-EVENT-REFS", "ARP-PROPS", "TIMING-VARIABLES", "REST-BOOLEAN-PROPERTY-DEF", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "NDP-PROPS", "PARAMETER-PORT-ANNOTATIONS", "MAX-LENGTH", "INSTANTIATION-TIMING-EVENT-PROPS", "MAXIMUM-INTENDED-BINDING-TIME", "SHORT-NAME-PATTERN", "OUTER-PORTS", "JITTER", "MAX-BLOCK-SIZE", "POSSIBLE-AP-ERROR-REF", "FILTER-PRIORITY", "SHORT-TERM-ADJUSTMENT", "TCP-FAST-RECOVERY-ENABLED", "MAX-AS", "DYNAMICALLY-ASSIGNED", "MSG-ID-MATCH", "FUNCTION-GROUP-ERROR-MAPPINGS", "LOCAL-CERTIFICATE-REF", "CRYPTO-CERTIFICATES", "TRCV-DELAY-COMPENSATION-OFFSET", "MAX-DELTA-COUNTER-INIT", "P", "BSW-RESOLUTION", "RVF-FRAME-RATE", "FRESHNESS-COUNTER-SYNC-ATTEMPTS", "V-2-X-SUPPORTED", "RECOVERY-NOTIFICATION-REF", "TLS-DEPLOYMENT", "ACF-BUS-REF", "MICRO-INITIAL-OFFSET-B", "KEYWORD", "AUTH-DATA-FRESHNESS-LENGTH", "LIN-SLAVE-CONFIG", "RTE-PLUGIN-PROPS", "FIELD-IREF", "ETHERNET-NETWORK-CONFIGURATION", "ELEMENT-REF", "RESOLUTION", "TIMING-CONDITION", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "TARGET-CATEGORY", "FIBEX-ELEMENT-REF", "TARGET-MODULE-REFS", "USER-DEFINED-PHYSICAL-CHANNEL", "P-PORT-IN-COMPOSITION-INSTANCE-REF", "TIMING-DESCRIPTION-REF", "ARTIFACT-CHECKSUMS", "TD-CP-SOFTWARE-CLUSTER-TO-TD-MAPPINGS", "RECOVERY", "INITIAL-OFFER-BEHAVIOR", "COMMUNICATION-BUFFER-LOCKING", "IO-HW-ABSTRACTION-SERVER-ANNOTATIONS", "BITFIELD-TEXT-TABLE-MASK-NV-BLOCK-DESCRIPTOR", "JOB-REF", "DATA-ID-NIBBLE-OFFSET", "SW-RECORD-LAYOUT-GROUP-FROM", "DIAGNOSTIC-AUTHENTICATION-REF", "TCP-NAGLE-ENABLED", "RUNNABLE-ENTITY-GROUP-IREFS", "SERVICE-POINT-SYMBOL-PRE", "GLOBAL-TIME-CORRECTION-PROPS", "RPT-SERVICE-POINT-POST-REFS", "INFO-TYPE", "CRYPTO-CERTIFICATE-INTERFACE", "NM-NODES", "DPD-ACTION", "TCP-IP-NDP-RANDOM-REACHABLE-TIME-ENABLED", "ECUC-NUMERICAL-PARAM-VALUE", "BLUEPRINT-CONDITION", "PARAMETER-ID", "MAX-SYNC-JUMP-WIDTH", "BUS-MIRROR-LIN-PID-TO-CAN-ID-MAPPING", "ENDPOINTS", "TIMESTAMP-RX-ACCEPTANCE-WINDOW", "ORDERED-MASTER-LIST", "TIME-PERIOD", "FILTER-ACTION-DEST-PORT-MODIFICATION", "NON-VERBOSE-MODE", "STREAMS-PER-FRAME", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "PARAMETER-PORT-ANNOTATION", "ETHERNET-PHYSICAL-CHANNEL", "TCP-IP-NDP-DESTINATION-CACHE-SIZE", "CONTROLLER-REF", "SDG-CLASSES", "ETHERNET-CLUSTER-REF", "TLS-CRYPTO-MAPPING-REF", "META-DATA-ITEMS", "TD-EVENT-TT-CAN-CYCLE-START", "LIN-MASTER-CONDITIONAL", "PRIORITY-ID", "TARGET-DATA-PROTOTYPE-GROUP-REF", "SIZE-OF-STRING-LENGTH-FIELD", "CLEAR-CONDITION-GROUP-REF", "TP-ADDRESS-EXTENSION-VALUE", "MAXIMUM-SUPPORTED-UCM-MASTER-VERSION", "BOR-TIME-L-2", "WAKEUP-RX-LOW", "USER-DEFINED-TRANSFORMATION-DESCRIPTION", "CLIENT-ID-DEFINITION-SET-REFS", "IDSM-QUALIFIED-EVENT-RECEIVER-MAPPING", "ASSIGNED-PORTS", "CRC-SOURCE-PORT-IDENTITY", "REQUIRED-AR-ELEMENT-REF", "RESOURCE-GROUP-REFS", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "IEEE-1722-TP-IIDC-CONNECTION", "HEALTH-CHANNEL-EXTERNAL-STATUS", "LEAF-ELEMENT-IREF", "VIEW-MAP-SET", "CS-ERROR-REACTION", "DATA-TYPE-MAPPING-REF", "MODE-DECLARATION-REF", "ORDERED-MASTER", "SW-VERSION", "TP-SDU-REFS", "VIEW-MAP", "ACL-OPERATION-REFS", "PNC-WAKEUP-DLC", "MAX-BUFFER-SIZE", "RATIO-KIND", "REMOTE-SUBJECT-REFS", "DEFAULT-PERIOD-END", "OWNERSHIP-KIND", "ENABLE-CONDITION-GROUP-REF", "DEPLOYMENT-URIS", "NM-BUSLOAD-REDUCTION-ENABLED", "WAKE-UP-BY-CONTROLLER-SUPPORTED", "EXECUTABLE-ENTITY-REFS", "NM-PDU-REF", "USER-REFS", "TLS-CONNECTION-REFS", "SIGNAL-BASED-METHOD-REF", "LITERAL-PREFIX", "SECURITY-EVENT-DEFINITION-REF-CONDITIONAL", "REFERRABLE-REF", "GETTER-CALL-SIGNAL-REF", "DDS-SERVICE-INSTANCE-OPERATION-CP", "EXTERNAL-SYNC", "SYNCHRONIZED-TRIGGERS", "CLIENT-ID-DEFINITIONS", "FLOW-LABEL", "LENGTH-SEQUENCE-COUNTER", "LIVELINESS-LEASE-DURATION", "SW-AXIS-INDIVIDUAL", "DETERMINISTIC-SYNC-MASTER-TO-TIME-BASE-CONSUMER-MAPPING", "SEPARATION-TIME-RESPONSE", "TIME-FAILED-THRESHOLD", "APMC-TEXTUAL-PARAM-VALUE", "TRANSFORMATION-I-SIGNAL-PROPSS", "REQUIRED-PACKAGE-ELEMENT-REFS", "NM-ACTIVE-COORDINATOR", "NM-SHUTDOWN-DELAY-TIMER", "RESTRICTION", "PDU-REPLICATION-VOTING", "SOMEIP-EVENT", "MESSAGE-TYPE-INFO", "IPV-6-ADDRESS-MASK", "PERSISTENCY-FILE-STORAGE-INTERFACE", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "DDS-METHOD-DEPLOYMENT", "INVOCATION", "CAN-ID-RANGE-MAPPINGS", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "DIAGNOSTIC-SERVICE-TABLE", "PER-STATE-TIMEOUT", "METHOD-REPLY-TOPIC-NAME", "SOFTWARE-CLUSTER-DESIGN-REF", "VARIANT-CRITERION-REF", "SERVER-CALL-POINTS", "SWC-MODE-SWITCH-EVENT", "ECUC-CONTAINER-VALUE", "DIAGNOSTIC-OPERATION-CYCLES", "SENDER-REC-RECORD-TYPE-MAPPING", "LDAP-URL", "AUTH-ROLE-REFS", "TD-EVENT-FRAME-ETHERNET", "LIFESPAN-DURATION", "DATA-RECEIVE-ERROR-EVENT", "GENERIC-MATH", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "FIRST-ELEMENT-REF", "NETWORK-INTERFACE-REFS", "SOVD-OPERATION-REF", "USE-AS-CRYPTOGRAPHIC-I-PDU", "CRYPTO-JOB-REF", "BASE-TYPE-REF", "DLT-CONTEXT-REF-CONDITIONAL", "GLOBAL-TIME-DOMAINS", "SUB-NODE-NUMBER", "SHORT-LABEL", "TCP-DELAYED-ACK-TIMEOUT", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "SWC-MODE-SWITCH-EVENT-REF", "PAYLOAD-BYTE-PATTERN-RULES", "REQUESTER-REFS", "MAC-MULTICAST-GROUP-REF", "REST-OBJECT-REF", "SW-AXIS-INDEX", "TCP-TP-PORT", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "PDUS", "TOOL", "SWITCH-ATS-INSTANCE-ENTRY", "SCALE-CONSTR", "COMPU-METHOD", "DESTINATION-CONTEXT", "AAF-AES-3-DATA-TYPE", "SENDER-INTENT", "SYNC-DATA-ID-LISTS", "FIREWALL-RULE-REF", "SWITCH-MAC-ADDRESS-LEARNING-MODE", "REFILL-AMOUNT", "RESOURCE-GROUP", "ICMP-V-6-PROPS", "FORMAL-BLUEPRINT-CONDITION", "INTERFACE-VERSION", "FUNCTION-ID", "STATUS-BIT-HANDLING-TEST-FAILED-SINCE-LAST-CLEAR", "TIME-TRIGGERED-CAN-LEVEL", "INCLUDED-MODE-DECLARATION-GROUP-SET", "LOCAL-IP-SEC-RULE-REFS", "DATA-CONSTR", "FLEXRAY-NM-NODE", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP-REF-CONDITIONAL", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "IE", "ECUC-REFERENCE-VALUE", "APMC-IP-V4-ADDRESS-PARAM-DEF", "SENSOR-INSTANCE-ID", "TCP-IP-DHCP-V-6-SOL-DELAY-MAX", "NETWORK-INTERFACE-ID", "PROCESS-PHM-ACTION-ITEM", "ALL-NM-MESSAGES-KEEP-AWAKE", "CONDITION-ACCESS", "REQUIRED-ENVIRONMENT-REF", "OPERATION-REQUEST-TRIGGERING-REF", "CRYPTO-KEY-SLOT-DESIGN", "GRANT-REFS", "SW-DATA-DEF-PROPS-VARIANTS", "ASSEMBLY-SW-CONNECTOR", "RPT-EVENT-ID", "TCP-IP-ICMP-V-6-ECHO-REPLY-ENABLED", "MACROTICK-DURATION", "DIAGNOSTIC-SOVD-SERVICE-VALIDATION-INTERFACE", "TRACE", "IDENTITY-REF", "PARAMETER-SW-COMPONENT-TYPE", "CUSTOM-SUB-FUNCTION-NUMBER", "FLEXRAY-CLUSTER-CONDITIONAL", "MODIFIED-DATAS", "TRANSFORMER-CLASS", "DIAGNOSTIC-UPLOAD-INTERFACE", "MSR-QUERY-NAME", "PORT-PROTOTYPE-IREF", "TIMESTAMP-INTERVAL", "REQUEST-SEED-ID", "CONTEXT-ID", "MACRO-INITIAL-OFFSET-A", "DIAGNOSTIC-OPERATION-CYCLE-REF-CONDITIONAL", "SEC-OC-COM-PROPS-FOR-MULTICAST-REFS", "REQUIRED-ENTRYS", "ECU-INSTANCES", "ECUC-DEFINITION-REF", "TP-ADDRESSS", "PROG-CODE", "CRYPTO-KEY-MANAGEMENT-NEEDS", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "SEARCH-INTENTION", "CAN-CONTROLLER-XL-REQUIREMENTS", "PRE-ACTIVATE-REFS", "NETWORK-IDLE-TIME", "MODE-GROUP-IREF", "DEFAULT-LOG-THRESHOLD", "RUNTIME-PORT-CONFIGURATION", "ECUC-MULTILINE-STRING-PARAM-DEF", "RATE-DEVIATION-MEASUREMENT-DURATION", "BINARY-MANIFEST-PROVIDE-RESOURCE", "REQUIRED-PACKAGE-ELEMENT-REF", "DISABLE-END-TO-END-CHECK", "SUPERVISION-CYCLE", "REPOSITORY", "SYSTEM-COM-SPEC-DEFINITION-REFS", "SENDER-REC-ARRAY-ELEMENT-MAPPING", "FLOW-CONTROL-PDU-REF", "SOVD-SNAPSHOT-CONTENT-REF", "FACTOR-SI-TO-UNIT", "UCM-SUBORDINATE-MODULE-INSTANTIATION", "TRAFFIC-CLASS", "SOFTWARE-CONTEXT", "EVENT-GROUP-IDENTIFIER", "NONQUEUED-SENDER-COM-SPEC", "DIAGNOSTIC-TEST-RESULT", "ACCESS-COUNT", "SECONDARY-DOWNLOAD-URIS", "SD", "FUNCTION", "DO-IP-LOGIC-ADDRESSS", "DATA-COM-PROPS", "PHM-LOGICAL-EXPRESSION", "CONSUMED-DDS-SERVICE-EVENTS", "DIFFERENTIATED-SERVICE-CODE-POINT", "HW-CATEGORY-REF", "SEC-OC-CRYPTO-MAPPING-REF", "RETURN-SIGNAL-TRIGGERING-REF", "SOMEIP-FIELD", "SYSTEM-SIGNAL-TO-COM-RESOURCE-MAPPINGS", "SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF", "COM-OFFER-SERVICE-GRANT-DESIGN", "RATE-LIMITATION-FILTERS", "SWC-BSW-MAPPING-REF", "TRCV-PWM-MODE-ENABLED", "IMPLIED-OPERATION-REF", "PROCESS", "MODE-ELEMENT-REF", "DEBOUNCE-ALGORITHM-REF", "PROVIDE", "TCP-IP-NDP-NUM-MULTICAST-SOLICITATIONS", "INITIAL-FIND-BEHAVIOR", "A-2-L-DISPLAY-TEXT", "PAYLOAD-LENGTH", "CAN-BAUDRATE", "REPLICA-PDUS-REF", "NOTIFIER-SIGNAL-TRIGGERING-REF", "IDENTITY", "TABLE-ENTRYS", "FOLLOW-UP-TIMEOUT-VALUE", "TRANSFORMATION-COM-SPEC-PROPSS", "FIRE-AND-FORGET-METHOD-MAPPINGS", "MAXIMUM", "RECOVERY-NOTIFICATION", "RECEIVER-IREF", "NOTIFIED-STATUSS", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "FLEXRAY-PHYSICAL-CHANNEL", "SERVICE-ITEMS", "CALL-SIGNAL-REF", "COMMUNICATION-RESOURCE-REF", "SPN-REF", "DIAGNOSTIC-SESSION-CONTROL", "DIAGNOSTIC-ECU-RESET", "SECURITY-EVENT-CONTEXT-DATA-ELEMENT-REF-CONDITIONAL", "EXECUTABLE-ENTITY-REF", "IDS-MGR-NEEDS", "TIME-OFFSET", "NO-SUPERVISION", "EXPORTABILITY", "IMPOSITION-TIME", "INHIBITION-MASK", "IP-SEC-CONFIG", "ECUC-VALUE-CONFIGURATION-CLASS", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "DATA-PROTOTYPE-IN-METHOD-ARGUMENT-INSTANCE-REF-IREF", "RATE-CORRECTION-THRESHOLD", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "PROFILE-BEHAVIOR", "METHOD-CALL-REFS", "COMM-CONTROLLER-MAPPINGS", "PRIMITIVE-ATTRIBUTE-TAILORING", "LOG-CHANNEL-ID", "DISPLAY-FORMAT", "RECEIVER-REFS", "SERVICE-DISCOVERY-CONFIGS", "SW-DIAGNOSTICS-NOTESS", "SYSTEM-COM-SPEC-DEFINITION-SET", "SERVICE-INTERFACE-FIELD-MAPPING", "LENGTH-VERIFICATION", "ACCEPTED-REMOTE-CERTIFICATE-REF", "ACCESS-DATA-RECORD-SIZE", "V-LAN-REF", "SD-CLIENT-TIMER-CONFIGS", "SYNCHRONIZED-TIME-BASE-PROVIDER", "DISCOVERY-PROTECTION-KIND", "PN-RESET-TIMER", "DIAGNOSTIC-EXTENDED-DATA-RECORD-REF", "START-ADDRESS", "HW-PIN-GROUP", "NM-MSG-CYCLE-TIME", "DATA-TRANSFORMATION-REF", "EXTERNAL-AUTHENTICATIONS", "REQUIRES-INDEX", "DISABLED-MODE-IREFS", "MULTICAST-CONNECTOR-REFS", "REQUESTER-REF", "TRIGGER-MAPPING", "TARGET-RTE-EVENT-REF", "EVENT-ELEMENTS", "RPT-EXECUTABLE-ENTITY", "TCP-IP-TLS-USE-SECURITY-EXTENSION-FORCE-ENCRYPT-THEN-MAC", "BSW-MODULE-ENTITY-REF", "MICRO-INITIAL-OFFSET-A", "COM-HANDLER-TASK-MAPPING-ENABLED", "ERROR-SIGNALING-ENABLED", "BUS-ID", "WRITTEN-READ-NV-DATA", "OBJECT-DEFINTION-REFS", "INTENDED-PARTIAL-INITIALIZATION-COUNT", "VERBATIM", "DETERMINISTIC-CLIENT-RESOURCE-NEEDSS", "CONTEXT-DATA-PROTOTYPE-IN-CS-REF", "BUILD-TYPE", "DEADLINE", "PRODUCT-RELEASE", "DDS-OPERATION-REQUEST-TRIGGERING-REF", "MAP", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "TARGET-HEALTH-CHANNEL-REF", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "MODE-SWITCH-RECEIVER-COM-SPEC", "GLOBAL-SUPERVISION", "DOCUMENTATION-CONTEXT", "MAX-TRANSIT-TIME", "ERROR-DOMAIN-REF", "DERIVED-PORT-PROTOTYPE-REF", "COMPILERS", "FROM-REF", "COMPU-SCALES", "LABEL-1", "NODE-IDENTIFIER", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "ROOT-ARGUMENT-DATA-PROTOTYPE-REF", "PORT-PROTOTYPE-REF", "HISTORY-KIND", "NETWORK-TIME-SLAVE-REF", "READ-STATUS", "IEEE-1722-TP-ACF-CAN-PART", "TIMEOUT-BS", "CHECKSUM-VALUE", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "MANUFACTURER-VALIDATION-ORDER-REF", "DIAGNOSTIC-ENV-SOVD-DATA-CONDITION", "CRYPTO-SERVICE-PRIMITIVE", "DDS-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "DCM-BUS-TYPE", "ASSOCIATED-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "I-PDU-TRIGGERING-REF-CONDITIONAL", "REVISION-LABEL", "ABSOLUTELY-SCHEDULED-TIMINGS", "MAC-MULTICAST-CONFIGURATION", "METHOD-TOPICS-ACCESS-RULE-REF", "RESPONSE-ERROR-REF", "INITIALIZED-PARAMETER-REF", "MACHINE-MODE-IREF", "FM-FEATURE-SELECTION", "ELEMENT-BYTE-VALUE", "CRYPTO-CERTIFICATE-REF", "MODE-MAPPING", "TD-EVENT-COMPLEX", "TARGET-TYPE", "NETWORK-REPRESENTATIONS", "PDELAY-LATENCY-THRESHOLD", "STOP", "REQUEST-FILE-TRANSFER-CLASS-REF", "FLAT-INSTANCE-REF", "NESTED-ELEMENT-REFS", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "IKE-OVER-TIME", "APMC-URI-INSTANCE-REFERENCE-DEF", "SOVD-CONTENT-REFS", "LINKERS", "PSK-IDENTITY", "PORT-PROTOTYPE-BLUEPRINT", "MAXIMUM-TRANSMISSION-UNIT", "FEATURE-IREF", "MIN-MARGIN", "REG-DOES-NOT-REQUIRE-STABILITYS", "NAS-TIMEOUT", "NM-NODE-ID", "FILE-ELEMENTS", "DATA-PROTOTYPE-GROUP", "MIN-OK-STATE-VALID", "INTERNAL-BEHAVIORS", "RPT-WRITES", "VTF", "ENVIRONMENT-DATA-CAPTURE", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "SOURCE-IPV-4-ADDRESS", "FM-FEATURE-MAP", "CALL-TYPE", "OS-TASK-PROXY", "HEALING-CYCLE-COUNTER-THRESHOLD", "COUPLING-PORT-CONNECTION", "LOCAL-UDP-PORT-REF", "OFFSETS", "RAW-DATA-STREAM-GRANT-DESIGN", "DESTINATION-IPV-6-ADDRESS", "TCP-IP-IP-REASSEMBLY-SEGMENT-COUNT", "USED-IMPOSITION-TIME-DEFINITION-GROUP-REF", "AGING-REQUIRES-TESTED-CYCLE", "SECURITY-EVENT-FILTER-CHAIN", "REMOTE-ID", "REQUIRED-ARTIFACTS", "OS-MODULE-INSTANTIATION", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-DIAGNOSTIC-EVENT-MAPPING", "DIAGNOSTIC-ADDRESSES", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID-REF", "TTCAN-CLUSTER-CONDITIONAL", "DATA-ELEMENT-IREFS", "SECURE-COM-CONFIGS", "TLS-CRYPTO-CIPHER-SUITE", "FIRST-PHYSICAL-DIMENSION-REF", "ECUC-DEF-EDITION", "SECURITY-ACCESS-LEVEL", "DESTINATION-PORTS", "SOCKET-CONNECTION-BUNDLE", "UCMS", "K-INIT-RESOURCE", "REMOTE-PORT-RANGE-END", "APPLICATION-ERROR", "LATENCY-TIMING-CONSTRAINT", "DIAGNOSTIC-SESSION", "DYNAMIC-PART-ALTERNATIVE", "OUT-OF-RANGE-BITFIELD-ERRORS-ENABLED", "MODE-ACCESS-POINT", "PERSISTENCY-FILE-REQUIRED-COM-SPEC", "STATIC-SLOT-DURATION", "TCP-PORT-REF", "ANONYMOUS", "REST-SERVICE-INTERFACE", "HEALTH-CHANNEL-SUPERVISION", "LOGICAL-EXPRESSION-ARGUMENT-REFS", "ENTER-SUSPEND-TO-RAM-ERROR-REF", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "VEHICLE-ANNOUNCEMENT-COUNT", "HW-ELEMENT-REF-CONDITIONAL", "APMC-CONTAINER-REFERENCE-VALUE", "POST-BUILD-VARIANT-CRITERION-VALUE-SET-REF", "REFERENCED-TD-EVENT-VFB-REF", "RUNNABLE-MAPPINGS", "SERVICE-INTERFACE-METHOD-MAPPING", "CLIENT-PORT-REF", "SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "ARTIFACT-CHECKSUM-REF", "PDU-COLLECTION-PDU-TIMEOUT", "PERSISTENCY-FILE-STORAGE", "QUEUED-RECEIVER-COM-SPEC", "MAC-MULTICAST-GROUPS", "BSW-PARAMETER-POLICY", "TIMING-EVENT", "COMPU-DENOMINATOR", "TESTED-ITEM-REFS", "SW-RECORD-LAYOUT-V", "IMPLEMENTATION-RECORD-ELEMENT-REF", "RELATIVE-REPRESENTATION-TIME", "MAXIMUM-DYNAMIC-PAYLOAD-LENGTH", "RESOURCE-CONSUMPTION", "USED-PARAMETER-ELEMENT", "DDS-RPC-SERVICE-REF", "PROVIDED-COM-SPECS", "STATIC-REMOTE-UNICAST-ADDRESSES", "CONFIG-CLASS", "WAKEUP-RX-IDLE", "AFFECTED-REFS", "VALUE-TYPE-TREF", "CONTEXT-SW-COMPONENT-PROTOTYPE-REF", "NM-NETWORK-TIMEOUT", "EVENT-QOS-PROPSS", "REQUIRED-FIBEX-ELEMENT-REF", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "GLOBAL-TIME-CAN-MASTER", "APMC-ENUMERATION-PARAM-DEF", "RESOURCE-DEFINITIONS", "INTERPOLATION-ROUTINE-MAPPING-SET-REF", "ELLIPTIC-CURVE-REFS", "IS-SERVICE", "RATE-CORRECTION-MEASUREMENT-DURATION", "SW-RECORD-LAYOUT", "VARIABLE-DATA-PROTOTYPE", "SD-CLIENT-EVENT-TIMING-CONFIG", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "MICROTICK-DURATION", "STATE-CLIENT-INTERFACE", "INGRESS-VLAN-ID", "USER-DEFINED-EVENT-DEPLOYMENT", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "BLUEPRINT-POLICY-NOT-MODIFIABLE", "DIAGNOSTIC-DATA-IDENTIFIERS", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "CONTEXT-COMPOSITION-REF", "IS-PRIVATE", "SW-GENERIC-AXIS-PARAMS", "REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS-REF", "OPERATION-IREF", "ENUMERATOR-VALUE", "TCP-IP-NDP-PREFIX-LIST-SIZE", "CAPACITY", "TASK", "COMBINED-NO-DATA-INIT-COUNT", "NONQUEUED-RECEIVER-COM-SPEC", "NM-PDU-RX-INDICATION-ENABLED", "APMC-ENUMERATION-LITERAL-DEF", "SRVT", "LOGGING-BEHAVIOR", "PATH-MTU-TIMEOUT", "E-2-E-PROFILE-COMPATIBILITY-PROPS-REF", "APMC-FLOAT-PARAM-DEF", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "DEFAULT-VLAN-REF", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "SUBFUNCTIONS", "CRYPTO-KEY-SLOT-USAGE-DESIGN-REF", "OFFSET-CORRECTION-START", "HEALTH-CHANNEL-ID", "NM-READY-SLEEP-TIME", "SW-DATA-DEF-PROPS", "LOCAL-PORT-RANGE-END", "MAC-ADDRESS-VLAN-MEMBERSHIP", "BSW-ENTRY-RELATIONSHIP-TYPE", "MAX-NUMBER-FREEZE-FRAME-RECORDS", "SUPPORTED-CONFIG-VARIANTS", "ROOT-COMPOSITION", "ANNOTATIONS", "FIRST-MODE-REFS", "COUNTER-OFFSET", "CONTEXT-DATA-PROTOTYPE-IN-SR-REF", "POST-BUILD-VALUE-ACCESS-REF", "SECURITY-EVENT-CONTEXT-PROPS", "ECUC-CHOICE-CONTAINER-DEF", "DIAGNOSTIC-MULTIPLE-MONITOR-INTERFACE", "INVALIDATION-POLICYS", "SCOPE", "SECURITY-DELAY-TIME", "DIAGNOSTIC-SOVD-BULK-DATA-PORT-MAPPING", "FIREWALL-STATE-IREF", "PROVIDED-MODE-GROUPS", "PRECONFIGURED-CONFIGURATION-REF", "USE-AUTH-DATA-FRESHNESS", "APPLICATION-ENDPOINT-REF", "ACCURACY-EXT", "I-SIGNAL-REF", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "ENHANCED-MODE-API", "VEHICLE-DESCRIPTION-REF", "APP-ID", "ECU-TIMING", "DIAGNOSTIC-COMMON-ELEMENT-REF-CONDITIONAL", "ITEM", "ECUC-FOREIGN-REFERENCE-DEF", "LOG-AND-TRACE-INSTANTIATION", "TARGET-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "LOGIC-ADDRESSS", "COMMITTED-BURST-SIZE", "METHOD-RETURN-REFS", "REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS-REF", "MSR-QUERY-P-1", "SOURCE-FRAME-REF", "COMPONENTS", "METHOD-MAPPING", "RTE-EVENT-IREFS", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "DIRECT-SUCCESSOR-REFS", "PLCA-LOCAL-NODE-ID", "MAX-DATA-LENGTH", "PATTERN-LENGTH", "HW-CATEGORY", "DIAGNOSTIC-READ-DTC-INFORMATION", "MAX-ACTION-LIST-DURATION", "CAN-COMMUNICATION-CONTROLLER-VARIANTS", "SOURCE-SERVICE-INTERFACE-REFS", "MC-DATA-INSTANCE-VAR-REF", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "AUXILIARY-FIELDS", "DID-SIZE", "COUNTER-BASED-FDC-THRESHOLD-STORAGE-VALUE", "ALL-PHYSICAL-CHANNELS", "LAST-EGRESS-SCHEDULER-REF", "ROUTING-GROUP-REFS", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "MAX-DESTINATION-PORT-NUMBER", "MACHINE-MODE-MACHINES", "CAN-NM-CLUSTER", "MSR-QUERY-PROPS", "SEGMENT-BYTE-ORDER", "DESTINATION-ORDER", "PACKING-BYTE-ORDER", "RULE-BASED-VALUES", "CONDITION", "SIGNAL-BASED-METHOD-DEPLOYMENT", "CURRENT-EXP", "TIMING-MODE-REF", "OPERATOR", "EVENT-OBD-READINESS-GROUP", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "SUSPEND-TO-RAM-SATELLITE-MAPPING", "OPERATION-MODE", "VARIATION-POINT", "APMC-INSTANCE-REFERENCE-DEF", "BUFFER-COMPUTATION", "MAXIMUM-RECEIVE-BUFFER-LENGTH", "IP-ADDRESS-PREFIX-LENGTH", "COMMUNICATION-CYCLE", "EVENT-TOPIC-ACCESS-RULE-REF", "DURABILITY-SERVICE", "PROVIDER-REF", "TP-ECU", "TCP-OPTION-FILTER-LISTS", "DDS-OPERATION-RESPONSE-TRIGGERING-REF", "UNIQUE-STREAM-ID", "USE-CASE", "REST-ENUMERATOR-DEF", "ECUC-MODULE-CONFIGURATION-VALUES-REF-CONDITIONAL", "VEHICLE-SYSTEM", "SOMEIP-TRANSFORMATION-DESCRIPTION", "AUTOSAR-DATA-TYPE-REF-CONDITIONAL", "PN-RESET-TIME", "BSW-DISTINGUISHED-PARTITION", "PARAMETER-INTERFACE", "TP-CONNECTION-CONTROL-REF", "REQUIRED-OUTER-PORT-REF", "PARAMETER-POLICYS", "RESPONSES", "TCP-CONGESTION-AVOIDANCE-ENABLED", "SHORT-NAME-FRAGMENTS", "ECUC-VALUES", "PARAMETER-ELEMENT-ACCESS", "STANDARD-REVISIONS", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "DELEGATION-SW-CONNECTOR", "TCP-IP-IP-REASSEMBLY-BUFFER-COUNT", "LIN-UNCONDITIONAL-FRAME", "SW-AXIS-CONTS", "SOMEIP-SERVICE-VERSION", "DDS-TRANSFORMATION-DESCRIPTION", "APPROVAL-STATUS", "ECUC-CHOICE-REFERENCE-DEF", "SWC-IMPLEMENTATION", "MAPPED-FLAT-SWC-SERVICE-DEPENDENCY-REF", "STORE-EVENT-ENABLED", "VARIABLE-DATA-PROTOTYPE-IREFS", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NODE-MAPPINGS", "METHODS", "SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "TTL-MIN", "GETTER-CALL-REF", "APMC-CONTAINER-VALUE", "STIMULUS-REF", "TRAFFIC-LIMITATION-FILTERS", "IEEE-1722-TP-ACF-LIN-PART", "REST-ENDPOINT-POST", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "ACCESS-COUNTS", "VALIDATION-REF", "ETHERNET-RAW-DATA-STREAM-MAPPING-REF", "BSW-INTERNAL-TRIGGERING-POINT-REF", "KEY-STORAGE-TYPE", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "HW-COMMUNICATION-PORT-REF", "PARENT-CATEGORY", "PERIODIC-EVENT-TRIGGERING", "ENABLE-LIVELINESS-PROTECTION", "DESTINATION-NETWORK-MASK", "ASYNCHRONOUS-SERVER-CALL-POINT-REF", "PACKAGING-PIN-NAME", "READ", "SWC-BSW-SYNCHRONIZED-MODE-GROUP-PROTOTYPE", "IIDC-CHANNEL", "MAPPED-SECURITY-EVENTS", "CONTEXT-LIMITATION-REF", "REMOTE-CERTIFICATE-REF", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "ACTIVATE-AT-START", "CLIENT-SERVER-ANNOTATION", "UDS-DTC-VALUE", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "SWC-MODE-MANAGER-ERROR-EVENT", "INTERNAL-TRIGGERING-POINT", "PORT-BLUEPRINT-MAPPINGS", "CONSUMED-SERVICE-INSTANCE", "MAXIMUM-SEGMENT-LENGTH", "MAX-RESIDENCE-TIME", "ACK-REQUEST", "DATA-PROTOTYPE-GROUP-IREF", "TCP-IP-ICMP-PROPS-REFS", "SENDER-TO-SIGNAL-TEXT-TABLE-MAPPING", "HW-ATTRIBUTE-DEFS", "WAKEUP-SYMBOL-RX-WINDOW", "I-SIGNAL-TRIGGERING-REF", "SOMEIP-RULE", "TIMEOUT-SUBSTITUTION-VALUE", "NM-PDU-REF-CONDITIONAL", "COMPU-INTERNAL-TO-PHYS", "IS-MANDATORY", "APMC-URI-INSTANCE-REFERENCE-VALUE", "PROVIEDE-DATA-REF", "SECTION-NAME-PREFIX", "JOB-REQUIREMENTS", "SERVICE-PROXY-SW-COMPONENT-TYPE", "ECUC-VALIDATION-CONDS", "BAM-TIMING", "DATA-PROTECTION-KIND", "CONSUMED-SERVICE-INSTANCE-REF-CONDITIONAL", "SERVICE-TABLES", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "DO-IP-ROUTING-ACTIVATION", "ASSIGNED-FRAME-TRIGGERING-REF", "I-PDU-PORT", "TRACE-SWITCH", "PERSISTENCY-CENTRAL-STORAGE-URI", "PHM-HEALTH-CHANNEL-STATUS", "INPUT", "ECUC-QUERYS", "DIAGNOSTIC-ENABLE-CONDITION-REF-CONDITIONAL", "AGGREGATION-REF", "PERMISSIBLE-SIGNAL-PATH", "CAN-FRAME", "FUNCTION-GROUP-STATE-IREF", "TCP-IP-NDP-SLAAC-DAD-NUMBER-OF-TRANSMISSIONS", "NETWORK-HANDLE-REF", "CONTROL-PNC-REF", "SW-MAINTENANCE-NOTES", "CRC-SECURED", "MINIMUM-SUPPORTED-UCM-MASTER-VERSION", "DIRECT-TP-SDU-REF", "TIME-SYNC-CLIENT", "E-2-E-CRC-ERROR-REF", "VERIFICATION-HASH", "SWITCH-STREAM-GATE-ENTRY", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "WRITE-VERIFICATION", "I-PDU-IDENTIFIER-TCP-REFS", "BLOCK-IF-STATE-ACTIVE-CP-REFS", "COUNTER-JUMP-UP-VALUE", "BLUEPRINT-MAPS", "TP-CONNECTION-REFS", "GID", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "BSW-MODE-RECEIVER-POLICY", "ENCAPSULATED-ENTRY-REF", "SAK-REKEY-TIME-SPAN", "SERVICE-INSTANCE-PORT-CONFIG", "PARTITIONS", "SOVD-SERVER-INSTANTIATION", "FLEXRAY-FIFOS", "STORE-AT-SHUTDOWN", "SOMEIP-FIELD-DEPLOYMENT", "CLEAR-FROM-VALID-TO-INVALID", "CRYPTO-SERVICE-KEY-REF", "TEMPLATE-ARGUMENTS", "IS-AUTO-VALUE", "STARTUP-OPTIONS", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "TCP-IP-IP-TX-FRAGMENT-BUFFER-SIZE", "ECU-INSTANCE-PROPSS", "WRITE", "VFB-TIMING", "FIRST-TIMEOUT", "SEC-OC-DEPLOYMENT", "SIGNATURE-SUPPORT-CP", "DATA-CONSTR-RULES", "LOWER-CAN-ID", "TYPE", "BSW-MODULE-ENTRY", "MAC-MULTICAST-GROUP", "SHORT-NAME-FRAGMENT", "IDS-PLATFORM-INSTANTIATION-REF", "REMOTE-NETWORK-ENDPOINT-REF", "RECOVERY-VIA-APPLICATION-ACTION-REF", "ETH-GLOBAL-TIME-MANAGED-COUPLING-PORT", "PNC-FILTER-ARRAY-MASK", "DIAGNOSTIC-EVENT-REF", "SW-RECORD-LAYOUT-GROUP-STEP", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "COND", "CLEAR-DIAGNOSTIC-INFORMATION-CLASS-REF", "PORT-ELEMENT-TO-COM-RESOURCE-MAPPINGS", "SPECIFICATION-SCOPE", "V-2-X-DATA-MANAGER-NEEDS", "PNC-PDUR-GROUP-REF", "DIAGNOSTIC-ACCESS-PERMISSION", "TIME-VALUE", "P-2-TIMING", "CIPHER-SUITE-PRIORITY", "SOURCE-METHOD-REF", "AR-PACKAGES", "PNC-MAPPING-REF", "SWC-TO-APPLICATION-PARTITION-MAPPING", "RESOURCE-GUARD-VALUE", "PSK-IDENTITY-HINT", "MINIMUM-MULTICAST-SEPERATION-TIME", "PROCESS-ARGUMENTS", "TIMING-GUARANTEES", "TRIGGERS", "MEMORY-RANGE-REF", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "SYNCHRONIZATION-TIMING-CONSTRAINT", "CONTAINER-TRIGGER", "PARAMETER-REF", "SYMBOLIC-NAME-VALUE", "CRYPTO-SERVICE-CERTIFICATE-REFS", "ARGUMENT-MAPPINGS", "PDU-REPLICATIONS", "VLAN-MODIFIER-REF", "BIT-POSITION", "SENDER-RECEIVER-COMPOSITE-ELEMENT-TO-SIGNAL-MAPPING", "NM-REPETITION-CYCLE", "READ-SCALING-DATA-CLASS-REF", "MAC-ADDRESS-VLAN-ASSIGNMENTS", "TRIGGER", "SWC-MAPPING-CONSTRAINT-REFS", "SECOND-MODE-REF", "STATUS-REF", "FIELD-MAPPINGS", "BSW-DEBUG-INFO", "CONTAINED-PDU-TRIGGERING-REF", "VARIABLE-ACCESSED-FOR-DEBUG-REF", "UDP-TP", "IPV-6-MULTICAST-IP-ADDRESS", "TCP-FAST-RETRANSMIT-ENABLED", "CONTEXT-COMPONENT-PROTOTYPE-REF", "FILE-TYPE-PATTERN", "MAC-SEC-PARTICIPANT-SET", "MAX", "BASE-FREQUENCY", "IS-FUNC-CLUSTER-ENDPOINT-REF", "SOCKET-ADDRESS-REF-CONDITIONAL", "J-1939-TP-CONFIG", "DIAGNOSTIC-SERVICE-INSTANCE-REF", "ECU-PARTITION-TO-CORE-MAPPING", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "STATE-MANAGEMENT-REQUEST-TRIGGER", "LET-INTERVAL-REFS", "SELECTIONS", "EXTERNAL-CLOCK-SYNCHRONISATION", "SYSTEM-SIGNAL-GROUP-REF", "PARTITION-REF", "API-SERVICE-PREFIX", "J-1939-NM-CLUSTER", "COMPOSITE-NETWORK-REPRESENTATIONS", "CONSTANT-SPECIFICATION-MAPPING-SET", "DECODING-CORRECTION", "DIAGNOSTIC-EVENT-INFO-NEEDS", "UDP-TP-PORT", "ACCESSED-MODE-GROUPS", "IN-SCOPE", "RTE-EVENT-IREF", "MODULE-INSTANTIATION-REFS", "DETERMINISTIC-CLIENT-REF", "COUPLING-PORT-SPEED", "WATCHDOG-ACTION-ITEM", "STATE-DEPENDENT-STARTUP-CONFIG", "CP-SOFTWARE-CLUSTER", "ECUC-LINKER-SYMBOL-DEF-CONDITIONAL", "EMPTY-SIGNAL", "DO-IP-LOGICAL-ADDRESS-REF", "DISPLAY-IDENTIFIER", "FLEXRAY-FRAME", "LOWER-LIMIT", "VARIABLE-INSTANCE-IREF", "ASSIGN-FRAME-ID", "EXECUTION-CONTEXT", "WATCHDOG-PHM-ACTION-ITEM", "AR-RELEASE-VERSION", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "RPT-EXECUTABLE-ENTITYS", "MAX-PWM-L", "NV-BLOCK-DATA-MAPPINGS", "DIAGNOSTIC-TROUBLE-CODE-REF", "SOURCE-SERVICE-INTERFACE-REF", "METHOD-QOS-PROPSS", "INTERNAL-TRIGGERING-POINT-POLICYS", "DEBOUNCE-COUNTER-STORAGE", "BULK-DATA-CATEGORY", "TD-EVENT-OPERATION", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPINGS", "L-10", "AUTHENTICATION-VERIFY-ATTEMPTS", "NO-SUPERVISIONS", "DESC", "TTCAN-CLUSTER", "DLT-CONTEXT-REF", "AUTHENTICATION-PROPSS", "REQUESTOR-REF", "APMC-MAC-ADDRESS-PARAM-DEF", "EXECUTE-DESPITE-DATA-UNAVAILABILITY", "CREATED-DATAS", "AUTOSAR-VARIABLE-IREF", "SETTER-CALL-SIGNAL-REF", "MODE-DECLARATION-MAPPING-SET-REF", "INDEXED-ARRAY-ELEMENT", "NM-CONTROL-BIT-VECTOR-ENABLED", "HANDLE-DATA-STATUS", "CYCLE-COUNT-MAX", "REPRESENTED-PORT-GROUP-REF", "SUB-SDG-REF", "APMC-REVISION-LABEL-PARAM-VALUE", "SCALING-INFO-SIZE", "SUPPORT-BUFFER-LOCKING", "MAPPING-REF", "TEXT", "PORT-PROTOTYPE-REF-CONDITIONAL", "DIAGNOSTIC-SOVD-CONFIGURATION-PORT-MAPPING", "EXPLICIT-WAKEUP-PNCS", "BSW-INTERRUPT-ENTITY", "CONTEXT-ROOT-COMPOSITION-REF", "DLT-ECU-REF", "PADDING-VALUE", "RESOURCE-NEEDS-REFS", "TCP-IP-ICMP-V-6-MSG-DESTINATION-UNREACHABLE-ENABLED", "SECURE-COM-CONFIG-TO-KEY-SLOT-MAPPINGS", "IEEE-1722-ACF-BUS-PART-RAW-DATA-STREAM-CONSUMER-MAPPING", "RTPS-PROTECTION-KIND", "EXT-HEADER-FILTER-LISTS", "TARGET-PHM-SUPERVISED-ENTITY-IREF", "IDSM-CONTEXT-PROVIDER-INTERFACE", "DATA-REF", "FMI", "WRITE-ONLY-ONCE", "SIGNAL-AGE", "FIGURE", "INDICATOR-REF", "CAN-TP-ADDRESS", "BSW-MODE-GROUP-REF", "ACTIVATION-POINTS", "MC-DATA-ACCESS-DETAILS", "INPUT-VARIABLE-TYPE-REF", "HEALING-CYCLE-REF", "EVENT-GROUP-REF", "TCP-IP-NDP-NEIGHBOR-UNREACHABILITY-DETECTION-ENABLED", "EXTERNAL-TRIGGERING-POINT", "REMOTE-CERTIFICATE-REFS", "DEF-LIST", "PHYSICAL-CHANNEL-REF-CONDITIONAL", "PNC-IDENTIFIER", "SERVICE-INTERFACE-REFS", "MAX-CYCLES", "COMMUNICATION-CONNECTOR-REF-CONDITIONAL", "I-PDU-TRIGGERING-REF", "RESOURCE-LIMITS", "INITIALIZATION-VECTOR-LENGTH", "DIAGNOSTIC-SERVICE-SW-MAPPING", "CONNECTIONS", "ECU-RESOURCE-MAPPINGS", "REQUIRES-SYMBOLIC-NAME-VALUE", "DDS-METHOD-REPLY-TOPIC-REF", "DIAGNOSTIC-TEST-RESULT-REF", "SELECTOR-FIELD-CODE", "COMPOSITE-SERVICE-INTERFACE-REF", "CONTEXT-P-PORT-PROTOTYPE-REF", "INDENT-SAMPLE", "SHARED-PARAMETERS", "SUPERVISED-ENTITY-ID", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "CLIENT-SERVER-PORTS", "BLOCK-STATE", "TLS-JOB-SEMANTIC", "LIN-SLAVES", "ENTRY-TYPE", "RELEASE-NOTES-REF", "ST-MIN", "DELAY", "RESOURCE-GROUP-REF", "PAYLOAD-PREAMBLE-INDICATOR", "CIPHER-SUITE-NAME", "SOCKET-ADDRESSS", "STATE-DEPENDENT-FIREWALL", "RUNS-INSIDE-EXCLUSIVE-AREA-REF", "STARTUP-CONFIGS", "CAN-CLUSTER", "INITIAL-DELAY-MIN-VALUE", "I-SIGNAL", "REFINED-EVENT-IREF", "APPL-CONSTANT-REF", "IN-MEASUREMENT-SET", "WINDOW-SIZE-INVALID", "OPERATION-RESPONSE-TRIGGERING-REF", "SENSOR-ACTUATOR-REF", "PURE-LOCAL-TIME-BASE", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPINGS", "FUNCTIONAL-CLUSTER-TO-DLT-LOG-SINK-MAPPING", "ADDITIONAL-INFORMATION", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "PAYLOAD-BYTE-PATTERN-RULE", "DIAGNOSTIC-SOVD-UPDATE-INTERFACE", "P-PORT-PROTOTYPE-IREF", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "UNIT-AND-SCALING-ID", "SYSTEM-SIGNAL-GROUP-TO-COM-RESOURCE-MAPPINGS", "DLT-MESSAGE-REF", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "SYSTEM-SIGNAL-TO-COMMUNICATION-RESOURCE-MAPPING", "DLT-LOG-SINK-REF", "EVENT-GROUP-REFS", "SWC-FLAT-SERVICE-DEPENDENCY-REF", "SOURCE-SIGNAL-REF", "TRIGGER-DIRECT-IMPLEMENTATIONS", "INPUT-DATAS", "FILE-STORAGE-REF", "MODULE-ID", "VALUE-ACCESS", "SWC-MODE-GROUP-IREF", "SUBNET-NUMBER", "MAXIMUM-INTER-ARRIVAL-TIME", "USER-DEFINED-TRANSFORMATION-I-SIGNAL-PROPS-CONDITIONAL", "DLT-LOG-CHANNELS", "ECUC-LINKER-SYMBOL-DEF", "SESSION-HANDLING-SR", "IIDC-STREAM-FORMAT", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "DIAGNOSTIC-STORAGE-CONDITION", "BSW-MODULE-DESCRIPTION-REF", "LIN-MASTER-VARIANTS", "MC-FUNCTION-REF", "CLUSTERED-COMPONENT-IREF", "AFFECTED-FUNCTIONAL-CLUSTER", "NUMBER-OF-NOTIFIER-SETS", "PHM-SUPERVISED-ENTITY-INTERFACE", "SHALL-NOT-RUN-ON-REFS", "SPN", "RESISTANT-TO-CHANGED-SW", "METADATA-PROTECTION-KIND", "SEGMENT-RECEPTION-TIMEOUT-TIME-REQUEST", "EXPIRED-SUPERVISION-TOLERANCE", "SOVD-CONTENT-REF", "HISTORY-ORDER-DEPTH", "CONFIG-VARIANT", "CHECKPOINT-IREF", "BSW-RELEASED-TRIGGER-POLICY", "RX-MASK", "TEAR-DOWN-ACTION-REFS", "CODE-LABEL", "READER-ENTITY-ID", "DEVELOPMENT-ERROR", "ECUC-ENUMERATION-PARAM-DEF", "DIAGNOSTIC-COM-CONTROL", "COM-MANAGEMENT-PORT-GROUP-IREF", "SW-BIT-REPRESENTATION", "COM-METHOD-GRANT", "DIAGNOSTIC-MULTIPLE-CONDITION-INTERFACE", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-SYSTEM-REF", "HEADER-FILE", "LOCAL-SUPERVISION-REF", "EVALUATED-VARIANT-SET", "CRYPTO-SERVICE-QUEUE-REF", "OFFSET-CORRECTION-ADAPTION-INTERVAL", "MEMORY-ALLOCATION-KEYWORD-POLICY", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "DEPENDENCIES", "SEPARATION-TIME-REQUEST", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "DO-IP-ENTITY-ROLE", "ARRAY-SIZE", "SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPINGS", "TARGET-VARIABLE-ACCESS-REF", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "MINIMUM-SUSTAINED-SIZE", "OBJECT-DEFINITION-REF", "REQUIRED-MODE-GROUP-REF", "TEST-ID", "DIAGNOSTIC-SOVD-ACCESS-ARGUMENT", "IMPLEMENTATION-CONFIG-CLASSES", "AUTO-COLLECT", "OFFSET-SI-TO-UNIT", "DO-IP-GID-NEEDS", "NUMBER-OF-RESTART-ATTEMPTS", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "WAKEUP-REPETITION-DELAY-OF-WAKEUP-REQUEST", "J-1939-TP-NODE", "PHYSICAL-REQUEST-REF", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "SERVICE-DEPLOYMENT-REF", "RETURN-SIGNAL-REF", "OPTION-KIND", "PRIMITIVE-FAMILY", "SD-CLIENT-CONFIG-REF", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "PGN", "PORT-DEFINED-ARGUMENT-BLUEPRINT", "KEY-EXCHANGE-REF", "FM-COND", "TP-PROTOCOL-TYPE", "CLASSIFICATIONS", "NUMBER-OF-PARALLEL-TCP-SESSIONS", "SECURE-COM-PROPSS", "IIDC-SY", "MODE-DECLARATION-GROUP-PROTOTYPE-IREF", "MAX-RPT-EVENT-ID", "DEF", "IS-DYNAMIC-LENGTH-FIELD-SIZE", "EVENT-GROUPS", "CONTAINER-I-PDU", "LIN-TP-N-SDU-REF", "TRIGGER-PORT-ANNOTATIONS", "APPROVAL-REQUIRED", "SECURITY-EVENT-FILTER-CHAIN-REF", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "J-1939-NODE-REF", "COUNTER-INCREMENT-STEP-SIZE", "USED-CODE-GENERATOR", "TOLERATED-FAILED-CYCLES", "COLLECTED-INSTANCE-IREFS", "STREAM-FILTER-RULE", "PRIORITYS", "TARGET-R-PORT-PROTOTYPE-REF", "DIAGNOSTIC-EVENT-WINDOW", "TCP-IP-IP-FRAGMENTATION-RX-ENABLED", "CAN-XL-PROPS", "DATA-TYPE-MAPPING-REFS", "TIME-SYNC-PORT-PROTOTYPE-IREF", "REQUEST-CONFIGURATIONS", "PROCESS-EXECUTION-ERROR-REF", "IIDC-DATA-BLOCK-SIZE", "AFFECTED-REF", "DEADLINE-SUPERVISIONS", "EOC-EVENT-REF", "SOFTWARE-CLUSTER-UDS-DIAGNOSTIC-ADDRESS", "CERTIFICATE-GROUP-REF", "CRC-MESSAGE-LENGTH", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "SOFTWARE-CLUSTER-ID", "VERIFY-UPDATE", "ACCESSED-PARAMETER", "UPPER-HEADER-BITS-TO-SHIFT", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "SYSTEM-MEMORY-USAGE", "RPT-HOOK-ACCESS", "PRE-ACTIVATE-REF", "RESOURCE-NEEDS-REF", "SECURED-RX-VERIFICATION", "MAX-COMM-MODE", "V", "APMC-CONTAINER-REFERENCE-DEF", "RVF-PIXEL-DEPTH", "DEF-CALPRM-SET", "APMC-CHOICE-CONTAINER-DEF", "TARGET-PORT-PROTOTYPE-REF", "FREEZE-FRAME-CONTENT-REF", "PERIOD", "CONFLICTS", "PROVIDED-DDS-SERVICE-INSTANCE-EVENTS", "TERMINATION-BEHAVIOR", "TRANSMISSION-ACKNOWLEDGE", "DO-IP-CONFIG", "PER-INSTANCE-MEMORYS", "PARAMETER-ACCESSED-FOR-DEBUG-REFS", "SOURCE-ADDRESS-MASK", "THEAD", "DATA-FORMAT-TAILORING", "SUB-TYPE", "REFERENCE-BASE", "NM-CONFIG", "INSTANTIATION-DATA-DEF-PROPSS", "EXECUTION-DEPENDENCY", "MSR-QUERY-RESULT-TOPIC-1", "TCP-KEEP-ALIVE-TIME", "PROXIMITY-PROOF-REQUIRED", "SENDING-NODE-REFS", "ALIAS-NAME-SET", "STATE-MANAGEMENT-SLEEP-ACTION-ITEM", "CUSTOM-SPECIFICATION-REFS", "IUMPR-REFS", "NOTIFIER", "CONTEXT-MODE-DECLARATION-GROUP-PROTOTYPE-REF", "IMPL-INIT-VALUE", "TARGET-SIGNAL-REF", "SCHEDULING-POLICY", "RELEASED-TRIGGER-POLICYS", "STATE-MANAGEMENT-REQUEST-ERROR", "SOURCE-I-PDU-REF", "VARIABLE-LENGTH", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "DDS-FIELD-QOS-PROPS", "ENCRYPTION-REF", "DOCUMENTATION-CONTENT", "CONTROL-PNC-REFS", "SERVICE-MINOR-VERSION", "REQUIREMENT-REF", "DATA-RECEIVE-POINTS", "MAPPED-SWC-SERVICE-DEPENDENCY-IREF", "ARGUMENT-DATA-PROTOTYPE", "LATEST-TX", "OPERATION-CYCLE-AUTOSTART", "REQUEST-VEHICLE-INFORMATION-CLASS-REF", "ERROR-REACTION-POLICY", "MC-DATA-INSTANCE", "SHARED-TIMER", "MAX-SLOTS-PER-CYCLE", "STATE-MANAGEMENT-ERROR-COMPARE-RULE", "DESTINATION-MIN-ADDRESS", "NM-PNC-PARTICIPATION", "IP-SEC-IAM-REMOTE-SUBJECT", "ECU-INSTANCE-PROPS", "PHYSICAL-DIMENSION-MAPPING-SET", "ENABLE-CONDITIONS", "SD-SERVER-EVENT-CONFIG", "TIME-MARK", "KEY-EXCHANGE-AUTHENTICATION-REF", "TEMPERATURE-EXP", "CP-SOFTWARE-CLUSTER-TO-APPLICATION-PARTITION-MAPPING", "HW-CATEGORY-REFS", "TLV-APP-RECORD-ELEMENT-IN-METHOD-IREF", "METHOD-RETURN-REF", "PARTIAL-NETWORK-REFS", "OBD-DTC-NUMBER", "PROTOCOL", "DIAGNOSTIC-EVENT-REF-CONDITIONAL", "TD-EVENT-TRIGGER", "DTC-SETTING-CLASS-REF", "PERSISTENCY-REDUNDANCY-HASH", "APPLICATION-CONTEXT-REF", "META-DATA-ITEM-TYPE", "EVENT-ID", "STATIC-PART", "BSW-TRIGGER-DIRECT-IMPLEMENTATION", "SD-CLIENT-CONFIG", "REFERENCES", "SECURITY-EVENT-CONTEXT-DATA", "DO-IP-ENTITY", "PHM-CHECKPOINT-IREF", "ACTIVE-SUPERVISION-REF", "SW-VALUE-CONT", "MODE-DECLARATION-GROUP-REFS", "UDP-CHECKSUM-HANDLING", "REST-ELEMENT-DEF", "REMOTE-IP-ADDRESS-REF", "START-ACTION-REF", "ALLOWED-SERVICE-PROVIDERS", "DDS-DOMAIN-RANGE", "N", "TD-EVENT-VFB-PORT-REFS", "XDOC", "VALIDATION-ROOT", "CONTEXT-BSW-IMPLEMENTATION-REF", "ACTION-TYPE", "ABSOLUTE-TOLERANCE", "SIGNAL-PATH-CONSTRAINTS", "LIN-TP-CONFIG", "ACCESSED-VARIABLE", "CONNECTION-IS-MANDATORY", "VARIATION", "COMPU-RATIONAL-COEFFS", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "SWC-BSW-MAPPING", "CRC-VALIDATED", "SECURE-COM-PROPS-FOR-DDS-REF", "STATE-MANAGEMENT-STATE-NOTIFICATION", "CRYPTO-KEY-SLOT-CONTENT-ALLOWED-USAGE", "PRECONFIGURED-CONFIGURATION-REFS", "CLAIMED-FUNCTION-GROUP-REF", "NM-IF-ECUS", "INTERNAL-TRIGGER-OCCURRED-EVENT", "CONTEXT-TYPE-LISTS", "LOCAL-IP-SEC-RULE-REF", "BUS-DEPENDENT-NM-ECUS", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "MAX-DEADLINE", "LOWER-MULTIPLICITY", "DEBUG-INFOS", "ARGUMENTS", "PNC-MAPPING-REFS", "SECURE-PROPS-FOR-UDP-REF", "J-1939-CLUSTER-VARIANTS", "CONTAINERS", "DIAGNOSTIC-DATA-TRANSFER", "PERSISTENCY-DATA-ELEMENT", "DDS-CP-SERVICE-INSTANCE-EVENT", "REQUIRED-CLIENT-SERVER-ENTRY-REF", "DOIP-CHANNEL-COLLECTION-REF", "BUS-MIRROR-CHANNEL-MAPPING-IP", "POST-BUILD-VARIANT-VALUE", "TT-CAN-CLUSTER-REF", "KEY-SIZE", "I-PDU-PORT-REF", "REMOTE-UNICAST-ADDRESSS", "APPLICATION-INTERFACE", "ARRAY-VALUE-SPECIFICATION", "INTERNAL-TRIGGERING-POINTS", "CRYPTO-NEED", "STATE-MANAGEMENT-TRIGGER-COMPARE-RULE", "NETWORK-CONFIGURATION-DESIGN-REF", "ON-CHANGE-DATA-PROTOTYPES", "CAN-XL-CONFIG-REQS", "CDD-TYPE", "GLOBAL-SUPERVISIONS", "MESSAGE-LINE-NUMBER", "ECU-PARTITION", "INTERNAL-PRIORITY-VALUE", "ON-FAIL-PERMISSIVE-MODE", "ACCESS-PERMISSION-REF", "REQUEST-RESPONSE-DELAY", "UNICAST-NETWORK-ENDPOINT-REF", "REQUESTS", "SECURE-COM-PROPS-FOR-UDP-REF", "FM-FEATURE-SELECTION-SET", "VLAN-MEMBERSHIPS", "ROOT-REF", "NM-IMMEDIATE-NM-CYCLE-TIME", "DDS-EVENT-TOPIC-REF", "ASYNCHRONOUS-TRAFFIC-SHAPER-REF", "AP-SOMEIP-TRANSFORMATION-PROPS", "CAN-CONFIG", "SESSION-HANDLING", "ARTIFACT-CHECKSUM-REFS", "DEST-SDG-REF", "PREVIOUS-CONTRACT-VERSION", "TROUBLE-CODE-J-1939-REF", "EVENT-DEPLOYMENTS", "CRYPTO-JOB", "METHOD-ID", "TRANSFER-REF", "P-2-MAX", "USES-TIME-BASE-RESOURCE", "TTCAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "DIAGNOSTIC-TEST-RESULT-REFS", "PRE-ACTIVATE-IREF", "NM-PASSIVE-MODE-ENABLED", "URI", "FLEXRAY-TP-CONFIG", "MAPPED-DATA-ELEMENT-IREF", "TLV-RECORD-ELEMENT-REF", "USER-DEFINED-GLOBAL-TIME-SLAVE", "ASSIGNED-TRAFFIC-CLASSS", "ERROR-CODE", "SW-CLASS-IMPLS", "SENDING-NODE-REF", "FILE-REF", "DATA-TYPE-POLICY", "LOCAL-SUPERVISION", "SSRC", "SW-HOST-VARIABLE", "DIAGNOSTIC-SESSION-REF", "BSW-VARIABLE-ACCESS", "TRANSMISSION-MODE", "E-2-E-COUNTER-ERROR-REF", "CONSIDER-BUILD-NUMBER", "MAX-NUMBER-OF-TIME-QUANTA-PER-BIT", "SDG-PRIMITIVE-ATTRIBUTE", "RULE", "TRANSITION-REF", "I-PDU-TIMING-SPECIFICATIONS", "APMC-PARAM-CONF-CONTAINER-DEF", "EXCLUSIVE-AREA-NESTING-ORDER-REF", "ADDRESS-SEMANTICS", "ENTER-TIMEOUT-VALUE", "NM-MULTIPLE-CHANNELS-ENABLED", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "IP-SEC-RULE", "KEY-VALUE-PAIRS", "MAX-INSTANCES", "SW-IMPL-MAPPINGS", "PERSISTENT-STORAGE", "LOCAL-TCP-PORT-REF", "OFFSET", "MODE-DEPENDENT-STARTUP-CONFIGS", "LOGICAL-SUPERVISIONS", "INIT-VALUE", "DO-IP-INTERFACE", "END-TO-END-TRANSFORMATION-I-SIGNAL-PROPS", "TIME-BASES", "INSTANTIATION-DATA-DEF-PROPS", "IP-ADDRESS", "STATUS-ID", "SLOT-ID", "VALUE-TYPE-REF", "SW-SYSTEMCONSTANT-VALUE-SET-REF", "BSW-CLIENT-POLICY", "SECOND-ELEMENT-INSTANCE-IREFS", "ELEMENT-ROLE", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "DDS-CP-I-SIGNAL-TO-DDS-TOPIC-MAPPING", "J-1939-PROTECTED-I-PDU", "COUPLED-CLUSTER-REFS", "TCP-RETRANSMISSION-TIMEOUT", "PASS-THROUGH-SW-CONNECTOR", "REQUEST-CURRENT-POWERTRAIN-DIAGNOSTIC-DATA-CLASS-REF", "SECOND-MODE-GROUP-REF", "SLAVE-REF", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "SOFTWARE-CLUSTER-DEPENDENCY-FORMULA", "SUPERVISED-ENTITY-NEEDS", "WAKEUP-FORWARD-LOCAL-ENABLED", "TD-EVENT-TYPE", "CONTAINED-AR-ELEMENT-REF", "EMULATION-SUPPORTS", "INTERFACE-BLUEPRINT-MAPPINGS", "PERSISTENCY-ACCESS", "RPT-READS", "FRAGMENT", "PLATFORM-PHM-ACTION-ITEM", "ECUC-STRING-PARAM-DEF", "SOURCE-INSTANCE-IREFS", "GROUP-REFS", "SERVER-COM-SPEC-PROPSS", "RPT-SW-PROTOTYPING-ACCESS", "APPLICATION-ARRAY-ELEMENT-REF", "RPT-AR-HOOK-IREF", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "PATTERN-PERIOD", "MC-GROUP-DATA-REF-SET-VARIANTS", "URI-DESCRIPTION", "TYPE-REFERENCE", "SWC-BSW-SYNCHRONIZED-TRIGGER", "PREVIOUS-DATA-TYPE-REF", "SOCKET-CONNECTION-IPDU-IDENTIFIER", "TIMEOUT", "COUPLED-CLUSTER-REF", "VLAN-IDENTIFIER", "TRIGGER-PORT-ANNOTATION", "ACL-ROLE-REFS", "SOURCE-CAN-ID-MASK", "FIELD-MAPPING", "DO-IP-REQUEST-CONFIGURATION-DESIGN", "GLOBAL-TIME-MASTERS", "SW-CARB-DOC", "UPPER-MULTIPLICITY", "GLOBAL-TIME-GATEWAY", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "CPP-TEMPLATE-ARGUMENT", "SEND-INDICATION", "REQUEST-CONFIGURATION-DESIGNS", "TD-PDU-TRIGGERING-FILTER-REF", "REG-REQUIRES-STABILITYS", "HARDWARE-TEST-NEEDS", "DO-IP-TARGET-ADDRESS-REFS", "ALIVE-CHECK-RESPONSE-TIMEOUT", "RESPONSE-ON-EVENT-ACTION", "PHM-HEALTH-CHANNEL-INTERFACE", "HTTP-TP", "DIAGNOSTIC-ECU-RESET-INTERFACE", "TARGET-I-PDU-REF", "HEAP-USAGES", "EXEC-IN-USER-MODE", "SWITCH-STREAM-FILTER-ENTRY", "NETWORK-MASK", "TP-PGS", "DO-IP-NETWORK-CONFIGURATION-DESIGN", "ANALYZED-EXECUTION-TIME", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "PORT-BLUEPRINT-REF", "EXECUTION-ERROR-REF", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "ARBITRARY-ADDRESS-CAPABLE", "EXPLICIT-CONGESTION-NOTIFICATION", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "TCP-IP-DHCP-V-6-INF-DELAY-MIN", "SNAPSHOT-RECORD-CONTENTS", "SW-CLUSTER-REFS", "SEC-OC-JOB-REQUIREMENT", "UCM-MODULE-INSTANTIATION-REF", "MODE-MANAGER-ERROR-BEHAVIOR", "SECURITY-EVENT-DEFINITION", "STATE-MANAGEMENT-ENTER-SUSPEND-TO-RAM-OS-ACTION-ITEM", "GLOBAL-ELEMENT", "TCP-IP-NDP-SLAAC-DELAY-ENABLED", "APMC-CHOICE-CONTAINER-REFERENCE-DEF", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "COMPRESSED-SOFTWARE-PACKAGE-SIZE", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "PORT-BLUEPRINT-MAPPING", "BASIC-CYCLE-LENGTH", "SW-MIN-AXIS-POINTS", "CLIENT-ID-DEFINITION-SET", "EXECUTION-TIME-CONSTRAINT", "SOCKET-CONNECTION-BUNDLE-REF", "SWITCH-ASYNCHRONOUS-TRAFFIC-SHAPER-GROUP-ENTRY", "SW-CALPRM-AXIS-SET", "PRODUCT-ID", "TARGET-I-PDU", "NM-LIGHT-TIMEOUT", "PORT-INTERFACE-BLUEPRINT-REF", "SERVER-SERVICE-MINOR-VERSION", "SUB-GROUP-REFS", "SW-MC-BASE-TYPE", "TRANSIT-TO-INVALID-EXTENDED", "SOCKET-CONNECTION-REFS", "CLAIMED-FUNCTION-GROUP-REFS", "CRYPTO-CERTIFICATE-GROUP-TO-PORT-PROTOTYPE-MAPPING", "CAN-CONTROLLER-XL-ATTRIBUTES", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "REMOTE-MODE-SENDER-QUEUE-LENGTH-CONTRIBUTION", "FRAME-TRIGGERINGS", "SUB-ELEMENT-MAPPING", "NM-CLUSTER-COUPLINGS", "TCP-IP-IP-REASSEMBLY-BUFFER-SIZE", "ANNOTATION", "TRANSFORMATION-PROPSS", "MAC-SEC-KAY-CONFIG", "CAN-XL-CONFIG-REF", "SEND-POLICYS", "RUNS-INSIDE-EXCLUSIVE-AREA-REFS", "DDS-CP-QOS-PROFILE", "MODE-DRIVEN-FALSE-CONDITIONS", "OFFSET-TIMING-CONSTRAINT", "TRANSMISSION-MODE-DECLARATION", "TEAR-DOWN-ACTION-REF", "APPLICATION-ENDPOINT-REF-CONDITIONAL", "PROCESS-ARGUMENT", "DATA-IDENTIFIER-REFS", "REQUEST-TOPIC-NAME", "REQUEST-TYPE", "MINIMUM-MEMORY-CONSUMPTION", "BY-PASS-POINT-IREFS", "INHIBITING-SECONDARY-FID-REF", "MIN-PWM-S", "RX-NM-PDU-REFS", "TLV-DATA-ID", "CHECK-PER-SOURCE-ID", "LOCAL-DEBUG-DATAS", "INTENDED-TARGET-MACHINE-REF", "MEMORY-USAGE", "WAKEUP-LOCAL-DETECTION-TIME", "TARGET-PARAMETER-DATA-PROTOTYPE-REF", "DATA-TYPE-MAPS", "KEY-SLOT-ONLY-ENABLED", "TCP-MSL", "FLAT-MAP-REF", "TARGET-EVENT-REF", "STORE-CYCLIC", "SEC-OC-JOB-REQUIREMENT-REF", "BURST-SIZE", "WRITING-STRATEGY-ROLE", "BUCKET-SIZE", "NM-READY-SLEEP-COUNT", "RECOVERY-ACTION-IREF", "DIAGNOSTIC-MESSAGE-TYPE", "EGRESS-PORT-REFS", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "DO-IP-REQUEST-CONFIGURATION", "OS-TASK-EXECUTION-EVENT", "PERSISTENCY-FILE-ARRAY-REF", "FUNCTIONAL-UNIT", "BUS-OFF-RECOVERY", "ECU-INSTANCE", "DEPENDENT-SOFTWARE-CLUSTER-DESIGN-REF", "REPORTED-SECURITY-EVENT-IREF", "MAX-SUPPORTED-DID-LENGTH", "VENDOR-SIGNATURE-REF", "SERVICE-POINT-SYMBOL-POST", "LOG-TRACE-PROCESS-DESC", "CONSUMED-EVENT-GROUP-REF", "INCLUDED-VARIANT-REF", "TRIGGER-IREF", "CONSTANT-VALUE-MAPPING-REF", "ITEM-LABEL", "STARTS-ON-EVENT-REF", "OBD-PID-SERVICE-NEEDS", "LOGICAL-EXPRESSION", "IEEE-1722-TP-ACF-CAN", "DIFFERENTIATED-SERVICE-FIELD", "ESTIMATED-DURATION-OF-CAMPAIGN", "NV-DATA-PORT-ANNOTATION", "RATE-BASED-MONITORED-EVENT-REF", "PER-INSTANCE-MEMORY-SIZES", "ETH-SWITCH-PORT-GROUP-DERIVATION", "CAN-BIT-RATE-SWITCH", "SERIALIZATION-TECHNOLOGY", "OPTIONS", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "STATE-MACHINE", "SUPPORT-DIRTY-FLAG", "DIAGNOSTIC-DATA-ELEMENT", "DLT-ARGUMENT-PROPS-SET", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "TRIGGER-MODE", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "OCCURRENCE-EXPRESSION", "CONTEXT-SWC-PROTOTYPE-REFS", "SW-COMPONENT-DOCUMENTATIONS", "DURABILITY-KIND", "START-POSITION", "SEC-OC-JOB-MAPPING", "COUPLING-ELEMENT-SWITCH-DETAILS", "IEEE-1722-STREAM-REF", "DIAGNOSTIC-MEMORY-IDENTIFIER", "TESTED-ITEM-REF", "SUB-GROUP-REF", "POST-BUILD-VARIANT-MULTIPLICITY", "DLT-MESSAGE-COLLECTION-SET", "I-SIGNAL-I-PDU-REF-CONDITIONAL", "MC-PARAMETER-ELEMENT-GROUP", "EVENT-REF", "ADAPTIVE-FIREWALL-MODULE-INSTANTIATION", "ALLOW-PLAIN-TEXT-EXPORT", "REQUEST-RESULT", "KEY-REF", "MODE-DECLARATION-GROUP-REF", "TLV-DATA-ID-DEFINITION-REF", "MODE-GROUP-REF", "IUMPR-REF", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "SOMEIP-SERVICE-INTERFACE", "DESIGN-REF", "EVENT-HANDLER", "DIAGNOSTIC-COM-CONTROL-CLASS", "OPT-ARGUMENT-REF", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "SOFTWARE-CLUSTER-DEPENDENCY", "SEGMENT-REF", "PREDECESSOR-REF", "PERIODIC-RESPONSE-UUDT-REF", "SOFTWARE-CLUSTER", "PHYSICAL-DIMENSION-REF", "SW-RECORD-LAYOUT-GROUP-TO", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY-TYPE", "MODE-REQUEST-TYPE-MAPS", "DDS-TRANSFORMATION-I-SIGNAL-PROPS-VARIANTS", "SW-AXIS-TYPE-REF", "CONSTANT-REF", "CRYPTO-PROVIDER-INTERFACE", "DEBOUNCE-ALGORITHM-PROPSS", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "I-SIGNAL-MAPPING", "ECU-ID", "EXPECTED-ALIVE-INDICATIONS", "ECU-EXTRACT-REFERENCE-IREF", "APPLICATION-ARRAY-DATA-TYPE", "TRIGGER-MAPPINGS", "DLT-ARGUMENT-PROPS", "ACTION-LIST", "CONTEXT-DATA-ELEMENTS", "MASTER-REF", "MAX-INITIAL-VEHICLE-ANNOUNCEMENT-TIME", "USER-DEFINED-I-PDU", "IMPLEMENTATION-REF", "I-PDUS", "COUPLING-PORT-SWITCHOFF-DELAY", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "COUPLING-ELEMENT", "COMPU-NUMERATOR", "MULTICAST-SEGMENTATION", "ACCEPTS-ENCODINGS", "LIN-ORDERED-CONFIGURABLE-FRAME", "ATTRIBUTE-REF", "OBD-MONITOR-SERVICE-NEEDS", "SUSPEND-TO-RAM-SATELLITE-INTERFACE", "BYPASS-VLAN", "FUP-DATA-ID-LIST", "SW-INTENDED-RESOLUTION", "OPT-ARGUMENT-REFS", "ESTIMATED-DURATION-OF-OPERATION", "KEY-EXCHANGE-REFS", "PERSISTENCY-FILE-ELEMENT", "COMPONENT-QUALIFIER", "SOVD-LOCK-REF", "SWC-TO-ECU-MAPPING-CONSTRAINT", "TCP-IP-NDP-SLAAC-DAD-RETRANSMISSION-DELAY", "DIAGNOSTIC-SOVD-METHOD-PRIMITIVE", "METHOD-MAPPINGS", "SEGMENTATION-SUPPORTED", "REMOTE-MULTICAST-CONFIG-REFS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "DO-IP-FUNCTIONAL-CLUSTER-DESIGN", "CERT-COMMON-NAME", "RAW-DATA-STREAM-INTERFACE", "E-2-E-EVENT-PROTECTION-PROPSS", "SDG-TAILORINGS", "CATEGORY", "BAUDRATE", "BUILD-ACTION-ENVIRONMENT", "MIN", "USE-MAC-ADDRESS-FOR-IDENTIFICATION", "ATTRIBUTE-VALUES", "CALLED-FROM-WITHIN-EXCLUSIVE-AREA-REF", "AUTHENTICATION-RETRIES", "MEMORY-RANGE-REFS", "KEY-SLOT-USED-FOR-START-UP", "J-1939-REQUESTABLE", "FUNCTIONAL-CLUSTER-REFS", "ACL-OPERATION", "FAILED-ALIVE-SUPERVISION-REF-CYCLE-TOL", "MODE", "ECU-COMM-PORT-INSTANCES", "CIPHER-SUITE-CONFIGS", "PHYSICAL-PROPS", "FORMAT", "CRYPTO-KEY-DESCRIPTION", "BUILD-ACTION", "NM-REPEAT-MSG-IND-ENABLED", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "J-1939-CONTROLLER-APPLICATION-TO-J-1939-NM-NODE-MAPPINGS", "EVENT-RECEPTION-DEFAULT-VALUE", "CHECKPOINT-FINAL-REFS", "PHM-CONTRIBUTION-REFS", "ETHERNET-COMMUNICATION-CONTROLLER", "SO-CON-I-PDU-IDENTIFIER-REF", "SYNCHRONIZED-SLAVE-TIME-BASE", "PRM", "TARGET-DATA-PROTOTYPE-IN-CS-REF", "SIGNAL-I-PDU-COUNTER", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG-REF", "GROUPED-ALIAS-EVENT-REFS", "TLV-DATA-IDS", "TD-HEADER-ID-RANGE", "NM-REPEAT-MESSAGE-BIT-ENABLE", "TARGET-DATA-PROTOTYPE-IN-SR-REF", "DDS-SERVICE-INSTANCE-EVENT-CP", "DATA-PROTOTYPE-GROUP-IREFS", "MIN-VALUE", "SEC-OC-CRYPTO-SERVICE-MAPPINGS", "NM-PDUS", "PROVIDED-DATAS", "DIAG-PDU-TYPE", "TRANSPORT-PLUGINS", "GLOBAL-SUPERVISION-ENTITYS", "NM-REMOTE-SLEEP-INDICATION-TIME", "GLOBAL-IN-PACKAGE-REF", "HEALTH-CHANNEL-ARGUMENT-REF", "URL", "SERVICE-ELEMENT-SECURE-COM-CONFIG-REF", "MULTICAST-REF", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "VARIABLE-ACCESS-IREF", "RPT-SYSTEM-REF", "PREDECESSOR-ACTION-REFS", "LEGISLATED-SOVD-FREEZE-FRAME-CONTENT-REF", "PHM-CONTRIBUTION-REF", "FIREWALL-RULE-PROPS", "UNIT-GROUP", "ETH-IP-PROPS", "TD-EVENT-SERVICE-INSTANCE-METHOD-TYPE", "SCHEDULE-CHANGE-NEXT-TIME-BASE", "PLCA-TRANSMIT-OPPORTUNITY-TIMER", "SEGMENT-RECEPTION-TIMEOUT-TIME", "PROVIDED-SERVICE-INSTANCE-REF", "APMC-FOREIGN-REFERENCE-VALUE", "CM-MODULE-INSTANTIATION", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "CRYPTO-CERTIFICATE-GROUPS", "CONTEXT-DATA-ELEMENT-IDENTIFICATIONS", "TCP-IP-ICMP-V-6-ECHO-REPLY-AVOID-FRAGMENTATION", "OPERATION-IREFS", "TROUBLE-CODE-UDS-REF", "OFS-SUB-TLV", "BUS-MIRROR-NETWORK-ID", "PROTOCOL-NAME", "AR-PARAMETER", "SEGMENT-POSITIONS", "FAILED-REFERENCE-CYCLES-TOLERANCE", "NETWORK-ENDPOINT-REFS", "DLT-ECU-ID", "SW-IS-VIRTUAL", "SW-TEST-DESC", "PERMISSION-TO-CREATE-CHILD-PROCESS", "DIAGNOSTIC-INDICATOR-NEEDS", "MIXED-BUS-TYPE-COLLECTION", "AES-3-DATA-TYPE-L", "ACF-BUS-PART-REFS", "ASSIGNED-CONTROLLER-REF", "AP-APPLICATION-ERROR-REF", "COM-FIND-SERVICE-GRANT-DESIGN", "DATA-TYPE-FOR-SERIALIZATION-REFS", "SW-CLUSTER-MAPPINGS", "RECEPTION-POLICYS", "PER-INSTANCE-MEMORY-SIZE", "BURST-SIZE-REQUEST", "STREAM-IDENTIFICATION-WILDCARD", "HANDLE-NEVER-RECEIVED", "ACCURACY", "L-2", "PNC-WAKEUP-CAN-ID-EXTENDED", "SECOND-ELEMENT-INSTANCE-IREF", "SW-CALIBRATION-NOTES", "DISABLED-MODE-IREF", "NETWORK-MANAGEMENT-VECTOR-LENGTH", "NM-NETWORK-HANDLE", "APMC-UPSTREAM-DOC-INSTANCE-REFERENCE-DEF", "REF-CALPRM-SET", "TD-EVENT-I-PDU", "REFERENCE-TABLE-REF", "DIAGNOSTIC-EVENTS", "INHIBIT-SOURCES", "TD-EVENT-BSW-INTERNAL-BEHAVIOR-TYPE", "BSW-OPERATION-INVOKED-EVENT", "VFC-IREF", "FRESHNESS-VALUE-ID", "RPT-WRITE-ACCESS", "MEASURED-STACK-USAGE", "SWC-INTERNAL-BEHAVIOR", "PROVIDED-MEMORY-REF", "STATE-MANAGEMENT-PHM-ERROR-INTERFACE", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "SOFTWARE-CLUSTER-DESIGN", "CRYPTO-ELLIPTIC-CURVE-PROPS", "IDSM-TRAFFIC-LIMITATION-REF-CONDITIONAL", "COLLECTION", "MONITORED-IDENTIFIER-REF", "STATE-DEPENDENT-FIREWALL-REF", "USE-FRESHNESS-TIMESTAMP", "PORT-PROTOTYPE-PROPS", "ROM-BLOCK-INIT-VALUE", "PNC-FILTER-DATA-MASK", "MODULE-REF", "MASTERED-TRIGGER-REF", "SERVICE-OFFER-TIME-TO-LIVE", "EXECUTABLE-LOGGING-IMPLEMENTATION-PROPS", "USE-VEHICLE-IDENTIFICATION-SYNC-STATUS", "HW-PORT-MAPPINGS", "TOL", "PROFILE-NAME", "ROUGH-ESTIMATE-STACK-USAGE", "OVERRIDE-INITIAL-STATE-IREF", "OFS-DATA-ID-LISTS", "COMMANDS", "AGING-CYCLES", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REF", "PNC-CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUPS", "MAX-ACTION-ITEM-DURATION", "TIME-SYNC-SERVER", "I-PDU-IDENTIFIER-UDP-REFS", "ROOT-AUTOSAR-DATA-PROTOTYPE-REF", "PROPS", "OPERATION-CYCLE", "ACTUAL-EVENT-REF", "CRYPTO-KEY-SLOT-USER-DESIGN-REF", "STOP-AGENT-ERROR-REF", "STATUSS", "PAYLOAD-TYPE", "UCM-DESCRIPTION", "SUPERVISION-ENTITYS", "SUB-NODE-CHANNEL-REF", "AUTHENTIC-CONNECTION-PROPSS", "OBD-DTC-VALUE", "MODE-GROUP", "BASE-REFERENCE-REF", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "STATIC-SOCKET-CONNECTIONS", "IEEE-1722-TP", "SETTER-CALL-REF", "MIN-LENGTH", "CONTEXT-PORT-PROTOTYPE-REF", "REMOTE-PORT-REF", "UPDATE-STRATEGY", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "J-1939-SHARED-ADDRESS-CLUSTER", "REQUIRED-DDS-SERVICE-INSTANCE", "SHARED", "MODE-SWITCH-POINT", "SECURITY-EVENT-CONTEXT-DATA-DEFINITION-REF-CONDITIONAL", "DATA-TYPE-MAPPING", "LOCAL-COM-ACCESS-CONTROL-ENABLED", "PERSISTENCY-DEPLOYMENT-ELEMENT-REF", "FIELD", "DIAGNOSTIC-REQUEST-DOWNLOAD", "SD-SERVER-CONFIG-REF", "ENABLE-UPDATE", "DENOMINATOR-GROUP", "MODE-SWITCH-EVENT-TRIGGERED-ACTIVITYS", "LIST", "EXECUTABLE-REQUIRED-PORT-IREF", "SO-CON-I-PDU-IDENTIFIER", "R-PORT-PROTOTYPE-REF", "PROVIDED-SERVICE-INSTANCES", "SERVICE-INSTANCE-REF", "STARTUP-CONFIG-SET", "DEPENDS-ON-REF", "PAYLOAD-LENGTH-STATIC", "ANNOTATION-ORIGIN", "DIAGNOSTIC-ENV-DATA-CONDITION", "PATH-MTU-DISCOVERY-ENABLED", "ADMIN-DATA", "DIAGNOSTIC-SOVD-CONFIGURATION-INTERFACE", "BY-PASS-POINT-IREF", "SLAVE-EVENT-REF", "KEYWORDS", "LOAD-BALANCING-WEIGHT", "TEXT-TABLE-MAPPING", "LIVELINESS", "PROVIDER-IREF", "SW-TEST-DESCS", "SW-SYSCOND", "VEHICLE-DRIVER-NOTIFICATION", "PROVIDER-RATE-DEVIATION-MAX", "PER-INSTANCE-MEMORY", "DIAGNOSTIC-SECURITY-ACCESS", "MAX-DELTA-COUNTER", "TP-CONNECTION-REF", "NUMBER-OF-BITS", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "SOURCE-EVENT-REFS", "FLEXRAY-FIFO-CONFIGURATION", "FRAME-MAPPINGS", "PHYSICAL-DIMENSION", "TYPE-TAILORINGS", "ON-FAIL-PERMISSIVE-MODE-TIMEOUT", "EXPECTED-ALIVE-CYCLE", "ACL-CONTEXTS", "DTC-NUMBER", "MIRRORING-PROTOCOL", "AP-APPLICATION-ERROR-DOMAIN", "MAX-DURATION-EXCEEDED-ERROR-REF", "FLOW-METERINGS", "REVERSED-TP-SDU-REF", "EXPRESSION-REF", "METHOD-REQUEST-PROPSS", "SECURE-COM-CONFIG-REF", "DLT-ARGUMENT-PROPS-REF", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "M", "PHYSICAL-LAYER-TYPE", "FRAME-SYNC-ENABLED", "AUTHENTICATION-ROLE-REF", "NEW-NAD", "TCP-IP-ICMP-V-4-TTL", "ETHERNET-PRIORITY-REGENERATIONS", "AH-CIPHER-SUITE-NAME", "NATIVE-DECLARATION", "DIAGNOSTIC-EDR-SERVER-PORT-MAPPING", "CYCLIC-WRITING-PERIOD", "FLAT-OBJECT-REF", "SUBSTITUTED-FRAME-REF", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "RESPONSE-DATA-SIZE", "SUPERVISED-ENTITY-IREF", "VERIFICATION-METHOD", "ASSIGNED-DATAS", "SERVICE-INTERFACE-SUB-ELEMENT", "INFRASTRUCTURE-SERVICES", "IDS-TIME-BASE-REFS", "DATA-WRITE-COMPLETED-EVENT", "PARAMETER-INSTANCE", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "NM-REMOTE-SLEEP-IND-ENABLED", "CALIBRATION-PARAMETER-VALUE-SET", "SYMBOL-PROPS", "ENUMS", "SEPARATION-TIME", "FIFO-RANGES", "SCALE-CONSTRS", "CONNECTION-NEGOTIATION-BEHAVIOR", "FIRST-ELEMENT-INSTANCE-IREFS", "LOGICAL-OPERATOR", "COLD-START-ATTEMPTS", "REMOTE-PORT-RANGE-START", "MEM-CLASS-SYMBOL", "OWNER-REF", "SYSTEM-MEMORY-USAGES", "BINARY-MANIFEST-ITEM-POINTER-VALUE", "IPV-4-CONFIGURATION", "MATCHING-RULE-REF", "TLV-IMPL-RECORD-ELEMENT", "WRITE-CLASS-REF", "APPLICATION-RULE-BASED-VALUE-SPECIFICATION", "CONNECTOR-REFS", "TYPE-BLUEPRINTS", "DATA-ID-MODE", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "REST-RESOURCE-DEF", "GETTER-RETURN-REF", "CLIENT-SERVER-ARRAY-ELEMENT-MAPPING", "OPT-IMPL-RECORD-ELEMENTS", "SUPPORT-INFO", "APMC-URI-PARAM-DEF", "RECOVERY-NOTIFICATION-TIMEOUT", "LIN-CLUSTER", "COLLECTION-TRIGGER", "CONTEXT-DESCRIPTION", "FLOAT", "SOMEIP-SERVICE-DISCOVERY-PORT", "MSR-QUERY-RESULT-P-1", "SYNC-CONFIRMATION-TIMEOUT", "NOMINAL-EXECUTION-TIME", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "HEALTH-CHANNEL-EXTERNAL-MODE", "TLV-SUB-ELEMENT-REF", "LOGICAL-EXPRESSIONS", "AUTOSAR-VARIABLE-INSTANCE", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "LIFESPAN", "STATE-MANAGEMENT-REQUEST-RULE", "DPG-REQUIRES-COHERENCYS", "NM-REPEAT-MESSAGE-BIT-ACTIVE", "USED-PIM-REF", "PHM-CHECKPOINT", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "TRANSMISSION-DEADLINE", "TGROUP", "ECUC-MODULE-CONFIGURATION-VALUES-REF", "NUMBER-OF-REPETITIONS", "RAW-DATA-STREAM-DEPLOYMENT", "NV-DATA-INTERFACE", "BULK-NV-DATA-DESCRIPTOR", "PERIODIC-RESPONSE-UUDT-REFS", "CAN-IDENTIFIER", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "THRESHOLD-SIZE", "MULTIPLICITY-CONFIG-CLASSES", "SYSTEM-DOCUMENTATIONS", "TD-EVENT-VFB-PORT-REF", "TRAFFIC-SHAPER-GROUP-REF", "HEADER-ID-LONG-HEADER", "MSR-QUERY-TOPIC-1", "SCHEDULER-NAME-PREFIXS", "SWC-MAPPING-CONSTRAINT-REF", "FLEXRAY-TP-CONNECTION-CONTROL", "BULK-NV-BLOCK", "ECU-RESET-CLASS-REF", "AAF-FORMAT", "SERVICE-INTERFACE", "MACHINE-REF", "BLOCKLISTED-VERSIONS", "CP-SOFTWARE-CLUSTER-REF", "PORT-GROUPS", "CRYPTO-PRIMITIVE", "MC-DATA-ASSIGNMENTS", "SYSC-STRING-REF", "SW-CLASS-ATTR-INSTANCE-IMPL", "KEY-SLOT-ALLOWED-MODIFICATION", "REF", "UDP-PORT", "SECURE-PROPS-FOR-EXTERNAL-COMM-REF", "BASE-CYCLE", "SERIALIZATION-TECHNOLOGY-REF", "SLOT-CAPACITY", "PNC-WAKEUP-CAN-ID", "DLT-APPLICATION", "CAT-2-ISR", "SECURITY-DELAY-TIME-ON-BOOT", "DNS-SERVER-ADDRESSES", "DIAGNOSTIC-ECU-RESET-CLASS", "SWC-SERVICE-DEPENDENCY-IN-SYSTEM-IREF", "REQUIRED-DDS-EVENT-QOS-PROPS", "NM-CAR-WAKE-UP-FILTER-ENABLED", "CRYPTO-KEY-SLOT-USER-DESIGN", "PRIMITIVE-ATTRIBUTE-CONDITION", "INIT-VALUES", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "SOURCE-ADDRESS-CODE", "SUB-FUNCTION-REF", "J-1939-CLUSTER", "CLIENT-SERVICE-MAJOR-VERSION", "FURTHER-ACTION-BYTE-NEEDS", "SDG-CLASS", "LOCAL-UNICAST-ADDRESSES", "TARGET-METHOD-REF", "SERVICE-ELEMENT-MAPPING-REF", "IDSM-INSTANCE-ID", "DOCUMENTATION", "APMC-REVISION-LABEL-PARAM-DEF", "MATCHING-EGRESS-RULE-REFS", "CAS-RX-LOW-MAX", "NODE-REF", "DOCUMENT-ELEMENT-SCOPE", "DURABILITY-SERVICE-MAX-INSTANCES", "SW-CLUSTER-REF", "SW-VALUE-BLOCK-SIZE-MULTS", "MAC-SEC-GLOBAL-KAY-PROPS", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REFS", "SIZE", "OBD-INFO-SERVICE-NEEDS", "TCP-IP-IP-TX-FRAGMENT-BUFFER-COUNT", "SHALL-RUN-ON-REFS", "POSITION-IN-TABLE", "SDG-TAILORING", "TROUBLE-CODE-OBD-REF", "SUB", "MAXIMUM-SEGMENT-LENGTH-REQUEST", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-EVALUATION", "RUNNABLE-ENTITY", "NM-ECU", "MAC-ADDRESS", "MC-GROUP-DATA-REF-SET-CONDITIONAL", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "NM-COORDINATOR-ROLE", "BUNDLED-CONNECTIONS", "PHYSICAL-DIMENSION-MAPPING", "MEMORY-LOW-ADDRESS-LABEL", "CLIENT-ID", "FLEXRAY-TP-PDU-POOL", "BURST-SIZE-RESPONSE", "SOCKET-OPTIONS", "STATE-REQUEST-REF", "EVALUATED-ELEMENT-REF", "I-SIGNAL-PORT-REFS", "MAXIMUM-SELECTED-BINDING-TIME", "CONTAINED-PROCESS-REFS", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "DLT-MESSAGES", "SECURITY-EVENT-REFS", "FIREWALL-STATE-MODE-DECLARATION-REFS", "PORT-SCHEDULER", "PORT-PROTOTYPE-BLUEPRINT-INIT-VALUE", "DIAGNOSTIC-IUMPR", "RPT-PROFILES", "SHUTDOWN", "IPV-6-ADDRESS", "SW-CLUSTER-DESIGN-REQUIRED-PORT-IREF", "APPL-INIT-VALUE", "PDU-COLLECTION-SEMANTICS", "VEHICLE-PACKAGE", "SW-MAX-AXIS-POINTS", "COM-MANAGEMENT-MAPPINGS", "SAFETY-POLICY", "SUPPORTED-CONFIG-VARIANT", "PROVIDE-RESOURCES", "IDSM-MODULE-INSTANTIATION", "OBD-SUPPORT", "INVERT", "WAKEUP-FRAME-REFS", "LOCAL-PORT-RANGE-START", "MAPPED-OPERATION-IREF", "ALTERNATIVE-NAME", "ACF-MAX-TRANSIT-TIME", "ALLOW-MASTER-RATE-CORRECTION", "PER-INSTANCE-PARAMETER-REF", "END-ADDRESS", "FUNCTION-GROUP-PHM-STATE-REFERENCE", "IMPLEMENTATION-PROPSS", "READ-LOCAL-VARIABLES", "DATA-IDENTIFIER-REF", "COMPILER", "MACHINE-DESIGN-REF", "TARGET-OPERATION-REF", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "DLT-SESSION-ID", "PADDING-ACTIVATION", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "FREEZE-FRAME-CONTENT-WWH-OBD-REF", "ARTIFACT-LOCATORS", "WAKEUP-FRAME-REF", "DATA-VALUE-ERROR-REF", "MAXIMUM-EXECUTION-TIME", "DIAGNOSTIC-COMPONENT-NEEDS", "MACHINE-MODE-IREFS", "UDP-TTL", "FUNCTION-GROUP-STATE-IREFS", "DHCP-PROPS", "PRIMARY-DOWNLOAD-URI", "BUILD-ACTION-MANIFEST", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF-CONDITIONAL", "STATIC-PARTS", "OPT-APP-RECORD-ELEMENT-IN-SERVICE-INTERFACE-IREF", "DISCOVERY-TECHNOLOGY", "REQUIRED-SERVICE-INSTANCE-REF", "SW-GENERIC-AXIS-PARAM-TYPE-REF", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "TIME-BASE-CONSUMER-REF", "PROCESS-TO-MACHINE-MAPPING", "TCP-ROLE", "BLOCK-STATES", "SW-SERVICE-IMPL-POLICY", "EXECUTION-DEPENDENCYS", "BSW-COMPOSITION-TIMING", "SOFTWARE-PACKAGE-STORING", "PROCESS-MODE-IREF", "BSW-ENTRY-KIND", "MAX-BASE-TYPE-SIZE", "CUSTOM-SERVICE-ID", "EVENT-CONTROLLED-TIMING", "CAN-COMMUNICATION-CONTROLLER-CONDITIONAL", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "IMPLEMENTATION-DATA-TYPE-REF", "STREAM-FILTERS", "DURABILITY-SERVICE-HISTORY-KIND", "HISTORY", "TIME-BR", "CAN-GLOBAL-TIME-DOMAIN-PROPS", "IMPLEMENTATION-DATA-TYPE-ELEMENT-REF", "SPORADIC-EVENT-TRIGGERING", "GLOBAL-TIME-SUB-DOMAINS", "SOURCE-INSTANCE-IREF", "TRANSFORMER-CHAIN-REFS", "PERSISTENCY-KEY-VALUE-PAIR", "CRYPTO-OBJECT-TYPE", "LOWER-BOUND", "SUB-ATTRIBUTE-TAILORINGS", "SW-COMPONENT-ASSIGNMENTS", "TARGET-VARIABLE-DATA-PROTOTYPE-REF", "MEMORY-ENTRY-STORAGE-TRIGGER", "RESOURCE-REF", "ATTRIBUTE-TAILORINGS", "CHECKPOINT-TRANSITION-REF", "ON-TRANSITION-VALUE", "FILTER-CHAINS", "APMC-FOREIGN-REFERENCE-DEF", "MEASURABLE-SYSTEM-CONSTANT-VALUES-REF", "SPECIFIC-PHYSICAL-CHANNEL-REF", "HANDLE-OUT-OF-RANGE-STATUS", "CONNECTORS", "CLIENT-SERVER-OPERATION", "AUDIENCES", "LIN-CONFIGURABLE-FRAME", "LIMIT-KIND", "MAX-FC-WAIT", "MULTICAST-SD-IP-ADDRESS-REF", "TIMEOUT-BR", "RPT-SERVICE-POINT", "TRIGGERING-EVENT-REF", "SCHEDULING-PRIORITY", "START-STATE-MACHINE-REF", "CAN-TP-ECU", "CONTROL-PROVIDED-EVENT-GROUP-REFS", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "SCHEDULER-NAME-PREFIX-REF", "DATA-TYPE-MAPPING-SET-REF", "CALLED-ENTRY-REF", "DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS-REF", "SW-SYSTEMCONST-REF", "IEEE-1722-RAW-DATA-STREAM-PRODUCER-INTERFACE", "TARGET-REFS", "BUS-SPECIFIC-NM-ECU", "TIMING-CLOCK-SYNC-ACCURACY", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "FIBEX-ELEMENTS", "SEC-OC-JOB-MAPPINGS", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "VFC-IREFS", "HW-PIN-REFS", "NOTIFICATIONS", "DESTINATION-URI-REFS", "TX-MASK", "IDENTITY-CERTIFICATE-AUTHORITY-REF", "TLV-DATA-ID-DEFINITIONS", "LIN-CLUSTER-CONDITIONAL", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "MODULE-DESCRIPTION-REF", "MODULE-INSTANTIATION-REF", "TCP-IP-ARP-NUM-GRATUITOUS-ARP-ON-STARTUP", "DERIVED-PORT-INTERFACE-REF", "KEY-SLOT-USAGES", "SYSTEM-TRIGGERED-EVENTS", "CLIENT-POLICYS", "PROCESSOR-MODE", "PARTIAL-NETWORK-REF", "FRAME-PID", "CALIBRATION-PARAMETER-VALUE-SET-REF", "TROUBLE-CODE-PROPSS", "ECU-RESOURCE-ESTIMATION", "BSW-MODULE-INSTANCE-REF", "BINARY-MANIFEST-META-DATA-FIELD", "COLOR-MODE", "BOOLEAN-VALUE-VARIATION-POINT", "CAN-COMMUNICATION-CONTROLLER", "DEPENDENCY-ON-ARTIFACT", "REQUIRED-MINOR-VERSION", "DESTINATION-URI-REF", "NM-HW-VOTE-ENABLED", "PROVIDED-INTERFACE-TREF", "VARIATION-POINT-PROXY", "WAKEUP-FORWARD-REMOTE-ENABLED", "RX-NM-PDU-REF", "ADDRESS-CONFIGURATION-CAPABILITY", "SUPPORTS-MULTIPLE-NOTIFIER-SETS", "IEEE-1722-TP-RULE", "NESTED-ELEMENTS", "MAX-PROGRESSION-MISMATCH-THRESHOLD", "MIN-SYNC-JUMP-WIDTH", "SW-DATA-DEF-PROPS-CONDITIONAL", "STREAM-IDENTIFICATION-HANDLE-REF", "HW-PORT-MAPPING", "INCLUDED-LIBRARY-REFS", "USER-DEFINED-GLOBAL-TIME-MASTER", "USER-DEFINED-PDU", "FLEXRAY-NM-CLUSTER", "DIAGNOSTIC-RESPONSE-ON-EVENT", "METHOD-ACTIVATION-ROUTING-GROUPS", "FUNCTION-NAMES", "CODE", "SECURED-AREA-LENGTH", "VALIDATION-FORMULA", "SECURITY-EVENT-REPORT-INSTANCE-DEFINITION-REF", "ARRAY-ELEMENT-REF", "COMMUNICATION-DIRECTION", "REQUIRED-DESIGN-ELEMENT-REFS", "POST-BUILD-VARIANT-CONDITION", "COUPLING-PORT-RATE-POLICY", "FIRST-ELEMENT-REFS", "PARAMETER-SIZE", "EVENT-HANDLER-REF", "PARAMETERS", "RESET-PENDING-BIT-ON-OVERFLOW", "MANUFACTURER-VALIDATION-ORDER-REFS", "FILE-TYPE", "TLS-CIPHER-SUITES", "CAN-COMMUNICATION-CONNECTOR", "VALUE-PAIRS", "AVERAGE-MEMORY-CONSUMPTION", "SENDER-RECEIVER-TO-SIGNAL-MAPPING", "CONTROL-CONSUMED-EVENT-GROUP-REF", "SW-CLASS-PROTOTYPE-REF", "ACCURACY-INT", "MAC-LAYER-TYPE", "DLT-LOG-CHANNEL-DESIGN", "APPLICATION-ASSOC-MAP-DATA-TYPE", "CALL-SIGNAL-TRIGGERING-REF", "SUPERVISION-MODES", "GLOBAL-TIME-DOMAIN-PROPERTYS", "ASSOCIATED-CROSS-SW-CLUSTER-COM-RTE-PLUGIN-REF", "NM-USER-DATA-OFFSET", "TCP-IP-ARP-TABLE-ENTRY-TIMEOUT", "VENDOR-ID", "DDS-CP-CONFIG", "ETS-AVAILABLE-BANDWIDTH-IN-WEIGHT-VALUE", "DIAGNOSTIC-CONNECTION", "TARGET-MODE-GROUP-REF", "RELATIONS", "I-PDU-TIMING", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "IEEE-1722-TP-ETHERNET-FRAME", "MATCHING-CRITERION-REF", "PATTERN", "GENERIC-MODULE-INSTANTIATION", "MODE-DRIVEN-TRANSMISSION-MODE-CONDITION", "J-1939-CLUSTER-CONDITIONAL", "CAN-ENTERS", "STD", "DIAGNOSTIC-CONDITION-REF", "WAKEUP-LOCAL-DURATION-TIME", "CAPTION", "ADDRESS-RANGE-LOWER-BOUND", "LIN-EVENT-TRIGGERED-FRAME", "META-DATA-ITEM", "I-PDU-TRIGGERINGS", "RAW-DATA-STREAM-REF", "CLEAR-CONDITION-REF", "ALL-CHANNELS-REF", "SW-AXIS-GENERIC", "EVENT-ELEMENT-MAPPINGS", "MANAGED-MODE-GROUPS", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "LOCAL-SUPERVISION-ENTITY-REF", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "REMOTE-SENDER-QUEUE-LENGTH-CONTRIBUTION", "PNC-MAPPING", "TIME-HARDWARE-CORRECTION-THRESHOLD", "DDS-CP-SERVICE-INSTANCE-OPERATION", "KEY-SLOT-CONTENT-ALLOWED-USAGES", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "DYNAMIC-PNC-TO-CHANNEL-MAPPING-ENABLED", "CLIENT-SERVER-OPERATION-IREF", "SIGNAL-BASED-FIRE-AND-FORGET-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "LOGICAL-ADDRESS", "EVENT-DISPLACEMENT-STRATEGY", "PORT-BLUEPRINT-INIT-VALUE", "TD-EVENT-FRAME", "ECU-SCOPE-REFS", "TRACE-REFS", "REQUIRED-INTERFACE-TREF", "DIAGNOSTIC-OPERATION-CYCLE", "IDSM-RATE-LIMITATION-REF", "ARRAY-IMPL-POLICY", "USER-DEFINED-CLUSTER-CONDITIONAL", "LIN-CHECKSUM", "DISPLAY-NAME", "VALID-BINDING-TIMES", "MAJOR-VERSION", "PNC-MAPPINGS", "GRAPHIC", "BANDWIDTH-LIMITATION", "SO-CON-I-PDU-IDENTIFIER-REF-CONDITIONAL", "FIELD-REQUEST-TOPIC-NAME", "PID-REF", "FIRE-AND-FORGET-MAPPINGS", "TRANSFORMER-STATUS-FORWARDING", "RX-IDENTIFIER-RANGE", "NETWORK-REPRESENTATION-PROPS", "DOC-REVISION", "TEXTUAL-CONDITION", "SDGS", "VALUE-REF", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "UNLIMITED-INTEGER-VALUE-VARIATION-POINT", "USE-SECURED-PDU-HEADER", "APPLICATION-RECORD-DATA-TYPE", "SYNC-JUMP-WIDTH", "FLEXRAY-NM-CLUSTER-COUPLING", "PHM-ACTION-LIST", "CLIENT-IP-ADDR-FROM-CONNECTION-REQUEST", "ECUC-QUERY-STRING-REF", "SOMEIP-PROVIDED-EVENT-GROUP", "BYPASS-ETHER-TYPE", "SOMEIP-EVENT-PROPS", "FIRE-AND-FORGET", "PORT-INTERFACE-MAPPING-SET", "SIGNAL-FAN", "HAS-INTERNAL-STATE", "COMPOSITION-SW-COMPONENT-TYPE-REF", "ACCEPT-VARIABLE-DA", "TARGET-MODE-REF", "ENDPOINT-CONFIGURATION-REF", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "MINIMUM-INTENDED-BINDING-TIME", "NETWORK-TIME-CONSUMER-REF", "CONTEXTS", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "EXTERN-OFFSET-CORRECTION", "INHIBITING-SECONDARY-FID-REFS", "INGRESS-PORT-REF", "SERVICE-RESOURCE-REF", "MODE-DECLARATION-GROUP", "DOMAIN", "SELECT-BLOCK-FOR-FIRST-INIT-ALL", "FILE-REFS", "MIN-SAMPLE-POINT", "SW-IMPL-POLICY", "ROLE-BASED-RESOURCE-DEPENDENCY", "LEAVE-SUSPEND-TO-RAM-ERROR-REF", "REPLAY-PROTECTION", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "REDUNDANCY", "ATTRIBUTE-NAME", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "ASSIGNMENT-PRIORITY", "ETHERNET-FRAME-TRIGGERING", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REFS", "NETWORK-ENDPOINT", "ACTIVATION-REASON-REPRESENTATION-REF", "SHAPER", "CLASS-CONTENT-CONDITIONAL", "OBJECT-IREF", "DDS-CP-TOPIC", "CLASSIFICATION", "MODE-DRIVEN-TRUE-CONDITIONS", "CRC-CORRECTION-FIELD", "UPPER-REF", "ROM-BLOCK", "CLEAR-DTC-LIMITATION", "COMMUNICATION-CLUSTER-REF", "MEMORY-LOW-ADDRESS", "RESOURCE-TO-APPLICATION-PARTITION-MAPPINGS", "NV-BLOCK-DESCRIPTORS", "DIAGNOSTIC-CONNECTED-INDICATOR", "SINGLE-ELEMENT-SERIALIZATION", "SOVD-DATA-CATEGORY-REF", "METHOD-REFS", "BSW-TIMING-EVENT", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "RVF-PIXEL-FORMAT", "DDS-SERVICE-VERSION", "SEPARATE-SIGNAL-PATH", "NON-OS-MODULE-INSTANTIATION-REF", "DIAGNOSTIC-DO-IP-ENTITY-IDENTIFICATION-INTERFACE", "KEY-EXCHANGE-AUTHENTICATION-REFS", "GLOBAL-TIME-ETH-SLAVE", "DTC-PROPS-REF", "INTERPOLATION-ROUTINE-REF", "END-TO-END-PROTECTION-I-SIGNAL-I-PDUS", "READ-CLASS-REF", "TARGET-COMPONENT-REF", "I-SIGNAL-PORT", "TCP-IP-ICMP-V-6-MSG-PARAMETER-PROBLEM-ENABLED", "SIGNAL-TO-RECEIVER-TEXT-TABLE-MAPPING", "BSW-MODULE-DESCRIPTION-REF-CONDITIONAL", "UNICAST-UDP-CREDENTIALS", "COMMON-SIGNAL-PATH", "MULTICAST-CREDENTIALS", "MAXIMUM-TRANSMIT-BUFFER-LENGTH", "TARGET-REF", "GROUPED-ALIAS-EVENT-REF", "PORT-API-OPTIONS", "NM-CHANNEL-ID", "MAX-SAMPLES-PER-INSTANCE", "VLAN-ID", "ECUC-PARAM-CONF-CONTAINER-DEF", "TIMING-DESCRIPTIONS", "FLEXRAY-AR-TP-CONNECTION", "MULTIPLICITY-RESTRICTION", "OFFSET-TIME-DOMAIN-REF", "ASSERTIONS", "ACF-BUS-REFS", "MAC-SEC-PROPS", "ARBITRATION", "ALIAS-EVENT-REF", "CYCLE-TIME-MAIN-FUNCTION", "SYSTEM-SIGNAL-REF", "ENGINEERING-OBJECTS", "BSW-ENTRY-RELATIONSHIP", "ERROR-MAPPINGS", "AUTOSAR-DATA-TYPE-REF", "COM-METHOD-GRANT-DESIGN", "MIN-DEADLINE", "SW-COMPARISON-VARIABLES", "DIAGNOSTIC-STORAGE-CONDITION-REF-CONDITIONAL", "POSTS", "ALIVE-SUPERVISION", "SECTION-INITIALIZATION-POLICY", "SIGNAL-BASED-FIELD-REF", "CONTEXT-ELEMENT-REF", "DESTINATION-MAC-ADDRESS-MASK", "SHALL-RUN-ON-REF", "PROCESS-TO-MACHINE-MAPPING-SET", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "MODE-SENDER-POLICYS", "SEC-OC-COM-PROPS-FOR-MULTICAST-REF", "RECOMMENDED-CONFIGURATION-REFS", "XREF-TARGET", "N-DATA-SETS", "NEXT-HEADER", "SUSPEND-TO-RAM-HUB-MAPPING", "DIAGNOSTIC-PROPS", "BLOCK-IF-STATE-ACTIVE-AP-IREF", "BYPASS-ETHER-TYPES", "PACKAGER-ID", "SSP-OFFSET", "DO-IP-SOURCE-ADDRESS-REF", "REMOTE-ENDPOINT-CONFIGURATION", "NM-SYNCHRONIZING-NETWORK", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "CRYPTO-SERVICE-MAPPINGS", "ACTIVE-SUPERVISION-REFS", "PCM-BIT-DEPTH", "CALC-RAM-BLOCK-CRC", "SA-RAND-TIME", "DIAGNOSTIC-SOVD-METHOD", "TYPE-APPROVAL", "CONTROLLED-DATA-ELEMENT-REFS", "CHANNELS-PER-FRAME", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "UDP-COLLECTION-BUFFER-SIZE-THRESHOLD", "TIME-INTERVAL", "SYSTEM-REF", "SIGNAL-BASED-FIELD-DEPLOYMENT", "BYPASS-VLANS", "RELEVANT-FOR-DYNAMIC-PNC-MAPPING-REFS", "STATE-REFERENCES", "DIAGNOSTIC-FIM-ALIAS-EVENT", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "CAN-XL-NM-PROPS", "DATA-TYPE-MAP", "CONSUMED-EVENT-GROUPS", "TARGET-POSSIBLE-ERROR-REF", "MEMORY-ID", "SECURE-PROPS-FOR-TCP-REF", "AUTHENTICATION-REF", "TCP-SYN-RECEIVED-TIMEOUT", "FLEXRAY-FRAME-TRIGGERING", "DIAGNOSTIC-TROUBLE-CODE-UDS", "EXECUTION-ORDER-CONSTRAINT", "DIAGNOSTIC-SOVD-LOG", "TIMEOUT-CHECK", "DYNAMIC-ACTION-REF", "NV-BLOCK-DESCRIPTOR", "ENVIRONMENT-CAPTURE-TO-REPORTING", "PORT-NUMBER", "TARGET-P-PORT-REF", "LOCAL-ENDPOINT-CONFIG", "META-DATA-LENGTH", "MODE-DECLARATIONS", "CLIENT-SERVER-OPERATION-REF", "CHECKPOINT-TRANSITION", "PDU-POOLS", "CONTAINER-TIMEOUT", "LIN-UNCONDITIONAL-FRAME-REF", "WORST-CASE-EXECUTION-TIME", "ACCESS-POINT-REF", "AUTO-IP-PROPS", "DATA-PROTOTYPE-WITH-APPLICATION-DATA-TYPE-IN-SYSTEM-REF", "I-PDU-REF", "DDS-TOPIC-ACCESS-RULE", "HW-COMMUNICATION-CONTROLLER-REF", "ANNOTATION-TEXT", "HANDLE-TIMEOUT-TYPE", "RTP-TP", "SDU-REFS", "METHOD-REQUEST-TOPIC-NAME", "APPLICATION-RECORD-ELEMENT-REF", "FUNCTION-GROUP-TRANSITION-REQUEST-FAILED-ERROR-REF", "FRAME-LENGTH", "COUPLING-PORT-CREDIT-BASED-SHAPER", "FUNCTION-GROUP-IREF", "OS-ARTI-ADAPTER-LAUNCH-BEHAVIOR", "ICMP-V-4-PROPS", "MINOR-PROTOCOL-VERSION", "STATE-MANAGEMENT-EM-ERROR-INTERFACE", "ALLOCATE-SHADOW-COPY", "VALUE-CONFIG-CLASSES", "EVENT-WINDOW-TIME", "ICV-VERIFICATION", "DIAGNOSTIC-SOVD-OPERATION", "SO-AD-ROUTING-GROUP", "REQUEST-DOWNLOAD-CLASS-REF", "REPRESENTS-VIN", "PORT-INSTANTIATION-BEHAVIOR", "FT", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "SPECIFIC-CHANNELS", "AGING-ALLOWED", "REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-REF", "ADDRESS-CLAIM-ENABLED", "TP-CHANNEL-REF", "RECEIVE-ACTIVITY", "SUPERVISION-MODE", "GRANT-REF", "START-AGENT-ERROR-REF", "ETHERNET-COMMUNICATION-CONTROLLER-CONDITIONAL", "SECTION-TYPE", "SW-SERVICE-ARG", "NOMINAL", "IEEE-1722-ACF-BUS-RAW-DATA-STREAM-CONSUMER-MAPPING", "EXECUTION-CONTEXT-REF", "OFFSET-CORRECTION-JUMP-THRESHOLD", "PORT-INTERFACE-MAPPINGS", "ELEMENT-GROUPS", "SYSTEM-SIGNAL-REFS", "INITIAL-DYNAMIC-PART", "RPT-COMPONENTS", "PR-PORT-PROTOTYPE", "SIGNAL-BASED-EVENT-REF", "FUNCTIONAL-REQUEST-REFS", "SOVD-GROUP-REFS", "UPPER-MULTIPLICITY-IS-INFINITE", "TIMEOUT-CR", "BSW-MODULE-TIMING", "MIRROR-STATUS-CAN-ID", "TCP-IP-ARP-PACKET-QUEUE-ENABLED", "EXCLUSIVE-AREA-NESTING-ORDERS", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "EVENT-TRIGGERING-REF", "LANGUAGE", "STATE-MANAGEMENT-COMPARE-FORMULA", "TCP-KEEP-ALIVE-INTERVAL", "IDSM-TRAFFIC-LIMITATION-REF", "NM-MSG-CYCLE-OFFSET", "NETWORK-ENDPOINT-ADDRESSES", "TX-DIRECT-PDU-PRIORITY", "SETTER-RETURN-REF", "RX-PDU-POOL-REF", "EXCLUSIVE-AREA-NESTING-ORDER-REFS", "MAC-UNICAST-ADDRESS", "MAX-BYTES-IN-INTERVAL", "DLT-ARGUMENT-ENTRYS", "LOG-TRACE-LOG-MODE", "DIAGNOSTIC-FIM-EVENT-GROUP", "SIGNAL-SERVICE-TRANSLATION-PROPSS", "DOIP-CONNECTION-REFS", "USED-SECONDARY-FID-REFS", "DATA-IDS", "FIREWALL-STATE-SWITCH-INTERFACE", "EXCLUSIVE-AREA-REFS", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "MODE-DECLARATION-MAPPINGS", "LIFE-CYCLE-INFO", "IP-V-4-ADDRESS", "NM-GLOBAL-COORDINATOR-TIME", "ALLOWED-TCP-OPTION", "I-SIGNAL-TO-I-PDU-MAPPINGS", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "MSR-QUERY-P-2", "ITEMS", "NM-NODE-REFS", "CLIENT-SERVER-INTERFACE-REF", "I-SIGNAL-PORT-REF", "NM-DATA-CYCLE", "ENCODING", "CONSUMED-FIELDS", "SECURITY-EVENT-REF", "SAMPLES-PER-MICROTICK", "SWITCH-STREAM-IDENTIFICATION", "PDUR-TP-CHUNK-SIZE", "PWM-L", "DATA-TYPE-REF", "PDU-COLLECTION-MAX-BUFFER-SIZE", "DIAGNOSTIC-AUTH-TRANSMIT-CERTIFICATE-MAPPING", "CRC-FLAGS", "EVENT-MULTICAST-UDP-PORT", "LABEL", "MESSAGE-LINK-LENGTH", "PROCESSORS", "POST-BUILD-VARIANT-CONDITIONS", "NETWORK-HANDLE-PORT-MAPPING", "MASS-EXP", "SOMEIP-TRANSFORMATION-PROPS", "TOPIC-NAME", "DERIVED-CERTIFICATE-ACCEPTED", "ALLOWED-I-PV-6-EXT-HEADER", "SOURCE-MAC-ADDRESS", "IMPLEMENTED-IN-REF", "REQUIRED-EVENT-GROUPS", "HW-TYPE-REF", "TX-BIT-RATE-SWITCH", "DYNAMIC-ACTION-REFS", "SW-GENERIC-AXIS-PARAM-TYPE", "MSR-QUERY-ARG", "I-SIGNAL-PORT-TO-DIAGNOSTIC-EVENT-MAPPING", "RECEIVED-DATA-REF", "EVENT-PROPSS", "BOR-TIME-L-1", "NM-USER-DATA-LENGTH", "AGGREGATION-TAILORING", "TD-EVENT-BSW-MODULE", "JOB-PRIMITIVE-REF", "TRACE-SWITCH-CONFIGURATIONS", "DIAGNOSTIC-VALUE-ACCESS", "CRYPTO-DESIGN-TO-CRYPTO-DRIVER-MAPPINGS", "STREAM-GATE-REF", "POSSIBLE-ERROR-REF", "LITERALS", "EVENT-SOURCE-REF", "DEADLINE-PERIOD", "RESPOND-TO-RESET", "FRAME-TRIGGERING-REF", "CRF-TYPE", "COMPONENT-IMPLEMENTATION-REF", "BSW-SCHEDULER-NAME-PREFIX", "TD-EVENT-TRIGGER-TYPE", "FLOAT-VALUE-VARIATION-POINT", "CONSTANT-REFERENCE", "NM-REPEAT-MESSAGE-TIME", "TIMING-ARGUMENT-REF", "SECOND-OPERATION-REF", "MAXIMUM-MEMORY-CONSUMPTION", "TCP-IP-NDP-MAX-RANDOM-FACTOR", "CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPING", "DEFAULT-VALUE-ELEMENT", "CONTEXT-APPLICATION-RECORD-ELEMENT-REF", "SW-DATA-DEPENDENCY-FORMULA", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "DEPENDS-ONS", "RETURN-NO-NEW-DATA-ENABLED", "ACF-TRANSPORTED-BUSS", "IO-HW-ABSTRACTION-SERVER-ANNOTATION", "TARGET-RUNNABLE-ENTITY-REF", "DIAGNOSTIC-ENV-MODE-CONDITION", "GROUP-IDENTIFIER", "INTRODUCTION", "UPDATE", "OBD-RATIO-DENOMINATOR-NEEDS", "TRIGGER-INTERFACE", "NOTIFICATION-STATE", "DIAG-REQUIREMENT", "PDUR-I-PDU-GROUP", "CAN-CLUSTER-VARIANTS", "CYCLE-STATUS-STORAGE", "RELATED-TRACE-ITEM-REF", "INVALID-VALUE", "GROUP-ID", "TIMING-RESOURCE", "DO-IP-RULE", "SERVICE-INTERFACE-ID", "IP-V-6-PATH-MTU-ENABLED", "COLLECTION-SEMANTICS", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "FLEXRAY-AR-TP-CONFIG", "REFERENCE-REF", "DDS-METHOD-REQUEST-TOPIC-REF", "TRANSFORMER-CHAIN-REF", "I-PDU-MAPPINGS", "GENERAL-PURPOSE-TIMER-SERVICE-NEEDS", "LOG-TRACE-FILE-PATH", "SWITCH-STREAM-IDENTIFICATIONS", "SYNCHRONOUS-SERVER-CALL-POINT", "SOFTWARE-CLUSTER-COM-RESOURCE-REF", "MORE-FRAGMENTS", "TRANSIENT-FAULT", "IMPLEMENTATION-DATA-TYPE-SUB-ELEMENT-REF", "DHCP-ADDRESS-ASSIGNMENT", "PERSISTENCY-PROVIDED-COM-SPEC", "NETWORK-CONFIGURATIONS", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "SAFETY-MARGIN", "I-SIGNAL-GROUP-REF", "CLIENT-ID-DEFINITION-SET-REF", "SW-ARRAYSIZE", "RESPONSE-ON-EVENT-CLASS-REF", "USE-CRC-COMP-MECHANISM", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG-REF-CONDITIONAL", "ACTION-LIST-ITEM-REF", "CRF-PULL", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "ECUC-FLOAT-PARAM-DEF", "TCP-RULE", "EXCLUSIVE-AREA-REF", "COMPU-PHYS-TO-INTERNAL", "TRANSPORT-PLUGIN", "KEEP-ALIVE-INTERVAL", "DIAGNOSTIC-SOVD-CONTENT-GROUP", "MINIMUM-EXECUTION-TIME", "LIVELINESS-PROTECTION-KIND", "SEPARATED-COMPONENT-IREF", "PROCESSING-UNIT-REF", "MAX-ERROR-STATE-INIT", "ECU-REF", "AUTOSAR-CONNECTOR", "DO-IP-TESTER-ROUTING-ACTIVATION-REF", "ECUC-INTEGER-PARAM-DEF", "PNC-PREPARE-SLEEP-TIMER", "DIAGNOSTIC-STORAGE-CONDITION-REF", "NV-RAM-BLOCK-ELEMENT", "UPDATE-SESSION-REJECTED", "DO-IP-DESIGN-REF", "OPTIONAL", "ARTIFACT-DESCRIPTOR", "COMPU-CONST", "CONSTANT-VALUE-MAPPING-REFS", "SUBSTITUTED-FRAME-REFS", "MKA-PARTICIPANT-REF", "ROOT-SW-COMPOSITION-PROTOTYPE", "STARTUP-CONFIG-REF", "CONTAINED-I-SIGNAL-I-PDU-GROUP-REFS", "TCP-REMOTE-PORT", "CONFIDENTIALITY-OFFSET", "HW-ELEMENT-REF", "FLEXRAY-COMMUNICATION-CONTROLLER", "ECUC-INSTANCE-REFERENCE-VALUE", "CONNECTION-TYPE", "CRYPTO-TRUST-MASTER-INTERFACE", "ERROR-CONTEXT-REFS", "TIME-FR-IF", "COUPLING-PORT-REF", "ALLOW-UNAUTHENTICATED-PARTICIPANTS", "DURABILITY-SERVICE-MAX-SAMPLES", "BSW-MODULE-DESCRIPTION", "RECORD-VALUE-SPECIFICATION", "LIN-MASTER", "SLEEP-REPETITIONS-OF-SLEEP-REQUEST", "SW-REFRESH-TIMING", "CONTROLLED-DATA-ELEMENT-REF", "P-2-STAR-SERVER-MAX", "TCP-IP-ICMP-V-4-ECHO-REPLY-ENABLED", "RAM-BLOCK-STATUS-CONTROL", "SETTER-RETURN-SIGNAL-REF", "CONTEXT-DATA-PROTOTYPE-REFS", "END-TO-END-CALL-RESPONSE-TIMEOUT", "IDSM-CONTEXT-PROVIDER-MAPPING", "DLT-ARGUMENT-PROPS-REF-CONDITIONAL", "CRYPTO-PROVIDERS", "LOCAL-SUPERVISION-ENTITY-REFS", "VARIABLE-DATA-PROTOTYPE-IREF", "NEIGHBOR-CACHE-SIZE", "IPV-6-DHCP-SERVER-CONFIGURATION", "SOFTWARE-CLUSTER-DEPENDENCY-COMPARE-CONDITION", "HW-PIN", "UPPER-LIMIT", "IS-STRUCT-WITH-OPTIONAL-ELEMENT", "RPT-HOOK", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "COM-CONFIGURATION-RX-TIME-BASE", "TCP-CONNECT-TIMEOUT", "FIELD-GET-SET-QOS-PROPSS", "IKE-AUTHENTICATION-METHOD", "DIAGNOSTIC-SOVD-RECORD-CONTENT-ELEMENT", "RATIONALE", "TRANSMISSION-TRIGGER", "DIAGNOSTIC-MEMORY-REF", "DIAGNOSTIC-SOVD-CONFIG-CONTENT-MAPPING", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "COM-FIND-SERVICE-GRANT", "BSW-DATA-SEND-POLICY", "SERVER-NAME-IDENTIFICATION", "I-SIGNAL-PROPS", "BINARY-MANIFEST-REQUIRE-RESOURCE", "FLEXRAY-TP-ECU", "MEM-ALIGNMENT", "STORING", "AGE-CONSTRAINT", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG-REF", "DLT-ARGUMENTS", "CONTENT-TYPE", "SOCKET-ADDRESS-REF", "RECEIVER-IREFS", "FM-FEATURE", "CRYPTO-KEY-SLOT-INTERFACE", "NM-STATE-REQUEST", "CRYPTO-KEY-SLOT-USAGE-DESIGN-MAPPING", "BURST-PATTERN-EVENT-TRIGGERING", "IMPLICIT-DATA-ACCESS-IREF", "DDS-EVENT-DEPLOYMENT", "RPT-PROFILE", "SW-CALIBRATION-ACCESS", "SEC-OC-SECURE-COM-PROPS", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-PORT-MAPPING", "SOCKET-CONNECTION", "DELAY-COMPENSATION-B", "CONSUMED-DDS-OPERATIONS", "SEARCH-BEHAVIOR", "BSW-INTERNAL-TRIGGERING-POINT-POLICY", "NETWORK-CONFIGURATION", "INTER-MESSAGE-TIME", "DIAGNOSTIC-SOVD-AUTHORIZATION-PORT-MAPPING", "STORAGE-CONDITIONS", "IMPLEMENTATION-DATA-TYPE-ELEMENT-IN-PORT-INTERFACE-REF", "DYNAMIC-PART-ALTERNATIVES", "TCP-SLOW-START-ENABLED", "DO-IP-POWER-MODE-STATUS-NEEDS", "DURABILITY-SERVICE-CLEANUP-DELAY", "TRIGGER-TO-SIGNAL-MAPPING", "ACCEPTED-CRYPTO-CIPHER-SUITE-WITH-PSK-REFS", "FIREWALL-REF", "SYNCHRONIZATION-CONSTRAINT-TYPE", "MIN-OK-STATE-INVALID", "CHECKPOINT-REF", "PORT-PROTOTYPE-SLUG-FRAGMENT", "TERMINATE", "VCID", "MAX-PERIODIC-DID-TO-READ", "DEFAULT-GATEWAY", "DDS-PARTITIONS", "MAX-SAMPLES", "MAX-TRCV-DELAY-COMPENSATION-OFFSET", "MODE-REQUEST-TYPE-MAP", "DESTINATION-TYPE", "DIRECT-SUCCESSOR-REF", "ETHERNET-CLUSTER", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "MAX-PWM-S", "RESOURCE-IDENTIFIER-TYPE", "ADMIT-WITHOUT-MESSAGE-ID", "ASSOCIATED-EVENT-IDENTIFICATION", "AUTHENTICATION-CLASS-REF", "MAX-NUM-COMPARISION-OF-VALUE-EVENTS", "AP-APPLICATION-ENDPOINT", "HOST-REF", "MODEL-OBJECT-REFERENCE", "MAX-NO-NEW-OR-REPEATED-DATA", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "MACHINE-ID", "TEMPLATE-TYPE-REF", "CRC-SEQUENCE-ID", "DURABILITY-SERVICE-HISTORY-DEPTH", "PORT-INTERFACE-REF", "NM-DETECTION-LOCK", "GET", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-PORT-MAPPING", "LIN-SLAVE-ECU-REF", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "TIME-SEG-2", "PLCA-PROPS", "BUILD-ACTION-MANIFEST-REF-CONDITIONAL", "APMC-UPSTREAM-DOC-FOREIGN-REFERENCE-VALUE", "CLEAR-EVENT-ALLOWED-BEHAVIOR", "VALUE-SPEC", "MAP-ELEMENT-TUPLES", "ECUC-QUERY", "DISPLAY-PRESENTATION", "TRANSPORT-PROTOCOL", "FRESHNESS-PROPSS", "GLOBAL-TIME-DOMAIN-REF", "E-2-E-OVERSAMPLING-PERIOD", "CONTEXT-ELEMENT-REFS", "TYPE-OF-EVENT-COMBINATION-SUPPORTED", "REQUEST-EMISSION-RELATED-DTC-CLASS-PERMANENT-STATUS-REF", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "ACCEPTED-REMOTE-CERTIFICATE-REFS", "RECOVERY-VIA-APPLICATION-ACTION", "CONTAINED-FIBEX-ELEMENT-REF", "RESPONSE-ERROR-POSITION", "GENERIC-TP-CONNECTION", "FM-ATTRIBUTE-DEF", "DIAGNOSTIC-SOVD-FAULT-MEMORY-ACCESS", "SW-INTERPOLATION-METHOD", "STD-REFS", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "EXTENDED-DATA-RECORDS", "I-SIGNAL-TRIGGERING", "IDSM-INSTANCE", "ASSOCIATED-PDUR-I-PDU-GROUP-REFS", "HW-PIN-GROUP-CONNECTIONS", "NUMBER-OF-MINISLOTS", "SWC-BEHAVIOR-REF", "REDUNDANCY-HANDLINGS", "FREE-FORMAT", "SECURE-COM-PROPS-FOR-TCP-REF", "TRANCEIVER-STANDBY-DELAY", "ATTRIBUTES", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "TAG-ID", "BEHAVIOR-REF", "END-TO-END-PROFILE", "SDG-REFERENCE", "IDENTIFIABLE-REF", "DISTINGUISHED-PARTITIONS", "REQUEST-ARGUMENT-REF", "COMPARE-TYPE", "DEFAULT-REPORTING-MODE", "CLIENT-ID-DEFINITION", "FIELDS", "NM-BUSLOAD-REDUCTION-ACTIVE", "DEFAULT-APPLICATION-TIMEOUT", "NV-BLOCK-SW-COMPONENT-TYPE", "DATA-FILTER-TYPE", "WRITING-FREQUENCY", "MAPPED-FUNCTION-REF", "INTERFACE-REF", "SERVICE-INTERFACE-MAPPING-SET", "DEF-ITEM", "PARENT-SHORT-LABEL", "INITIAL-REPETITIONS-MAX", "COMPARE-VALUE", "INFORMAL-FORMULA", "IDSM-TIMESTAMP-PROVIDER-MAPPING", "ARGUMENT-REF", "ELEMENT-MAPPINGS", "SERVICE-INTERFACE-MAPPING", "E-2-E-PROFILE-CONFIGURATION-REF", "OBD-CONTROL-SERVICE-NEEDS", "DIAG-FAULT-DETECTION-COUNTER-PORT-REF", "SW-RECORD-LAYOUT-GROUP-INDEX", "IUMPR-GROUP-IDENTIFIERS", "PRIORITY", "FILTER", "IAM-RELEVANT-TLS-SECURE-COM-PROPS-REF", "DATA-PROTOTYPE-TRANSFORMATION-PROPSS", "CAN-ID-TO-CAN-ID-MAPPINGS", "SOMEIP-TRANSFORMATION-I-SIGNAL-PROPS", "CHECKPOINT-ID", "INHIBITING-FID-REF", "REST-STRING-PROPERTY-DEF", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "TLS-SECURE-COM-PROPS", "REFINED-MODULE-DEF-REF", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG-REF", "TD-EVENT-SERVICE-INSTANCE-FIELD-TYPE", "R-PORT-PROTOTYPE-PROPS", "VERSION", "IDSM-REPORTING-MODE-PROVIDER-INTERFACE", "PERSISTENCY-DEPLOYMENT-URI", "OFFER-CYCLIC-DELAY", "MC-GROUP", "DURABILITY-SERVICE-MAX-SAMPLES-PER-INSTANCE", "DEBOUNCE-ALGORITHM", "CYCLIC-HANDLING-COM-DATA-TO-OS-TASK-PROXY-MAPPINGS", "INTERFACE-MAPPINGS", "INNER-GROUP-IREFS", "RUNS-INSIDES", "WORST-CASE-STACK-USAGE", "SAMPLE-CLOCK-PERIOD", "DOC-REVISIONS", "TCP-IP-NDP-NUM-UNICAST-SOLICITATIONS", "LC-OBJECT-REF"];

    /// derive an enum entry from an input string using a perfect hash function
    ///
    /// # Errors
    ///
    /// [`ParseElementNameError`]: The input string did not match the name of any enum item
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseElementNameError> {
        #[rustfmt::skip]
        static DISPLACEMENTS: [(u16, u16); 951] = [(2, 4193), (0, 53), (0, 7), (0, 22), (0, 479), (0, 2), (0, 3), (0, 106), (0, 17), (0, 219), (0, 0), (0, 1), (0, 3), (0, 2), (0, 141), (0, 154), (0, 5851), (0, 3), (0, 127), (0, 11), (0, 3), (0, 11), (0, 1174), (0, 368), (0, 8), (0, 92), (0, 284), (0, 61), (0, 5), (0, 6338), (1, 3344), (0, 1683), (0, 4354), (0, 5), (0, 0), (0, 3815), (0, 126), (0, 2227), (0, 20), (0, 64), (0, 1), (1, 4), (0, 125), (1, 4206), (0, 151), (2, 2508), (1, 5188), (0, 893), (0, 0), (0, 122), (0, 0), (0, 328), (0, 9), (0, 17), (0, 1985), (0, 3), (3, 1058), (7, 4589), (0, 2), (0, 3), (0, 41), (0, 13), (0, 97), (0, 4), (0, 47), (14, 4622), (0, 570), (0, 195), (0, 350), (0, 1), (0, 2686), (0, 553), (0, 576), (0, 804), (0, 0), (0, 2868), (0, 18), (0, 2), (1, 5507), (0, 13), (0, 0), (0, 0), (1, 2074), (0, 5928), (0, 10), (0, 2), (0, 106), (0, 582), (25, 5286), (0, 1161), (0, 3), (0, 366), (3, 3120), (0, 14), (6, 1906), (0, 1864), (0, 49), (8, 1392), (0, 276), (0, 494), (0, 169), (0, 4058), (0, 760), (0, 0), (0, 0), (1, 1588), (0, 151), (0, 17), (0, 0), (0, 551), (0, 118), (1, 2292), (0, 1), (0, 0), (0, 4), (0, 2), (0, 4019), (0, 13), (0, 0), (0, 195), (0, 5089), (0, 622), (0, 4), (0, 2716), (0, 21), (0, 1793), (0, 976), (2, 2858), (0, 4836), (1, 241), (0, 0), (0, 260), (4, 3155), (0, 0), (0, 1038), (0, 723), (0, 127), (0, 57), (0, 2), (0, 1), (0, 0), (0, 61), (0, 638), (0, 541), (2, 6446), (0, 1), (0, 287), (0, 306), (1, 168), (0, 192), (10, 358), (0, 6076), (0, 13), (0, 9), (0, 1245), (0, 1), (0, 9), (0, 141), (0, 651), (0, 417), (4, 315), (2, 1109), (0, 400), (0, 195), (8, 3106), (8, 4426), (5, 1128), (0, 944), (1, 1386), (5, 4725), (0, 3831), (4, 2666), (0, 1856), (0, 0), (0, 5238), (0, 2720), (0, 30), (0, 133), (0, 1), (0, 756), (2, 1608), (0, 0), (0, 58), (0, 60), (0, 138), (0, 5101), (0, 40), (30, 4119), (11, 1454), (0, 6), (0, 0), (0, 248), (0, 5), (10, 5), (0, 4962), (0, 532), (3, 1672), (0, 6281), (0, 2), (24, 560), (0, 2), (8, 2827), (0, 26), (0, 4), (0, 1), (1, 37), (2, 932), (0, 16), (19, 4119), (0, 89), (0, 1), (0, 942), (0, 2), (0, 0), (0, 3795), (1, 3472), (0, 166), (0, 323), (21, 6194), (0, 1162), (0, 3), (0, 27), (0, 1449), (0, 112), (0, 2303), (2, 148), (0, 70), (1, 3802), (4, 620), (0, 87), (0, 48), (0, 924), (0, 123), (0, 360), (0, 2), (0, 1463), (0, 11), (0, 122), (10, 3136), (2, 2523), (0, 22), (0, 6), (0, 5), (0, 0), (0, 0), (2, 1825), (0, 1180), (33, 1443), (0, 10), (0, 2), (0, 238), (0, 19), (0, 318), (0, 2861), (0, 49), (1, 1821), (0, 1094), (0, 1070), (0, 2), (0, 10), (0, 0), (0, 2288), (0, 1), (0, 663), (2, 4675), (0, 2928), (0, 459), (0, 2151), (0, 18), (4, 5699), (1, 4666), (7, 2141), (0, 2822), (0, 19), (0, 5205), (0, 9), (7, 6324), (0, 1541), (9, 884), (0, 647), (0, 27), (0, 79), (1, 4991), (0, 4), (0, 1943), (0, 5039), (0, 1399), (0, 382), (0, 10), (0, 7), (0, 869), (6, 4014), (0, 8), (0, 23), (0, 0), (0, 3651), (0, 129), (4, 694), (0, 1262), (0, 0), (0, 163), (0, 38), (0, 1626), (0, 162), (0, 1), (6, 823), (0, 6), (0, 7), (0, 66), (5, 5424), (0, 8), (0, 0), (0, 135), (0, 27), (0, 1), (0, 183), (0, 0), (0, 60), (0, 123), (0, 42), (0, 36), (2, 342), (2, 658), (0, 28), (0, 137), (0, 6), (1, 231), (0, 5172), (2, 3574), (0, 1037), (0, 0), (0, 2925), (12, 1375), (0, 0), (0, 19), (0, 9), (0, 653), (0, 4), (1, 265), (0, 66), (0, 1823), (0, 28), (0, 62), (30, 2915), (0, 25), (0, 4), (0, 3), (0, 0), (0, 5948), (0, 14), (0, 4958), (2, 1056), (0, 3), (3, 6011), (0, 18), (0, 259), (0, 619), (31, 128), (0, 239), (0, 3907), (0, 302), (8, 1729), (0, 28), (0, 3834), (0, 28), (10, 2896), (0, 3), (0, 4252), (0, 550), (0, 28), (0, 2), (0, 3156), (0, 1575), (0, 651), (0, 1546), (0, 0), (50, 25), (0, 1), (0, 1634), (0, 454), (6, 4573), (1, 2799), (0, 37), (0, 187), (0, 2), (1, 5259), (11, 4264), (0, 39), (0, 2533), (0, 1908), (0, 277), (0, 158), (5, 5558), (0, 1), (1, 2528), (7, 2119), (0, 10), (41, 4619), (2, 1106), (0, 4455), (0, 1479), (0, 4448), (1, 792), (0, 1378), (0, 177), (0, 78), (0, 1106), (0, 2), (0, 3586), (0, 209), (0, 1345), (0, 205), (1, 5912), (0, 6021), (0, 81), (0, 3466), (0, 0), (0, 45), (7, 4914), (0, 3682), (3, 715), (0, 255), (0, 638), (0, 73), (5, 246), (0, 19), (0, 1), (0, 463), (1, 1812), (0, 11), (0, 215), (0, 2407), (0, 1108), (0, 144), (1, 6397), (0, 635), (0, 84), (7, 3207), (0, 2), (0, 668), (0, 15), (0, 3278), (4, 1294), (0, 60), (0, 668), (0, 2684), (0, 17), (0, 185), (0, 9), (0, 1691), (1, 4634), (0, 1), (42, 1790), (0, 117), (0, 160), (0, 110), (0, 0), (0, 1030), (0, 0), (0, 1983), (4, 2611), (7, 5539), (0, 38), (0, 655), (2, 6553), (9, 4663), (0, 1545), (52, 2415), (0, 0), (0, 3), (8, 1862), (0, 3), (0, 654), (0, 336), (34, 2351), (11, 2989), (29, 3406), (10, 746), (0, 42), (0, 76), (0, 2795), (0, 179), (34, 4639), (18, 5486), (10, 4001), (0, 40), (0, 80), (4, 136), (31, 4581), (0, 351), (0, 35), (0, 1334), (0, 503), (20, 970), (0, 442), (0, 3), (0, 35), (0, 9), (3, 894), (5, 4110), (0, 1), (0, 0), (0, 2167), (0, 1071), (0, 1181), (9, 3586), (0, 10), (0, 6), (11, 4130), (14, 4949), (0, 5740), (0, 10), (0, 152), (0, 377), (42, 1329), (0, 55), (0, 966), (0, 286), (0, 238), (0, 26), (0, 67), (13, 194), (0, 50), (2, 2628), (20, 1307), (1, 6137), (0, 18), (0, 266), (0, 8), (0, 524), (0, 350), (0, 1663), (0, 28), (0, 15), (9, 17), (4, 4041), (0, 287), (16, 6553), (0, 1671), (0, 1014), (0, 1182), (0, 136), (0, 2967), (0, 2851), (0, 149), (14, 6077), (23, 5145), (0, 1135), (2, 4201), (7, 6249), (6, 2033), (0, 6445), (0, 20), (41, 6274), (0, 610), (0, 43), (3, 5937), (69, 1375), (73, 3461), (0, 5032), (0, 1784), (0, 682), (0, 157), (0, 5670), (0, 490), (0, 28), (1, 4777), (0, 2856), (24, 3743), (4, 2500), (0, 39), (0, 2), (53, 2543), (85, 3132), (0, 11), (0, 18), (0, 0), (0, 39), (0, 713), (23, 4050), (0, 0), (90, 3024), (0, 41), (91, 2351), (0, 3779), (0, 311), (0, 1042), (0, 4498), (0, 1027), (0, 2932), (0, 16), (25, 3975), (0, 4), (0, 3768), (0, 5238), (18, 5508), (0, 119), (0, 0), (4, 4464), (0, 4838), (82, 608), (0, 40), (130, 2240), (0, 1982), (0, 19), (0, 171), (4, 2051), (0, 323), (1, 2707), (0, 12), (8, 3506), (1, 4899), (0, 60), (0, 497), (0, 52), (0, 1030), (0, 151), (22, 474), (0, 6249), (43, 2601), (0, 669), (5, 3794), (89, 5292), (50, 3194), (0, 14), (0, 199), (0, 105), (0, 527), (0, 168), (0, 2040), (0, 77), (0, 1058), (1, 6331), (0, 680), (0, 70), (2, 203), (0, 9), (0, 2264), (1, 2258), (4, 4417), (0, 316), (0, 5348), (0, 18), (0, 51), (0, 178), (0, 17), (2, 6237), (16, 5874), (0, 1), (0, 2), (29, 1995), (89, 1098), (2, 6649), (0, 6610), (0, 1983), (125, 515), (12, 559), (0, 1), (13, 2486), (0, 5216), (0, 338), (0, 3479), (0, 2), (0, 134), (0, 0), (0, 0), (7, 5836), (0, 1), (0, 19), (0, 0), (0, 252), (23, 4042), (0, 323), (46, 5249), (0, 116), (3, 5672), (0, 2004), (29, 3528), (72, 1854), (82, 3504), (0, 2412), (0, 13), (162, 2926), (2, 437), (0, 22), (0, 6), (0, 5), (32, 1735), (0, 181), (10, 1125), (98, 4380), (0, 32), (1, 1922), (0, 62), (0, 2696), (0, 1), (112, 1797), (0, 2), (0, 66), (0, 1975), (1, 103), (0, 4029), (6, 6636), (0, 29), (0, 30), (0, 23), (0, 22), (54, 5703), (0, 39), (0, 1624), (0, 2767), (0, 823), (24, 5798), (0, 1), (2, 195), (51, 1644), (15, 4934), (0, 1349), (0, 11), (0, 81), (0, 5913), (0, 426), (0, 589), (0, 296), (2, 95), (0, 37), (0, 26), (134, 5131), (42, 6380), (0, 1248), (4, 3152), (0, 1), (0, 855), (0, 23), (7, 4262), (0, 0), (61, 4994), (0, 1), (0, 14), (0, 8), (18, 5320), (0, 77), (93, 4952), (2, 4819), (18, 573), (0, 1212), (33, 622), (0, 19), (4, 5895), (0, 1362), (0, 13), (39, 1636), (0, 196), (0, 4689), (2, 478), (0, 285), (0, 2351), (9, 950), (1, 3027), (155, 6107), (25, 3776), (0, 501), (0, 47), (0, 926), (147, 381), (221, 5206), (0, 15), (5, 5547), (144, 4532), (0, 1167), (0, 3796), (0, 687), (0, 276), (7, 4967), (0, 5), (0, 0), (39, 5360), (0, 1798), (0, 155), (31, 4203), (6, 1272), (378, 3761), (7, 5989), (23, 2429), (0, 26), (0, 80), (8, 4638), (5, 979), (0, 306), (0, 6368), (0, 21), (127, 5975), (0, 26), (0, 125), (0, 899), (0, 23), (0, 399), (0, 1), (139, 4898), (0, 0), (0, 163), (0, 20), (37, 2596), (0, 50), (1, 3754), (81, 798), (0, 0), (3, 2513), (0, 47), (57, 3680), (0, 57), (14, 1194), (0, 18), (0, 100), (72, 6354), (0, 1114), (0, 95), (0, 24), (0, 1), (0, 2), (0, 0), (0, 2), (0, 6475), (0, 0), (0, 417), (0, 325), (0, 36), (34, 4679), (0, 976), (14, 5328), (77, 2164), (84, 5502), (33, 5862), (0, 19), (6, 1877), (0, 11), (7, 4717), (4, 2513), (6, 5784), (0, 1222), (24, 453), (212, 6260), (0, 1593), (0, 3657), (0, 0), (0, 68), (0, 4483), (0, 96), (244, 6535), (37, 1033), (71, 3452), (124, 6079), (0, 768), (4, 2365), (0, 0), (692, 2853), (0, 5944), (0, 4), (0, 1254), (5, 875), (0, 11), (0, 0), (0, 3173), (0, 477), (0, 5), (0, 2021), (0, 1522), (2, 3631), (47, 5292), (0, 0), (144, 231), (62, 805), (0, 281), (0, 155), (0, 640), (0, 95), (0, 103), (0, 8), (0, 143), (0, 0), (0, 537), (45, 4308), (0, 19), (0, 948), (23, 2910), (0, 0), (0, 1371), (1, 3507), (0, 24), (27, 6128), (125, 2955), (0, 82), (0, 132), (0, 5753), (0, 1), (0, 6), (0, 1229), (73, 1356), (0, 1435), (126, 4589), (4, 3470), (0, 109), (15, 3107), (0, 205), (0, 27), (101, 2573), (268, 4476), (0, 513), (0, 2175), (9, 5676), (0, 5698), (0, 0), (8, 5188), (1, 153), (2, 6574), (0, 1097), (223, 5747), (1, 916), (0, 267), (0, 230), (11, 3165), (98, 1489), (3, 1986), (0, 24), (0, 2), (3, 1316), (0, 5353), (9, 3694), (67, 362), (0, 132), (0, 81), (1, 4145), (0, 37), (1127, 1024), (4, 5947), (60, 243), (0, 33), (0, 2702), (0, 1610), (17, 5080), (2, 4091), (74, 1597), (6, 2906), (0, 5), (0, 1325), (0, 17), (144, 4060), (5, 2706), (0, 0), (0, 2798), (0, 26)];

        let (g, f1, f2) = hashfunc(input);
        let (d1, d2) = DISPLACEMENTS[(g % 951) as usize];
        let item_idx = u32::from(d2)
            .wrapping_add(f1.wrapping_mul(u32::from(d1)))
            .wrapping_add(f2) as usize
            % 6651;
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
