use crate::hashfunc;

#[derive(Debug)]
/// The error type ParseEnumItemError is returned when from_str() / parse() fails for EnumItem
pub struct ParseEnumItemError;

#[allow(dead_code, non_camel_case_types)]
#[derive(Clone, Copy, Eq, PartialEq, Hash)]
#[repr(u16)]
/// Enum of all possible enum values in Autosar
pub enum EnumItem {
    /// -500-MILES
    _500Miles                                                              = 0,
    /// 1000BASE-T
    _1000baseT                                                             = 1,
    /// 1000BASE-T1
    _1000baseT1                                                            = 2,
    /// 100BASE-T1
    _100baseT1                                                             = 3,
    /// 100BASE-TX
    _100baseTx                                                             = 4,
    /// 10BASE-T1S
    _10baseT1s                                                             = 5,
    /// AA
    Aa                                                                     = 6,
    /// AB
    Ab                                                                     = 7,
    /// ABSTRACT
    Abstract                                                               = 8,
    /// ABSTRACT-ACCESS-POINT
    AbstractAccessPoint                                                    = 9,
    /// ABSTRACT-CAN-CLUSTER
    AbstractCanCluster                                                     = 10,
    /// ABSTRACT-CAN-COMMUNICATION-CONNECTOR
    AbstractCanCommunicationConnector                                      = 11,
    /// ABSTRACT-CAN-COMMUNICATION-CONTROLLER
    AbstractCanCommunicationController                                     = 12,
    /// ABSTRACT-CAN-PHYSICAL-CHANNEL
    AbstractCanPhysicalChannel                                             = 13,
    /// ABSTRACT-CLASS-TAILORING
    AbstractClassTailoring                                                 = 14,
    /// ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS
    AbstractDoIpLogicAddressProps                                          = 15,
    /// ABSTRACT-ETHERNET-FRAME
    AbstractEthernetFrame                                                  = 16,
    /// ABSTRACT-EVENT
    AbstractEvent                                                          = 17,
    /// ABSTRACT-EXECUTION-CONTEXT
    AbstractExecutionContext                                               = 18,
    /// ABSTRACT-IAM-REMOTE-SUBJECT
    AbstractIamRemoteSubject                                               = 19,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE
    AbstractImplementationDataType                                         = 20,
    /// ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT
    AbstractImplementationDataTypeElement                                  = 21,
    /// ABSTRACT-PROVIDED-PORT-PROTOTYPE
    AbstractProvidedPortPrototype                                          = 22,
    /// ABSTRACT-RAW-DATA-STREAM-INTERFACE
    AbstractRawDataStreamInterface                                         = 23,
    /// ABSTRACT-REQUIRED-PORT-PROTOTYPE
    AbstractRequiredPortPrototype                                          = 24,
    /// ABSTRACT-SECURITY-EVENT-FILTER
    AbstractSecurityEventFilter                                            = 25,
    /// ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER
    AbstractSecurityIdsmInstanceFilter                                     = 26,
    /// ABSTRACT-SERVICE-INSTANCE
    AbstractServiceInstance                                                = 27,
    /// ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING
    AbstractSignalBasedToISignalTriggeringMapping                          = 28,
    /// ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE
    AbstractSynchronizedTimeBaseInterface                                  = 29,
    /// ACCEPT-ALL
    AcceptAll                                                              = 30,
    /// ACCEPT-CONFIGURED
    AcceptConfigured                                                       = 31,
    /// ACCES-PERRMISSION-SERVICE-CLASS
    AccesPerrmissionServiceClass                                           = 32,
    /// ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS
    AccessPermissionInstanceOverridesClass                                 = 33,
    /// ACCESS-PERMISSION-SERVICE-CLASS
    AccessPermissionServiceClass                                           = 34,
    /// ACCESS-PERMISSION-SERVICE-INSTANCE
    AccessPermissionServiceInstance                                        = 35,
    /// ACK-WITH-RT
    AckWithRt                                                              = 36,
    /// ACK-WITHOUT-RT
    AckWithoutRt                                                           = 37,
    /// ACL-OBJECT-SET
    AclObjectSet                                                           = 38,
    /// ACL-OPERATION
    AclOperation                                                           = 39,
    /// ACL-PERMISSION
    AclPermission                                                          = 40,
    /// ACL-ROLE
    AclRole                                                                = 41,
    /// ACTION
    Action                                                                 = 42,
    /// ACTION-ITEM
    ActionItem                                                             = 43,
    /// ACTION-LIST
    ActionList                                                             = 44,
    /// ACTIVATE
    Activate                                                               = 45,
    /// ACTIVATION-AND-TRIGGER-UNICAST
    ActivationAndTriggerUnicast                                            = 46,
    /// ACTIVATION-MULTICAST
    ActivationMulticast                                                    = 47,
    /// ACTIVATION-UNICAST
    ActivationUnicast                                                      = 48,
    /// ACTIVE
    Active                                                                 = 49,
    /// ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE
    AdaptiveApplicationSwComponentType                                     = 50,
    /// ADAPTIVE-AUTOSAR-APPLICATION
    AdaptiveAutosarApplication                                             = 51,
    /// ADAPTIVE-EVENT-RECEIVED
    AdaptiveEventReceived                                                  = 52,
    /// ADAPTIVE-EVENT-SENT
    AdaptiveEventSent                                                      = 53,
    /// ADAPTIVE-FIELD-GETTER-CALLED
    AdaptiveFieldGetterCalled                                              = 54,
    /// ADAPTIVE-FIELD-GETTER-COMPLETED
    AdaptiveFieldGetterCompleted                                           = 55,
    /// ADAPTIVE-FIELD-NOTIFICATION-RECEIVED
    AdaptiveFieldNotificationReceived                                      = 56,
    /// ADAPTIVE-FIELD-NOTIFICATION-SENT
    AdaptiveFieldNotificationSent                                          = 57,
    /// ADAPTIVE-FIELD-SETTER-CALLED
    AdaptiveFieldSetterCalled                                              = 58,
    /// ADAPTIVE-FIELD-SETTER-COMPLETED
    AdaptiveFieldSetterCompleted                                           = 59,
    /// ADAPTIVE-METHOD-CALL-RECEIVED
    AdaptiveMethodCallReceived                                             = 60,
    /// ADAPTIVE-METHOD-CALLED
    AdaptiveMethodCalled                                                   = 61,
    /// ADAPTIVE-METHOD-RESPONSE-RECEIVED
    AdaptiveMethodResponseReceived                                         = 62,
    /// ADAPTIVE-METHOD-RESPONSE-SENT
    AdaptiveMethodResponseSent                                             = 63,
    /// ADAPTIVE-MODULE-INSTANTIATION
    AdaptiveModuleInstantiation                                            = 64,
    /// ADAPTIVE-PLATFORM-SERVICE-INSTANCE
    AdaptivePlatformServiceInstance                                        = 65,
    /// ADAPTIVE-SERVICE-FIND-COMPLETED
    AdaptiveServiceFindCompleted                                           = 66,
    /// ADAPTIVE-SERVICE-FIND-STARTED
    AdaptiveServiceFindStarted                                             = 67,
    /// ADAPTIVE-SERVICE-OFFER-COMPLETED
    AdaptiveServiceOfferCompleted                                          = 68,
    /// ADAPTIVE-SERVICE-OFFER-STARTED
    AdaptiveServiceOfferStarted                                            = 69,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED
    AdaptiveServiceStopSubscriptionCompleted                               = 70,
    /// ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED
    AdaptiveServiceStopSubscriptionStarted                                 = 71,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED
    AdaptiveServiceSubscriptionAcknowledgeCompleted                        = 72,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED
    AdaptiveServiceSubscriptionAcknowledgeStarted                          = 73,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED
    AdaptiveServiceSubscriptionCompleted                                   = 74,
    /// ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED
    AdaptiveServiceSubscriptionStarted                                     = 75,
    /// ADAPTIVE-SWC-INTERNAL-BEHAVIOR
    AdaptiveSwcInternalBehavior                                            = 76,
    /// ADDR-METHOD-SHORT-NAME
    AddrMethodShortName                                                    = 77,
    /// ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT
    AddrMethodShortNameAndAlignment                                        = 78,
    /// AF
    Af                                                                     = 79,
    /// AFTER-SALES
    AfterSales                                                             = 80,
    /// AFTERMAKET
    Aftermaket                                                             = 81,
    /// AFTERMARKET
    Aftermarket                                                            = 82,
    /// AGE
    Age                                                                    = 83,
    /// AGE-CONSTRAINT
    AgeConstraint                                                          = 84,
    /// AGGREGATION-TAILORING
    AggregationTailoring                                                   = 85,
    /// AGREED
    Agreed                                                                 = 86,
    /// AH
    Ah                                                                     = 87,
    /// ALIAS-NAME-SET
    AliasNameSet                                                           = 88,
    /// ALIVE-SUPERVISION
    AliveSupervision                                                       = 89,
    /// ALL
    All                                                                    = 90,
    /// ALL-16-BIT
    All16Bit                                                               = 91,
    /// ALL-INDICES-DIFFERENT-ARRAY-SIZE
    AllIndicesDifferentArraySize                                           = 92,
    /// ALL-INDICES-SAME-ARRAY-SIZE
    AllIndicesSameArraySize                                                = 93,
    /// ALL-SUPPORTED-DTCS
    AllSupportedDtcs                                                       = 94,
    /// ALLOCATOR
    Allocator                                                              = 95,
    /// ALTERNATING-8-BIT
    Alternating8Bit                                                        = 96,
    /// ALWAYS
    Always                                                                 = 97,
    /// AM
    Am                                                                     = 98,
    /// AMBER-WARNING
    AmberWarning                                                           = 99,
    /// ANALYZED-EXECUTION-TIME
    AnalyzedExecutionTime                                                  = 100,
    /// AND
    And                                                                    = 101,
    /// ANY
    Any                                                                    = 102,
    /// ANY-SEND-OPERATION
    AnySendOperation                                                       = 103,
    /// ANY-STANDARDIZED
    AnyStandardized                                                        = 104,
    /// AP
    Ap                                                                     = 105,
    /// AP-APPLICATION-ENDPOINT
    ApApplicationEndpoint                                                  = 106,
    /// AP-APPLICATION-ERROR
    ApApplicationError                                                     = 107,
    /// AP-APPLICATION-ERROR-DOMAIN
    ApApplicationErrorDomain                                               = 108,
    /// AP-APPLICATION-ERROR-SET
    ApApplicationErrorSet                                                  = 109,
    /// AP-SOMEIP-TRANSFORMATION-PROPS
    ApSomeipTransformationProps                                            = 110,
    /// API
    Api                                                                    = 111,
    /// API-BASED
    ApiBased                                                               = 112,
    /// API-USE
    ApiUse                                                                 = 113,
    /// APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING
    AppOsTaskProxyToEcuTaskProxyMapping                                    = 114,
    /// APPLICATION
    Application                                                            = 115,
    /// APPLICATION-ACTION-ITEM
    ApplicationActionItem                                                  = 116,
    /// APPLICATION-ARRAY-DATA-TYPE
    ApplicationArrayDataType                                               = 117,
    /// APPLICATION-ARRAY-ELEMENT
    ApplicationArrayElement                                                = 118,
    /// APPLICATION-ASSOC-MAP-DATA-TYPE
    ApplicationAssocMapDataType                                            = 119,
    /// APPLICATION-ASSOC-MAP-ELEMENT
    ApplicationAssocMapElement                                             = 120,
    /// APPLICATION-COMPOSITE-DATA-TYPE
    ApplicationCompositeDataType                                           = 121,
    /// APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE
    ApplicationCompositeElementDataPrototype                               = 122,
    /// APPLICATION-DATA-TYPE
    ApplicationDataType                                                    = 123,
    /// APPLICATION-DEFERRED-DATA-TYPE
    ApplicationDeferredDataType                                            = 124,
    /// APPLICATION-ENDPOINT
    ApplicationEndpoint                                                    = 125,
    /// APPLICATION-ERROR
    ApplicationError                                                       = 126,
    /// APPLICATION-INTERFACE
    ApplicationInterface                                                   = 127,
    /// APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM
    ApplicationModeRequestPhmActionItem                                    = 128,
    /// APPLICATION-ONLY
    ApplicationOnly                                                        = 129,
    /// APPLICATION-PARTITION
    ApplicationPartition                                                   = 130,
    /// APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING
    ApplicationPartitionToEcuPartitionMapping                              = 131,
    /// APPLICATION-PRIMITIVE-DATA-TYPE
    ApplicationPrimitiveDataType                                           = 132,
    /// APPLICATION-RECORD-DATA-TYPE
    ApplicationRecordDataType                                              = 133,
    /// APPLICATION-RECORD-ELEMENT
    ApplicationRecordElement                                               = 134,
    /// APPLICATION-SW-COMPONENT-TYPE
    ApplicationSwComponentType                                             = 135,
    /// AR
    Ar                                                                     = 136,
    /// AR--CLIENT--SERVER
    ArClientServer                                                         = 137,
    /// AR-ELEMENT
    ArElement                                                              = 138,
    /// AR-PACKAGE
    ArPackage                                                              = 139,
    /// ARBITRARY-EVENT-TRIGGERING
    ArbitraryEventTriggering                                               = 140,
    /// ARBITRATION
    Arbitration                                                            = 141,
    /// ARGUMENT-DATA-PROTOTYPE
    ArgumentDataPrototype                                                  = 142,
    /// ARRAY
    Array                                                                  = 143,
    /// ARTIFACT-CHECKSUM
    ArtifactChecksum                                                       = 144,
    /// ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING
    ArtifactChecksumToCryptoProviderMapping                                = 145,
    /// AS
    As                                                                     = 146,
    /// AS-IS
    AsIs                                                                   = 147,
    /// ASSEMBLY-SW-CONNECTOR
    AssemblySwConnector                                                    = 148,
    /// ASYMMETRIC-FROM-BYTE-ARRAY
    AsymmetricFromByteArray                                                = 149,
    /// ASYMMETRIC-TO-BYTE-ARRAY
    AsymmetricToByteArray                                                  = 150,
    /// ASYNCHRONOUS
    Asynchronous                                                           = 151,
    /// ASYNCHRONOUS-SERVER-CALL-POINT
    AsynchronousServerCallPoint                                            = 152,
    /// ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    AsynchronousServerCallResultPoint                                      = 153,
    /// ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    AsynchronousServerCallReturnsEvent                                     = 154,
    /// ATOMIC-SW-COMPONENT-TYPE
    AtomicSwComponentType                                                  = 155,
    /// ATP-BLUEPRINT
    AtpBlueprint                                                           = 156,
    /// ATP-BLUEPRINTABLE
    AtpBlueprintable                                                       = 157,
    /// ATP-CLASSIFIER
    AtpClassifier                                                          = 158,
    /// ATP-DEFINITION
    AtpDefinition                                                          = 159,
    /// ATP-FEATURE
    AtpFeature                                                             = 160,
    /// ATP-PROTOTYPE
    AtpPrototype                                                           = 161,
    /// ATP-STRUCTURE-ELEMENT
    AtpStructureElement                                                    = 162,
    /// ATP-TYPE
    AtpType                                                                = 163,
    /// ATTRIBUTE-TAILORING
    AttributeTailoring                                                     = 164,
    /// AUTHENTICATE
    Authenticate                                                           = 165,
    /// AUTO
    Auto                                                                   = 166,
    /// AUTO-IP
    AutoIp                                                                 = 167,
    /// AUTO-IP--DOIP
    AutoIpDoip                                                             = 168,
    /// AUTO-IPDHCPV-4
    AutoIpdhcpv4                                                           = 169,
    /// AUTONOMOUS
    Autonomous                                                             = 170,
    /// AUTOSAR-DATA-PROTOTYPE
    AutosarDataPrototype                                                   = 171,
    /// AUTOSAR-DATA-TYPE
    AutosarDataType                                                        = 172,
    /// AUTOSAR-OPERATION-ARGUMENT-INSTANCE
    AutosarOperationArgumentInstance                                       = 173,
    /// AUTOSAR-VARIABLE-INSTANCE
    AutosarVariableInstance                                                = 174,
    /// AVB--IEEE-802--1-AS
    AvbIeee8021As                                                          = 175,
    /// AY
    Ay                                                                     = 176,
    /// AZ
    Az                                                                     = 177,
    /// BA
    Ba                                                                     = 178,
    /// BACKGROUND-EVENT
    BackgroundEvent                                                        = 179,
    /// BASE-T
    BaseT                                                                  = 180,
    /// BASE-TYPE
    BaseType                                                               = 181,
    /// BASIC-SOFTWARE-MODE-MANAGER
    BasicSoftwareModeManager                                               = 182,
    /// BE
    Be                                                                     = 183,
    /// BG
    Bg                                                                     = 184,
    /// BH
    Bh                                                                     = 185,
    /// BI
    Bi                                                                     = 186,
    /// BIDIRECTIONAL
    Bidirectional                                                          = 187,
    /// BINARY-MANIFEST-ADDRESSABLE-OBJECT
    BinaryManifestAddressableObject                                        = 188,
    /// BINARY-MANIFEST-ITEM
    BinaryManifestItem                                                     = 189,
    /// BINARY-MANIFEST-ITEM-DEFINITION
    BinaryManifestItemDefinition                                           = 190,
    /// BINARY-MANIFEST-META-DATA-FIELD
    BinaryManifestMetaDataField                                            = 191,
    /// BINARY-MANIFEST-PROVIDE-RESOURCE
    BinaryManifestProvideResource                                          = 192,
    /// BINARY-MANIFEST-REQUIRE-RESOURCE
    BinaryManifestRequireResource                                          = 193,
    /// BINARY-MANIFEST-RESOURCE
    BinaryManifestResource                                                 = 194,
    /// BINARY-MANIFEST-RESOURCE-DEFINITION
    BinaryManifestResourceDefinition                                       = 195,
    /// BLINK-MODE
    BlinkMode                                                              = 196,
    /// BLINK-OR-CONTINUOUS-ON-MODE
    BlinkOrContinuousOnMode                                                = 197,
    /// BLOCK-SOURCE
    BlockSource                                                            = 198,
    /// BLOCK-STATE
    BlockState                                                             = 199,
    /// BLUEPRINT-DERIVATION-TIME
    BlueprintDerivationTime                                                = 200,
    /// BLUEPRINT-MAPPING-SET
    BlueprintMappingSet                                                    = 201,
    /// BMP
    Bmp                                                                    = 202,
    /// BN
    Bn                                                                     = 203,
    /// BO
    Bo                                                                     = 204,
    /// BOLD
    Bold                                                                   = 205,
    /// BOLDITALIC
    Bolditalic                                                             = 206,
    /// BONJOUR
    Bonjour                                                                = 207,
    /// BOTTOM
    Bottom                                                                 = 208,
    /// BR
    Br                                                                     = 209,
    /// BREAK
    Break                                                                  = 210,
    /// BRIEF
    Brief                                                                  = 211,
    /// BRIEF-BYPASSING-FILTERS
    BriefBypassingFilters                                                  = 212,
    /// BROAD-R-REACH
    BroadRReach                                                            = 213,
    /// BSW
    Bsw                                                                    = 214,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-POINT
    BswAsynchronousServerCallPoint                                         = 215,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT
    BswAsynchronousServerCallResultPoint                                   = 216,
    /// BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT
    BswAsynchronousServerCallReturnsEvent                                  = 217,
    /// BSW-BACKGROUND-EVENT
    BswBackgroundEvent                                                     = 218,
    /// BSW-CALLED-ENTITY
    BswCalledEntity                                                        = 219,
    /// BSW-COMPOSITION-TIMING
    BswCompositionTiming                                                   = 220,
    /// BSW-DATA-RECEIVED-EVENT
    BswDataReceivedEvent                                                   = 221,
    /// BSW-DEBUG-INFO
    BswDebugInfo                                                           = 222,
    /// BSW-DIRECT-CALL-POINT
    BswDirectCallPoint                                                     = 223,
    /// BSW-DISTINGUISHED-PARTITION
    BswDistinguishedPartition                                              = 224,
    /// BSW-ENTRY-RELATIONSHIP-SET
    BswEntryRelationshipSet                                                = 225,
    /// BSW-EVENT
    BswEvent                                                               = 226,
    /// BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT
    BswExternalTriggerOccurredEvent                                        = 227,
    /// BSW-IMPLEMENTATION
    BswImplementation                                                      = 228,
    /// BSW-INTERNAL-BEHAVIOR
    BswInternalBehavior                                                    = 229,
    /// BSW-INTERNAL-TRIGGER-OCCURRED-EVENT
    BswInternalTriggerOccurredEvent                                        = 230,
    /// BSW-INTERNAL-TRIGGERING-POINT
    BswInternalTriggeringPoint                                             = 231,
    /// BSW-INTERRUPT-ENTITY
    BswInterruptEntity                                                     = 232,
    /// BSW-M-ENTRY-CALL-RETURNED
    BswMEntryCallReturned                                                  = 233,
    /// BSW-M-ENTRY-CALLED
    BswMEntryCalled                                                        = 234,
    /// BSW-MGR-NEEDS
    BswMgrNeeds                                                            = 235,
    /// BSW-MODE-MANAGER-ERROR-EVENT
    BswModeManagerErrorEvent                                               = 236,
    /// BSW-MODE-SWITCH-EVENT
    BswModeSwitchEvent                                                     = 237,
    /// BSW-MODE-SWITCHED-ACK-EVENT
    BswModeSwitchedAckEvent                                                = 238,
    /// BSW-MODULE-CALL-POINT
    BswModuleCallPoint                                                     = 239,
    /// BSW-MODULE-CLIENT-SERVER-ENTRY
    BswModuleClientServerEntry                                             = 240,
    /// BSW-MODULE-DEPENDENCY
    BswModuleDependency                                                    = 241,
    /// BSW-MODULE-DESCRIPTION
    BswModuleDescription                                                   = 242,
    /// BSW-MODULE-ENTITY
    BswModuleEntity                                                        = 243,
    /// BSW-MODULE-ENTITY-ACTIVATED
    BswModuleEntityActivated                                               = 244,
    /// BSW-MODULE-ENTITY-STARTED
    BswModuleEntityStarted                                                 = 245,
    /// BSW-MODULE-ENTITY-TERMINATED
    BswModuleEntityTerminated                                              = 246,
    /// BSW-MODULE-ENTRY
    BswModuleEntry                                                         = 247,
    /// BSW-MODULE-TIMING
    BswModuleTiming                                                        = 248,
    /// BSW-OPERATION-INVOKED-EVENT
    BswOperationInvokedEvent                                               = 249,
    /// BSW-OS-TASK-EXECUTION-EVENT
    BswOsTaskExecutionEvent                                                = 250,
    /// BSW-SCHEDULABLE-ENTITY
    BswSchedulableEntity                                                   = 251,
    /// BSW-SCHEDULE-EVENT
    BswScheduleEvent                                                       = 252,
    /// BSW-SCHEDULER-NAME-PREFIX
    BswSchedulerNamePrefix                                                 = 253,
    /// BSW-SERVICE-DEPENDENCY-IDENT
    BswServiceDependencyIdent                                              = 254,
    /// BSW-SYNCHRONOUS-SERVER-CALL-POINT
    BswSynchronousServerCallPoint                                          = 255,
    /// BSW-TIMING-EVENT
    BswTimingEvent                                                         = 256,
    /// BSW-VARIABLE-ACCESS
    BswVariableAccess                                                      = 257,
    /// BUILD
    Build                                                                  = 258,
    /// BUILD-ACTION
    BuildAction                                                            = 259,
    /// BUILD-ACTION-ENTITY
    BuildActionEntity                                                      = 260,
    /// BUILD-ACTION-ENVIRONMENT
    BuildActionEnvironment                                                 = 261,
    /// BUILD-ACTION-MANIFEST
    BuildActionManifest                                                    = 262,
    /// BUILD-TYPE-DEBUG
    BuildTypeDebug                                                         = 263,
    /// BUILD-TYPE-RELEASE
    BuildTypeRelease                                                       = 264,
    /// BULK-NV-DATA-DESCRIPTOR
    BulkNvDataDescriptor                                                   = 265,
    /// BURST-PATTERN-EVENT-TRIGGERING
    BurstPatternEventTriggering                                            = 266,
    /// BUS-MIRROR-CHANNEL-MAPPING
    BusMirrorChannelMapping                                                = 267,
    /// BUS-MIRROR-CHANNEL-MAPPING-CAN
    BusMirrorChannelMappingCan                                             = 268,
    /// BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY
    BusMirrorChannelMappingFlexray                                         = 269,
    /// BUS-MIRROR-CHANNEL-MAPPING-IP
    BusMirrorChannelMappingIp                                              = 270,
    /// BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED
    BusMirrorChannelMappingUserDefined                                     = 271,
    /// C
    C                                                                      = 272,
    /// CA
    Ca                                                                     = 273,
    /// CALCULATED
    Calculated                                                             = 274,
    /// CALIBRATION-OFFLINE
    CalibrationOffline                                                     = 275,
    /// CALIBRATION-ONLINE
    CalibrationOnline                                                      = 276,
    /// CALIBRATION-PARAMETER-VALUE-SET
    CalibrationParameterValueSet                                           = 277,
    /// CALIBRATION-VARIABLES
    CalibrationVariables                                                   = 278,
    /// CALLBACK
    Callback                                                               = 279,
    /// CALLOUT
    Callout                                                                = 280,
    /// CALPRM
    Calprm                                                                 = 281,
    /// CAN-20
    Can20                                                                  = 282,
    /// CAN-BE-TERMINATED
    CanBeTerminated                                                        = 283,
    /// CAN-BE-TERMINATED-AND-RESTARTED
    CanBeTerminatedAndRestarted                                            = 284,
    /// CAN-CLUSTER
    CanCluster                                                             = 285,
    /// CAN-COMMUNICATION-CONNECTOR
    CanCommunicationConnector                                              = 286,
    /// CAN-COMMUNICATION-CONTROLLER
    CanCommunicationController                                             = 287,
    /// CAN-FD
    CanFd                                                                  = 288,
    /// CAN-FRAME
    CanFrame                                                               = 289,
    /// CAN-FRAME-TRIGGERING
    CanFrameTriggering                                                     = 290,
    /// CAN-NM-CLUSTER
    CanNmCluster                                                           = 291,
    /// CAN-NM-NODE
    CanNmNode                                                              = 292,
    /// CAN-PHYSICAL-CHANNEL
    CanPhysicalChannel                                                     = 293,
    /// CAN-TP-ADDRESS
    CanTpAddress                                                           = 294,
    /// CAN-TP-CHANNEL
    CanTpChannel                                                           = 295,
    /// CAN-TP-CONFIG
    CanTpConfig                                                            = 296,
    /// CAN-TP-NODE
    CanTpNode                                                              = 297,
    /// CANCEL
    Cancel                                                                 = 298,
    /// CAPTION
    Caption                                                                = 299,
    /// CAPTURE-ASYNCHRONOUS-TO-REPORTING
    CaptureAsynchronousToReporting                                         = 300,
    /// CAPTURE-ASYNCHRONOUSLY-TO-REPORTING
    CaptureAsynchronouslyToReporting                                       = 301,
    /// CAPTURE-SYNCHRONOUS-TO-REPORTING
    CaptureSynchronousToReporting                                          = 302,
    /// CAPTURE-SYNCHRONOUSLY-TO-REPORTING
    CaptureSynchronouslyToReporting                                        = 303,
    /// CAT-1
    Cat1                                                                   = 304,
    /// CAT-2
    Cat2                                                                   = 305,
    /// CAUTION
    Caution                                                                = 306,
    /// CENTER
    Center                                                                 = 307,
    /// CHANNEL-A
    ChannelA                                                               = 308,
    /// CHANNEL-B
    ChannelB                                                               = 309,
    /// CHAPTER
    Chapter                                                                = 310,
    /// CHECK-AT-NEXT-HALT
    CheckAtNextHalt                                                        = 311,
    /// CHECKPOINT-TRANSITION
    CheckpointTransition                                                   = 312,
    /// CIRCLE
    Circle                                                                 = 313,
    /// CLASS-CONTENT-CONDITIONAL
    ClassContentConditional                                                = 314,
    /// CLASSIC
    Classic                                                                = 315,
    /// CLEAR
    Clear                                                                  = 316,
    /// CLEAR-ALL-DTCS
    ClearAllDtcs                                                           = 317,
    /// CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    ClearDynamicallyDefineDataIdentifier                                   = 318,
    /// CLIENT-AUTHENTICATE
    ClientAuthenticate                                                     = 319,
    /// CLIENT-DECRYPT
    ClientDecrypt                                                          = 320,
    /// CLIENT-ENCRYPT
    ClientEncrypt                                                          = 321,
    /// CLIENT-ID-DEFINITION
    ClientIdDefinition                                                     = 322,
    /// CLIENT-ID-DEFINITION-SET
    ClientIdDefinitionSet                                                  = 323,
    /// CLIENT-MAC-GENERATE
    ClientMacGenerate                                                      = 324,
    /// CLIENT-MAC-VERIFY
    ClientMacVerify                                                        = 325,
    /// CLIENT-SERVER-INTERFACE
    ClientServerInterface                                                  = 326,
    /// CLIENT-SERVER-INTERFACE-MAPPING
    ClientServerInterfaceMapping                                           = 327,
    /// CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING
    ClientServerInterfaceToBswModuleEntryBlueprintMapping                  = 328,
    /// CLIENT-SERVER-OPERATION
    ClientServerOperation                                                  = 329,
    /// CLIENT-VERIFY
    ClientVerify                                                           = 330,
    /// CLOSED
    Closed                                                                 = 331,
    /// CO
    Co                                                                     = 332,
    /// CODE
    Code                                                                   = 333,
    /// CODE-GENERATION-TIME
    CodeGenerationTime                                                     = 334,
    /// CODEGENERATION
    Codegeneration                                                         = 335,
    /// COLDSTART
    Coldstart                                                              = 336,
    /// COLLECTABLE-ELEMENT
    CollectableElement                                                     = 337,
    /// COLLECTION
    Collection                                                             = 338,
    /// COM-AXIS
    ComAxis                                                                = 339,
    /// COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING
    ComCertificateToCryptoCertificateMapping                               = 340,
    /// COM-EVENT-GRANT
    ComEventGrant                                                          = 341,
    /// COM-EVENT-GRANT-DESIGN
    ComEventGrantDesign                                                    = 342,
    /// COM-FIELD-GRANT
    ComFieldGrant                                                          = 343,
    /// COM-FIELD-GRANT-DESIGN
    ComFieldGrantDesign                                                    = 344,
    /// COM-FIND-SERVICE-GRANT
    ComFindServiceGrant                                                    = 345,
    /// COM-FIND-SERVICE-GRANT-DESIGN
    ComFindServiceGrantDesign                                              = 346,
    /// COM-GRANT
    ComGrant                                                               = 347,
    /// COM-GRANT-DESIGN
    ComGrantDesign                                                         = 348,
    /// COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING
    ComKeyToCryptoKeySlotMapping                                           = 349,
    /// COM-MANAGEMENT-MAPPING
    ComManagementMapping                                                   = 350,
    /// COM-MANAGER
    ComManager                                                             = 351,
    /// COM-METHOD-GRANT
    ComMethodGrant                                                         = 352,
    /// COM-METHOD-GRANT-DESIGN
    ComMethodGrantDesign                                                   = 353,
    /// COM-MGR-USER-NEEDS
    ComMgrUserNeeds                                                        = 354,
    /// COM-OFFER-SERVICE-GRANT
    ComOfferServiceGrant                                                   = 355,
    /// COM-OFFER-SERVICE-GRANT-DESIGN
    ComOfferServiceGrantDesign                                             = 356,
    /// COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING
    ComSecOcToCryptoKeySlotMapping                                         = 357,
    /// COM-TRIGGER-GRANT-DESIGN
    ComTriggerGrantDesign                                                  = 358,
    /// COMM-CONNECTOR-PORT
    CommConnectorPort                                                      = 359,
    /// COMMAND-LINE-LONG-FORM
    CommandLineLongForm                                                    = 360,
    /// COMMAND-LINE-SHORT-FORM
    CommandLineShortForm                                                   = 361,
    /// COMMAND-LINE-SIMPLE-FORM
    CommandLineSimpleForm                                                  = 362,
    /// COMMON
    Common                                                                 = 363,
    /// COMMUNICATION-CLUSTER
    CommunicationCluster                                                   = 364,
    /// COMMUNICATION-CONNECTOR
    CommunicationConnector                                                 = 365,
    /// COMMUNICATION-CONTROLLER
    CommunicationController                                                = 366,
    /// COMMUNICATION-INTER-ECU
    CommunicationInterEcu                                                  = 367,
    /// COMMUNICATION-INTRA-PARTITION
    CommunicationIntraPartition                                            = 368,
    /// COMPILE
    Compile                                                                = 369,
    /// COMPILER
    Compiler                                                               = 370,
    /// COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE
    ComplexDeviceDriverSwComponentType                                     = 371,
    /// COMPOSITE-INTERFACE
    CompositeInterface                                                     = 372,
    /// COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING
    CompositionPPortToExecutablePPortMapping                               = 373,
    /// COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING
    CompositionPortToExecutablePortMapping                                 = 374,
    /// COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING
    CompositionRPortToExecutableRPortMapping                               = 375,
    /// COMPOSITION-SW-COMPONENT-TYPE
    CompositionSwComponentType                                             = 376,
    /// COMPU-METHOD
    CompuMethod                                                            = 377,
    /// COM_AXIS
    Comaxis                                                                = 378,
    /// CONCRETE
    Concrete                                                               = 379,
    /// CONCRETE-CLASS-TAILORING
    ConcreteClassTailoring                                                 = 380,
    /// CONCRETE-PATTERN-EVENT-TRIGGERING
    ConcretePatternEventTriggering                                         = 381,
    /// CONDITIONAL
    Conditional                                                            = 382,
    /// CONFIG-DATA
    ConfigData                                                             = 383,
    /// CONFIGURED
    Configured                                                             = 384,
    /// CONFIRMED
    Confirmed                                                              = 385,
    /// CONFIRMED-DTC-BIT
    ConfirmedDtcBit                                                        = 386,
    /// CONNECT
    Connect                                                                = 387,
    /// CONSISTENCY-NEEDS
    ConsistencyNeeds                                                       = 388,
    /// CONSISTENCY-NEEDS-BLUEPRINT-SET
    ConsistencyNeedsBlueprintSet                                           = 389,
    /// CONSOLE
    Console                                                                = 390,
    /// CONST
    Const                                                                  = 391,
    /// CONSTANT-SPECIFICATION
    ConstantSpecification                                                  = 392,
    /// CONSTANT-SPECIFICATION-MAPPING-SET
    ConstantSpecificationMappingSet                                        = 393,
    /// CONSTRAINT-TAILORING
    ConstraintTailoring                                                    = 394,
    /// CONSUMED-EVENT-GROUP
    ConsumedEventGroup                                                     = 395,
    /// CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP
    ConsumedProvidedServiceInstanceGroup                                   = 396,
    /// CONSUMED-SERVICE-INSTANCE
    ConsumedServiceInstance                                                = 397,
    /// CONSUMER
    Consumer                                                               = 398,
    /// CONTAINER-I-PDU
    ContainerIPdu                                                          = 399,
    /// CONTINUE-AT-IT-POSITION
    ContinueAtItPosition                                                   = 400,
    /// CONTINUOUS-ON-MODE
    ContinuousOnMode                                                       = 401,
    /// COUPLING-ELEMENT
    CouplingElement                                                        = 402,
    /// COUPLING-PORT
    CouplingPort                                                           = 403,
    /// COUPLING-PORT-FIFO
    CouplingPortFifo                                                       = 404,
    /// COUPLING-PORT-SCHEDULER
    CouplingPortScheduler                                                  = 405,
    /// COUPLING-PORT-SHAPER
    CouplingPortShaper                                                     = 406,
    /// COUPLING-PORT-STRUCTURAL-ELEMENT
    CouplingPortStructuralElement                                          = 407,
    /// COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT
    CouplingPortTrafficClassAssignment                                     = 408,
    /// CP
    Cp                                                                     = 409,
    /// CP-SOFTWARE-CLUSTER
    CpSoftwareCluster                                                      = 410,
    /// CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR
    CpSoftwareClusterBinaryManifestDescriptor                              = 411,
    /// CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE
    CpSoftwareClusterCommunicationResource                                 = 412,
    /// CP-SOFTWARE-CLUSTER-MAPPING-SET
    CpSoftwareClusterMappingSet                                            = 413,
    /// CP-SOFTWARE-CLUSTER-RESOURCE
    CpSoftwareClusterResource                                              = 414,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-POOL
    CpSoftwareClusterResourcePool                                          = 415,
    /// CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING
    CpSoftwareClusterResourceToApplicationPartitionMapping                 = 416,
    /// CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE
    CpSoftwareClusterServiceResource                                       = 417,
    /// CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING
    CpSoftwareClusterToEcuInstanceMapping                                  = 418,
    /// CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING
    CpSoftwareClusterToResourceMapping                                     = 419,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING
    CpSwClusterResourceToDiagDataElemMapping                               = 420,
    /// CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING
    CpSwClusterResourceToDiagFunctionIdMapping                             = 421,
    /// CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING
    CpSwClusterToDiagEventMapping                                          = 422,
    /// CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING
    CpSwClusterToDiagRoutineSubfunctionMapping                             = 423,
    /// CPP
    Cpp                                                                    = 424,
    /// CPP-IMPLEMENTATION-DATA-TYPE
    CppImplementationDataType                                              = 425,
    /// CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET
    CppImplementationDataTypeContextTarget                                 = 426,
    /// CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT
    CppImplementationDataTypeElement                                       = 427,
    /// CRC-IGNORED
    CrcIgnored                                                             = 428,
    /// CRC-NOT-SUPPORTED
    CrcNotSupported                                                        = 429,
    /// CRC-NOT-VALIDATED
    CrcNotValidated                                                        = 430,
    /// CRC-OPTIONAL
    CrcOptional                                                            = 431,
    /// CRC-SUPPORTED
    CrcSupported                                                           = 432,
    /// CRC-VALIDATED
    CrcValidated                                                           = 433,
    /// CRYPTO-CERTIFICATE
    CryptoCertificate                                                      = 434,
    /// CRYPTO-CERTIFICATE-INTERFACE
    CryptoCertificateInterface                                             = 435,
    /// CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS
    CryptoCertificateKeySlotNeeds                                          = 436,
    /// CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING
    CryptoCertificateToPortPrototypeMapping                                = 437,
    /// CRYPTO-DRIVER
    CryptoDriver                                                           = 438,
    /// CRYPTO-ELLIPTIC-CURVE-PROPS
    CryptoEllipticCurveProps                                               = 439,
    /// CRYPTO-INTERFACE
    CryptoInterface                                                        = 440,
    /// CRYPTO-JOB
    CryptoJob                                                              = 441,
    /// CRYPTO-KEY-MANAGEMENT
    CryptoKeyManagement                                                    = 442,
    /// CRYPTO-KEY-MANAGEMENT-NEEDS
    CryptoKeyManagementNeeds                                               = 443,
    /// CRYPTO-KEY-SLOT
    CryptoKeySlot                                                          = 444,
    /// CRYPTO-KEY-SLOT-INTERFACE
    CryptoKeySlotInterface                                                 = 445,
    /// CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING
    CryptoKeySlotToPortPrototypeMapping                                    = 446,
    /// CRYPTO-MODULE-INSTANTIATION
    CryptoModuleInstantiation                                              = 447,
    /// CRYPTO-NEED
    CryptoNeed                                                             = 448,
    /// CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING
    CryptoNeedToCryptoJobMapping                                           = 449,
    /// CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING
    CryptoNeedToPortPrototypeMapping                                       = 450,
    /// CRYPTO-NEEDS
    CryptoNeeds                                                            = 451,
    /// CRYPTO-PRIMITIVE
    CryptoPrimitive                                                        = 452,
    /// CRYPTO-PROVIDER
    CryptoProvider                                                         = 453,
    /// CRYPTO-PROVIDER-INTERFACE
    CryptoProviderInterface                                                = 454,
    /// CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING
    CryptoProviderToPortPrototypeMapping                                   = 455,
    /// CRYPTO-SERVICE-CERTIFICATE
    CryptoServiceCertificate                                               = 456,
    /// CRYPTO-SERVICE-JOB-NEEDS
    CryptoServiceJobNeeds                                                  = 457,
    /// CRYPTO-SERVICE-KEY
    CryptoServiceKey                                                       = 458,
    /// CRYPTO-SERVICE-MANAGER
    CryptoServiceManager                                                   = 459,
    /// CRYPTO-SERVICE-MAPPING
    CryptoServiceMapping                                                   = 460,
    /// CRYPTO-SERVICE-NEEDS
    CryptoServiceNeeds                                                     = 461,
    /// CRYPTO-SERVICE-PRIMITIVE
    CryptoServicePrimitive                                                 = 462,
    /// CRYPTO-SERVICE-QUEUE
    CryptoServiceQueue                                                     = 463,
    /// CRYPTO-SIGNATURE-SCHEME
    CryptoSignatureScheme                                                  = 464,
    /// CRYPTO-TRUST-MASTER-INTERFACE
    CryptoTrustMasterInterface                                             = 465,
    /// CS
    Cs                                                                     = 466,
    /// CURVE-AXIS
    CurveAxis                                                              = 467,
    /// CURVE_AXIS
    Curveaxis                                                              = 468,
    /// CUSTOM
    Custom                                                                 = 469,
    /// CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE
    CustomCppImplementationDataType                                        = 470,
    /// CVC
    Cvc                                                                    = 471,
    /// CY
    Cy                                                                     = 472,
    /// CYCLE-REPETITION-1
    CycleRepetition1                                                       = 473,
    /// CYCLE-REPETITION-10
    CycleRepetition10                                                      = 474,
    /// CYCLE-REPETITION-16
    CycleRepetition16                                                      = 475,
    /// CYCLE-REPETITION-2
    CycleRepetition2                                                       = 476,
    /// CYCLE-REPETITION-20
    CycleRepetition20                                                      = 477,
    /// CYCLE-REPETITION-32
    CycleRepetition32                                                      = 478,
    /// CYCLE-REPETITION-4
    CycleRepetition4                                                       = 479,
    /// CYCLE-REPETITION-40
    CycleRepetition40                                                      = 480,
    /// CYCLE-REPETITION-5
    CycleRepetition5                                                       = 481,
    /// CYCLE-REPETITION-50
    CycleRepetition50                                                      = 482,
    /// CYCLE-REPETITION-64
    CycleRepetition64                                                      = 483,
    /// CYCLE-REPETITION-8
    CycleRepetition8                                                       = 484,
    /// CYCLIC
    Cyclic                                                                 = 485,
    /// CYCLIC-AND-ON-CHANGE
    CyclicAndOnChange                                                      = 486,
    /// DA
    Da                                                                     = 487,
    /// DATA-CONSTR
    DataConstr                                                             = 488,
    /// DATA-EXCHANGE-POINT
    DataExchangePoint                                                      = 489,
    /// DATA-FORMAT-ELEMENT-REFERENCE
    DataFormatElementReference                                             = 490,
    /// DATA-FORMAT-ELEMENT-SCOPE
    DataFormatElementScope                                                 = 491,
    /// DATA-INTERFACE
    DataInterface                                                          = 492,
    /// DATA-PROTOTYPE
    DataPrototype                                                          = 493,
    /// DATA-PROTOTYPE-GROUP
    DataPrototypeGroup                                                     = 494,
    /// DATA-RECEIVE-ERROR-EVENT
    DataReceiveErrorEvent                                                  = 495,
    /// DATA-RECEIVED-EVENT
    DataReceivedEvent                                                      = 496,
    /// DATA-SEND-COMPLETED-EVENT
    DataSendCompletedEvent                                                 = 497,
    /// DATA-TRANSFORMATION
    DataTransformation                                                     = 498,
    /// DATA-TRANSFORMATION-SET
    DataTransformationSet                                                  = 499,
    /// DATA-TYPE-MAPPING-SET
    DataTypeMappingSet                                                     = 500,
    /// DATA-WRITE-COMPLETED-EVENT
    DataWriteCompletedEvent                                                = 501,
    /// DCM-I-PDU
    DcmIPdu                                                                = 502,
    /// DDS-DOMAIN-RANGE
    DdsDomainRange                                                         = 503,
    /// DDS-EVENT-DEPLOYMENT
    DdsEventDeployment                                                     = 504,
    /// DDS-FIELD-DEPLOYMENT
    DdsFieldDeployment                                                     = 505,
    /// DDS-METHOD-DEPLOYMENT
    DdsMethodDeployment                                                    = 506,
    /// DDS-PROVIDED-SERVICE-INSTANCE
    DdsProvidedServiceInstance                                             = 507,
    /// DDS-REQUIRED-SERVICE-INSTANCE
    DdsRequiredServiceInstance                                             = 508,
    /// DDS-RPC-SERVICE-DEPLOYMENT
    DdsRpcServiceDeployment                                                = 509,
    /// DDS-SECURE-COM-PROPS
    DdsSecureComProps                                                      = 510,
    /// DDS-SECURE-GOVERNANCE
    DdsSecureGovernance                                                    = 511,
    /// DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    DdsServiceInstanceToMachineMapping                                     = 512,
    /// DDS-SERVICE-INTERFACE-DEPLOYMENT
    DdsServiceInterfaceDeployment                                          = 513,
    /// DDS-TOPIC-ACCESS-RULE
    DdsTopicAccessRule                                                     = 514,
    /// DE
    De                                                                     = 515,
    /// DEADLINE-SUPERVISION
    DeadlineSupervision                                                    = 516,
    /// DEBOUNCE-DATA
    DebounceData                                                           = 517,
    /// DEBUG
    Debug                                                                  = 518,
    /// DECREASING
    Decreasing                                                             = 519,
    /// DEDICATED
    Dedicated                                                              = 520,
    /// DEF-ITEM
    DefItem                                                                = 521,
    /// DEFAULT
    Default                                                                = 522,
    /// DEFAULT-ERROR-TRACER
    DefaultErrorTracer                                                     = 523,
    /// DEFAULT-IF-REVISION-UPDATE
    DefaultIfRevisionUpdate                                                = 524,
    /// DEFAULT-IF-UNDEFINED
    DefaultIfUndefined                                                     = 525,
    /// DEFAULT-MODE
    DefaultMode                                                            = 526,
    /// DEFAULT-TRACE-STATE-DISABLED
    DefaultTraceStateDisabled                                              = 527,
    /// DEFAULT-TRACE-STATE-ENABLED
    DefaultTraceStateEnabled                                               = 528,
    /// DEFAULT-TRIGGER
    DefaultTrigger                                                         = 529,
    /// DEFERRED
    Deferred                                                               = 530,
    /// DEFICIT-ROUND-ROBIN
    DeficitRoundRobin                                                      = 531,
    /// DEFINE-BY-IDENTIFIER
    DefineByIdentifier                                                     = 532,
    /// DEFINE-BY-MEMORY-ADDRESS
    DefineByMemoryAddress                                                  = 533,
    /// DEFLATE
    Deflate                                                                = 534,
    /// DELEGATION-SW-CONNECTOR
    DelegationSwConnector                                                  = 535,
    /// DELETE
    Delete                                                                 = 536,
    /// DEPENDANT
    Dependant                                                              = 537,
    /// DEPENDENCY-ON-ARTIFACT
    DependencyOnArtifact                                                   = 538,
    /// DERIVED-FROM
    DerivedFrom                                                            = 539,
    /// DESCENDANT
    Descendant                                                             = 540,
    /// DESELECTED
    Deselected                                                             = 541,
    /// DETAILED
    Detailed                                                               = 542,
    /// DETAILED-BYPASSING-FILTERS
    DetailedBypassingFilters                                               = 543,
    /// DETERMINISTIC-CLIENT
    DeterministicClient                                                    = 544,
    /// DETERMINISTIC-CLIENT-RESOURCE-NEEDS
    DeterministicClientResourceNeeds                                       = 545,
    /// DEVELOPMENT
    Development                                                            = 546,
    /// DEVELOPMENT-ERROR
    DevelopmentError                                                       = 547,
    /// DEVELOPMENT-ERROR-TRACER
    DevelopmentErrorTracer                                                 = 548,
    /// DHCPV-4
    Dhcpv4                                                                 = 549,
    /// DHCPV-6
    Dhcpv6                                                                 = 550,
    /// DIAG-EVENT-DEBOUNCE-ALGORITHM
    DiagEventDebounceAlgorithm                                             = 551,
    /// DIAG-EVENT-DEBOUNCE-COUNTER-BASED
    DiagEventDebounceCounterBased                                          = 552,
    /// DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL
    DiagEventDebounceMonitorInternal                                       = 553,
    /// DIAG-EVENT-DEBOUNCE-TIME-BASED
    DiagEventDebounceTimeBased                                             = 554,
    /// DIAG-REQUEST
    DiagRequest                                                            = 555,
    /// DIAG-RESPONSE
    DiagResponse                                                           = 556,
    /// DIAGNOSTIC-ABSTRACT-ALIAS-EVENT
    DiagnosticAbstractAliasEvent                                           = 557,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER
    DiagnosticAbstractDataIdentifier                                       = 558,
    /// DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE
    DiagnosticAbstractDataIdentifierInterface                              = 559,
    /// DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE
    DiagnosticAbstractRoutineInterface                                     = 560,
    /// DIAGNOSTIC-ACCESS-PERMISSION
    DiagnosticAccessPermission                                             = 561,
    /// DIAGNOSTIC-AGING
    DiagnosticAging                                                        = 562,
    /// DIAGNOSTIC-AUTH-ROLE
    DiagnosticAuthRole                                                     = 563,
    /// DIAGNOSTIC-AUTHENTICATION
    DiagnosticAuthentication                                               = 564,
    /// DIAGNOSTIC-AUTHENTICATION-CLASS
    DiagnosticAuthenticationClass                                          = 565,
    /// DIAGNOSTIC-AUTHENTICATION-CONFIGURATION
    DiagnosticAuthenticationConfiguration                                  = 566,
    /// DIAGNOSTIC-AUTHENTICATION-INTERFACE
    DiagnosticAuthenticationInterface                                      = 567,
    /// DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING
    DiagnosticAuthenticationPortMapping                                    = 568,
    /// DIAGNOSTIC-CAPABILITY-ELEMENT
    DiagnosticCapabilityElement                                            = 569,
    /// DIAGNOSTIC-CLEAR-CONDITION
    DiagnosticClearCondition                                               = 570,
    /// DIAGNOSTIC-CLEAR-CONDITION-GROUP
    DiagnosticClearConditionGroup                                          = 571,
    /// DIAGNOSTIC-CLEAR-CONDITION-NEEDS
    DiagnosticClearConditionNeeds                                          = 572,
    /// DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING
    DiagnosticClearConditionPortMapping                                    = 573,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION
    DiagnosticClearDiagnosticInformation                                   = 574,
    /// DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS
    DiagnosticClearDiagnosticInformationClass                              = 575,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO
    DiagnosticClearResetEmissionRelatedInfo                                = 576,
    /// DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS
    DiagnosticClearResetEmissionRelatedInfoClass                           = 577,
    /// DIAGNOSTIC-COM-CONTROL
    DiagnosticComControl                                                   = 578,
    /// DIAGNOSTIC-COM-CONTROL-CLASS
    DiagnosticComControlClass                                              = 579,
    /// DIAGNOSTIC-COM-CONTROL-INTERFACE
    DiagnosticComControlInterface                                          = 580,
    /// DIAGNOSTIC-COMMON-ELEMENT
    DiagnosticCommonElement                                                = 581,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER
    DiagnosticCommunicationManager                                         = 582,
    /// DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS
    DiagnosticCommunicationManagerNeeds                                    = 583,
    /// DIAGNOSTIC-COMPONENT-NEEDS
    DiagnosticComponentNeeds                                               = 584,
    /// DIAGNOSTIC-CONDITION
    DiagnosticCondition                                                    = 585,
    /// DIAGNOSTIC-CONDITION-GROUP
    DiagnosticConditionGroup                                               = 586,
    /// DIAGNOSTIC-CONDITION-INTERFACE
    DiagnosticConditionInterface                                           = 587,
    /// DIAGNOSTIC-CONNECTED-INDICATOR
    DiagnosticConnectedIndicator                                           = 588,
    /// DIAGNOSTIC-CONNECTION
    DiagnosticConnection                                                   = 589,
    /// DIAGNOSTIC-CONTRIBUTION-SET
    DiagnosticContributionSet                                              = 590,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING
    DiagnosticControlDtcSetting                                            = 591,
    /// DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS
    DiagnosticControlDtcSettingClass                                       = 592,
    /// DIAGNOSTIC-CONTROL-NEEDS
    DiagnosticControlNeeds                                                 = 593,
    /// DIAGNOSTIC-CUSTOM-SERVICE-CLASS
    DiagnosticCustomServiceClass                                           = 594,
    /// DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE
    DiagnosticCustomServiceInstance                                        = 595,
    /// DIAGNOSTIC-DATA-BY-IDENTIFIER
    DiagnosticDataByIdentifier                                             = 596,
    /// DIAGNOSTIC-DATA-ELEMENT
    DiagnosticDataElement                                                  = 597,
    /// DIAGNOSTIC-DATA-ELEMENT-INTERFACE
    DiagnosticDataElementInterface                                         = 598,
    /// DIAGNOSTIC-DATA-IDENTIFIER
    DiagnosticDataIdentifier                                               = 599,
    /// DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE
    DiagnosticDataIdentifierGenericInterface                               = 600,
    /// DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE
    DiagnosticDataIdentifierInterface                                      = 601,
    /// DIAGNOSTIC-DATA-IDENTIFIER-SET
    DiagnosticDataIdentifierSet                                            = 602,
    /// DIAGNOSTIC-DATA-PORT-MAPPING
    DiagnosticDataPortMapping                                              = 603,
    /// DIAGNOSTIC-DATA-TRANSFER
    DiagnosticDataTransfer                                                 = 604,
    /// DIAGNOSTIC-DATA-TRANSFER-CLASS
    DiagnosticDataTransferClass                                            = 605,
    /// DIAGNOSTIC-DE-AUTHENTICATION
    DiagnosticDeAuthentication                                             = 606,
    /// DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS
    DiagnosticDebounceAlgorithmProps                                       = 607,
    /// DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING
    DiagnosticDemProvidedDataMapping                                       = 608,
    /// DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE
    DiagnosticDoIpActivationLineInterface                                  = 609,
    /// DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE
    DiagnosticDoIpGroupIdentificationInterface                             = 610,
    /// DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE
    DiagnosticDoIpPowerModeInterface                                       = 611,
    /// DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE
    DiagnosticDoIpTriggerVehicleAnnouncementInterface                      = 612,
    /// DIAGNOSTIC-DOWNLOAD-INTERFACE
    DiagnosticDownloadInterface                                            = 613,
    /// DIAGNOSTIC-DTC-INFORMATION-INTERFACE
    DiagnosticDtcInformationInterface                                      = 614,
    /// DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER
    DiagnosticDynamicDataIdentifier                                        = 615,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER
    DiagnosticDynamicallyDefineDataIdentifier                              = 616,
    /// DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS
    DiagnosticDynamicallyDefineDataIdentifierClass                         = 617,
    /// DIAGNOSTIC-ECU-INSTANCE-PROPS
    DiagnosticEcuInstanceProps                                             = 618,
    /// DIAGNOSTIC-ECU-RESET
    DiagnosticEcuReset                                                     = 619,
    /// DIAGNOSTIC-ECU-RESET-CLASS
    DiagnosticEcuResetClass                                                = 620,
    /// DIAGNOSTIC-ECU-RESET-INTERFACE
    DiagnosticEcuResetInterface                                            = 621,
    /// DIAGNOSTIC-ENABLE-CONDITION
    DiagnosticEnableCondition                                              = 622,
    /// DIAGNOSTIC-ENABLE-CONDITION-GROUP
    DiagnosticEnableConditionGroup                                         = 623,
    /// DIAGNOSTIC-ENABLE-CONDITION-NEEDS
    DiagnosticEnableConditionNeeds                                         = 624,
    /// DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING
    DiagnosticEnableConditionPortMapping                                   = 625,
    /// DIAGNOSTIC-ENV-BSW-MODE-ELEMENT
    DiagnosticEnvBswModeElement                                            = 626,
    /// DIAGNOSTIC-ENV-MODE-ELEMENT
    DiagnosticEnvModeElement                                               = 627,
    /// DIAGNOSTIC-ENV-SWC-MODE-ELEMENT
    DiagnosticEnvSwcModeElement                                            = 628,
    /// DIAGNOSTIC-ENVIRONMENTAL-CONDITION
    DiagnosticEnvironmentalCondition                                       = 629,
    /// DIAGNOSTIC-EVENT
    DiagnosticEvent                                                        = 630,
    /// DIAGNOSTIC-EVENT-INFO-NEEDS
    DiagnosticEventInfoNeeds                                               = 631,
    /// DIAGNOSTIC-EVENT-INTERFACE
    DiagnosticEventInterface                                               = 632,
    /// DIAGNOSTIC-EVENT-MANAGER
    DiagnosticEventManager                                                 = 633,
    /// DIAGNOSTIC-EVENT-MANAGER-NEEDS
    DiagnosticEventManagerNeeds                                            = 634,
    /// DIAGNOSTIC-EVENT-NEEDS
    DiagnosticEventNeeds                                                   = 635,
    /// DIAGNOSTIC-EVENT-PORT-MAPPING
    DiagnosticEventPortMapping                                             = 636,
    /// DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING
    DiagnosticEventToDebounceAlgorithmMapping                              = 637,
    /// DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING
    DiagnosticEventToEnableConditionGroupMapping                           = 638,
    /// DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING
    DiagnosticEventToOperationCycleMapping                                 = 639,
    /// DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING
    DiagnosticEventToSecurityEventMapping                                  = 640,
    /// DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING
    DiagnosticEventToStorageConditionGroupMapping                          = 641,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING
    DiagnosticEventToTroubleCodeJ1939Mapping                               = 642,
    /// DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING
    DiagnosticEventToTroubleCodeUdsMapping                                 = 643,
    /// DIAGNOSTIC-EXTENDED-DATA-RECORD
    DiagnosticExtendedDataRecord                                           = 644,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE
    DiagnosticExternalAuthenticationInterface                              = 645,
    /// DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING
    DiagnosticExternalAuthenticationPortMapping                            = 646,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT
    DiagnosticFimAliasEvent                                                = 647,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP
    DiagnosticFimAliasEventGroup                                           = 648,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING
    DiagnosticFimAliasEventGroupMapping                                    = 649,
    /// DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING
    DiagnosticFimAliasEventMapping                                         = 650,
    /// DIAGNOSTIC-FIM-EVENT-GROUP
    DiagnosticFimEventGroup                                                = 651,
    /// DIAGNOSTIC-FIM-FUNCTION-MAPPING
    DiagnosticFimFunctionMapping                                           = 652,
    /// DIAGNOSTIC-FREEZE-FRAME
    DiagnosticFreezeFrame                                                  = 653,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER
    DiagnosticFunctionIdentifier                                           = 654,
    /// DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT
    DiagnosticFunctionIdentifierInhibit                                    = 655,
    /// DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE
    DiagnosticFunctionInhibitSource                                        = 656,
    /// DIAGNOSTIC-GENERIC-UDS-INTERFACE
    DiagnosticGenericUdsInterface                                          = 657,
    /// DIAGNOSTIC-GENERIC-UDS-NEEDS
    DiagnosticGenericUdsNeeds                                              = 658,
    /// DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING
    DiagnosticGenericUdsPortMapping                                        = 659,
    /// DIAGNOSTIC-INDICATOR
    DiagnosticIndicator                                                    = 660,
    /// DIAGNOSTIC-INDICATOR-INTERFACE
    DiagnosticIndicatorInterface                                           = 661,
    /// DIAGNOSTIC-INDICATOR-NEEDS
    DiagnosticIndicatorNeeds                                               = 662,
    /// DIAGNOSTIC-INDICATOR-PORT-MAPPING
    DiagnosticIndicatorPortMapping                                         = 663,
    /// DIAGNOSTIC-INFO-TYPE
    DiagnosticInfoType                                                     = 664,
    /// DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING
    DiagnosticInhibitSourceEventMapping                                    = 665,
    /// DIAGNOSTIC-IO-CONTROL
    DiagnosticIoControl                                                    = 666,
    /// DIAGNOSTIC-IO-CONTROL-CLASS
    DiagnosticIoControlClass                                               = 667,
    /// DIAGNOSTIC-IO-CONTROL-NEEDS
    DiagnosticIoControlNeeds                                               = 668,
    /// DIAGNOSTIC-IUMPR
    DiagnosticIumpr                                                        = 669,
    /// DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP
    DiagnosticIumprDenominatorGroup                                        = 670,
    /// DIAGNOSTIC-IUMPR-GROUP
    DiagnosticIumprGroup                                                   = 671,
    /// DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING
    DiagnosticIumprToFunctionIdentifierMapping                             = 672,
    /// DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME
    DiagnosticJ1939ExpandedFreezeFrame                                     = 673,
    /// DIAGNOSTIC-J-1939-FREEZE-FRAME
    DiagnosticJ1939FreezeFrame                                             = 674,
    /// DIAGNOSTIC-J-1939-NODE
    DiagnosticJ1939Node                                                    = 675,
    /// DIAGNOSTIC-J-1939-SPN
    DiagnosticJ1939Spn                                                     = 676,
    /// DIAGNOSTIC-J-1939-SPN-MAPPING
    DiagnosticJ1939SpnMapping                                              = 677,
    /// DIAGNOSTIC-J-1939-SW-MAPPING
    DiagnosticJ1939SwMapping                                               = 678,
    /// DIAGNOSTIC-LOG-AND-TRACE
    DiagnosticLogAndTrace                                                  = 679,
    /// DIAGNOSTIC-MAPPING
    DiagnosticMapping                                                      = 680,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING
    DiagnosticMasterToSlaveEventMapping                                    = 681,
    /// DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET
    DiagnosticMasterToSlaveEventMappingSet                                 = 682,
    /// DIAGNOSTIC-MEASUREMENT-IDENTIFIER
    DiagnosticMeasurementIdentifier                                        = 683,
    /// DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS
    DiagnosticMemoryAddressableRangeAccess                                 = 684,
    /// DIAGNOSTIC-MEMORY-BY-ADDRESS
    DiagnosticMemoryByAddress                                              = 685,
    /// DIAGNOSTIC-MEMORY-DESTINATION
    DiagnosticMemoryDestination                                            = 686,
    /// DIAGNOSTIC-MEMORY-DESTINATION-MIRROR
    DiagnosticMemoryDestinationMirror                                      = 687,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING
    DiagnosticMemoryDestinationPortMapping                                 = 688,
    /// DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY
    DiagnosticMemoryDestinationPrimary                                     = 689,
    /// DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED
    DiagnosticMemoryDestinationUserDefined                                 = 690,
    /// DIAGNOSTIC-MEMORY-IDENTIFIER
    DiagnosticMemoryIdentifier                                             = 691,
    /// DIAGNOSTIC-MONITOR-INTERFACE
    DiagnosticMonitorInterface                                             = 692,
    /// DIAGNOSTIC-MONITOR-PORT-MAPPING
    DiagnosticMonitorPortMapping                                           = 693,
    /// DIAGNOSTIC-OPERATION-CYCLE
    DiagnosticOperationCycle                                               = 694,
    /// DIAGNOSTIC-OPERATION-CYCLE-INTERFACE
    DiagnosticOperationCycleInterface                                      = 695,
    /// DIAGNOSTIC-OPERATION-CYCLE-NEEDS
    DiagnosticOperationCycleNeeds                                          = 696,
    /// DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING
    DiagnosticOperationCyclePortMapping                                    = 697,
    /// DIAGNOSTIC-PARAMETER-IDENTIFIER
    DiagnosticParameterIdentifier                                          = 698,
    /// DIAGNOSTIC-PORT-INTERFACE
    DiagnosticPortInterface                                                = 699,
    /// DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME
    DiagnosticPowertrainFreezeFrame                                        = 700,
    /// DIAGNOSTIC-PROOF-OF-OWNERSHIP
    DiagnosticProofOfOwnership                                             = 701,
    /// DIAGNOSTIC-PROTOCOL
    DiagnosticProtocol                                                     = 702,
    /// DIAGNOSTIC-PROVIDED-DATA-MAPPING
    DiagnosticProvidedDataMapping                                          = 703,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER
    DiagnosticReadDataByIdentifier                                         = 704,
    /// DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadDataByIdentifierClass                                    = 705,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID
    DiagnosticReadDataByPeriodicId                                         = 706,
    /// DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS
    DiagnosticReadDataByPeriodicIdClass                                    = 707,
    /// DIAGNOSTIC-READ-DTC-INFORMATION
    DiagnosticReadDtcInformation                                           = 708,
    /// DIAGNOSTIC-READ-DTC-INFORMATION-CLASS
    DiagnosticReadDtcInformationClass                                      = 709,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS
    DiagnosticReadMemoryByAddress                                          = 710,
    /// DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS
    DiagnosticReadMemoryByAddressClass                                     = 711,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER
    DiagnosticReadScalingDataByIdentifier                                  = 712,
    /// DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS
    DiagnosticReadScalingDataByIdentifierClass                             = 713,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE
    DiagnosticRequestControlOfOnBoardDevice                                = 714,
    /// DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS
    DiagnosticRequestControlOfOnBoardDeviceClass                           = 715,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA
    DiagnosticRequestCurrentPowertrainData                                 = 716,
    /// DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS
    DiagnosticRequestCurrentPowertrainDataClass                            = 717,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD
    DiagnosticRequestDownload                                              = 718,
    /// DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS
    DiagnosticRequestDownloadClass                                         = 719,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC
    DiagnosticRequestEmissionRelatedDtc                                    = 720,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS
    DiagnosticRequestEmissionRelatedDtcClass                               = 721,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS
    DiagnosticRequestEmissionRelatedDtcPermanentStatus                     = 722,
    /// DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS
    DiagnosticRequestEmissionRelatedDtcPermanentStatusClass                = 723,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER
    DiagnosticRequestFileTransfer                                          = 724,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS
    DiagnosticRequestFileTransferClass                                     = 725,
    /// DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS
    DiagnosticRequestFileTransferNeeds                                     = 726,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS
    DiagnosticRequestOnBoardMonitoringTestResults                          = 727,
    /// DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS
    DiagnosticRequestOnBoardMonitoringTestResultsClass                     = 728,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA
    DiagnosticRequestPowertrainFreezeFrameData                             = 729,
    /// DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS
    DiagnosticRequestPowertrainFreezeFrameDataClass                        = 730,
    /// DIAGNOSTIC-REQUEST-ROUTINE-RESULTS
    DiagnosticRequestRoutineResults                                        = 731,
    /// DIAGNOSTIC-REQUEST-UPLOAD
    DiagnosticRequestUpload                                                = 732,
    /// DIAGNOSTIC-REQUEST-UPLOAD-CLASS
    DiagnosticRequestUploadClass                                           = 733,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO
    DiagnosticRequestVehicleInfo                                           = 734,
    /// DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS
    DiagnosticRequestVehicleInfoClass                                      = 735,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT
    DiagnosticResponseOnEvent                                              = 736,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS
    DiagnosticResponseOnEventClass                                         = 737,
    /// DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS
    DiagnosticResponseOnEventNeeds                                         = 738,
    /// DIAGNOSTIC-ROUTINE
    DiagnosticRoutine                                                      = 739,
    /// DIAGNOSTIC-ROUTINE-CONTROL
    DiagnosticRoutineControl                                               = 740,
    /// DIAGNOSTIC-ROUTINE-CONTROL-CLASS
    DiagnosticRoutineControlClass                                          = 741,
    /// DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE
    DiagnosticRoutineGenericInterface                                      = 742,
    /// DIAGNOSTIC-ROUTINE-INTERFACE
    DiagnosticRoutineInterface                                             = 743,
    /// DIAGNOSTIC-ROUTINE-NEEDS
    DiagnosticRoutineNeeds                                                 = 744,
    /// DIAGNOSTIC-ROUTINE-SUBFUNCTION
    DiagnosticRoutineSubfunction                                           = 745,
    /// DIAGNOSTIC-SECURITY-ACCESS
    DiagnosticSecurityAccess                                               = 746,
    /// DIAGNOSTIC-SECURITY-ACCESS-CLASS
    DiagnosticSecurityAccessClass                                          = 747,
    /// DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING
    DiagnosticSecurityEventReportingModeMapping                            = 748,
    /// DIAGNOSTIC-SECURITY-LEVEL
    DiagnosticSecurityLevel                                                = 749,
    /// DIAGNOSTIC-SECURITY-LEVEL-INTERFACE
    DiagnosticSecurityLevelInterface                                       = 750,
    /// DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING
    DiagnosticSecurityLevelPortMapping                                     = 751,
    /// DIAGNOSTIC-SERVICE-CLASS
    DiagnosticServiceClass                                                 = 752,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING
    DiagnosticServiceDataIdentifierMapping                                 = 753,
    /// DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING
    DiagnosticServiceDataIdentifierPortMapping                             = 754,
    /// DIAGNOSTIC-SERVICE-DATA-MAPPING
    DiagnosticServiceDataMapping                                           = 755,
    /// DIAGNOSTIC-SERVICE-GENERIC-MAPPING
    DiagnosticServiceGenericMapping                                        = 756,
    /// DIAGNOSTIC-SERVICE-INSTANCE
    DiagnosticServiceInstance                                              = 757,
    /// DIAGNOSTIC-SERVICE-SW-MAPPING
    DiagnosticServiceSwMapping                                             = 758,
    /// DIAGNOSTIC-SERVICE-TABLE
    DiagnosticServiceTable                                                 = 759,
    /// DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE
    DiagnosticServiceValidationInterface                                   = 760,
    /// DIAGNOSTIC-SERVICE-VALIDATION-MAPPING
    DiagnosticServiceValidationMapping                                     = 761,
    /// DIAGNOSTIC-SESSION
    DiagnosticSession                                                      = 762,
    /// DIAGNOSTIC-SESSION-CONTROL
    DiagnosticSessionControl                                               = 763,
    /// DIAGNOSTIC-SESSION-CONTROL-CLASS
    DiagnosticSessionControlClass                                          = 764,
    /// DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS
    DiagnosticSoftwareClusterProps                                         = 765,
    /// DIAGNOSTIC-START-ROUTINE
    DiagnosticStartRoutine                                                 = 766,
    /// DIAGNOSTIC-STOP-ROUTINE
    DiagnosticStopRoutine                                                  = 767,
    /// DIAGNOSTIC-STORAGE-CONDITION
    DiagnosticStorageCondition                                             = 768,
    /// DIAGNOSTIC-STORAGE-CONDITION-GROUP
    DiagnosticStorageConditionGroup                                        = 769,
    /// DIAGNOSTIC-STORAGE-CONDITION-NEEDS
    DiagnosticStorageConditionNeeds                                        = 770,
    /// DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING
    DiagnosticStorageConditionPortMapping                                  = 771,
    /// DIAGNOSTIC-SW-MAPPING
    DiagnosticSwMapping                                                    = 772,
    /// DIAGNOSTIC-TEST-RESULT
    DiagnosticTestResult                                                   = 773,
    /// DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER
    DiagnosticTestRoutineIdentifier                                        = 774,
    /// DIAGNOSTIC-TRANSFER-EXIT
    DiagnosticTransferExit                                                 = 775,
    /// DIAGNOSTIC-TRANSFER-EXIT-CLASS
    DiagnosticTransferExitClass                                            = 776,
    /// DIAGNOSTIC-TROUBLE-CODE
    DiagnosticTroubleCode                                                  = 777,
    /// DIAGNOSTIC-TROUBLE-CODE-GROUP
    DiagnosticTroubleCodeGroup                                             = 778,
    /// DIAGNOSTIC-TROUBLE-CODE-J-1939
    DiagnosticTroubleCodeJ1939                                             = 779,
    /// DIAGNOSTIC-TROUBLE-CODE-OBD
    DiagnosticTroubleCodeObd                                               = 780,
    /// DIAGNOSTIC-TROUBLE-CODE-PROPS
    DiagnosticTroubleCodeProps                                             = 781,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS
    DiagnosticTroubleCodeUds                                               = 782,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING
    DiagnosticTroubleCodeUdsToClearConditionGroupMapping                   = 783,
    /// DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING
    DiagnosticTroubleCodeUdsToTroubleCodeObdMapping                        = 784,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS
    DiagnosticUploadDownloadNeeds                                          = 785,
    /// DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING
    DiagnosticUploadDownloadPortMapping                                    = 786,
    /// DIAGNOSTIC-UPLOAD-INTERFACE
    DiagnosticUploadInterface                                              = 787,
    /// DIAGNOSTIC-VALUE-NEEDS
    DiagnosticValueNeeds                                                   = 788,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL
    DiagnosticVerifyCertificateBidirectional                               = 789,
    /// DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL
    DiagnosticVerifyCertificateUnidirectional                              = 790,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER
    DiagnosticWriteDataByIdentifier                                        = 791,
    /// DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS
    DiagnosticWriteDataByIdentifierClass                                   = 792,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS
    DiagnosticWriteMemoryByAddress                                         = 793,
    /// DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS
    DiagnosticWriteMemoryByAddressClass                                    = 794,
    /// DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS
    DiagnosticsCommunicationSecurityNeeds                                  = 795,
    /// DISABLE
    Disable                                                                = 796,
    /// DLNA
    Dlna                                                                   = 797,
    /// DLT-APPLICATION
    DltApplication                                                         = 798,
    /// DLT-APPLICATION-TO-PROCESS-MAPPING
    DltApplicationToProcessMapping                                         = 799,
    /// DLT-ARGUMENT
    DltArgument                                                            = 800,
    /// DLT-CONTEXT
    DltContext                                                             = 801,
    /// DLT-ECU
    DltEcu                                                                 = 802,
    /// DLT-LOG-CHANNEL
    DltLogChannel                                                          = 803,
    /// DLT-LOG-CHANNEL-DESIGN
    DltLogChannelDesign                                                    = 804,
    /// DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING
    DltLogChannelDesignToProcessDesignMapping                              = 805,
    /// DLT-LOG-CHANNEL-TO-PROCESS-MAPPING
    DltLogChannelToProcessMapping                                          = 806,
    /// DLT-LOG-SINK
    DltLogSink                                                             = 807,
    /// DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING
    DltLogSinkToPortPrototypeMapping                                       = 808,
    /// DLT-MESSAGE
    DltMessage                                                             = 809,
    /// DLT-MESSAGE-COLLECTION-SET
    DltMessageCollectionSet                                                = 810,
    /// DLT-USER-NEEDS
    DltUserNeeds                                                           = 811,
    /// DO-IP
    DoIp                                                                   = 812,
    /// DO-IP-ACTIVATION-LINE-NEEDS
    DoIpActivationLineNeeds                                                = 813,
    /// DO-IP-GID-NEEDS
    DoIpGidNeeds                                                           = 814,
    /// DO-IP-GID-SYNCHRONIZATION-NEEDS
    DoIpGidSynchronizationNeeds                                            = 815,
    /// DO-IP-INSTANTIATION
    DoIpInstantiation                                                      = 816,
    /// DO-IP-INTERFACE
    DoIpInterface                                                          = 817,
    /// DO-IP-LOGIC-ADDRESS
    DoIpLogicAddress                                                       = 818,
    /// DO-IP-LOGIC-TARGET-ADDRESS-PROPS
    DoIpLogicTargetAddressProps                                            = 819,
    /// DO-IP-LOGIC-TESTER-ADDRESS-PROPS
    DoIpLogicTesterAddressProps                                            = 820,
    /// DO-IP-POWER-MODE-STATUS-NEEDS
    DoIpPowerModeStatusNeeds                                               = 821,
    /// DO-IP-ROUTING-ACTIVATION
    DoIpRoutingActivation                                                  = 822,
    /// DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS
    DoIpRoutingActivationAuthenticationNeeds                               = 823,
    /// DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS
    DoIpRoutingActivationConfirmationNeeds                                 = 824,
    /// DO-IP-SERVICE-NEEDS
    DoIpServiceNeeds                                                       = 825,
    /// DO-IP-TP-CONFIG
    DoIpTpConfig                                                           = 826,
    /// DOCUMENT-ELEMENT-SCOPE
    DocumentElementScope                                                   = 827,
    /// DOCUMENTATION
    Documentation                                                          = 828,
    /// DOCUMENTATION-CONTEXT
    DocumentationContext                                                   = 829,
    /// DOES-NOT-REPORT-EXECUTION-STATE
    DoesNotReportExecutionState                                            = 830,
    /// DOES-NOT-SUPPORT-BUFFER-LOCKING
    DoesNotSupportBufferLocking                                            = 831,
    /// DOES-NOT-USE-LOGGING
    DoesNotUseLogging                                                      = 832,
    /// DOMAIN-PARTICIPANT-USER-DATA-QOS
    DomainParticipantUserDataQos                                           = 833,
    /// DONT-INVALIDATE
    DontInvalidate                                                         = 834,
    /// DROP
    Drop                                                                   = 835,
    /// DROP-FRAME
    DropFrame                                                              = 836,
    /// DROP-UNTAGGED
    DropUntagged                                                           = 837,
    /// DSA
    Dsa                                                                    = 838,
    /// DTC-STATUS-CHANGE-NOTIFICATION-NEEDS
    DtcStatusChangeNotificationNeeds                                       = 839,
    /// DYNAMIC-PART-TRIGGER
    DynamicPartTrigger                                                     = 840,
    /// DZ
    Dz                                                                     = 841,
    /// E-2-E-PROFILE-COMPATIBILITY-PROPS
    E2EProfileCompatibilityProps                                           = 842,
    /// E-2-E-PROFILE-CONFIGURATION
    E2EProfileConfiguration                                                = 843,
    /// E-2-E-PROFILE-CONFIGURATION-SET
    E2EProfileConfigurationSet                                             = 844,
    /// ECC
    Ecc                                                                    = 845,
    /// ECU
    Ecu                                                                    = 846,
    /// ECU-ABSTRACTION-SW-COMPONENT-TYPE
    EcuAbstractionSwComponentType                                          = 847,
    /// ECU-INSTANCE
    EcuInstance                                                            = 848,
    /// ECU-MANAGER
    EcuManager                                                             = 849,
    /// ECU-MAPPING
    EcuMapping                                                             = 850,
    /// ECU-PARTITION
    EcuPartition                                                           = 851,
    /// ECU-STATE-MGR-USER-NEEDS
    EcuStateMgrUserNeeds                                                   = 852,
    /// ECU-TIMING
    EcuTiming                                                              = 853,
    /// ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF
    EcucAbstractExternalReferenceDef                                       = 854,
    /// ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF
    EcucAbstractInternalReferenceDef                                       = 855,
    /// ECUC-ABSTRACT-REFERENCE-DEF
    EcucAbstractReferenceDef                                               = 856,
    /// ECUC-ABSTRACT-STRING-PARAM-DEF
    EcucAbstractStringParamDef                                             = 857,
    /// ECUC-ADD-INFO-PARAM-DEF
    EcucAddInfoParamDef                                                    = 858,
    /// ECUC-BOOLEAN-PARAM-DEF
    EcucBooleanParamDef                                                    = 859,
    /// ECUC-CHOICE-CONTAINER-DEF
    EcucChoiceContainerDef                                                 = 860,
    /// ECUC-CHOICE-REFERENCE-DEF
    EcucChoiceReferenceDef                                                 = 861,
    /// ECUC-COMMON-ATTRIBUTES
    EcucCommonAttributes                                                   = 862,
    /// ECUC-CONTAINER-DEF
    EcucContainerDef                                                       = 863,
    /// ECUC-CONTAINER-VALUE
    EcucContainerValue                                                     = 864,
    /// ECUC-DEFINITION-COLLECTION
    EcucDefinitionCollection                                               = 865,
    /// ECUC-DEFINITION-ELEMENT
    EcucDefinitionElement                                                  = 866,
    /// ECUC-DESTINATION-URI-DEF
    EcucDestinationUriDef                                                  = 867,
    /// ECUC-DESTINATION-URI-DEF-SET
    EcucDestinationUriDefSet                                               = 868,
    /// ECUC-ENUMERATION-LITERAL-DEF
    EcucEnumerationLiteralDef                                              = 869,
    /// ECUC-ENUMERATION-PARAM-DEF
    EcucEnumerationParamDef                                                = 870,
    /// ECUC-FLOAT-PARAM-DEF
    EcucFloatParamDef                                                      = 871,
    /// ECUC-FOREIGN-REFERENCE-DEF
    EcucForeignReferenceDef                                                = 872,
    /// ECUC-FUNCTION-NAME-DEF
    EcucFunctionNameDef                                                    = 873,
    /// ECUC-INSTANCE-REFERENCE-DEF
    EcucInstanceReferenceDef                                               = 874,
    /// ECUC-INTEGER-PARAM-DEF
    EcucIntegerParamDef                                                    = 875,
    /// ECUC-LINKER-SYMBOL-DEF
    EcucLinkerSymbolDef                                                    = 876,
    /// ECUC-MODULE-CONFIGURATION-VALUES
    EcucModuleConfigurationValues                                          = 877,
    /// ECUC-MODULE-DEF
    EcucModuleDef                                                          = 878,
    /// ECUC-MULTILINE-STRING-PARAM-DEF
    EcucMultilineStringParamDef                                            = 879,
    /// ECUC-PARAM-CONF-CONTAINER-DEF
    EcucParamConfContainerDef                                              = 880,
    /// ECUC-PARAMETER-DEF
    EcucParameterDef                                                       = 881,
    /// ECUC-QUERY
    EcucQuery                                                              = 882,
    /// ECUC-QUERY-EXPRESSION
    EcucQueryExpression                                                    = 883,
    /// ECUC-REFERENCE-DEF
    EcucReferenceDef                                                       = 884,
    /// ECUC-STRING-PARAM-DEF
    EcucStringParamDef                                                     = 885,
    /// ECUC-SYMBOLIC-NAME-REFERENCE-DEF
    EcucSymbolicNameReferenceDef                                           = 886,
    /// ECUC-URI-REFERENCE-DEF
    EcucUriReferenceDef                                                    = 887,
    /// ECUC-VALIDATION-CONDITION
    EcucValidationCondition                                                = 888,
    /// ECUC-VALUE-COLLECTION
    EcucValueCollection                                                    = 889,
    /// EDGE-NODE
    EdgeNode                                                               = 890,
    /// EL
    El                                                                     = 891,
    /// EMISSION-RELATED-DTC
    EmissionRelatedDtc                                                     = 892,
    /// EN
    En                                                                     = 893,
    /// ENABLE
    Enable                                                                 = 894,
    /// ENABLED
    Enabled                                                                = 895,
    /// ENCRYPT-AND-SIGN
    EncryptAndSign                                                         = 896,
    /// ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION
    EncryptAndSignWithOriginAuthentication                                 = 897,
    /// ENCRYPTION
    Encryption                                                             = 898,
    /// END-2-END-EVENT-PROTECTION-PROPS
    End2EndEventProtectionProps                                            = 899,
    /// END-2-END-METHOD-PROTECTION-PROPS
    End2EndMethodProtectionProps                                           = 900,
    /// END-TO-END-PROTECTION
    EndToEndProtection                                                     = 901,
    /// END-TO-END-PROTECTION-I-SIGNAL-I-PDU
    EndToEndProtectionISignalIPdu                                          = 902,
    /// END-TO-END-PROTECTION-SET
    EndToEndProtectionSet                                                  = 903,
    /// END-TO-END-PROTECTION-VARIABLE-PROTOTYPE
    EndToEndProtectionVariablePrototype                                    = 904,
    /// ENHANCED
    Enhanced                                                               = 905,
    /// ENUMERATION-MAPPING-TABLE
    EnumerationMappingTable                                                = 906,
    /// EO
    Eo                                                                     = 907,
    /// EOC-EVENT-REF
    EocEventRef                                                            = 908,
    /// EOC-EXECUTABLE-ENTITY-REF
    EocExecutableEntityRef                                                 = 909,
    /// EOC-EXECUTABLE-ENTITY-REF-ABSTRACT
    EocExecutableEntityRefAbstract                                         = 910,
    /// EOC-EXECUTABLE-ENTITY-REF-GROUP
    EocExecutableEntityRefGroup                                            = 911,
    /// EPS
    Eps                                                                    = 912,
    /// EQUAL
    Equal                                                                  = 913,
    /// ERROR
    Error                                                                  = 914,
    /// ERROR-CORRECTION
    ErrorCorrection                                                        = 915,
    /// ERROR-DETECTION
    ErrorDetection                                                         = 916,
    /// ERROR-TRACER
    ErrorTracer                                                            = 917,
    /// ERROR-TRACER-NEEDS
    ErrorTracerNeeds                                                       = 918,
    /// ES
    Es                                                                     = 919,
    /// ESP
    Esp                                                                    = 920,
    /// ET
    Et                                                                     = 921,
    /// ETH-IP-PROPS
    EthIpProps                                                             = 922,
    /// ETH-TCP-IP-ICMP-PROPS
    EthTcpIpIcmpProps                                                      = 923,
    /// ETH-TCP-IP-PROPS
    EthTcpIpProps                                                          = 924,
    /// ETH-TP-CONFIG
    EthTpConfig                                                            = 925,
    /// ETHERNET-CLUSTER
    EthernetCluster                                                        = 926,
    /// ETHERNET-COMMUNICATION-CONNECTOR
    EthernetCommunicationConnector                                         = 927,
    /// ETHERNET-COMMUNICATION-CONTROLLER
    EthernetCommunicationController                                        = 928,
    /// ETHERNET-FRAME
    EthernetFrame                                                          = 929,
    /// ETHERNET-FRAME-TRIGGERING
    EthernetFrameTriggering                                                = 930,
    /// ETHERNET-NETWORK-CONFIGURATION
    EthernetNetworkConfiguration                                           = 931,
    /// ETHERNET-PHYSICAL-CHANNEL
    EthernetPhysicalChannel                                                = 932,
    /// ETHERNET-PRIORITY-REGENERATION
    EthernetPriorityRegeneration                                           = 933,
    /// ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING
    EthernetRawDataStreamClientMapping                                     = 934,
    /// ETHERNET-RAW-DATA-STREAM-GRANT
    EthernetRawDataStreamGrant                                             = 935,
    /// ETHERNET-RAW-DATA-STREAM-MAPPING
    EthernetRawDataStreamMapping                                           = 936,
    /// ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING
    EthernetRawDataStreamServerMapping                                     = 937,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG
    EthernetWakeupSleepOnDatalineConfig                                    = 938,
    /// ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET
    EthernetWakeupSleepOnDatalineConfigSet                                 = 939,
    /// EU
    Eu                                                                     = 940,
    /// EVALUATED-VARIANT-SET
    EvaluatedVariantSet                                                    = 941,
    /// EVAP
    Evap                                                                   = 942,
    /// EVENT-ACCEPTANCE-DISABLED
    EventAcceptanceDisabled                                                = 943,
    /// EVENT-ACCEPTANCE-ENABLED
    EventAcceptanceEnabled                                                 = 944,
    /// EVENT-COMBINATION-ON-RETRIEVAL
    EventCombinationOnRetrieval                                            = 945,
    /// EVENT-COMBINATION-ON-STORAGE
    EventCombinationOnStorage                                              = 946,
    /// EVENT-HANDLER
    EventHandler                                                           = 947,
    /// EVENT-MAPPING
    EventMapping                                                           = 948,
    /// EVENT-STORAGE-DISABLED
    EventStorageDisabled                                                   = 949,
    /// EVENT-STORAGE-ENABLED
    EventStorageEnabled                                                    = 950,
    /// EVENT-TRIGGERING-CONSTRAINT
    EventTriggeringConstraint                                              = 951,
    /// EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE
    EventWindowCurrentAndFollowingCycle                                    = 952,
    /// EVENT-WINDOW-CURRENT-CYCLE
    EventWindowCurrentCycle                                                = 953,
    /// EVENT-WINDOW-INFINITE
    EventWindowInfinite                                                    = 954,
    /// EXACT-OR-ANY-MINOR-VERSION
    ExactOrAnyMinorVersion                                                 = 955,
    /// EXAMPLE
    Example                                                                = 956,
    /// EXCLUDE-FROM-FLASH
    ExcludeFromFlash                                                       = 957,
    /// EXCLUSIVE
    Exclusive                                                              = 958,
    /// EXCLUSIVE-AREA
    ExclusiveArea                                                          = 959,
    /// EXCLUSIVE-AREA-NESTING-ORDER
    ExclusiveAreaNestingOrder                                              = 960,
    /// EXECUTABLE
    Executable                                                             = 961,
    /// EXECUTABLE-ENTITY
    ExecutableEntity                                                       = 962,
    /// EXECUTABLE-ENTITY-ACTIVATION-REASON
    ExecutableEntityActivationReason                                       = 963,
    /// EXECUTABLE-GROUP
    ExecutableGroup                                                        = 964,
    /// EXECUTABLE-TIMING
    ExecutableTiming                                                       = 965,
    /// EXECUTE
    Execute                                                                = 966,
    /// EXECUTION-ORDER-CONSTRAINT
    ExecutionOrderConstraint                                               = 967,
    /// EXECUTION-TIME
    ExecutionTime                                                          = 968,
    /// EXECUTION-TIME-CONSTRAINT
    ExecutionTimeConstraint                                                = 969,
    /// EXERCISE
    Exercise                                                               = 970,
    /// EXPLICIT
    Explicit                                                               = 971,
    /// EXTENDED
    Extended                                                               = 972,
    /// EXTERNAL-REPLACEMENT
    ExternalReplacement                                                    = 973,
    /// EXTERNAL-TRIGGER-OCCURRED-EVENT
    ExternalTriggerOccurredEvent                                           = 974,
    /// EXTERNAL-TRIGGERING-POINT-IDENT
    ExternalTriggeringPointIdent                                           = 975,
    /// FA
    Fa                                                                     = 976,
    /// FAILURE-AND-SUCCESS
    FailureAndSuccess                                                      = 977,
    /// FAILURE-ONLY
    FailureOnly                                                            = 978,
    /// FALSE
    False                                                                  = 979,
    /// FAST-FLASHING-MODE
    FastFlashingMode                                                       = 980,
    /// FATAL
    Fatal                                                                  = 981,
    /// FAULT
    Fault                                                                  = 982,
    /// FDC-THRESHOLD
    FdcThreshold                                                           = 983,
    /// FI
    Fi                                                                     = 984,
    /// FIBEX-ELEMENT
    FibexElement                                                           = 985,
    /// FIELD
    Field                                                                  = 986,
    /// FIELD-MAPPING
    FieldMapping                                                           = 987,
    /// FILE
    File                                                                   = 988,
    /// FILTERED
    Filtered                                                               = 989,
    /// FINISH
    Finish                                                                 = 990,
    /// FIRE-AND-FORGET-MAPPING
    FireAndForgetMapping                                                   = 991,
    /// FIRST-CONTAINED-TRIGGER
    FirstContainedTrigger                                                  = 992,
    /// FIRST-TO-SECOND
    FirstToSecond                                                          = 993,
    /// FIT-TO-PAGE
    FitToPage                                                              = 994,
    /// FIT-TO-TEXT
    FitToText                                                              = 995,
    /// FIX-AXIS
    FixAxis                                                                = 996,
    /// FIXED
    Fixed                                                                  = 997,
    /// FIXED-SIZE
    FixedSize                                                              = 998,
    /// FIX_AXIS
    Fixaxis                                                                = 999,
    /// FJ
    Fj                                                                     = 1000,
    /// FLAT-INSTANCE-DESCRIPTOR
    FlatInstanceDescriptor                                                 = 1001,
    /// FLAT-MAP
    FlatMap                                                                = 1002,
    /// FLEXRAY-AR-TP-CONFIG
    FlexrayArTpConfig                                                      = 1003,
    /// FLEXRAY-AR-TP-NODE
    FlexrayArTpNode                                                        = 1004,
    /// FLEXRAY-CLUSTER
    FlexrayCluster                                                         = 1005,
    /// FLEXRAY-COMMUNICATION-CONNECTOR
    FlexrayCommunicationConnector                                          = 1006,
    /// FLEXRAY-COMMUNICATION-CONTROLLER
    FlexrayCommunicationController                                         = 1007,
    /// FLEXRAY-FRAME
    FlexrayFrame                                                           = 1008,
    /// FLEXRAY-FRAME-TRIGGERING
    FlexrayFrameTriggering                                                 = 1009,
    /// FLEXRAY-NM-CLUSTER
    FlexrayNmCluster                                                       = 1010,
    /// FLEXRAY-NM-NODE
    FlexrayNmNode                                                          = 1011,
    /// FLEXRAY-PHYSICAL-CHANNEL
    FlexrayPhysicalChannel                                                 = 1012,
    /// FLEXRAY-TP-CONFIG
    FlexrayTpConfig                                                        = 1013,
    /// FLEXRAY-TP-CONNECTION-CONTROL
    FlexrayTpConnectionControl                                             = 1014,
    /// FLEXRAY-TP-NODE
    FlexrayTpNode                                                          = 1015,
    /// FLEXRAY-TP-PDU-POOL
    FlexrayTpPduPool                                                       = 1016,
    /// FLOAT
    Float                                                                  = 1017,
    /// FM-ATTRIBUTE-DEF
    FmAttributeDef                                                         = 1018,
    /// FM-FEATURE
    FmFeature                                                              = 1019,
    /// FM-FEATURE-MAP
    FmFeatureMap                                                           = 1020,
    /// FM-FEATURE-MAP-ASSERTION
    FmFeatureMapAssertion                                                  = 1021,
    /// FM-FEATURE-MAP-CONDITION
    FmFeatureMapCondition                                                  = 1022,
    /// FM-FEATURE-MAP-ELEMENT
    FmFeatureMapElement                                                    = 1023,
    /// FM-FEATURE-MODEL
    FmFeatureModel                                                         = 1024,
    /// FM-FEATURE-RELATION
    FmFeatureRelation                                                      = 1025,
    /// FM-FEATURE-RESTRICTION
    FmFeatureRestriction                                                   = 1026,
    /// FM-FEATURE-SELECTION
    FmFeatureSelection                                                     = 1027,
    /// FM-FEATURE-SELECTION-SET
    FmFeatureSelectionSet                                                  = 1028,
    /// FO
    Fo                                                                     = 1029,
    /// FOR-ALL
    ForAll                                                                 = 1030,
    /// FORGET
    Forget                                                                 = 1031,
    /// FORWARD-AS-IS
    ForwardAsIs                                                            = 1032,
    /// FR
    Fr                                                                     = 1033,
    /// FRAME
    Frame                                                                  = 1034,
    /// FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION
    FrameEthernetQueuedForTransmission                                     = 1035,
    /// FRAME-ETHERNET-RECEIVED-BY-IF
    FrameEthernetReceivedByIf                                              = 1036,
    /// FRAME-ETHERNET-RECEIVED-ON-BUS
    FrameEthernetReceivedOnBus                                             = 1037,
    /// FRAME-ETHERNET-SENT-ON-BUS
    FrameEthernetSentOnBus                                                 = 1038,
    /// FRAME-PORT
    FramePort                                                              = 1039,
    /// FRAME-QUEUED-FOR-TRANSMISSION
    FrameQueuedForTransmission                                             = 1040,
    /// FRAME-RECEIVED-BY-IF
    FrameReceivedByIf                                                      = 1041,
    /// FRAME-TRANSMITTED-ON-BUS
    FrameTransmittedOnBus                                                  = 1042,
    /// FRAME-TRIGGERING
    FrameTriggering                                                        = 1043,
    /// FULL
    Full                                                                   = 1044,
    /// FULL-DUPLEX-MODE
    FullDuplexMode                                                         = 1045,
    /// FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM
    FunctionGroupModeRequestPhmActionItem                                  = 1046,
    /// FUNCTION-GROUP-SET
    FunctionGroupSet                                                       = 1047,
    /// FUNCTION-INHIBITION-AVAILABILITY-NEEDS
    FunctionInhibitionAvailabilityNeeds                                    = 1048,
    /// FUNCTION-INHIBITION-MANAGER
    FunctionInhibitionManager                                              = 1049,
    /// FUNCTION-INHIBITION-NEEDS
    FunctionInhibitionNeeds                                                = 1050,
    /// FUNCTIONAL
    Functional                                                             = 1051,
    /// FUNCTIONAL-ADDRESS
    FunctionalAddress                                                      = 1052,
    /// FUNCTIONAL-CAN-FD
    FunctionalCanFd                                                        = 1053,
    /// FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING
    FunctionalClusterInteractsWithFunctionalClusterMapping                 = 1054,
    /// FURTHER-ACTION-BYTE-NEEDS
    FurtherActionByteNeeds                                                 = 1055,
    /// FY
    Fy                                                                     = 1056,
    /// GA
    Ga                                                                     = 1057,
    /// GATEWAY
    Gateway                                                                = 1058,
    /// GD
    Gd                                                                     = 1059,
    /// GENERAL-PARAMETER
    GeneralParameter                                                       = 1060,
    /// GENERAL-PURPOSE-CONNECTION
    GeneralPurposeConnection                                               = 1061,
    /// GENERAL-PURPOSE-I-PDU
    GeneralPurposeIPdu                                                     = 1062,
    /// GENERAL-PURPOSE-PDU
    GeneralPurposePdu                                                      = 1063,
    /// GENERIC-ETHERNET-FRAME
    GenericEthernetFrame                                                   = 1064,
    /// GENERIC-MODULE-INSTANTIATION
    GenericModuleInstantiation                                             = 1065,
    /// GET
    Get                                                                    = 1066,
    /// GETTER
    Getter                                                                 = 1067,
    /// GETTER-SETTER
    GetterSetter                                                           = 1068,
    /// GIF
    Gif                                                                    = 1069,
    /// GL
    Gl                                                                     = 1070,
    /// GLOBAL-SUPERVISION
    GlobalSupervision                                                      = 1071,
    /// GLOBAL-SUPERVISION-ENTITY
    GlobalSupervisionEntity                                                = 1072,
    /// GLOBAL-SUPERVISION-NEEDS
    GlobalSupervisionNeeds                                                 = 1073,
    /// GLOBAL-TIME-CAN-MASTER
    GlobalTimeCanMaster                                                    = 1074,
    /// GLOBAL-TIME-CAN-SLAVE
    GlobalTimeCanSlave                                                     = 1075,
    /// GLOBAL-TIME-DOMAIN
    GlobalTimeDomain                                                       = 1076,
    /// GLOBAL-TIME-ETH-MASTER
    GlobalTimeEthMaster                                                    = 1077,
    /// GLOBAL-TIME-ETH-SLAVE
    GlobalTimeEthSlave                                                     = 1078,
    /// GLOBAL-TIME-FR-MASTER
    GlobalTimeFrMaster                                                     = 1079,
    /// GLOBAL-TIME-FR-SLAVE
    GlobalTimeFrSlave                                                      = 1080,
    /// GLOBAL-TIME-GATEWAY
    GlobalTimeGateway                                                      = 1081,
    /// GLOBAL-TIME-MASTER
    GlobalTimeMaster                                                       = 1082,
    /// GLOBAL-TIME-SLAVE
    GlobalTimeSlave                                                        = 1083,
    /// GN
    Gn                                                                     = 1084,
    /// GRANT
    Grant                                                                  = 1085,
    /// GRANT-DESIGN
    GrantDesign                                                            = 1086,
    /// GROSS
    Gross                                                                  = 1087,
    /// GU
    Gu                                                                     = 1088,
    /// GZIP
    Gzip                                                                   = 1089,
    /// HA
    Ha                                                                     = 1090,
    /// HALF-DUPLEX-MODE
    HalfDuplexMode                                                         = 1091,
    /// HARDWARE-TEST-MANAGER
    HardwareTestManager                                                    = 1092,
    /// HARDWARE-TEST-NEEDS
    HardwareTestNeeds                                                      = 1093,
    /// HEAD
    Head                                                                   = 1094,
    /// HEALTH-CHANNEL
    HealthChannel                                                          = 1095,
    /// HEALTH-CHANNEL-EXTERNAL-MODE
    HealthChannelExternalMode                                              = 1096,
    /// HEALTH-CHANNEL-EXTERNAL-STATUS
    HealthChannelExternalStatus                                            = 1097,
    /// HEALTH-CHANNEL-SUPERVISION
    HealthChannelSupervision                                               = 1098,
    /// HEAP-USAGE
    HeapUsage                                                              = 1099,
    /// HI
    Hi                                                                     = 1100,
    /// HIERARCHICAL-EOC
    HierarchicalEoc                                                        = 1101,
    /// HIGH
    High                                                                   = 1102,
    /// HINT
    Hint                                                                   = 1103,
    /// HOOK
    Hook                                                                   = 1104,
    /// HOST-PORT
    HostPort                                                               = 1105,
    /// HR
    Hr                                                                     = 1106,
    /// HU
    Hu                                                                     = 1107,
    /// HUB
    Hub                                                                    = 1108,
    /// HW-ATTRIBUTE-DEF
    HwAttributeDef                                                         = 1109,
    /// HW-ATTRIBUTE-LITERAL-DEF
    HwAttributeLiteralDef                                                  = 1110,
    /// HW-CATEGORY
    HwCategory                                                             = 1111,
    /// HW-DESCRIPTION-ENTITY
    HwDescriptionEntity                                                    = 1112,
    /// HW-ELEMENT
    HwElement                                                              = 1113,
    /// HW-PIN
    HwPin                                                                  = 1114,
    /// HW-PIN-GROUP
    HwPinGroup                                                             = 1115,
    /// HW-TYPE
    HwType                                                                 = 1116,
    /// HY
    Hy                                                                     = 1117,
    /// I-4-G
    I4G                                                                    = 1118,
    /// I-PDU
    IPdu                                                                   = 1119,
    /// I-PDU-PORT
    IPduPort                                                               = 1120,
    /// I-PDU-RECEIVED-BY-COM
    IPduReceivedByCom                                                      = 1121,
    /// I-PDU-SENT-TO-IF
    IPduSentToIf                                                           = 1122,
    /// I-PDU-TRIGGERING
    IPduTriggering                                                         = 1123,
    /// I-PV-6-EXT-HEADER-FILTER-LIST
    IPv6ExtHeaderFilterList                                                = 1124,
    /// I-PV-6-EXT-HEADER-FILTER-SET
    IPv6ExtHeaderFilterSet                                                 = 1125,
    /// I-SIGNAL
    ISignal                                                                = 1126,
    /// I-SIGNAL-AVAILABLE-FOR-RTE
    ISignalAvailableForRte                                                 = 1127,
    /// I-SIGNAL-GROUP
    ISignalGroup                                                           = 1128,
    /// I-SIGNAL-I-PDU
    ISignalIPdu                                                            = 1129,
    /// I-SIGNAL-I-PDU-GROUP
    ISignalIPduGroup                                                       = 1130,
    /// I-SIGNAL-PORT
    ISignalPort                                                            = 1131,
    /// I-SIGNAL-SENT-TO-COM
    ISignalSentToCom                                                       = 1132,
    /// I-SIGNAL-TO-I-PDU-MAPPING
    ISignalToIPduMapping                                                   = 1133,
    /// I-SIGNAL-TRIGGERING
    ISignalTriggering                                                      = 1134,
    /// IA
    Ia                                                                     = 1135,
    /// IAM-MODULE-INSTANTIATION
    IamModuleInstantiation                                                 = 1136,
    /// ICMP
    Icmp                                                                   = 1137,
    /// IDENT-CAPTION
    IdentCaption                                                           = 1138,
    /// IDENTIFIABLE
    Identifiable                                                           = 1139,
    /// IDS-COMMON-ELEMENT
    IdsCommonElement                                                       = 1140,
    /// IDS-DESIGN
    IdsDesign                                                              = 1141,
    /// IDS-MAPPING
    IdsMapping                                                             = 1142,
    /// IDS-MGR-CUSTOM-TIMESTAMP-NEEDS
    IdsMgrCustomTimestampNeeds                                             = 1143,
    /// IDS-MGR-NEEDS
    IdsMgrNeeds                                                            = 1144,
    /// IDS-PLATFORM-INSTANTIATION
    IdsPlatformInstantiation                                               = 1145,
    /// IDSM-INSTANCE
    IdsmInstance                                                           = 1146,
    /// IDSM-MODULE-INSTANTIATION
    IdsmModuleInstantiation                                                = 1147,
    /// IDSM-PROPERTIES
    IdsmProperties                                                         = 1148,
    /// IDSM-RATE-LIMITATION
    IdsmRateLimitation                                                     = 1149,
    /// IDSM-TRAFFIC-LIMITATION
    IdsmTrafficLimitation                                                  = 1150,
    /// IE
    Ie                                                                     = 1151,
    /// IEEE-1722-TP-ETHERNET-FRAME
    Ieee1722TpEthernetFrame                                                = 1152,
    /// IEEE802-11P
    Ieee80211p                                                             = 1153,
    /// IEEE802-1AS
    Ieee8021as                                                             = 1154,
    /// IEEE802-1AS-AUTOSAR
    Ieee8021asAutosar                                                      = 1155,
    /// IGNITION
    Ignition                                                               = 1156,
    /// IGNORE
    Ignore                                                                 = 1157,
    /// IK
    Ik                                                                     = 1158,
    /// IMMEDIATE
    Immediate                                                              = 1159,
    /// IMMEDIATELY
    Immediately                                                            = 1160,
    /// IMPLEMENTATION
    Implementation                                                         = 1161,
    /// IMPLEMENTATION-DATA-TYPE
    ImplementationDataType                                                 = 1162,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT
    ImplementationDataTypeElement                                          = 1163,
    /// IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION
    ImplementationDataTypeElementExtension                                 = 1164,
    /// IMPLEMENTATION-DATA-TYPE-EXTENSION
    ImplementationDataTypeExtension                                        = 1165,
    /// IMPLEMENTATION-PROPS
    ImplementationProps                                                    = 1166,
    /// IN
    In                                                                     = 1167,
    /// INCREASING
    Increasing                                                             = 1168,
    /// INDICATE
    Indicate                                                               = 1169,
    /// INDICATOR-STATUS-NEEDS
    IndicatorStatusNeeds                                                   = 1170,
    /// INDIVIDUAL
    Individual                                                             = 1171,
    /// INFINITE
    Infinite                                                               = 1172,
    /// INFINITE-TIME-TO-RESPONSE
    InfiniteTimeToResponse                                                 = 1173,
    /// INFO
    Info                                                                   = 1174,
    /// INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE
    InheritedFromArrayElementTypeSize                                      = 1175,
    /// INIT-EVENT
    InitEvent                                                              = 1176,
    /// INLINE
    Inline                                                                 = 1177,
    /// INLINE-CONDITIONAL
    InlineConditional                                                      = 1178,
    /// INOUT
    Inout                                                                  = 1179,
    /// INSTALL
    Install                                                                = 1180,
    /// INSTANCE-ID
    InstanceId                                                             = 1181,
    /// INSTRUCTION
    Instruction                                                            = 1182,
    /// INTER-PARTITION-INTRA-ECU
    InterPartitionIntraEcu                                                 = 1183,
    /// INTERFACE-MAPPING
    InterfaceMapping                                                       = 1184,
    /// INTERFACE-MAPPING-SET
    InterfaceMappingSet                                                    = 1185,
    /// INTERNAL-BEHAVIOR
    InternalBehavior                                                       = 1186,
    /// INTERNAL-TRIGGER-OCCURRED-EVENT
    InternalTriggerOccurredEvent                                           = 1187,
    /// INTERNAL-TRIGGERING-POINT
    InternalTriggeringPoint                                                = 1188,
    /// INTERPOLATION-ROUTINE-MAPPING-SET
    InterpolationRoutineMappingSet                                         = 1189,
    /// INTERRUPT
    Interrupt                                                              = 1190,
    /// INTERRUPT-CAT-1
    InterruptCat1                                                          = 1191,
    /// INTERRUPT-CAT-2
    InterruptCat2                                                          = 1192,
    /// INTRUSION-DETECTION-SECURITY-MANAGEMENT
    IntrusionDetectionSecurityManagement                                   = 1193,
    /// INVALID
    Invalid                                                                = 1194,
    /// IP-IAM-REMOTE-SUBJECT
    IpIamRemoteSubject                                                     = 1195,
    /// IP-SEC-CONFIG-PROPS
    IpSecConfigProps                                                       = 1196,
    /// IP-SEC-IAM-REMOTE-SUBJECT
    IpSecIamRemoteSubject                                                  = 1197,
    /// IP-SEC-RULE
    IpSecRule                                                              = 1198,
    /// IPSEC
    Ipsec                                                                  = 1199,
    /// IS
    Is                                                                     = 1200,
    /// IS-EQUAL
    IsEqual                                                                = 1201,
    /// IS-EXPIRED
    IsExpired                                                              = 1202,
    /// IS-FAILED
    IsFailed                                                               = 1203,
    /// IS-GREATER-OR-EQUAL
    IsGreaterOrEqual                                                       = 1204,
    /// IS-GREATER-THAN
    IsGreaterThan                                                          = 1205,
    /// IS-GREATER-THAN-OR-EQUAL
    IsGreaterThanOrEqual                                                   = 1206,
    /// IS-LESS-OR-EQUAL
    IsLessOrEqual                                                          = 1207,
    /// IS-LESS-THAN
    IsLessThan                                                             = 1208,
    /// IS-LESS-THAN-OR-EQUAL
    IsLessThanOrEqual                                                      = 1209,
    /// IS-NOT-EQUAL
    IsNotEqual                                                             = 1210,
    /// IS-NOT-RELEVANT
    IsNotRelevant                                                          = 1211,
    /// IS-OK
    IsOk                                                                   = 1212,
    /// IS-RELEVANT
    IsRelevant                                                             = 1213,
    /// IS-STOPPED
    IsStopped                                                              = 1214,
    /// ISO
    Iso                                                                    = 1215,
    /// ISO-11992--4
    Iso119924                                                              = 1216,
    /// ISO-14229--1
    Iso142291                                                              = 1217,
    /// ISO-15031--6
    Iso150316                                                              = 1218,
    /// ISO-6
    Iso6                                                                   = 1219,
    /// IT
    It                                                                     = 1220,
    /// ITALIC
    Italic                                                                 = 1221,
    /// IW
    Iw                                                                     = 1222,
    /// J-1939
    J1939                                                                  = 1223,
    /// J-1939-CLUSTER
    J1939Cluster                                                           = 1224,
    /// J-1939-CONTROLLER-APPLICATION
    J1939ControllerApplication                                             = 1225,
    /// J-1939-DCM
    J1939Dcm                                                               = 1226,
    /// J-1939-DCM-DM-19-SUPPORT
    J1939DcmDm19Support                                                    = 1227,
    /// J-1939-DCM-I-PDU
    J1939DcmIPdu                                                           = 1228,
    /// J-1939-NM-CLUSTER
    J1939NmCluster                                                         = 1229,
    /// J-1939-NM-NODE
    J1939NmNode                                                            = 1230,
    /// J-1939-REQUEST-MANAGER
    J1939RequestManager                                                    = 1231,
    /// J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS
    J1939RmIncomingRequestServiceNeeds                                     = 1232,
    /// J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS
    J1939RmOutgoingRequestServiceNeeds                                     = 1233,
    /// J-1939-SHARED-ADDRESS-CLUSTER
    J1939SharedAddressCluster                                              = 1234,
    /// J-1939-TP-CONFIG
    J1939TpConfig                                                          = 1235,
    /// J-1939-TP-NODE
    J1939TpNode                                                            = 1236,
    /// JA
    Ja                                                                     = 1237,
    /// JAVA
    Java                                                                   = 1238,
    /// JI
    Ji                                                                     = 1239,
    /// JPG
    Jpg                                                                    = 1240,
    /// JUSTIFY
    Justify                                                                = 1241,
    /// JW
    Jw                                                                     = 1242,
    /// KA
    Ka                                                                     = 1243,
    /// KEEP
    Keep                                                                   = 1244,
    /// KEEP-EXISTING
    KeepExisting                                                           = 1245,
    /// KEY-DERIVATION
    KeyDerivation                                                          = 1246,
    /// KEY-STORAGE
    KeyStorage                                                             = 1247,
    /// KEYWORD
    Keyword                                                                = 1248,
    /// KEYWORD-SET
    KeywordSet                                                             = 1249,
    /// KK
    Kk                                                                     = 1250,
    /// KL
    Kl                                                                     = 1251,
    /// KM
    Km                                                                     = 1252,
    /// KN
    Kn                                                                     = 1253,
    /// KO
    Ko                                                                     = 1254,
    /// KS
    Ks                                                                     = 1255,
    /// KU
    Ku                                                                     = 1256,
    /// KY
    Ky                                                                     = 1257,
    /// LA
    La                                                                     = 1258,
    /// LAND
    Land                                                                   = 1259,
    /// LAST-FAILED
    LastFailed                                                             = 1260,
    /// LAST-IS-BEST
    LastIsBest                                                             = 1261,
    /// LAST-MODE
    LastMode                                                               = 1262,
    /// LATENCY-TIMING-CONSTRAINT
    LatencyTimingConstraint                                                = 1263,
    /// LEAF-OF-TARGET-CONTAINER
    LeafOfTargetContainer                                                  = 1264,
    /// LEFT
    Left                                                                   = 1265,
    /// LEGACY
    Legacy                                                                 = 1266,
    /// LIFE-CYCLE-INFO-SET
    LifeCycleInfoSet                                                       = 1267,
    /// LIFE-CYCLE-STATE
    LifeCycleState                                                         = 1268,
    /// LIFE-CYCLE-STATE-DEFINITION-GROUP
    LifeCycleStateDefinitionGroup                                          = 1269,
    /// LIMIT-TO-PAGE
    LimitToPage                                                            = 1270,
    /// LIMIT-TO-TEXT
    LimitToText                                                            = 1271,
    /// LIN-CLUSTER
    LinCluster                                                             = 1272,
    /// LIN-COMMUNICATION-CONNECTOR
    LinCommunicationConnector                                              = 1273,
    /// LIN-COMMUNICATION-CONTROLLER
    LinCommunicationController                                             = 1274,
    /// LIN-EVENT-TRIGGERED-FRAME
    LinEventTriggeredFrame                                                 = 1275,
    /// LIN-FRAME
    LinFrame                                                               = 1276,
    /// LIN-FRAME-TRIGGERING
    LinFrameTriggering                                                     = 1277,
    /// LIN-MASTER
    LinMaster                                                              = 1278,
    /// LIN-NM-CLUSTER
    LinNmCluster                                                           = 1279,
    /// LIN-PHYSICAL-CHANNEL
    LinPhysicalChannel                                                     = 1280,
    /// LIN-SCHEDULE-TABLE
    LinScheduleTable                                                       = 1281,
    /// LIN-SLAVE
    LinSlave                                                               = 1282,
    /// LIN-SLAVE-CONFIG-IDENT
    LinSlaveConfigIdent                                                    = 1283,
    /// LIN-SPORADIC-FRAME
    LinSporadicFrame                                                       = 1284,
    /// LIN-TP-CONFIG
    LinTpConfig                                                            = 1285,
    /// LIN-TP-NODE
    LinTpNode                                                              = 1286,
    /// LIN-UNCONDITIONAL-FRAME
    LinUnconditionalFrame                                                  = 1287,
    /// LINK
    Link                                                                   = 1288,
    /// LINK-LOCAL
    LinkLocal                                                              = 1289,
    /// LINK-LOCAL--DOIP
    LinkLocalDoip                                                          = 1290,
    /// LINK-TIME
    LinkTime                                                               = 1291,
    /// LINKER
    Linker                                                                 = 1292,
    /// LISTEN
    Listen                                                                 = 1293,
    /// LN
    Ln                                                                     = 1294,
    /// LO
    Lo                                                                     = 1295,
    /// LOCAL
    Local                                                                  = 1296,
    /// LOCAL-SUPERVISION
    LocalSupervision                                                       = 1297,
    /// LOG-AND-TRACE-INSTANTIATION
    LogAndTraceInstantiation                                               = 1298,
    /// LOG-AND-TRACE-INTERFACE
    LogAndTraceInterface                                                   = 1299,
    /// LOG-AND-TRACE-MESSAGE-COLLECTION-SET
    LogAndTraceMessageCollectionSet                                        = 1300,
    /// LOGIC-ADDRESS
    LogicAddress                                                           = 1301,
    /// LOGICAL-AND
    LogicalAnd                                                             = 1302,
    /// LOGICAL-EXPRESSION
    LogicalExpression                                                      = 1303,
    /// LOGICAL-OR
    LogicalOr                                                              = 1304,
    /// LOGICAL-SUPERVISION
    LogicalSupervision                                                     = 1305,
    /// LONG-HEADER
    LongHeader                                                             = 1306,
    /// LOW
    Low                                                                    = 1307,
    /// LOWER-12-BIT
    Lower12Bit                                                             = 1308,
    /// LOWER-8-BIT
    Lower8Bit                                                              = 1309,
    /// LT
    Lt                                                                     = 1310,
    /// LT-AFFECTS-PB
    LtAffectsPb                                                            = 1311,
    /// LTS-13
    Lts13                                                                  = 1312,
    /// LV
    Lv                                                                     = 1313,
    /// MAC-MULTICAST-GROUP
    MacMulticastGroup                                                      = 1314,
    /// MACHINE
    Machine                                                                = 1315,
    /// MACHINE-DESIGN
    MachineDesign                                                          = 1316,
    /// MACHINE-MODE-REQUEST-PHM-ACTION-ITEM
    MachineModeRequestPhmActionItem                                        = 1317,
    /// MACHINE-TIMING
    MachineTiming                                                          = 1318,
    /// MACRO
    Macro                                                                  = 1319,
    /// MAINTENANCE-ONLY
    MaintenanceOnly                                                        = 1320,
    /// MALFUNCTION
    Malfunction                                                            = 1321,
    /// MANUFACTURING
    Manufacturing                                                          = 1322,
    /// MAPPING-SCOPE-CORE
    MappingScopeCore                                                       = 1323,
    /// MAPPING-SCOPE-ECU
    MappingScopeEcu                                                        = 1324,
    /// MAPPING-SCOPE-PARTITION
    MappingScopePartition                                                  = 1325,
    /// MASEKD-NEW-EQUALS-MASKED-OLD
    MasekdNewEqualsMaskedOld                                               = 1326,
    /// MASEKD-NEW-EQUALS-X
    MasekdNewEqualsX                                                       = 1327,
    /// MASKED-NEW-DIFFERS-MASKED-OLD
    MaskedNewDiffersMaskedOld                                              = 1328,
    /// MASKED-NEW-DIFFERS-X
    MaskedNewDiffersX                                                      = 1329,
    /// MASKED-NEW-EQUALS-MASKED-OLD
    MaskedNewEqualsMaskedOld                                               = 1330,
    /// MASKED-NEW-EQUALS-X
    MaskedNewEqualsX                                                       = 1331,
    /// MASTER
    Master                                                                 = 1332,
    /// MASTER-ECU
    MasterEcu                                                              = 1333,
    /// MAX
    Max                                                                    = 1334,
    /// MC-DATA-INSTANCE
    McDataInstance                                                         = 1335,
    /// MC-FUNCTION
    McFunction                                                             = 1336,
    /// MC-GROUP
    McGroup                                                                = 1337,
    /// MEASURED-EXECUTION-TIME
    MeasuredExecutionTime                                                  = 1338,
    /// MEASURED-HEAP-USAGE
    MeasuredHeapUsage                                                      = 1339,
    /// MEASURED-STACK-USAGE
    MeasuredStackUsage                                                     = 1340,
    /// MEASUREMENT-POINT
    MeasurementPoint                                                       = 1341,
    /// MEDIUM
    Medium                                                                 = 1342,
    /// MEMORY-SECTION
    MemorySection                                                          = 1343,
    /// METHOD-MAPPING
    MethodMapping                                                          = 1344,
    /// MG
    Mg                                                                     = 1345,
    /// MI
    Mi                                                                     = 1346,
    /// MIDDLE
    Middle                                                                 = 1347,
    /// MIN
    Min                                                                    = 1348,
    /// MINIMUM-MINOR-VERSION
    MinimumMinorVersion                                                    = 1349,
    /// MIXED
    Mixed                                                                  = 1350,
    /// MIXED-29-BIT
    Mixed29Bit                                                             = 1351,
    /// MK
    Mk                                                                     = 1352,
    /// ML
    Ml                                                                     = 1353,
    /// MN
    Mn                                                                     = 1354,
    /// MO
    Mo                                                                     = 1355,
    /// MODE-ACCESS-POINT-IDENT
    ModeAccessPointIdent                                                   = 1356,
    /// MODE-DECLARATION
    ModeDeclaration                                                        = 1357,
    /// MODE-DECLARATION-GROUP
    ModeDeclarationGroup                                                   = 1358,
    /// MODE-DECLARATION-GROUP-PROTOTYPE
    ModeDeclarationGroupPrototype                                          = 1359,
    /// MODE-DECLARATION-MAPPING
    ModeDeclarationMapping                                                 = 1360,
    /// MODE-DECLARATION-MAPPING-SET
    ModeDeclarationMappingSet                                              = 1361,
    /// MODE-DECLARATION-REQUESTED
    ModeDeclarationRequested                                               = 1362,
    /// MODE-DECLARATION-SWITCH-COMPLETED
    ModeDeclarationSwitchCompleted                                         = 1363,
    /// MODE-DECLARATION-SWITCH-INITIATED
    ModeDeclarationSwitchInitiated                                         = 1364,
    /// MODE-INTERFACE-MAPPING
    ModeInterfaceMapping                                                   = 1365,
    /// MODE-SWITCH-INTERFACE
    ModeSwitchInterface                                                    = 1366,
    /// MODE-SWITCH-POINT
    ModeSwitchPoint                                                        = 1367,
    /// MODE-SWITCHED-ACK-EVENT
    ModeSwitchedAckEvent                                                   = 1368,
    /// MODE-TRANSITION
    ModeTransition                                                         = 1369,
    /// MODELED
    Modeled                                                                = 1370,
    /// MONITOR-MODE
    MonitorMode                                                            = 1371,
    /// MONO
    Mono                                                                   = 1372,
    /// MONOTONOUS
    Monotonous                                                             = 1373,
    /// MOST-SIGNIFICANT-BYTE-FIRST
    MostSignificantByteFirst                                               = 1374,
    /// MOST-SIGNIFICANT-BYTE-LAST
    MostSignificantByteLast                                                = 1375,
    /// MR
    Mr                                                                     = 1376,
    /// MS
    Ms                                                                     = 1377,
    /// MT
    Mt                                                                     = 1378,
    /// MULTICORE-REENTRANT
    MulticoreReentrant                                                     = 1379,
    /// MULTILANGUAGE-REFERRABLE
    MultilanguageReferrable                                                = 1380,
    /// MULTIPLE
    Multiple                                                               = 1381,
    /// MULTIPLE-OCCURRENCES
    MultipleOccurrences                                                    = 1382,
    /// MULTIPLEXED-I-PDU
    MultiplexedIPdu                                                        = 1383,
    /// MY
    My                                                                     = 1384,
    /// N-PDU
    NPdu                                                                   = 1385,
    /// NA
    Na                                                                     = 1386,
    /// NAND
    Nand                                                                   = 1387,
    /// NE
    Ne                                                                     = 1388,
    /// NET
    Net                                                                    = 1389,
    /// NETWORK
    Network                                                                = 1390,
    /// NETWORK-CONFIGURATION
    NetworkConfiguration                                                   = 1391,
    /// NETWORK-ENDPOINT
    NetworkEndpoint                                                        = 1392,
    /// NETWORK-REPRESENTATION-FROM-COM-SPEC
    NetworkRepresentationFromComSpec                                       = 1393,
    /// NEVER
    Never                                                                  = 1394,
    /// NEW-IS-DIFFERENT
    NewIsDifferent                                                         = 1395,
    /// NEW-IS-EQUAL
    NewIsEqual                                                             = 1396,
    /// NEW-IS-GREATER
    NewIsGreater                                                           = 1397,
    /// NEW-IS-GREATER-OR-EQUAL
    NewIsGreaterOrEqual                                                    = 1398,
    /// NEW-IS-LESS
    NewIsLess                                                              = 1399,
    /// NEW-IS-LESS-OR-EQUAL
    NewIsLessOrEqual                                                       = 1400,
    /// NEW-IS-OUTSIDE
    NewIsOutside                                                           = 1401,
    /// NEW-IS-WITHIN
    NewIsWithin                                                            = 1402,
    /// NEWLINE
    Newline                                                                = 1403,
    /// NEWLINE-IF-NECESSARY
    NewlineIfNecessary                                                     = 1404,
    /// NFOLD
    Nfold                                                                  = 1405,
    /// NL
    Nl                                                                     = 1406,
    /// NM-CLUSTER
    NmCluster                                                              = 1407,
    /// NM-CONFIG
    NmConfig                                                               = 1408,
    /// NM-ECU
    NmEcu                                                                  = 1409,
    /// NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING
    NmHandleToFunctionGroupStateMapping                                    = 1410,
    /// NM-INSTANTIATION
    NmInstantiation                                                        = 1411,
    /// NM-NETWORK-HANDLE
    NmNetworkHandle                                                        = 1412,
    /// NM-NODE
    NmNode                                                                 = 1413,
    /// NM-PDU
    NmPdu                                                                  = 1414,
    /// NO
    No                                                                     = 1415,
    /// NO-ACK
    NoAck                                                                  = 1416,
    /// NO-AFFECT
    NoAffect                                                               = 1417,
    /// NO-BOOT
    NoBoot                                                                 = 1418,
    /// NO-BREAK
    NoBreak                                                                = 1419,
    /// NO-DEFAULT
    NoDefault                                                              = 1420,
    /// NO-FLOAT
    NoFloat                                                                = 1421,
    /// NO-HEADER
    NoHeader                                                               = 1422,
    /// NO-KEEP
    NoKeep                                                                 = 1423,
    /// NO-MONOTONY
    NoMonotony                                                             = 1424,
    /// NO-NEWLINE
    NoNewline                                                              = 1425,
    /// NO-OBD-SUPPORT
    NoObdSupport                                                           = 1426,
    /// NO-PGWIDE
    NoPgwide                                                               = 1427,
    /// NO-PROTECTION
    NoProtection                                                           = 1428,
    /// NO-RETURN-VALUE-PROVIDED
    NoReturnValueProvided                                                  = 1429,
    /// NO-SEVERITY
    NoSeverity                                                             = 1430,
    /// NO-SHOW-ALIAS-NAME
    NoShowAliasName                                                        = 1431,
    /// NO-SHOW-CATEGORY
    NoShowCategory                                                         = 1432,
    /// NO-SHOW-CONTENT
    NoShowContent                                                          = 1433,
    /// NO-SHOW-LONG-NAME
    NoShowLongName                                                         = 1434,
    /// NO-SHOW-NUMBER
    NoShowNumber                                                           = 1435,
    /// NO-SHOW-PAGE
    NoShowPage                                                             = 1436,
    /// NO-SHOW-SEE
    NoShowSee                                                              = 1437,
    /// NO-SHOW-SHORT-NAME
    NoShowShortName                                                        = 1438,
    /// NO-SHOW-TYPE
    NoShowType                                                             = 1439,
    /// NO-SLOPPY
    NoSloppy                                                               = 1440,
    /// NO-STATUS-BYTE-CHANGE
    NoStatusByteChange                                                     = 1441,
    /// NO-STORE-EVENT
    NoStoreEvent                                                           = 1442,
    /// NO-SUPERVISION
    NoSupervision                                                          = 1443,
    /// NO-SUPPORT
    NoSupport                                                              = 1444,
    /// NO-TRANSFORMER-ERROR-HANDLING
    NoTransformerErrorHandling                                             = 1445,
    /// NO-TRANSFORMER-STATUS-FORWARDING
    NoTransformerStatusForwarding                                          = 1446,
    /// NO-TRUSTED-PLATFORM-SUPPORT
    NoTrustedPlatformSupport                                               = 1447,
    /// NODE
    Node                                                                   = 1448,
    /// NOHREF
    Nohref                                                                 = 1449,
    /// NON-EMMISSION-RELATED-DTC
    NonEmmissionRelatedDtc                                                 = 1450,
    /// NON-OS-MODULE-INSTANTIATION
    NonOsModuleInstantiation                                               = 1451,
    /// NON-REENTRANT
    NonReentrant                                                           = 1452,
    /// NON-VOLATILE
    NonVolatile                                                            = 1453,
    /// NON-VOLATILE-RAM-MANAGER
    NonVolatileRamManager                                                  = 1454,
    /// NONE
    None                                                                   = 1455,
    /// NORMALFIXED
    Normalfixed                                                            = 1456,
    /// NOT
    Not                                                                    = 1457,
    /// NOT-ACCESSIBLE
    NotAccessible                                                          = 1458,
    /// NOT-AVAILABLE
    NotAvailable                                                           = 1459,
    /// NOT-DEFINED
    NotDefined                                                             = 1460,
    /// NOT-EQUAL
    NotEqual                                                               = 1461,
    /// NOT-SENT
    NotSent                                                                = 1462,
    /// NOT-TESTED
    NotTested                                                              = 1463,
    /// NOT-VALID
    NotValid                                                               = 1464,
    /// NOTHING
    Nothing                                                                = 1465,
    /// NOTIFICATION
    Notification                                                           = 1466,
    /// NTP--RFC-958
    NtpRfc958                                                              = 1467,
    /// NUMBER
    Number                                                                 = 1468,
    /// NV-BLOCK-DESCRIPTOR
    NvBlockDescriptor                                                      = 1469,
    /// NV-BLOCK-NEEDS
    NvBlockNeeds                                                           = 1470,
    /// NV-BLOCK-SW-COMPONENT-TYPE
    NvBlockSwComponentType                                                 = 1471,
    /// NV-DATA-INTERFACE
    NvDataInterface                                                        = 1472,
    /// NV-RAM-MANAGER
    NvRamManager                                                           = 1473,
    /// OBD
    Obd                                                                    = 1474,
    /// OBD-CONTROL-SERVICE-NEEDS
    ObdControlServiceNeeds                                                 = 1475,
    /// OBD-DCY
    ObdDcy                                                                 = 1476,
    /// OBD-DRIVING-CYCLE
    ObdDrivingCycle                                                        = 1477,
    /// OBD-INFO-SERVICE-NEEDS
    ObdInfoServiceNeeds                                                    = 1478,
    /// OBD-MONITOR-SERVICE-NEEDS
    ObdMonitorServiceNeeds                                                 = 1479,
    /// OBD-PID-SERVICE-NEEDS
    ObdPidServiceNeeds                                                     = 1480,
    /// OBD-RATIO-DENOMINATOR-NEEDS
    ObdRatioDenominatorNeeds                                               = 1481,
    /// OBD-RATIO-SERVICE-NEEDS
    ObdRatioServiceNeeds                                                   = 1482,
    /// OBSERVER
    Observer                                                               = 1483,
    /// OBSERVER-BASED
    ObserverBased                                                          = 1484,
    /// OC
    Oc                                                                     = 1485,
    /// OCCURENCE
    Occurence                                                              = 1486,
    /// OEM-BOOT
    OemBoot                                                                = 1487,
    /// OEM-BOOT-RESP-APP
    OemBootRespApp                                                         = 1488,
    /// OFF
    Off                                                                    = 1489,
    /// OFFSET
    Offset                                                                 = 1490,
    /// OFFSET-TIMING-CONSTRAINT
    OffsetTimingConstraint                                                 = 1491,
    /// OM
    Om                                                                     = 1492,
    /// ON-CHANGE-OF-DATA-IDENTIFIER
    OnChangeOfDataIdentifier                                               = 1493,
    /// ON-COMPARISON-OF-VALUES
    OnComparisonOfValues                                                   = 1494,
    /// ON-DTC-STATUS-CHANGE
    OnDtcStatusChange                                                      = 1495,
    /// ON-ENTRY
    OnEntry                                                                = 1496,
    /// ON-EXIT
    OnExit                                                                 = 1497,
    /// ON-TRANSITION
    OnTransition                                                           = 1498,
    /// ONE-EVERY-N
    OneEveryN                                                              = 1499,
    /// ONLY-THIS-CYCLE-AND-READINESS
    OnlyThisCycleAndReadiness                                              = 1500,
    /// OPAQUE
    Opaque                                                                 = 1501,
    /// OPEN
    Open                                                                   = 1502,
    /// OPERATING-SYSTEM
    OperatingSystem                                                        = 1503,
    /// OPERATION-CALL-RECEIVED
    OperationCallReceived                                                  = 1504,
    /// OPERATION-CALL-RESPONSE-RECEIVED
    OperationCallResponseReceived                                          = 1505,
    /// OPERATION-CALL-RESPONSE-SENT
    OperationCallResponseSent                                              = 1506,
    /// OPERATION-CALLED
    OperationCalled                                                        = 1507,
    /// OPERATION-INVOKED-EVENT
    OperationInvokedEvent                                                  = 1508,
    /// OPTIONS
    Options                                                                = 1509,
    /// OR
    Or                                                                     = 1510,
    /// ORDINARY-EOC
    OrdinaryEoc                                                            = 1511,
    /// OS-MODULE-INSTANTIATION
    OsModuleInstantiation                                                  = 1512,
    /// OS-TASK-EXECUTION-EVENT
    OsTaskExecutionEvent                                                   = 1513,
    /// OS-TASK-PROXY
    OsTaskProxy                                                            = 1514,
    /// OTHER
    Other                                                                  = 1515,
    /// OUT
    Out                                                                    = 1516,
    /// OVERRIDE
    Override                                                               = 1517,
    /// OVERWRITE
    Overwrite                                                              = 1518,
    /// P-PORT-PROTOTYPE
    PPortPrototype                                                         = 1519,
    /// PA
    Pa                                                                     = 1520,
    /// PACKAGEABLE-ELEMENT
    PackageableElement                                                     = 1521,
    /// PARAMETER-ACCESS
    ParameterAccess                                                        = 1522,
    /// PARAMETER-DATA-PROTOTYPE
    ParameterDataPrototype                                                 = 1523,
    /// PARAMETER-INTERFACE
    ParameterInterface                                                     = 1524,
    /// PARAMETER-SW-COMPONENT-TYPE
    ParameterSwComponentType                                               = 1525,
    /// PARTIAL-NETWORK
    PartialNetwork                                                         = 1526,
    /// PARTITION
    Partition                                                              = 1527,
    /// PASS-THROUGH-SW-CONNECTOR
    PassThroughSwConnector                                                 = 1528,
    /// PASSIVE
    Passive                                                                = 1529,
    /// PASSTHROUGH
    Passthrough                                                            = 1530,
    /// PAYLOAD-AS-ARRAY
    PayloadAsArray                                                         = 1531,
    /// PAYLOAD-AS-POINTER-TO-ARRAY
    PayloadAsPointerToArray                                                = 1532,
    /// PC-AFFECTS-LT
    PcAffectsLt                                                            = 1533,
    /// PC-AFFECTS-LT-AND-PB
    PcAffectsLtAndPb                                                       = 1534,
    /// PC-AFFECTS-PB
    PcAffectsPb                                                            = 1535,
    /// PDF
    Pdf                                                                    = 1536,
    /// PDU
    Pdu                                                                    = 1537,
    /// PDU-ACTIVATION-ROUTING-GROUP
    PduActivationRoutingGroup                                              = 1538,
    /// PDU-R
    PduR                                                                   = 1539,
    /// PDU-TO-FRAME-MAPPING
    PduToFrameMapping                                                      = 1540,
    /// PDU-TRIGGERING
    PduTriggering                                                          = 1541,
    /// PDUR-I-PDU-GROUP
    PdurIPduGroup                                                          = 1542,
    /// PENDING
    Pending                                                                = 1543,
    /// PER-EXECUTABLE
    PerExecutable                                                          = 1544,
    /// PER-INSTANCE-MEMORY
    PerInstanceMemory                                                      = 1545,
    /// PERIODIC-EVENT-TRIGGERING
    PeriodicEventTriggering                                                = 1546,
    /// PERIODIC-RATE-FAST
    PeriodicRateFast                                                       = 1547,
    /// PERIODIC-RATE-MEDIUM
    PeriodicRateMedium                                                     = 1548,
    /// PERIODIC-RATE-SLOW
    PeriodicRateSlow                                                       = 1549,
    /// PERSISTENCY-DATA-ELEMENT
    PersistencyDataElement                                                 = 1550,
    /// PERSISTENCY-DEPLOYMENT
    PersistencyDeployment                                                  = 1551,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT
    PersistencyDeploymentElement                                           = 1552,
    /// PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentElementToCryptoKeySlotMapping                     = 1553,
    /// PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING
    PersistencyDeploymentToCryptoKeySlotMapping                            = 1554,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING
    PersistencyDeploymentToDltLogChannelMapping                            = 1555,
    /// PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING
    PersistencyDeploymentToDltLogSinkMapping                               = 1556,
    /// PERSISTENCY-FILE
    PersistencyFile                                                        = 1557,
    /// PERSISTENCY-FILE-ARRAY
    PersistencyFileArray                                                   = 1558,
    /// PERSISTENCY-FILE-ELEMENT
    PersistencyFileElement                                                 = 1559,
    /// PERSISTENCY-FILE-PROXY
    PersistencyFileProxy                                                   = 1560,
    /// PERSISTENCY-FILE-PROXY-INTERFACE
    PersistencyFileProxyInterface                                          = 1561,
    /// PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING
    PersistencyFileProxyToFileMapping                                      = 1562,
    /// PERSISTENCY-FILE-STORAGE
    PersistencyFileStorage                                                 = 1563,
    /// PERSISTENCY-FILE-STORAGE-INTERFACE
    PersistencyFileStorageInterface                                        = 1564,
    /// PERSISTENCY-INTERFACE
    PersistencyInterface                                                   = 1565,
    /// PERSISTENCY-INTERFACE-ELEMENT
    PersistencyInterfaceElement                                            = 1566,
    /// PERSISTENCY-KEY-VALUE-DATABASE
    PersistencyKeyValueDatabase                                            = 1567,
    /// PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE
    PersistencyKeyValueDatabaseInterface                                   = 1568,
    /// PERSISTENCY-KEY-VALUE-PAIR
    PersistencyKeyValuePair                                                = 1569,
    /// PERSISTENCY-KEY-VALUE-STORAGE
    PersistencyKeyValueStorage                                             = 1570,
    /// PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE
    PersistencyKeyValueStorageInterface                                    = 1571,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING
    PersistencyPortPrototypeToDeploymentMapping                            = 1572,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING
    PersistencyPortPrototypeToFileArrayMapping                             = 1573,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING
    PersistencyPortPrototypeToFileStorageMapping                           = 1574,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING
    PersistencyPortPrototypeToKeyValueDatabaseMapping                      = 1575,
    /// PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING
    PersistencyPortPrototypeToKeyValueStorageMapping                       = 1576,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE
    PersistencyRedundancyHandlingScopeDatabase                             = 1577,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT
    PersistencyRedundancyHandlingScopeElement                              = 1578,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE
    PersistencyRedundancyHandlingScopeFile                                 = 1579,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY
    PersistencyRedundancyHandlingScopeKey                                  = 1580,
    /// PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE
    PersistencyRedundancyHandlingScopeStorage                              = 1581,
    /// PGWIDE
    Pgwide                                                                 = 1582,
    /// PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE
    PhmAbstractRecoveryNotificationInterface                               = 1583,
    /// PHM-ACTION
    PhmAction                                                              = 1584,
    /// PHM-ACTION-ITEM
    PhmActionItem                                                          = 1585,
    /// PHM-ACTION-LIST
    PhmActionList                                                          = 1586,
    /// PHM-ARBITRATION
    PhmArbitration                                                         = 1587,
    /// PHM-CHECKPOINT
    PhmCheckpoint                                                          = 1588,
    /// PHM-CONTRIBUTION-TO-MACHINE-MAPPING
    PhmContributionToMachineMapping                                        = 1589,
    /// PHM-HEALTH-CHANNEL-INTERFACE
    PhmHealthChannelInterface                                              = 1590,
    /// PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE
    PhmHealthChannelRecoveryNotificationInterface                          = 1591,
    /// PHM-HEALTH-CHANNEL-STATUS
    PhmHealthChannelStatus                                                 = 1592,
    /// PHM-LOGICAL-EXPRESSION
    PhmLogicalExpression                                                   = 1593,
    /// PHM-RECOVERY-ACTION-INTERFACE
    PhmRecoveryActionInterface                                             = 1594,
    /// PHM-RULE
    PhmRule                                                                = 1595,
    /// PHM-SUPERVISED-ENTITY-INTERFACE
    PhmSupervisedEntityInterface                                           = 1596,
    /// PHM-SUPERVISION
    PhmSupervision                                                         = 1597,
    /// PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE
    PhmSupervisionRecoveryNotificationInterface                            = 1598,
    /// PHYSICAL
    Physical                                                               = 1599,
    /// PHYSICAL-ADDRESS
    PhysicalAddress                                                        = 1600,
    /// PHYSICAL-CAN-FD
    PhysicalCanFd                                                          = 1601,
    /// PHYSICAL-CHANNEL
    PhysicalChannel                                                        = 1602,
    /// PHYSICAL-DIMENSION
    PhysicalDimension                                                      = 1603,
    /// PHYSICAL-DIMENSION-MAPPING-SET
    PhysicalDimensionMappingSet                                            = 1604,
    /// PL
    Pl                                                                     = 1605,
    /// PLAIN
    Plain                                                                  = 1606,
    /// PLATFORM-ACTION-ITEM
    PlatformActionItem                                                     = 1607,
    /// PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION
    PlatformHealthManagementContribution                                   = 1608,
    /// PLATFORM-HEALTH-MANAGEMENT-INTERFACE
    PlatformHealthManagementInterface                                      = 1609,
    /// PLATFORM-MODULE-ENDPOINT-CONFIGURATION
    PlatformModuleEndpointConfiguration                                    = 1610,
    /// PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION
    PlatformModuleEthernetEndpointConfiguration                            = 1611,
    /// PLATFORM-PHM-ACTION-ITEM
    PlatformPhmActionItem                                                  = 1612,
    /// PNC-MAPPING-IDENT
    PncMappingIdent                                                        = 1613,
    /// PNG
    Png                                                                    = 1614,
    /// POLY
    Poly                                                                   = 1615,
    /// PORT
    Port                                                                   = 1616,
    /// PORT-BLUEPRINT
    PortBlueprint                                                          = 1617,
    /// PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING
    PortElementToCommunicationResourceMapping                              = 1618,
    /// PORT-GROUP
    PortGroup                                                              = 1619,
    /// PORT-INTERFACE
    PortInterface                                                          = 1620,
    /// PORT-INTERFACE-DEFINITION
    PortInterfaceDefinition                                                = 1621,
    /// PORT-INTERFACE-MAPPING
    PortInterfaceMapping                                                   = 1622,
    /// PORT-INTERFACE-MAPPING-SET
    PortInterfaceMappingSet                                                = 1623,
    /// PORT-INTERFACE-TO-DATA-TYPE-MAPPING
    PortInterfaceToDataTypeMapping                                         = 1624,
    /// PORT-PROTOTYPE
    PortPrototype                                                          = 1625,
    /// PORT-PROTOTYPE-BLUEPRINT
    PortPrototypeBlueprint                                                 = 1626,
    /// POSSIBLE-ERROR-REACTION
    PossibleErrorReaction                                                  = 1627,
    /// POST
    Post                                                                   = 1628,
    /// POST-BUILD
    PostBuild                                                              = 1629,
    /// POST-BUILD-VARIANT-CRITERION
    PostBuildVariantCriterion                                              = 1630,
    /// POST-BUILD-VARIANT-CRITERION-VALUE-SET
    PostBuildVariantCriterionValueSet                                      = 1631,
    /// POWER
    Power                                                                  = 1632,
    /// POWER-WINDOW-TIME
    PowerWindowTime                                                        = 1633,
    /// PR-PORT-PROTOTYPE
    PrPortPrototype                                                        = 1634,
    /// PRE--R-4--2
    PreR42                                                                 = 1635,
    /// PRE-COMPILE
    PreCompile                                                             = 1636,
    /// PRE-COMPILE-TIME
    PreCompileTime                                                         = 1637,
    /// PRECONFIGURED-CONFIGURATION
    PreconfiguredConfiguration                                             = 1638,
    /// PREDEFINED-VARIANT
    PredefinedVariant                                                      = 1639,
    /// PRESENTATION-CONTINUOUS
    PresentationContinuous                                                 = 1640,
    /// PRESENTATION-DISCRETE
    PresentationDiscrete                                                   = 1641,
    /// PRESHARED-KEY-IDENTITY
    PresharedKeyIdentity                                                   = 1642,
    /// PRIMARY-ECU
    PrimaryEcu                                                             = 1643,
    /// PRIMITIVE
    Primitive                                                              = 1644,
    /// PRIMITIVE-ATTRIBUTE-TAILORING
    PrimitiveAttributeTailoring                                            = 1645,
    /// PRIO-OCC
    PrioOcc                                                                = 1646,
    /// PRIVATE-KEY
    PrivateKey                                                             = 1647,
    /// PROCESS
    Process                                                                = 1648,
    /// PROCESS-DESIGN
    ProcessDesign                                                          = 1649,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING
    ProcessDesignToMachineDesignMapping                                    = 1650,
    /// PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET
    ProcessDesignToMachineDesignMappingSet                                 = 1651,
    /// PROCESS-EXECUTION-ERROR
    ProcessExecutionError                                                  = 1652,
    /// PROCESS-IS-NOT-SELF-TERMINATING
    ProcessIsNotSelfTerminating                                            = 1653,
    /// PROCESS-IS-SELF-TERMINATING
    ProcessIsSelfTerminating                                               = 1654,
    /// PROCESS-PHM-ACTION-ITEM
    ProcessPhmActionItem                                                   = 1655,
    /// PROCESS-TO-MACHINE-MAPPING
    ProcessToMachineMapping                                                = 1656,
    /// PROCESS-TO-MACHINE-MAPPING-SET
    ProcessToMachineMappingSet                                             = 1657,
    /// PROCESSING-STYLE-ASYNCHRONOUS
    ProcessingStyleAsynchronous                                            = 1658,
    /// PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR
    ProcessingStyleAsynchronousWithError                                   = 1659,
    /// PROCESSING-STYLE-SYNCHRONOUS
    ProcessingStyleSynchronous                                             = 1660,
    /// PROCESSOR
    Processor                                                              = 1661,
    /// PROCESSOR-CORE
    ProcessorCore                                                          = 1662,
    /// PRODUCER
    Producer                                                               = 1663,
    /// PROTECT-LAMP
    ProtectLamp                                                            = 1664,
    /// PROTECTED
    Protected                                                              = 1665,
    /// PROVIDED-AP-SERVICE-INSTANCE
    ProvidedApServiceInstance                                              = 1666,
    /// PROVIDED-DDS-SERVICE-INSTANCE
    ProvidedDdsServiceInstance                                             = 1667,
    /// PROVIDED-SERVICE-INSTANCE
    ProvidedServiceInstance                                                = 1668,
    /// PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING
    ProvidedServiceInstanceToSwClusterDesignPPortPrototypeMapping          = 1669,
    /// PROVIDED-SOMEIP-SERVICE-INSTANCE
    ProvidedSomeipServiceInstance                                          = 1670,
    /// PROVIDED-USER-DEFINED-SERVICE-INSTANCE
    ProvidedUserDefinedServiceInstance                                     = 1671,
    /// PS
    Ps                                                                     = 1672,
    /// PSK
    Psk                                                                    = 1673,
    /// PSK-IDENTITY-TO-KEY-SLOT-MAPPING
    PskIdentityToKeySlotMapping                                            = 1674,
    /// PT
    Pt                                                                     = 1675,
    /// PTP--IEEE-1588--2002
    PtpIeee15882002                                                        = 1676,
    /// PTP--IEEE-1588--2008
    PtpIeee15882008                                                        = 1677,
    /// PUBLIC-KEY
    PublicKey                                                              = 1678,
    /// PUBLISHED-INFORMATION
    PublishedInformation                                                   = 1679,
    /// PURE-LOCAL-TIME-BASE
    PureLocalTimeBase                                                      = 1680,
    /// PUT
    Put                                                                    = 1681,
    /// QU
    Qu                                                                     = 1682,
    /// QUEUED
    Queued                                                                 = 1683,
    /// R-4--2
    R42                                                                    = 1684,
    /// R-PORT-PROTOTYPE
    RPortPrototype                                                         = 1685,
    /// RAPID-PROTOTYPING-SCENARIO
    RapidPrototypingScenario                                               = 1686,
    /// RAW
    Raw                                                                    = 1687,
    /// RAW-DATA
    RawData                                                                = 1688,
    /// RAW-DATA-STREAM-CLIENT-INTERFACE
    RawDataStreamClientInterface                                           = 1689,
    /// RAW-DATA-STREAM-DEPLOYMENT
    RawDataStreamDeployment                                                = 1690,
    /// RAW-DATA-STREAM-GRANT
    RawDataStreamGrant                                                     = 1691,
    /// RAW-DATA-STREAM-GRANT-DESIGN
    RawDataStreamGrantDesign                                               = 1692,
    /// RAW-DATA-STREAM-INTERFACE
    RawDataStreamInterface                                                 = 1693,
    /// RAW-DATA-STREAM-MAPPING
    RawDataStreamMapping                                                   = 1694,
    /// RAW-DATA-STREAM-METHOD-DEPLOYMENT
    RawDataStreamMethodDeployment                                          = 1695,
    /// RAW-DATA-STREAM-SERVER-INTERFACE
    RawDataStreamServerInterface                                           = 1696,
    /// REACTION
    Reaction                                                               = 1697,
    /// READ-ONLY
    ReadOnly                                                               = 1698,
    /// READ-WRITE
    ReadWrite                                                              = 1699,
    /// REBOOT
    Reboot                                                                 = 1700,
    /// RECOMMENDED-CONFIGURATION
    RecommendedConfiguration                                               = 1701,
    /// RECORD-VALUE-FIELD
    RecordValueField                                                       = 1702,
    /// RECOVERY-NOTIFICATION
    RecoveryNotification                                                   = 1703,
    /// RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING
    RecoveryNotificationToPPortPrototypeMapping                            = 1704,
    /// RECOVERY-VIA-APPLICATION-ACTION
    RecoveryViaApplicationAction                                           = 1705,
    /// RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING
    RecoveryViaApplicationActionToClientServerOperationMapping             = 1706,
    /// RECT
    Rect                                                                   = 1707,
    /// RED-STOP-LAMP
    RedStopLamp                                                            = 1708,
    /// REDUNDANT
    Redundant                                                              = 1709,
    /// REDUNDANT-PER-ELEMENT
    RedundantPerElement                                                    = 1710,
    /// REDUNDANT-PER-KEY
    RedundantPerKey                                                        = 1711,
    /// REF-ALL
    RefAll                                                                 = 1712,
    /// REF-NON-STANDARD
    RefNonStandard                                                         = 1713,
    /// REF-NONE
    RefNone                                                                = 1714,
    /// REFERENCE-TAILORING
    ReferenceTailoring                                                     = 1715,
    /// REFERRABLE
    Referrable                                                             = 1716,
    /// REGULAR
    Regular                                                                = 1717,
    /// REJECT
    Reject                                                                 = 1718,
    /// REMOVE
    Remove                                                                 = 1719,
    /// REPETITIVE-EOC
    RepetitiveEoc                                                          = 1720,
    /// REPLACE
    Replace                                                                = 1721,
    /// REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE
    ReplaceByTimeoutSubstitutionValue                                      = 1722,
    /// REPORT
    Report                                                                 = 1723,
    /// REPORT-AFTER-INIT
    ReportAfterInit                                                        = 1724,
    /// REPORT-BEFORE-INIT
    ReportBeforeInit                                                       = 1725,
    /// REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE
    ReportDtcRecordInformationOnDtcStatusChange                            = 1726,
    /// REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE
    ReportMostRecentDtcOnStatusChange                                      = 1727,
    /// REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST
    ReportingInChronlogicalOrderOldestFirst                                = 1728,
    /// REPORTS-EXECUTION-STATE
    ReportsExecutionState                                                  = 1729,
    /// REQUEST
    Request                                                                = 1730,
    /// REQUEST-CALLBACK-TYPE-MANUFACTURER
    RequestCallbackTypeManufacturer                                        = 1731,
    /// REQUEST-CALLBACK-TYPE-SUPPLIER
    RequestCallbackTypeSupplier                                            = 1732,
    /// REQUEST-NO-RETURN
    RequestNoReturn                                                        = 1733,
    /// REQUIRED-AP-SERVICE-INSTANCE
    RequiredApServiceInstance                                              = 1734,
    /// REQUIRED-DDS-SERVICE-INSTANCE
    RequiredDdsServiceInstance                                             = 1735,
    /// REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING
    RequiredServiceInstanceToSwClusterDesignRPortPrototypeMapping          = 1736,
    /// REQUIRED-SOMEIP-SERVICE-INSTANCE
    RequiredSomeipServiceInstance                                          = 1737,
    /// REQUIRED-USER-DEFINED-SERVICE-INSTANCE
    RequiredUserDefinedServiceInstance                                     = 1738,
    /// REQUIRES-CALLBACK-EXECUTION
    RequiresCallbackExecution                                              = 1739,
    /// RES-AXIS
    ResAxis                                                                = 1740,
    /// RESET-ECU
    ResetEcu                                                               = 1741,
    /// RESET-MACHINE
    ResetMachine                                                           = 1742,
    /// RESET-MCU
    ResetMcu                                                               = 1743,
    /// RESET-VM
    ResetVm                                                                = 1744,
    /// RESOURCE-CONSUMPTION
    ResourceConsumption                                                    = 1745,
    /// RESOURCE-GROUP
    ResourceGroup                                                          = 1746,
    /// RESPOND-AFTER-RESET
    RespondAfterReset                                                      = 1747,
    /// RESPOND-BEFORE-RESET
    RespondBeforeReset                                                     = 1748,
    /// RESPONSE
    Response                                                               = 1749,
    /// RESPONSE-SYNCHRONIZATION
    ResponseSynchronization                                                = 1750,
    /// REST-ABSTRACT-ENDPOINT
    RestAbstractEndpoint                                                   = 1751,
    /// REST-ABSTRACT-NUMERICAL-PROPERTY-DEF
    RestAbstractNumericalPropertyDef                                       = 1752,
    /// REST-ABSTRACT-PROPERTY-DEF
    RestAbstractPropertyDef                                                = 1753,
    /// REST-ARRAY-PROPERTY-DEF
    RestArrayPropertyDef                                                   = 1754,
    /// REST-BOOLEAN-PROPERTY-DEF
    RestBooleanPropertyDef                                                 = 1755,
    /// REST-ELEMENT-DEF
    RestElementDef                                                         = 1756,
    /// REST-ENDPOINT-DELETE
    RestEndpointDelete                                                     = 1757,
    /// REST-ENDPOINT-GET
    RestEndpointGet                                                        = 1758,
    /// REST-ENDPOINT-POST
    RestEndpointPost                                                       = 1759,
    /// REST-ENDPOINT-PUT
    RestEndpointPut                                                        = 1760,
    /// REST-HTTP-PORT-PROTOTYPE-MAPPING
    RestHttpPortPrototypeMapping                                           = 1761,
    /// REST-INTEGER-PROPERTY-DEF
    RestIntegerPropertyDef                                                 = 1762,
    /// REST-NUMBER-PROPERTY-DEF
    RestNumberPropertyDef                                                  = 1763,
    /// REST-OBJECT-REF
    RestObjectRef                                                          = 1764,
    /// REST-PRIMITIVE-PROPERTY-DEF
    RestPrimitivePropertyDef                                               = 1765,
    /// REST-RESOURCE-DEF
    RestResourceDef                                                        = 1766,
    /// REST-SERVICE-INTERFACE
    RestServiceInterface                                                   = 1767,
    /// REST-STRING-PROPERTY-DEF
    RestStringPropertyDef                                                  = 1768,
    /// RESTART
    Restart                                                                = 1769,
    /// RESTART-APPLICATION
    RestartApplication                                                     = 1770,
    /// RES_AXIS
    Resaxis                                                                = 1771,
    /// RETURN-ON-EVENT-CLEARED
    ReturnOnEventCleared                                                   = 1772,
    /// RETURN-ON-EVENT-STOPPED
    ReturnOnEventStopped                                                   = 1773,
    /// RETURN-VALUE-PROVIDED
    ReturnValueProvided                                                    = 1774,
    /// RIGHT
    Right                                                                  = 1775,
    /// RM
    Rm                                                                     = 1776,
    /// RN
    Rn                                                                     = 1777,
    /// RO
    Ro                                                                     = 1778,
    /// ROLL-BACK
    RollBack                                                               = 1779,
    /// ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE
    RootSwClusterDesignComponentPrototype                                  = 1780,
    /// ROOT-SW-COMPONENT-PROTOTYPE
    RootSwComponentPrototype                                               = 1781,
    /// ROOT-SW-COMPOSITION-PROTOTYPE
    RootSwCompositionPrototype                                             = 1782,
    /// ROTATE-180
    Rotate180                                                              = 1783,
    /// ROTATE-180-LIMIT-TO-TEXT
    Rotate180LimitToText                                                   = 1784,
    /// ROTATE-90-CCW
    Rotate90Ccw                                                            = 1785,
    /// ROTATE-90-CCW-FIT-TO-TEXT
    Rotate90CcwFitToText                                                   = 1786,
    /// ROTATE-90-CCW-LIMIT-TO-TEXT
    Rotate90CcwLimitToText                                                 = 1787,
    /// ROTATE-90-CW
    Rotate90Cw                                                             = 1788,
    /// ROTATE-90-CW-FIT-TO-TEXT
    Rotate90CwFitToText                                                    = 1789,
    /// ROTATE-90-CW-LIMIT-TO-TEXT
    Rotate90CwLimitToText                                                  = 1790,
    /// ROUGH-ESTIMATE-HEAP-USAGE
    RoughEstimateHeapUsage                                                 = 1791,
    /// ROUGH-ESTIMATE-OF-EXECUTION-TIME
    RoughEstimateOfExecutionTime                                           = 1792,
    /// ROUGH-ESTIMATE-STACK-USAGE
    RoughEstimateStackUsage                                                = 1793,
    /// ROUTER
    Router                                                                 = 1794,
    /// ROUTER-ADVERTISEMENT
    RouterAdvertisement                                                    = 1795,
    /// RPT-COMPONENT
    RptComponent                                                           = 1796,
    /// RPT-CONTAINER
    RptContainer                                                           = 1797,
    /// RPT-ENABLER-RAM
    RptEnablerRam                                                          = 1798,
    /// RPT-ENABLER-RAM-AND-ROM
    RptEnablerRamAndRom                                                    = 1799,
    /// RPT-ENABLER-ROM
    RptEnablerRom                                                          = 1800,
    /// RPT-EXECUTABLE-ENTITY
    RptExecutableEntity                                                    = 1801,
    /// RPT-EXECUTABLE-ENTITY-EVENT
    RptExecutableEntityEvent                                               = 1802,
    /// RPT-EXECUTION-CONTEXT
    RptExecutionContext                                                    = 1803,
    /// RPT-LEVEL-1
    RptLevel1                                                              = 1804,
    /// RPT-LEVEL-2
    RptLevel2                                                              = 1805,
    /// RPT-LEVEL-3
    RptLevel3                                                              = 1806,
    /// RPT-PROFILE
    RptProfile                                                             = 1807,
    /// RPT-SERVICE-POINT
    RptServicePoint                                                        = 1808,
    /// RSA
    Rsa                                                                    = 1809,
    /// RTE-EVENT
    RteEvent                                                               = 1810,
    /// RTE-EVENT-IN-COMPOSITION-SEPARATION
    RteEventInCompositionSeparation                                        = 1811,
    /// RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING
    RteEventInCompositionToOsTaskProxyMapping                              = 1812,
    /// RTE-EVENT-IN-SYSTEM-SEPARATION
    RteEventInSystemSeparation                                             = 1813,
    /// RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING
    RteEventInSystemToOsTaskProxyMapping                                   = 1814,
    /// RTPGE
    Rtpge                                                                  = 1815,
    /// RU
    Ru                                                                     = 1816,
    /// RULE
    Rule                                                                   = 1817,
    /// RUN-CONTINUOUS
    RunContinuous                                                          = 1818,
    /// RUN-ONCE
    RunOnce                                                                = 1819,
    /// RUNNABLE-ENTITY
    RunnableEntity                                                         = 1820,
    /// RUNNABLE-ENTITY-ACTIVATED
    RunnableEntityActivated                                                = 1821,
    /// RUNNABLE-ENTITY-GROUP
    RunnableEntityGroup                                                    = 1822,
    /// RUNNABLE-ENTITY-STARTED
    RunnableEntityStarted                                                  = 1823,
    /// RUNNABLE-ENTITY-TERMINATED
    RunnableEntityTerminated                                               = 1824,
    /// RUNNABLE-ENTITY-VARIABLE-ACCESS
    RunnableEntityVariableAccess                                           = 1825,
    /// RUNTIME-ERROR
    RuntimeError                                                           = 1826,
    /// RW
    Rw                                                                     = 1827,
    /// RX-TRIGGER
    RxTrigger                                                              = 1828,
    /// SA
    Sa                                                                     = 1829,
    /// SAE-J-1939--73
    SaeJ193973                                                             = 1830,
    /// SAE-J-2012--DA
    SaeJ2012Da                                                             = 1831,
    /// SAFETY
    Safety                                                                 = 1832,
    /// SATURATE
    Saturate                                                               = 1833,
    /// SCHEDULE-VARIANT-1
    ScheduleVariant1                                                       = 1834,
    /// SCHEDULE-VARIANT-2
    ScheduleVariant2                                                       = 1835,
    /// SCHEDULE-VARIANT-3
    ScheduleVariant3                                                       = 1836,
    /// SCHEDULE-VARIANT-4
    ScheduleVariant4                                                       = 1837,
    /// SCHEDULE-VARIANT-5
    ScheduleVariant5                                                       = 1838,
    /// SCHEDULE-VARIANT-6
    ScheduleVariant6                                                       = 1839,
    /// SCHEDULE-VARIANT-7
    ScheduleVariant7                                                       = 1840,
    /// SCHEDULED
    Scheduled                                                              = 1841,
    /// SD
    Sd                                                                     = 1842,
    /// SDG-ABSTRACT-FOREIGN-REFERENCE
    SdgAbstractForeignReference                                            = 1843,
    /// SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE
    SdgAbstractPrimitiveAttribute                                          = 1844,
    /// SDG-AGGREGATION-WITH-VARIATION
    SdgAggregationWithVariation                                            = 1845,
    /// SDG-ATTRIBUTE
    SdgAttribute                                                           = 1846,
    /// SDG-CAPTION
    SdgCaption                                                             = 1847,
    /// SDG-CLASS
    SdgClass                                                               = 1848,
    /// SDG-DEF
    SdgDef                                                                 = 1849,
    /// SDG-FOREIGN-REFERENCE
    SdgForeignReference                                                    = 1850,
    /// SDG-FOREIGN-REFERENCE-WITH-VARIATION
    SdgForeignReferenceWithVariation                                       = 1851,
    /// SDG-PRIMITIVE-ATTRIBUTE
    SdgPrimitiveAttribute                                                  = 1852,
    /// SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION
    SdgPrimitiveAttributeWithVariation                                     = 1853,
    /// SDG-REFERENCE
    SdgReference                                                           = 1854,
    /// SDG-TAILORING
    SdgTailoring                                                           = 1855,
    /// SEARCH-FOR-ALL
    SearchForAll                                                           = 1856,
    /// SEARCH-FOR-ALL-INSTANCES
    SearchForAllInstances                                                  = 1857,
    /// SEARCH-FOR-ANY
    SearchForAny                                                           = 1858,
    /// SEARCH-FOR-ID
    SearchForId                                                            = 1859,
    /// SEARCH-FOR-SPECIFIC-INSTANCE
    SearchForSpecificInstance                                              = 1860,
    /// SEC-OC-CRYPTO-SERVICE-MAPPING
    SecOcCryptoServiceMapping                                              = 1861,
    /// SEC-OC-DEPLOYMENT
    SecOcDeployment                                                        = 1862,
    /// SEC-OC-JOB-MAPPING
    SecOcJobMapping                                                        = 1863,
    /// SEC-OC-JOB-REQUIREMENT
    SecOcJobRequirement                                                    = 1864,
    /// SEC-OC-SECURE-COM-PROPS
    SecOcSecureComProps                                                    = 1865,
    /// SECOND-TO-FIRST
    SecondToFirst                                                          = 1866,
    /// SECONDARY-ECU
    SecondaryEcu                                                           = 1867,
    /// SECRET-SEED
    SecretSeed                                                             = 1868,
    /// SECTION-NAME-PREFIX
    SectionNamePrefix                                                      = 1869,
    /// SECURE-COM-PROPS
    SecureComProps                                                         = 1870,
    /// SECURE-COM-PROPS-SET
    SecureComPropsSet                                                      = 1871,
    /// SECURE-COMMUNICATION-AUTHENTICATION-PROPS
    SecureCommunicationAuthenticationProps                                 = 1872,
    /// SECURE-COMMUNICATION-DEPLOYMENT
    SecureCommunicationDeployment                                          = 1873,
    /// SECURE-COMMUNICATION-FRESHNESS-PROPS
    SecureCommunicationFreshnessProps                                      = 1874,
    /// SECURE-COMMUNICATION-PROPS-SET
    SecureCommunicationPropsSet                                            = 1875,
    /// SECURE-ON-BOARD-COMMUNICATION
    SecureOnBoardCommunication                                             = 1876,
    /// SECURE-ON-BOARD-COMMUNICATION-NEEDS
    SecureOnBoardCommunicationNeeds                                        = 1877,
    /// SECURED-I-PDU
    SecuredIPdu                                                            = 1878,
    /// SECURED-PDU-HEADER-08-BIT
    SecuredPduHeader08Bit                                                  = 1879,
    /// SECURED-PDU-HEADER-16-BIT
    SecuredPduHeader16Bit                                                  = 1880,
    /// SECURED-PDU-HEADER-32-BIT
    SecuredPduHeader32Bit                                                  = 1881,
    /// SECURITY
    Security                                                               = 1882,
    /// SECURITY-EVENT-AGGREGATION-FILTER
    SecurityEventAggregationFilter                                         = 1883,
    /// SECURITY-EVENT-CONTEXT-MAPPING
    SecurityEventContextMapping                                            = 1884,
    /// SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION
    SecurityEventContextMappingApplication                                 = 1885,
    /// SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE
    SecurityEventContextMappingBswModule                                   = 1886,
    /// SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR
    SecurityEventContextMappingCommConnector                               = 1887,
    /// SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER
    SecurityEventContextMappingFunctionalCluster                           = 1888,
    /// SECURITY-EVENT-CONTEXT-PROPS
    SecurityEventContextProps                                              = 1889,
    /// SECURITY-EVENT-DEFINITION
    SecurityEventDefinition                                                = 1890,
    /// SECURITY-EVENT-FILTER-CHAIN
    SecurityEventFilterChain                                               = 1891,
    /// SECURITY-EVENT-MAPPING
    SecurityEventMapping                                                   = 1892,
    /// SECURITY-EVENT-ONE-EVERY-N-FILTER
    SecurityEventOneEveryNFilter                                           = 1893,
    /// SECURITY-EVENT-REPORT-INTERFACE
    SecurityEventReportInterface                                           = 1894,
    /// SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING
    SecurityEventReportToSecurityEventDefinitionMapping                    = 1895,
    /// SECURITY-EVENT-STATE-FILTER
    SecurityEventStateFilter                                               = 1896,
    /// SECURITY-EVENT-THRESHOLD-FILTER
    SecurityEventThresholdFilter                                           = 1897,
    /// SELECTED
    Selected                                                               = 1898,
    /// SENDER-RECEIVER-INTERFACE
    SenderReceiverInterface                                                = 1899,
    /// SENSOR-ACTUATOR-SW-COMPONENT-TYPE
    SensorActuatorSwComponentType                                          = 1900,
    /// SENT-TAGGED
    SentTagged                                                             = 1901,
    /// SENT-UNTAGGED
    SentUntagged                                                           = 1902,
    /// SERIALIZATION-TECHNOLOGY
    SerializationTechnology                                                = 1903,
    /// SERIALIZER
    Serializer                                                             = 1904,
    /// SERVER-AUTHENTICATE
    ServerAuthenticate                                                     = 1905,
    /// SERVER-CALL-POINT
    ServerCallPoint                                                        = 1906,
    /// SERVER-DECRYPT
    ServerDecrypt                                                          = 1907,
    /// SERVER-ENCRYPT
    ServerEncrypt                                                          = 1908,
    /// SERVER-MAC-GENERATE
    ServerMacGenerate                                                      = 1909,
    /// SERVER-MAC-VERIFY
    ServerMacVerify                                                        = 1910,
    /// SERVER-VERIFY
    ServerVerify                                                           = 1911,
    /// SERVICE-DISCOVERY
    ServiceDiscovery                                                       = 1912,
    /// SERVICE-EVENT-DEPLOYMENT
    ServiceEventDeployment                                                 = 1913,
    /// SERVICE-FIELD-DEPLOYMENT
    ServiceFieldDeployment                                                 = 1914,
    /// SERVICE-INSTANCE-COLLECTION-SET
    ServiceInstanceCollectionSet                                           = 1915,
    /// SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING
    ServiceInstanceToApplicationEndpointMapping                            = 1916,
    /// SERVICE-INSTANCE-TO-MACHINE-MAPPING
    ServiceInstanceToMachineMapping                                        = 1917,
    /// SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToPortPrototypeMapping                                  = 1918,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING
    ServiceInstanceToSignalMapping                                         = 1919,
    /// SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET
    ServiceInstanceToSignalMappingSet                                      = 1920,
    /// SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING
    ServiceInstanceToSwClusterDesignPortPrototypeMapping                   = 1921,
    /// SERVICE-INTERFACE
    ServiceInterface                                                       = 1922,
    /// SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING
    ServiceInterfaceApplicationErrorMapping                                = 1923,
    /// SERVICE-INTERFACE-DEPLOYMENT
    ServiceInterfaceDeployment                                             = 1924,
    /// SERVICE-INTERFACE-ELEMENT-MAPPING
    ServiceInterfaceElementMapping                                         = 1925,
    /// SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG
    ServiceInterfaceElementSecureComConfig                                 = 1926,
    /// SERVICE-INTERFACE-EVENT-MAPPING
    ServiceInterfaceEventMapping                                           = 1927,
    /// SERVICE-INTERFACE-FIELD-MAPPING
    ServiceInterfaceFieldMapping                                           = 1928,
    /// SERVICE-INTERFACE-MAPPING
    ServiceInterfaceMapping                                                = 1929,
    /// SERVICE-INTERFACE-MAPPING-SET
    ServiceInterfaceMappingSet                                             = 1930,
    /// SERVICE-INTERFACE-METHOD-MAPPING
    ServiceInterfaceMethodMapping                                          = 1931,
    /// SERVICE-INTERFACE-PEDIGREE
    ServiceInterfacePedigree                                               = 1932,
    /// SERVICE-INTERFACE-TRIGGER-MAPPING
    ServiceInterfaceTriggerMapping                                         = 1933,
    /// SERVICE-METHOD-DEPLOYMENT
    ServiceMethodDeployment                                                = 1934,
    /// SERVICE-NEEDS
    ServiceNeeds                                                           = 1935,
    /// SERVICE-ONLY
    ServiceOnly                                                            = 1936,
    /// SERVICE-PROXY-SW-COMPONENT-TYPE
    ServiceProxySwComponentType                                            = 1937,
    /// SERVICE-SW-COMPONENT-TYPE
    ServiceSwComponentType                                                 = 1938,
    /// SERVICE-TIMING
    ServiceTiming                                                          = 1939,
    /// SESSION-HANDLING-ACTIVE
    SessionHandlingActive                                                  = 1940,
    /// SESSION-HANDLING-INACTIVE
    SessionHandlingInactive                                                = 1941,
    /// SETTER
    Setter                                                                 = 1942,
    /// SG
    Sg                                                                     = 1943,
    /// SH
    Sh                                                                     = 1944,
    /// SHORT-HEADER
    ShortHeader                                                            = 1945,
    /// SHOW-ALIAS-NAME
    ShowAliasName                                                          = 1946,
    /// SHOW-CATEGORY
    ShowCategory                                                           = 1947,
    /// SHOW-CONTENT
    ShowContent                                                            = 1948,
    /// SHOW-LONG-NAME
    ShowLongName                                                           = 1949,
    /// SHOW-NUMBER
    ShowNumber                                                             = 1950,
    /// SHOW-PAGE
    ShowPage                                                               = 1951,
    /// SHOW-SEE
    ShowSee                                                                = 1952,
    /// SHOW-SHORT-NAME
    ShowShortName                                                          = 1953,
    /// SHOW-TYPE
    ShowType                                                               = 1954,
    /// SI
    Si                                                                     = 1955,
    /// SIDES
    Sides                                                                  = 1956,
    /// SIGN
    Sign                                                                   = 1957,
    /// SIGN-WITH-ORIGIN-AUTHENTICATION
    SignWithOriginAuthentication                                           = 1958,
    /// SIGNAL-BASED
    SignalBased                                                            = 1959,
    /// SIGNAL-BASED-EVENT-DEPLOYMENT
    SignalBasedEventDeployment                                             = 1960,
    /// SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedEventElementToISignalTriggeringMapping                      = 1961,
    /// SIGNAL-BASED-FIELD-DEPLOYMENT
    SignalBasedFieldDeployment                                             = 1962,
    /// SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedFieldToISignalTriggeringMapping                             = 1963,
    /// SIGNAL-BASED-METHOD-DEPLOYMENT
    SignalBasedMethodDeployment                                            = 1964,
    /// SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedMethodToISignalTriggeringMapping                            = 1965,
    /// SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT
    SignalBasedServiceInterfaceDeployment                                  = 1966,
    /// SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING
    SignalBasedTriggerToISignalTriggeringMapping                           = 1967,
    /// SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS
    SignalServiceTranslationElementProps                                   = 1968,
    /// SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS
    SignalServiceTranslationEventProps                                     = 1969,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS
    SignalServiceTranslationProps                                          = 1970,
    /// SIGNAL-SERVICE-TRANSLATION-PROPS-SET
    SignalServiceTranslationPropsSet                                       = 1971,
    /// SIGNATURE
    Signature                                                              = 1972,
    /// SILENT
    Silent                                                                 = 1973,
    /// SIMULATED-EXECUTION-TIME
    SimulatedExecutionTime                                                 = 1974,
    /// SINGLE
    Single                                                                 = 1975,
    /// SINGLE-CORE-REENTRANT
    SingleCoreReentrant                                                    = 1976,
    /// SINGLE-LANGUAGE-REFERRABLE
    SingleLanguageReferrable                                               = 1977,
    /// SINGLE-OCCURRENCE
    SingleOccurrence                                                       = 1978,
    /// SK
    Sk                                                                     = 1979,
    /// SL
    Sl                                                                     = 1980,
    /// SLAVE
    Slave                                                                  = 1981,
    /// SLOPPY
    Sloppy                                                                 = 1982,
    /// SLOW-FLASHING-MODE
    SlowFlashingMode                                                       = 1983,
    /// SLP
    Slp                                                                    = 1984,
    /// SM
    Sm                                                                     = 1985,
    /// SN
    Sn                                                                     = 1986,
    /// SO
    So                                                                     = 1987,
    /// SO-AD-ROUTING-GROUP
    SoAdRoutingGroup                                                       = 1988,
    /// SO-CON-I-PDU-IDENTIFIER
    SoConIPduIdentifier                                                    = 1989,
    /// SOCKET-ADDRESS
    SocketAddress                                                          = 1990,
    /// SOCKET-CONNECTION-BUNDLE
    SocketConnectionBundle                                                 = 1991,
    /// SOCKET-CONNECTION-IPDU-IDENTIFIER-SET
    SocketConnectionIpduIdentifierSet                                      = 1992,
    /// SOFTWARE-ACTIVATION-DEPENDENCY
    SoftwareActivationDependency                                           = 1993,
    /// SOFTWARE-CLUSTER
    SoftwareCluster                                                        = 1994,
    /// SOFTWARE-CLUSTER-DESIGN
    SoftwareClusterDesign                                                  = 1995,
    /// SOFTWARE-CLUSTER-REQUIREMENT
    SoftwareClusterRequirement                                             = 1996,
    /// SOFTWARE-PACKAGE
    SoftwarePackage                                                        = 1997,
    /// SOFTWARE-PACKAGE-STEP
    SoftwarePackageStep                                                    = 1998,
    /// SOMEIP
    Someip                                                                 = 1999,
    /// SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS
    SomeipDataPrototypeTransformationProps                                 = 2000,
    /// SOMEIP-EVENT
    SomeipEvent                                                            = 2001,
    /// SOMEIP-EVENT-DEPLOYMENT
    SomeipEventDeployment                                                  = 2002,
    /// SOMEIP-EVENT-GROUP
    SomeipEventGroup                                                       = 2003,
    /// SOMEIP-FIELD
    SomeipField                                                            = 2004,
    /// SOMEIP-FIELD-DEPLOYMENT
    SomeipFieldDeployment                                                  = 2005,
    /// SOMEIP-METHOD
    SomeipMethod                                                           = 2006,
    /// SOMEIP-METHOD-DEPLOYMENT
    SomeipMethodDeployment                                                 = 2007,
    /// SOMEIP-PROVIDED-EVENT-GROUP
    SomeipProvidedEventGroup                                               = 2008,
    /// SOMEIP-REQUIRED-EVENT-GROUP
    SomeipRequiredEventGroup                                               = 2009,
    /// SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG
    SomeipSdClientEventGroupTimingConfig                                   = 2010,
    /// SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG
    SomeipSdClientServiceInstanceConfig                                    = 2011,
    /// SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG
    SomeipSdServerEventGroupTimingConfig                                   = 2012,
    /// SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG
    SomeipSdServerServiceInstanceConfig                                    = 2013,
    /// SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    SomeipServiceInstanceToMachineMapping                                  = 2014,
    /// SOMEIP-SERVICE-INTERFACE
    SomeipServiceInterface                                                 = 2015,
    /// SOMEIP-SERVICE-INTERFACE-DEPLOYMENT
    SomeipServiceInterfaceDeployment                                       = 2016,
    /// SOMEIP-TP-CHANNEL
    SomeipTpChannel                                                        = 2017,
    /// SOMEIP-TP-CONFIG
    SomeipTpConfig                                                         = 2018,
    /// SOMEIP-TRANSFORMATION-PROPS
    SomeipTransformationProps                                              = 2019,
    /// SPEC-ELEMENT-REFERENCE
    SpecElementReference                                                   = 2020,
    /// SPEC-ELEMENT-SCOPE
    SpecElementScope                                                       = 2021,
    /// SPECIFICATION-DOCUMENT-SCOPE
    SpecificationDocumentScope                                             = 2022,
    /// SPORADIC-EVENT-TRIGGERING
    SporadicEventTriggering                                                = 2023,
    /// SQ
    Sq                                                                     = 2024,
    /// SR
    Sr                                                                     = 2025,
    /// SS
    Ss                                                                     = 2026,
    /// SSDP
    Ssdp                                                                   = 2027,
    /// ST
    St                                                                     = 2028,
    /// STACK-USAGE
    StackUsage                                                             = 2029,
    /// STANDARD
    Standard                                                               = 2030,
    /// STANDARD-PORT
    StandardPort                                                           = 2031,
    /// START
    Start                                                                  = 2032,
    /// START-FROM-BEGINNING
    StartFromBeginning                                                     = 2033,
    /// STARTUP-CONFIG
    StartupConfig                                                          = 2034,
    /// STARTUP-CONFIG-SET
    StartupConfigSet                                                       = 2035,
    /// STATIC-OR-DYNAMIC-PART-TRIGGER
    StaticOrDynamicPartTrigger                                             = 2036,
    /// STATIC-PART-TRIGGER
    StaticPartTrigger                                                      = 2037,
    /// STATIC-SOCKET-CONNECTION
    StaticSocketConnection                                                 = 2038,
    /// STATUS-BIT-AGING-AND-DISPLACEMENT
    StatusBitAgingAndDisplacement                                          = 2039,
    /// STATUS-BIT-NORMAL
    StatusBitNormal                                                        = 2040,
    /// STD
    Std                                                                    = 2041,
    /// STD-AXIS
    StdAxis                                                                = 2042,
    /// STD-CPP-IMPLEMENTATION-DATA-TYPE
    StdCppImplementationDataType                                           = 2043,
    /// STD_AXIS
    Stdaxis                                                                = 2044,
    /// STEADY
    Steady                                                                 = 2045,
    /// STIMULUS-SYNCHRONIZATION
    StimulusSynchronization                                                = 2046,
    /// STOP
    Stop                                                                   = 2047,
    /// STOP-TRIGGER
    StopTrigger                                                            = 2048,
    /// STORE-EVENT
    StoreEvent                                                             = 2049,
    /// STORE-PERSISTENTLY
    StorePersistently                                                      = 2050,
    /// STRICT-MODE
    StrictMode                                                             = 2051,
    /// STRICT-MONOTONOUS
    StrictMonotonous                                                       = 2052,
    /// STRICT-PRIORITY
    StrictPriority                                                         = 2053,
    /// STRICTLY-DECREASING
    StrictlyDecreasing                                                     = 2054,
    /// STRICTLY-INCREASING
    StrictlyIncreasing                                                     = 2055,
    /// STRUCTURED-REQ
    StructuredReq                                                          = 2056,
    /// SU
    Su                                                                     = 2057,
    /// SUPERVISED-ENTITY-CHECKPOINT-NEEDS
    SupervisedEntityCheckpointNeeds                                        = 2058,
    /// SUPERVISED-ENTITY-NEEDS
    SupervisedEntityNeeds                                                  = 2059,
    /// SUPERVISION-CHECKPOINT
    SupervisionCheckpoint                                                  = 2060,
    /// SUPERVISION-ENTITY
    SupervisionEntity                                                      = 2061,
    /// SUPERVISION-MODE
    SupervisionMode                                                        = 2062,
    /// SUPERVISION-MODE-CONDITION
    SupervisionModeCondition                                               = 2063,
    /// SUPPLIER
    Supplier                                                               = 2064,
    /// SUPPORTS-BUFFER-LOCKING
    SupportsBufferLocking                                                  = 2065,
    /// SV
    Sv                                                                     = 2066,
    /// SVG
    Svg                                                                    = 2067,
    /// SW
    Sw                                                                     = 2068,
    /// SW-ADDR-METHOD
    SwAddrMethod                                                           = 2069,
    /// SW-AXIS-TYPE
    SwAxisType                                                             = 2070,
    /// SW-BASE-TYPE
    SwBaseType                                                             = 2071,
    /// SW-CALIBRATION-METHOD
    SwCalibrationMethod                                                    = 2072,
    /// SW-CALPRM-PROTOTYPE
    SwCalprmPrototype                                                      = 2073,
    /// SW-CLASS-ATTR-IMPL
    SwClassAttrImpl                                                        = 2074,
    /// SW-CLASS-INSTANCE
    SwClassInstance                                                        = 2075,
    /// SW-CLASS-PROTOTYPE
    SwClassPrototype                                                       = 2076,
    /// SW-CODE-SYNTAX
    SwCodeSyntax                                                           = 2077,
    /// SW-COMPONENT-MAPPING-CONSTRAINTS
    SwComponentMappingConstraints                                          = 2078,
    /// SW-COMPONENT-PROTOTYPE
    SwComponentPrototype                                                   = 2079,
    /// SW-COMPONENT-TYPE
    SwComponentType                                                        = 2080,
    /// SW-CONNECTOR
    SwConnector                                                            = 2081,
    /// SW-GENERIC-AXIS-PARAM-TYPE
    SwGenericAxisParamType                                                 = 2082,
    /// SW-INSTANCE
    SwInstance                                                             = 2083,
    /// SW-MC-BASE-TYPE
    SwMcBaseType                                                           = 2084,
    /// SW-MC-FRAME
    SwMcFrame                                                              = 2085,
    /// SW-MC-INTERFACE
    SwMcInterface                                                          = 2086,
    /// SW-MC-INTERFACE-SOURCE
    SwMcInterfaceSource                                                    = 2087,
    /// SW-RECORD-LAYOUT
    SwRecordLayout                                                         = 2088,
    /// SW-SERVICE-ARG
    SwServiceArg                                                           = 2089,
    /// SW-SERVICE-PROTOTYPE
    SwServicePrototype                                                     = 2090,
    /// SW-SYSTEMCONST
    SwSystemconst                                                          = 2091,
    /// SW-SYSTEMCONSTANT-VALUE-SET
    SwSystemconstantValueSet                                               = 2092,
    /// SW-VARIABLE-PROTOTYPE
    SwVariablePrototype                                                    = 2093,
    /// SWC
    Swc                                                                    = 2094,
    /// SWC-BSW-MAPPING
    SwcBswMapping                                                          = 2095,
    /// SWC-IMPLEMENTATION
    SwcImplementation                                                      = 2096,
    /// SWC-INTERNAL-BEHAVIOR
    SwcInternalBehavior                                                    = 2097,
    /// SWC-MODE-MANAGER-ERROR-EVENT
    SwcModeManagerErrorEvent                                               = 2098,
    /// SWC-MODE-SWITCH-EVENT
    SwcModeSwitchEvent                                                     = 2099,
    /// SWC-SERVICE-DEPENDENCY
    SwcServiceDependency                                                   = 2100,
    /// SWC-TIMING
    SwcTiming                                                              = 2101,
    /// SWC-TO-APPLICATION-PARTITION-MAPPING
    SwcToApplicationPartitionMapping                                       = 2102,
    /// SWC-TO-ECU-MAPPING
    SwcToEcuMapping                                                        = 2103,
    /// SWC-TO-IMPL-MAPPING
    SwcToImplMapping                                                       = 2104,
    /// SWITCH
    Switch                                                                 = 2105,
    /// SYMBOL-PROPS
    SymbolProps                                                            = 2106,
    /// SYMBOLIC-NAME-PROPS
    SymbolicNameProps                                                      = 2107,
    /// SYMMETRIC
    Symmetric                                                              = 2108,
    /// SYMMETRIC-KEY
    SymmetricKey                                                           = 2109,
    /// SYNC-BASE-TIME-MANAGER
    SyncBaseTimeManager                                                    = 2110,
    /// SYNC-TIME-BASE-MGR-USER-NEEDS
    SyncTimeBaseMgrUserNeeds                                               = 2111,
    /// SYNCHRONIZATION-POINT-CONSTRAINT
    SynchronizationPointConstraint                                         = 2112,
    /// SYNCHRONIZATION-TIMING-CONSTRAINT
    SynchronizationTimingConstraint                                        = 2113,
    /// SYNCHRONIZED
    Synchronized                                                           = 2114,
    /// SYNCHRONIZED-MASTER-TIME-BASE
    SynchronizedMasterTimeBase                                             = 2115,
    /// SYNCHRONIZED-SLAVE-TIME-BASE
    SynchronizedSlaveTimeBase                                              = 2116,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER
    SynchronizedTimeBaseConsumer                                           = 2117,
    /// SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE
    SynchronizedTimeBaseConsumerInterface                                  = 2118,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER
    SynchronizedTimeBaseProvider                                           = 2119,
    /// SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE
    SynchronizedTimeBaseProviderInterface                                  = 2120,
    /// SYNCHRONOUS
    Synchronous                                                            = 2121,
    /// SYNCHRONOUS-SERVER-CALL-POINT
    SynchronousServerCallPoint                                             = 2122,
    /// SYSTEM
    System                                                                 = 2123,
    /// SYSTEM-DESIGN-TIME
    SystemDesignTime                                                       = 2124,
    /// SYSTEM-MAPPING
    SystemMapping                                                          = 2125,
    /// SYSTEM-MEMORY-USAGE
    SystemMemoryUsage                                                      = 2126,
    /// SYSTEM-SIGNAL
    SystemSignal                                                           = 2127,
    /// SYSTEM-SIGNAL-GROUP
    SystemSignalGroup                                                      = 2128,
    /// SYSTEM-SUPPLIER-BOOT
    SystemSupplierBoot                                                     = 2129,
    /// SYSTEM-SUPPLIER-BOOT-RESP-APP
    SystemSupplierBootRespApp                                              = 2130,
    /// SYSTEM-TIMING
    SystemTiming                                                           = 2131,
    /// TA
    Ta                                                                     = 2132,
    /// TARGET-CONTAINER
    TargetContainer                                                        = 2133,
    /// TASK
    Task                                                                   = 2134,
    /// TC
    Tc                                                                     = 2135,
    /// TCP
    Tcp                                                                    = 2136,
    /// TCP-OPTION-FILTER-LIST
    TcpOptionFilterList                                                    = 2137,
    /// TCP-OPTION-FILTER-SET
    TcpOptionFilterSet                                                     = 2138,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING
    TdCpSoftwareClusterMapping                                             = 2139,
    /// TD-CP-SOFTWARE-CLUSTER-MAPPING-SET
    TdCpSoftwareClusterMappingSet                                          = 2140,
    /// TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING
    TdCpSoftwareClusterResourceMapping                                     = 2141,
    /// TD-EVENT-BSW
    TdEventBsw                                                             = 2142,
    /// TD-EVENT-BSW-INTERNAL-BEHAVIOR
    TdEventBswInternalBehavior                                             = 2143,
    /// TD-EVENT-BSW-MODE-DECLARATION
    TdEventBswModeDeclaration                                              = 2144,
    /// TD-EVENT-BSW-MODULE
    TdEventBswModule                                                       = 2145,
    /// TD-EVENT-COM
    TdEventCom                                                             = 2146,
    /// TD-EVENT-COMPLEX
    TdEventComplex                                                         = 2147,
    /// TD-EVENT-CYCLE-START
    TdEventCycleStart                                                      = 2148,
    /// TD-EVENT-FR-CLUSTER-CYCLE-START
    TdEventFrClusterCycleStart                                             = 2149,
    /// TD-EVENT-FRAME
    TdEventFrame                                                           = 2150,
    /// TD-EVENT-FRAME-ETHERNET
    TdEventFrameEthernet                                                   = 2151,
    /// TD-EVENT-I-PDU
    TdEventIPdu                                                            = 2152,
    /// TD-EVENT-I-SIGNAL
    TdEventISignal                                                         = 2153,
    /// TD-EVENT-MODE-DECLARATION
    TdEventModeDeclaration                                                 = 2154,
    /// TD-EVENT-OPERATION
    TdEventOperation                                                       = 2155,
    /// TD-EVENT-SERVICE-INSTANCE
    TdEventServiceInstance                                                 = 2156,
    /// TD-EVENT-SERVICE-INSTANCE-DISCOVERY
    TdEventServiceInstanceDiscovery                                        = 2157,
    /// TD-EVENT-SERVICE-INSTANCE-EVENT
    TdEventServiceInstanceEvent                                            = 2158,
    /// TD-EVENT-SERVICE-INSTANCE-FIELD
    TdEventServiceInstanceField                                            = 2159,
    /// TD-EVENT-SERVICE-INSTANCE-METHOD
    TdEventServiceInstanceMethod                                           = 2160,
    /// TD-EVENT-SWC
    TdEventSwc                                                             = 2161,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR
    TdEventSwcInternalBehavior                                             = 2162,
    /// TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE
    TdEventSwcInternalBehaviorReference                                    = 2163,
    /// TD-EVENT-TRIGGER
    TdEventTrigger                                                         = 2164,
    /// TD-EVENT-TT-CAN-CYCLE-START
    TdEventTtCanCycleStart                                                 = 2165,
    /// TD-EVENT-VARIABLE-DATA-PROTOTYPE
    TdEventVariableDataPrototype                                           = 2166,
    /// TD-EVENT-VFB
    TdEventVfb                                                             = 2167,
    /// TD-EVENT-VFB-PORT
    TdEventVfbPort                                                         = 2168,
    /// TD-EVENT-VFB-REFERENCE
    TdEventVfbReference                                                    = 2169,
    /// TE
    Te                                                                     = 2170,
    /// TERMINATE
    Terminate                                                              = 2171,
    /// TEST-FAILED
    TestFailed                                                             = 2172,
    /// TEST-FAILED-BIT
    TestFailedBit                                                          = 2173,
    /// TEST-FAILED-THIS-OPERATION-CYCLE
    TestFailedThisOperationCycle                                           = 2174,
    /// TESTED
    Tested                                                                 = 2175,
    /// TESTED-AND-FAILED
    TestedAndFailed                                                        = 2176,
    /// TG
    Tg                                                                     = 2177,
    /// TH
    Th                                                                     = 2178,
    /// TI
    Ti                                                                     = 2179,
    /// TIFF
    Tiff                                                                   = 2180,
    /// TIME
    Time                                                                   = 2181,
    /// TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING
    TimeBaseProviderToPersistencyMapping                                   = 2182,
    /// TIME-BASE-RESOURCE
    TimeBaseResource                                                       = 2183,
    /// TIME-SYNC-MODULE-INSTANTIATION
    TimeSyncModuleInstantiation                                            = 2184,
    /// TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING
    TimeSyncPortPrototypeToTimeBaseMapping                                 = 2185,
    /// TIME-SYNC-SERVER-CONFIGURATION
    TimeSyncServerConfiguration                                            = 2186,
    /// TIME-SYNCHRONIZATION-INTERFACE
    TimeSynchronizationInterface                                           = 2187,
    /// TIME-SYNCHRONIZATION-MASTER-INTERFACE
    TimeSynchronizationMasterInterface                                     = 2188,
    /// TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE
    TimeSynchronizationPureLocalInterface                                  = 2189,
    /// TIME-SYNCHRONIZATION-SLAVE-INTERFACE
    TimeSynchronizationSlaveInterface                                      = 2190,
    /// TIMING-CONDITION
    TimingCondition                                                        = 2191,
    /// TIMING-CONSTRAINT
    TimingConstraint                                                       = 2192,
    /// TIMING-DESCRIPTION
    TimingDescription                                                      = 2193,
    /// TIMING-DESCRIPTION-EVENT
    TimingDescriptionEvent                                                 = 2194,
    /// TIMING-DESCRIPTION-EVENT-CHAIN
    TimingDescriptionEventChain                                            = 2195,
    /// TIMING-EVENT
    TimingEvent                                                            = 2196,
    /// TIMING-EXTENSION
    TimingExtension                                                        = 2197,
    /// TIMING-EXTENSION-RESOURCE
    TimingExtensionResource                                                = 2198,
    /// TIMING-MODE-INSTANCE
    TimingModeInstance                                                     = 2199,
    /// TIP
    Tip                                                                    = 2200,
    /// TK
    Tk                                                                     = 2201,
    /// TL
    Tl                                                                     = 2202,
    /// TLS-12
    Tls12                                                                  = 2203,
    /// TLS-13
    Tls13                                                                  = 2204,
    /// TLS-CONNECTION-GROUP
    TlsConnectionGroup                                                     = 2205,
    /// TLS-CRYPTO-CIPHER-SUITE
    TlsCryptoCipherSuite                                                   = 2206,
    /// TLS-CRYPTO-CIPHER-SUITE-PROPS
    TlsCryptoCipherSuiteProps                                              = 2207,
    /// TLS-CRYPTO-SERVICE-MAPPING
    TlsCryptoServiceMapping                                                = 2208,
    /// TLS-DEPLOYMENT
    TlsDeployment                                                          = 2209,
    /// TLS-IAM-REMOTE-SUBJECT
    TlsIamRemoteSubject                                                    = 2210,
    /// TLS-JOB-MAPPING
    TlsJobMapping                                                          = 2211,
    /// TLS-JOB-REQUIREMENT
    TlsJobRequirement                                                      = 2212,
    /// TLS-SECURE-COM-PROPS
    TlsSecureComProps                                                      = 2213,
    /// TLV-DATA-ID-DEFINITION-SET
    TlvDataIdDefinitionSet                                                 = 2214,
    /// TN
    Tn                                                                     = 2215,
    /// TO
    To                                                                     = 2216,
    /// TOP
    Top                                                                    = 2217,
    /// TOPBOT
    Topbot                                                                 = 2218,
    /// TOPIC
    Topic                                                                  = 2219,
    /// TOPIC-1
    Topic1                                                                 = 2220,
    /// TOPIC-PREFIX
    TopicPrefix                                                            = 2221,
    /// TP-ADDRESS
    TpAddress                                                              = 2222,
    /// TP-CONFIG
    TpConfig                                                               = 2223,
    /// TP-CONNECTION-IDENT
    TpConnectionIdent                                                      = 2224,
    /// TR
    Tr                                                                     = 2225,
    /// TRACE
    Trace                                                                  = 2226,
    /// TRACE-REFERRABLE
    TraceReferrable                                                        = 2227,
    /// TRACEABLE
    Traceable                                                              = 2228,
    /// TRACEABLE-TABLE
    TraceableTable                                                         = 2229,
    /// TRACEABLE-TEXT
    TraceableText                                                          = 2230,
    /// TRACED-FAILURE
    TracedFailure                                                          = 2231,
    /// TRANSFER
    Transfer                                                               = 2232,
    /// TRANSFORMATION-PROPS
    TransformationProps                                                    = 2233,
    /// TRANSFORMATION-PROPS-SET
    TransformationPropsSet                                                 = 2234,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING
    TransformationPropsToServiceInterfaceElementMapping                    = 2235,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET
    TransformationPropsToServiceInterfaceElementMappingSet                 = 2236,
    /// TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET
    TransformationPropsToServiceInterfaceMappingSet                        = 2237,
    /// TRANSFORMATION-TECHNOLOGY
    TransformationTechnology                                               = 2238,
    /// TRANSFORMER-ERROR-HANDLING
    TransformerErrorHandling                                               = 2239,
    /// TRANSFORMER-HARD-ERROR-EVENT
    TransformerHardErrorEvent                                              = 2240,
    /// TRANSFORMER-STATUS-FORWARDING
    TransformerStatusForwarding                                            = 2241,
    /// TRANSFORMING-I-SIGNAL
    TransformingISignal                                                    = 2242,
    /// TRANSIENT-FAULT
    TransientFault                                                         = 2243,
    /// TRANSLATION-START
    TranslationStart                                                       = 2244,
    /// TRANSPORT
    Transport                                                              = 2245,
    /// TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET
    TransportLayerIndependentIdCollectionSet                               = 2246,
    /// TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID
    TransportLayerIndependentInstanceId                                    = 2247,
    /// TRAP
    Trap                                                                   = 2248,
    /// TRIGGER
    Trigger                                                                = 2249,
    /// TRIGGER-ACTIVATED
    TriggerActivated                                                       = 2250,
    /// TRIGGER-INTERFACE
    TriggerInterface                                                       = 2251,
    /// TRIGGER-INTERFACE-MAPPING
    TriggerInterfaceMapping                                                = 2252,
    /// TRIGGER-RELEASED
    TriggerReleased                                                        = 2253,
    /// TRIGGER-UNICAST
    TriggerUnicast                                                         = 2254,
    /// TRIGGERED
    Triggered                                                              = 2255,
    /// TRIGGERED-ON-CHANGE
    TriggeredOnChange                                                      = 2256,
    /// TRIGGERED-ON-CHANGE-WITHOUT-REPETITION
    TriggeredOnChangeWithoutRepetition                                     = 2257,
    /// TRIGGERED-ON-EVALUATION
    TriggeredOnEvaluation                                                  = 2258,
    /// TRIGGERED-WITHOUT-REPETITION
    TriggeredWithoutRepetition                                             = 2259,
    /// TRUE
    True                                                                   = 2260,
    /// TS
    Ts                                                                     = 2261,
    /// TT
    Tt                                                                     = 2262,
    /// TTCAN-CLUSTER
    TtcanCluster                                                           = 2263,
    /// TTCAN-COMMUNICATION-CONNECTOR
    TtcanCommunicationConnector                                            = 2264,
    /// TTCAN-COMMUNICATION-CONTROLLER
    TtcanCommunicationController                                           = 2265,
    /// TTCAN-PHYSICAL-CHANNEL
    TtcanPhysicalChannel                                                   = 2266,
    /// TUNNEL
    Tunnel                                                                 = 2267,
    /// TW
    Tw                                                                     = 2268,
    /// TX-REF-TRIGGER
    TxRefTrigger                                                           = 2269,
    /// TX-REF-TRIGGER-GAP
    TxRefTriggerGap                                                        = 2270,
    /// TX-TRIGGER-MERGED
    TxTriggerMerged                                                        = 2271,
    /// TX-TRIGGER-SINGLE
    TxTriggerSingle                                                        = 2272,
    /// UCM
    Ucm                                                                    = 2273,
    /// UCM-DESCRIPTION
    UcmDescription                                                         = 2274,
    /// UCM-MASTER
    UcmMaster                                                              = 2275,
    /// UCM-MODULE-INSTANTIATION
    UcmModuleInstantiation                                                 = 2276,
    /// UCM-STEP
    UcmStep                                                                = 2277,
    /// UDP
    Udp                                                                    = 2278,
    /// UDP-CHECKSUM-DISABLED
    UdpChecksumDisabled                                                    = 2279,
    /// UDP-CHECKSUM-ENABLED
    UdpChecksumEnabled                                                     = 2280,
    /// UDP-NM
    UdpNm                                                                  = 2281,
    /// UDP-NM-CLUSTER
    UdpNmCluster                                                           = 2282,
    /// UDP-NM-NODE
    UdpNmNode                                                              = 2283,
    /// UDS
    Uds                                                                    = 2284,
    /// UK
    Uk                                                                     = 2285,
    /// UNDECIDED
    Undecided                                                              = 2286,
    /// UNDEFINED
    Undefined                                                              = 2287,
    /// UNIT
    Unit                                                                   = 2288,
    /// UNIT-GROUP
    UnitGroup                                                              = 2289,
    /// UNNUMBER
    Unnumber                                                               = 2290,
    /// UNSPECIFIED
    Unspecified                                                            = 2291,
    /// UP-LINK-PORT
    UpLinkPort                                                             = 2292,
    /// UPDATE
    Update                                                                 = 2293,
    /// UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT
    UploadableExclusivePackageElement                                      = 2294,
    /// UPLOADABLE-PACKAGE-ELEMENT
    UploadablePackageElement                                               = 2295,
    /// UR
    Ur                                                                     = 2296,
    /// USE-ARGUMENT-TYPE
    UseArgumentType                                                        = 2297,
    /// USE-ARRAY-BASE-TYPE
    UseArrayBaseType                                                       = 2298,
    /// USE-FIRST-CONTEXT-DATA
    UseFirstContextData                                                    = 2299,
    /// USE-LAST-CONTEXT-DATA
    UseLastContextData                                                     = 2300,
    /// USE-VOID
    UseVoid                                                                = 2301,
    /// USER-DEFINED
    UserDefined                                                            = 2302,
    /// USER-DEFINED-CLUSTER
    UserDefinedCluster                                                     = 2303,
    /// USER-DEFINED-COMMUNICATION-CONNECTOR
    UserDefinedCommunicationConnector                                      = 2304,
    /// USER-DEFINED-COMMUNICATION-CONTROLLER
    UserDefinedCommunicationController                                     = 2305,
    /// USER-DEFINED-ETHERNET-FRAME
    UserDefinedEthernetFrame                                               = 2306,
    /// USER-DEFINED-EVENT-DEPLOYMENT
    UserDefinedEventDeployment                                             = 2307,
    /// USER-DEFINED-FIELD-DEPLOYMENT
    UserDefinedFieldDeployment                                             = 2308,
    /// USER-DEFINED-GLOBAL-TIME-MASTER
    UserDefinedGlobalTimeMaster                                            = 2309,
    /// USER-DEFINED-GLOBAL-TIME-SLAVE
    UserDefinedGlobalTimeSlave                                             = 2310,
    /// USER-DEFINED-I-PDU
    UserDefinedIPdu                                                        = 2311,
    /// USER-DEFINED-METHOD-DEPLOYMENT
    UserDefinedMethodDeployment                                            = 2312,
    /// USER-DEFINED-PDU
    UserDefinedPdu                                                         = 2313,
    /// USER-DEFINED-PHYSICAL-CHANNEL
    UserDefinedPhysicalChannel                                             = 2314,
    /// USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING
    UserDefinedServiceInstanceToMachineMapping                             = 2315,
    /// USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT
    UserDefinedServiceInterfaceDeployment                                  = 2316,
    /// USER-DEFINED-TRANSFORMATION-PROPS
    UserDefinedTransformationProps                                         = 2317,
    /// USES-LOGGING
    UsesLogging                                                            = 2318,
    /// UZ
    Uz                                                                     = 2319,
    /// V-2-X-ACTIVE-SUPPORTED
    V2XActiveSupported                                                     = 2320,
    /// V-2-X-FAC-USER-NEEDS
    V2XFacUserNeeds                                                        = 2321,
    /// V-2-X-FACILITIES
    V2XFacilities                                                          = 2322,
    /// V-2-X-M-USER-NEEDS
    V2XMUserNeeds                                                          = 2323,
    /// V-2-X-MANAGEMENT
    V2XManagement                                                          = 2324,
    /// V-2-X-NOT-SUPPORTED
    V2XNotSupported                                                        = 2325,
    /// VALID
    Valid                                                                  = 2326,
    /// VAR
    Var                                                                    = 2327,
    /// VAR-FAST
    VarFast                                                                = 2328,
    /// VAR-NO-INIT
    VarNoInit                                                              = 2329,
    /// VAR-POWER-ON-INIT
    VarPowerOnInit                                                         = 2330,
    /// VARIABLE-ACCESS
    VariableAccess                                                         = 2331,
    /// VARIABLE-AND-PARAMETER-INTERFACE-MAPPING
    VariableAndParameterInterfaceMapping                                   = 2332,
    /// VARIABLE-DATA-PROTOTYPE
    VariableDataPrototype                                                  = 2333,
    /// VARIABLE-DATA-PROTOTYPE-RECEIVED
    VariableDataPrototypeReceived                                          = 2334,
    /// VARIABLE-DATA-PROTOTYPE-SENT
    VariableDataPrototypeSent                                              = 2335,
    /// VARIABLE-SIZE
    VariableSize                                                           = 2336,
    /// VARIANT-LINK-TIME
    VariantLinkTime                                                        = 2337,
    /// VARIANT-POST-BUILD
    VariantPostBuild                                                       = 2338,
    /// VARIANT-POST-BUILD-LOADABLE
    VariantPostBuildLoadable                                               = 2339,
    /// VARIANT-POST-BUILD-SELECTABLE
    VariantPostBuildSelectable                                             = 2340,
    /// VARIANT-PRE-COMPILE
    VariantPreCompile                                                      = 2341,
    /// VARIATION-POINT-PROXY
    VariationPointProxy                                                    = 2342,
    /// VEHICLE-PACKAGE
    VehiclePackage                                                         = 2343,
    /// VEHICLE-ROLLOUT-STEP
    VehicleRolloutStep                                                     = 2344,
    /// VENDOR-SPECIFIC
    VendorSpecific                                                         = 2345,
    /// VENDOR-SPECIFIC-SERVICE-NEEDS
    VendorSpecificServiceNeeds                                             = 2346,
    /// VERBOSE
    Verbose                                                                = 2347,
    /// VERIFICATION
    Verification                                                           = 2348,
    /// VERIFY
    Verify                                                                 = 2349,
    /// VERTEX-OF-TARGET-CONTAINER
    VertexOfTargetContainer                                                = 2350,
    /// VFB-TIMING
    VfbTiming                                                              = 2351,
    /// VI
    Vi                                                                     = 2352,
    /// VIEW-MAP
    ViewMap                                                                = 2353,
    /// VIEW-MAP-SET
    ViewMapSet                                                             = 2354,
    /// VLAN-CONFIG
    VlanConfig                                                             = 2355,
    /// VO
    Vo                                                                     = 2356,
    /// VOLATILE
    Volatile                                                               = 2357,
    /// WAIT-POINT
    WaitPoint                                                              = 2358,
    /// WAIT-TIME-DATE
    WaitTimeDate                                                           = 2359,
    /// WARMUP
    Warmup                                                                 = 2360,
    /// WARN
    Warn                                                                   = 2361,
    /// WARNING
    Warning                                                                = 2362,
    /// WARNING-INDICATOR-REQUESTED-BIT-NEEDS
    WarningIndicatorRequestedBitNeeds                                      = 2363,
    /// WATCH-DOG-MANAGER
    WatchDogManager                                                        = 2364,
    /// WATCH-TRIGGER
    WatchTrigger                                                           = 2365,
    /// WATCH-TRIGGER-GAP
    WatchTriggerGap                                                        = 2366,
    /// WATCHDOG-ACTION-ITEM
    WatchdogActionItem                                                     = 2367,
    /// WATCHDOG-PHM-ACTION-ITEM
    WatchdogPhmActionItem                                                  = 2368,
    /// WEIGHTED-ROUND-ROBIN
    WeightedRoundRobin                                                     = 2369,
    /// WILL-CALL
    WillCall                                                               = 2370,
    /// WILL-RECEIVE
    WillReceive                                                            = 2371,
    /// WILL-SEND
    WillSend                                                               = 2372,
    /// WO
    Wo                                                                     = 2373,
    /// WONT-CALL
    WontCall                                                               = 2374,
    /// WONT-RECEIVE
    WontReceive                                                            = 2375,
    /// WONT-SEND
    WontSend                                                               = 2376,
    /// WORST-CASE-HEAP-USAGE
    WorstCaseHeapUsage                                                     = 2377,
    /// WORST-CASE-STACK-USAGE
    WorstCaseStackUsage                                                    = 2378,
    /// WRITE-ONLY
    WriteOnly                                                              = 2379,
    /// WRONG-TRIGGER
    WrongTrigger                                                           = 2380,
    /// X-509
    X509                                                                   = 2381,
    /// X-MII
    XMii                                                                   = 2382,
    /// X-MMI
    XMmi                                                                   = 2383,
    /// XCP
    Xcp                                                                    = 2384,
    /// XCP-PDU
    XcpPdu                                                                 = 2385,
    /// XDOC
    Xdoc                                                                   = 2386,
    /// XFILE
    Xfile                                                                  = 2387,
    /// XG-MII
    XgMii                                                                  = 2388,
    /// XH
    Xh                                                                     = 2389,
    /// XOR
    Xor                                                                    = 2390,
    /// XREF-TARGET
    XrefTarget                                                             = 2391,
    /// XXG-MII
    XxgMii                                                                 = 2392,
    /// YO
    Yo                                                                     = 2393,
    /// ZH
    Zh                                                                     = 2394,
    /// ZU
    Zu                                                                     = 2395,
    /// default
    default                                                                = 2396,
    /// preserve
    preserve                                                               = 2397,
}

impl EnumItem {
    const STRING_TABLE: [&'static str; 2398] = ["-500-MILES", "1000BASE-T", "1000BASE-T1", "100BASE-T1", "100BASE-TX", "10BASE-T1S", "AA", "AB", "ABSTRACT", "ABSTRACT-ACCESS-POINT", "ABSTRACT-CAN-CLUSTER", "ABSTRACT-CAN-COMMUNICATION-CONNECTOR", "ABSTRACT-CAN-COMMUNICATION-CONTROLLER", "ABSTRACT-CAN-PHYSICAL-CHANNEL", "ABSTRACT-CLASS-TAILORING", "ABSTRACT-DO-IP-LOGIC-ADDRESS-PROPS", "ABSTRACT-ETHERNET-FRAME", "ABSTRACT-EVENT", "ABSTRACT-EXECUTION-CONTEXT", "ABSTRACT-IAM-REMOTE-SUBJECT", "ABSTRACT-IMPLEMENTATION-DATA-TYPE", "ABSTRACT-IMPLEMENTATION-DATA-TYPE-ELEMENT", "ABSTRACT-PROVIDED-PORT-PROTOTYPE", "ABSTRACT-RAW-DATA-STREAM-INTERFACE", "ABSTRACT-REQUIRED-PORT-PROTOTYPE", "ABSTRACT-SECURITY-EVENT-FILTER", "ABSTRACT-SECURITY-IDSM-INSTANCE-FILTER", "ABSTRACT-SERVICE-INSTANCE", "ABSTRACT-SIGNAL-BASED-TO-I-SIGNAL-TRIGGERING-MAPPING", "ABSTRACT-SYNCHRONIZED-TIME-BASE-INTERFACE", "ACCEPT-ALL", "ACCEPT-CONFIGURED", "ACCES-PERRMISSION-SERVICE-CLASS", "ACCESS-PERMISSION-INSTANCE-OVERRIDES-CLASS", "ACCESS-PERMISSION-SERVICE-CLASS", "ACCESS-PERMISSION-SERVICE-INSTANCE", "ACK-WITH-RT", "ACK-WITHOUT-RT", "ACL-OBJECT-SET", "ACL-OPERATION", "ACL-PERMISSION", "ACL-ROLE", "ACTION", "ACTION-ITEM", "ACTION-LIST", "ACTIVATE", "ACTIVATION-AND-TRIGGER-UNICAST", "ACTIVATION-MULTICAST", "ACTIVATION-UNICAST", "ACTIVE", "ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE", "ADAPTIVE-AUTOSAR-APPLICATION", "ADAPTIVE-EVENT-RECEIVED", "ADAPTIVE-EVENT-SENT", "ADAPTIVE-FIELD-GETTER-CALLED", "ADAPTIVE-FIELD-GETTER-COMPLETED", "ADAPTIVE-FIELD-NOTIFICATION-RECEIVED", "ADAPTIVE-FIELD-NOTIFICATION-SENT", "ADAPTIVE-FIELD-SETTER-CALLED", "ADAPTIVE-FIELD-SETTER-COMPLETED", "ADAPTIVE-METHOD-CALL-RECEIVED", "ADAPTIVE-METHOD-CALLED", "ADAPTIVE-METHOD-RESPONSE-RECEIVED", "ADAPTIVE-METHOD-RESPONSE-SENT", "ADAPTIVE-MODULE-INSTANTIATION", "ADAPTIVE-PLATFORM-SERVICE-INSTANCE", "ADAPTIVE-SERVICE-FIND-COMPLETED", "ADAPTIVE-SERVICE-FIND-STARTED", "ADAPTIVE-SERVICE-OFFER-COMPLETED", "ADAPTIVE-SERVICE-OFFER-STARTED", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-COMPLETED", "ADAPTIVE-SERVICE-STOP-SUBSCRIPTION-STARTED", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-COMPLETED", "ADAPTIVE-SERVICE-SUBSCRIPTION-ACKNOWLEDGE-STARTED", "ADAPTIVE-SERVICE-SUBSCRIPTION-COMPLETED", "ADAPTIVE-SERVICE-SUBSCRIPTION-STARTED", "ADAPTIVE-SWC-INTERNAL-BEHAVIOR", "ADDR-METHOD-SHORT-NAME", "ADDR-METHOD-SHORT-NAME-AND-ALIGNMENT", "AF", "AFTER-SALES", "AFTERMAKET", "AFTERMARKET", "AGE", "AGE-CONSTRAINT", "AGGREGATION-TAILORING", "AGREED", "AH", "ALIAS-NAME-SET", "ALIVE-SUPERVISION", "ALL", "ALL-16-BIT", "ALL-INDICES-DIFFERENT-ARRAY-SIZE", "ALL-INDICES-SAME-ARRAY-SIZE", "ALL-SUPPORTED-DTCS", "ALLOCATOR", "ALTERNATING-8-BIT", "ALWAYS", "AM", "AMBER-WARNING", "ANALYZED-EXECUTION-TIME", "AND", "ANY", "ANY-SEND-OPERATION", "ANY-STANDARDIZED", "AP", "AP-APPLICATION-ENDPOINT", "AP-APPLICATION-ERROR", "AP-APPLICATION-ERROR-DOMAIN", "AP-APPLICATION-ERROR-SET", "AP-SOMEIP-TRANSFORMATION-PROPS", "API", "API-BASED", "API-USE", "APP-OS-TASK-PROXY-TO-ECU-TASK-PROXY-MAPPING", "APPLICATION", "APPLICATION-ACTION-ITEM", "APPLICATION-ARRAY-DATA-TYPE", "APPLICATION-ARRAY-ELEMENT", "APPLICATION-ASSOC-MAP-DATA-TYPE", "APPLICATION-ASSOC-MAP-ELEMENT", "APPLICATION-COMPOSITE-DATA-TYPE", "APPLICATION-COMPOSITE-ELEMENT-DATA-PROTOTYPE", "APPLICATION-DATA-TYPE", "APPLICATION-DEFERRED-DATA-TYPE", "APPLICATION-ENDPOINT", "APPLICATION-ERROR", "APPLICATION-INTERFACE", "APPLICATION-MODE-REQUEST-PHM-ACTION-ITEM", "APPLICATION-ONLY", "APPLICATION-PARTITION", "APPLICATION-PARTITION-TO-ECU-PARTITION-MAPPING", "APPLICATION-PRIMITIVE-DATA-TYPE", "APPLICATION-RECORD-DATA-TYPE", "APPLICATION-RECORD-ELEMENT", "APPLICATION-SW-COMPONENT-TYPE", "AR", "AR--CLIENT--SERVER", "AR-ELEMENT", "AR-PACKAGE", "ARBITRARY-EVENT-TRIGGERING", "ARBITRATION", "ARGUMENT-DATA-PROTOTYPE", "ARRAY", "ARTIFACT-CHECKSUM", "ARTIFACT-CHECKSUM-TO-CRYPTO-PROVIDER-MAPPING", "AS", "AS-IS", "ASSEMBLY-SW-CONNECTOR", "ASYMMETRIC-FROM-BYTE-ARRAY", "ASYMMETRIC-TO-BYTE-ARRAY", "ASYNCHRONOUS", "ASYNCHRONOUS-SERVER-CALL-POINT", "ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "ATOMIC-SW-COMPONENT-TYPE", "ATP-BLUEPRINT", "ATP-BLUEPRINTABLE", "ATP-CLASSIFIER", "ATP-DEFINITION", "ATP-FEATURE", "ATP-PROTOTYPE", "ATP-STRUCTURE-ELEMENT", "ATP-TYPE", "ATTRIBUTE-TAILORING", "AUTHENTICATE", "AUTO", "AUTO-IP", "AUTO-IP--DOIP", "AUTO-IPDHCPV-4", "AUTONOMOUS", "AUTOSAR-DATA-PROTOTYPE", "AUTOSAR-DATA-TYPE", "AUTOSAR-OPERATION-ARGUMENT-INSTANCE", "AUTOSAR-VARIABLE-INSTANCE", "AVB--IEEE-802--1-AS", "AY", "AZ", "BA", "BACKGROUND-EVENT", "BASE-T", "BASE-TYPE", "BASIC-SOFTWARE-MODE-MANAGER", "BE", "BG", "BH", "BI", "BIDIRECTIONAL", "BINARY-MANIFEST-ADDRESSABLE-OBJECT", "BINARY-MANIFEST-ITEM", "BINARY-MANIFEST-ITEM-DEFINITION", "BINARY-MANIFEST-META-DATA-FIELD", "BINARY-MANIFEST-PROVIDE-RESOURCE", "BINARY-MANIFEST-REQUIRE-RESOURCE", "BINARY-MANIFEST-RESOURCE", "BINARY-MANIFEST-RESOURCE-DEFINITION", "BLINK-MODE", "BLINK-OR-CONTINUOUS-ON-MODE", "BLOCK-SOURCE", "BLOCK-STATE", "BLUEPRINT-DERIVATION-TIME", "BLUEPRINT-MAPPING-SET", "BMP", "BN", "BO", "BOLD", "BOLDITALIC", "BONJOUR", "BOTTOM", "BR", "BREAK", "BRIEF", "BRIEF-BYPASSING-FILTERS", "BROAD-R-REACH", "BSW", "BSW-ASYNCHRONOUS-SERVER-CALL-POINT", "BSW-ASYNCHRONOUS-SERVER-CALL-RESULT-POINT", "BSW-ASYNCHRONOUS-SERVER-CALL-RETURNS-EVENT", "BSW-BACKGROUND-EVENT", "BSW-CALLED-ENTITY", "BSW-COMPOSITION-TIMING", "BSW-DATA-RECEIVED-EVENT", "BSW-DEBUG-INFO", "BSW-DIRECT-CALL-POINT", "BSW-DISTINGUISHED-PARTITION", "BSW-ENTRY-RELATIONSHIP-SET", "BSW-EVENT", "BSW-EXTERNAL-TRIGGER-OCCURRED-EVENT", "BSW-IMPLEMENTATION", "BSW-INTERNAL-BEHAVIOR", "BSW-INTERNAL-TRIGGER-OCCURRED-EVENT", "BSW-INTERNAL-TRIGGERING-POINT", "BSW-INTERRUPT-ENTITY", "BSW-M-ENTRY-CALL-RETURNED", "BSW-M-ENTRY-CALLED", "BSW-MGR-NEEDS", "BSW-MODE-MANAGER-ERROR-EVENT", "BSW-MODE-SWITCH-EVENT", "BSW-MODE-SWITCHED-ACK-EVENT", "BSW-MODULE-CALL-POINT", "BSW-MODULE-CLIENT-SERVER-ENTRY", "BSW-MODULE-DEPENDENCY", "BSW-MODULE-DESCRIPTION", "BSW-MODULE-ENTITY", "BSW-MODULE-ENTITY-ACTIVATED", "BSW-MODULE-ENTITY-STARTED", "BSW-MODULE-ENTITY-TERMINATED", "BSW-MODULE-ENTRY", "BSW-MODULE-TIMING", "BSW-OPERATION-INVOKED-EVENT", "BSW-OS-TASK-EXECUTION-EVENT", "BSW-SCHEDULABLE-ENTITY", "BSW-SCHEDULE-EVENT", "BSW-SCHEDULER-NAME-PREFIX", "BSW-SERVICE-DEPENDENCY-IDENT", "BSW-SYNCHRONOUS-SERVER-CALL-POINT", "BSW-TIMING-EVENT", "BSW-VARIABLE-ACCESS", "BUILD", "BUILD-ACTION", "BUILD-ACTION-ENTITY", "BUILD-ACTION-ENVIRONMENT", "BUILD-ACTION-MANIFEST", "BUILD-TYPE-DEBUG", "BUILD-TYPE-RELEASE", "BULK-NV-DATA-DESCRIPTOR", "BURST-PATTERN-EVENT-TRIGGERING", "BUS-MIRROR-CHANNEL-MAPPING", "BUS-MIRROR-CHANNEL-MAPPING-CAN", "BUS-MIRROR-CHANNEL-MAPPING-FLEXRAY", "BUS-MIRROR-CHANNEL-MAPPING-IP", "BUS-MIRROR-CHANNEL-MAPPING-USER-DEFINED", "C", "CA", "CALCULATED", "CALIBRATION-OFFLINE", "CALIBRATION-ONLINE", "CALIBRATION-PARAMETER-VALUE-SET", "CALIBRATION-VARIABLES", "CALLBACK", "CALLOUT", "CALPRM", "CAN-20", "CAN-BE-TERMINATED", "CAN-BE-TERMINATED-AND-RESTARTED", "CAN-CLUSTER", "CAN-COMMUNICATION-CONNECTOR", "CAN-COMMUNICATION-CONTROLLER", "CAN-FD", "CAN-FRAME", "CAN-FRAME-TRIGGERING", "CAN-NM-CLUSTER", "CAN-NM-NODE", "CAN-PHYSICAL-CHANNEL", "CAN-TP-ADDRESS", "CAN-TP-CHANNEL", "CAN-TP-CONFIG", "CAN-TP-NODE", "CANCEL", "CAPTION", "CAPTURE-ASYNCHRONOUS-TO-REPORTING", "CAPTURE-ASYNCHRONOUSLY-TO-REPORTING", "CAPTURE-SYNCHRONOUS-TO-REPORTING", "CAPTURE-SYNCHRONOUSLY-TO-REPORTING", "CAT-1", "CAT-2", "CAUTION", "CENTER", "CHANNEL-A", "CHANNEL-B", "CHAPTER", "CHECK-AT-NEXT-HALT", "CHECKPOINT-TRANSITION", "CIRCLE", "CLASS-CONTENT-CONDITIONAL", "CLASSIC", "CLEAR", "CLEAR-ALL-DTCS", "CLEAR-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "CLIENT-AUTHENTICATE", "CLIENT-DECRYPT", "CLIENT-ENCRYPT", "CLIENT-ID-DEFINITION", "CLIENT-ID-DEFINITION-SET", "CLIENT-MAC-GENERATE", "CLIENT-MAC-VERIFY", "CLIENT-SERVER-INTERFACE", "CLIENT-SERVER-INTERFACE-MAPPING", "CLIENT-SERVER-INTERFACE-TO-BSW-MODULE-ENTRY-BLUEPRINT-MAPPING", "CLIENT-SERVER-OPERATION", "CLIENT-VERIFY", "CLOSED", "CO", "CODE", "CODE-GENERATION-TIME", "CODEGENERATION", "COLDSTART", "COLLECTABLE-ELEMENT", "COLLECTION", "COM-AXIS", "COM-CERTIFICATE-TO-CRYPTO-CERTIFICATE-MAPPING", "COM-EVENT-GRANT", "COM-EVENT-GRANT-DESIGN", "COM-FIELD-GRANT", "COM-FIELD-GRANT-DESIGN", "COM-FIND-SERVICE-GRANT", "COM-FIND-SERVICE-GRANT-DESIGN", "COM-GRANT", "COM-GRANT-DESIGN", "COM-KEY-TO-CRYPTO-KEY-SLOT-MAPPING", "COM-MANAGEMENT-MAPPING", "COM-MANAGER", "COM-METHOD-GRANT", "COM-METHOD-GRANT-DESIGN", "COM-MGR-USER-NEEDS", "COM-OFFER-SERVICE-GRANT", "COM-OFFER-SERVICE-GRANT-DESIGN", "COM-SEC-OC-TO-CRYPTO-KEY-SLOT-MAPPING", "COM-TRIGGER-GRANT-DESIGN", "COMM-CONNECTOR-PORT", "COMMAND-LINE-LONG-FORM", "COMMAND-LINE-SHORT-FORM", "COMMAND-LINE-SIMPLE-FORM", "COMMON", "COMMUNICATION-CLUSTER", "COMMUNICATION-CONNECTOR", "COMMUNICATION-CONTROLLER", "COMMUNICATION-INTER-ECU", "COMMUNICATION-INTRA-PARTITION", "COMPILE", "COMPILER", "COMPLEX-DEVICE-DRIVER-SW-COMPONENT-TYPE", "COMPOSITE-INTERFACE", "COMPOSITION-P-PORT-TO-EXECUTABLE-P-PORT-MAPPING", "COMPOSITION-PORT-TO-EXECUTABLE-PORT-MAPPING", "COMPOSITION-R-PORT-TO-EXECUTABLE-R-PORT-MAPPING", "COMPOSITION-SW-COMPONENT-TYPE", "COMPU-METHOD", "COM_AXIS", "CONCRETE", "CONCRETE-CLASS-TAILORING", "CONCRETE-PATTERN-EVENT-TRIGGERING", "CONDITIONAL", "CONFIG-DATA", "CONFIGURED", "CONFIRMED", "CONFIRMED-DTC-BIT", "CONNECT", "CONSISTENCY-NEEDS", "CONSISTENCY-NEEDS-BLUEPRINT-SET", "CONSOLE", "CONST", "CONSTANT-SPECIFICATION", "CONSTANT-SPECIFICATION-MAPPING-SET", "CONSTRAINT-TAILORING", "CONSUMED-EVENT-GROUP", "CONSUMED-PROVIDED-SERVICE-INSTANCE-GROUP", "CONSUMED-SERVICE-INSTANCE", "CONSUMER", "CONTAINER-I-PDU", "CONTINUE-AT-IT-POSITION", "CONTINUOUS-ON-MODE", "COUPLING-ELEMENT", "COUPLING-PORT", "COUPLING-PORT-FIFO", "COUPLING-PORT-SCHEDULER", "COUPLING-PORT-SHAPER", "COUPLING-PORT-STRUCTURAL-ELEMENT", "COUPLING-PORT-TRAFFIC-CLASS-ASSIGNMENT", "CP", "CP-SOFTWARE-CLUSTER", "CP-SOFTWARE-CLUSTER-BINARY-MANIFEST-DESCRIPTOR", "CP-SOFTWARE-CLUSTER-COMMUNICATION-RESOURCE", "CP-SOFTWARE-CLUSTER-MAPPING-SET", "CP-SOFTWARE-CLUSTER-RESOURCE", "CP-SOFTWARE-CLUSTER-RESOURCE-POOL", "CP-SOFTWARE-CLUSTER-RESOURCE-TO-APPLICATION-PARTITION-MAPPING", "CP-SOFTWARE-CLUSTER-SERVICE-RESOURCE", "CP-SOFTWARE-CLUSTER-TO-ECU-INSTANCE-MAPPING", "CP-SOFTWARE-CLUSTER-TO-RESOURCE-MAPPING", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-DATA-ELEM-MAPPING", "CP-SW-CLUSTER-RESOURCE-TO-DIAG-FUNCTION-ID-MAPPING", "CP-SW-CLUSTER-TO-DIAG-EVENT-MAPPING", "CP-SW-CLUSTER-TO-DIAG-ROUTINE-SUBFUNCTION-MAPPING", "CPP", "CPP-IMPLEMENTATION-DATA-TYPE", "CPP-IMPLEMENTATION-DATA-TYPE-CONTEXT-TARGET", "CPP-IMPLEMENTATION-DATA-TYPE-ELEMENT", "CRC-IGNORED", "CRC-NOT-SUPPORTED", "CRC-NOT-VALIDATED", "CRC-OPTIONAL", "CRC-SUPPORTED", "CRC-VALIDATED", "CRYPTO-CERTIFICATE", "CRYPTO-CERTIFICATE-INTERFACE", "CRYPTO-CERTIFICATE-KEY-SLOT-NEEDS", "CRYPTO-CERTIFICATE-TO-PORT-PROTOTYPE-MAPPING", "CRYPTO-DRIVER", "CRYPTO-ELLIPTIC-CURVE-PROPS", "CRYPTO-INTERFACE", "CRYPTO-JOB", "CRYPTO-KEY-MANAGEMENT", "CRYPTO-KEY-MANAGEMENT-NEEDS", "CRYPTO-KEY-SLOT", "CRYPTO-KEY-SLOT-INTERFACE", "CRYPTO-KEY-SLOT-TO-PORT-PROTOTYPE-MAPPING", "CRYPTO-MODULE-INSTANTIATION", "CRYPTO-NEED", "CRYPTO-NEED-TO-CRYPTO-JOB-MAPPING", "CRYPTO-NEED-TO-PORT-PROTOTYPE-MAPPING", "CRYPTO-NEEDS", "CRYPTO-PRIMITIVE", "CRYPTO-PROVIDER", "CRYPTO-PROVIDER-INTERFACE", "CRYPTO-PROVIDER-TO-PORT-PROTOTYPE-MAPPING", "CRYPTO-SERVICE-CERTIFICATE", "CRYPTO-SERVICE-JOB-NEEDS", "CRYPTO-SERVICE-KEY", "CRYPTO-SERVICE-MANAGER", "CRYPTO-SERVICE-MAPPING", "CRYPTO-SERVICE-NEEDS", "CRYPTO-SERVICE-PRIMITIVE", "CRYPTO-SERVICE-QUEUE", "CRYPTO-SIGNATURE-SCHEME", "CRYPTO-TRUST-MASTER-INTERFACE", "CS", "CURVE-AXIS", "CURVE_AXIS", "CUSTOM", "CUSTOM-CPP-IMPLEMENTATION-DATA-TYPE", "CVC", "CY", "CYCLE-REPETITION-1", "CYCLE-REPETITION-10", "CYCLE-REPETITION-16", "CYCLE-REPETITION-2", "CYCLE-REPETITION-20", "CYCLE-REPETITION-32", "CYCLE-REPETITION-4", "CYCLE-REPETITION-40", "CYCLE-REPETITION-5", "CYCLE-REPETITION-50", "CYCLE-REPETITION-64", "CYCLE-REPETITION-8", "CYCLIC", "CYCLIC-AND-ON-CHANGE", "DA", "DATA-CONSTR", "DATA-EXCHANGE-POINT", "DATA-FORMAT-ELEMENT-REFERENCE", "DATA-FORMAT-ELEMENT-SCOPE", "DATA-INTERFACE", "DATA-PROTOTYPE", "DATA-PROTOTYPE-GROUP", "DATA-RECEIVE-ERROR-EVENT", "DATA-RECEIVED-EVENT", "DATA-SEND-COMPLETED-EVENT", "DATA-TRANSFORMATION", "DATA-TRANSFORMATION-SET", "DATA-TYPE-MAPPING-SET", "DATA-WRITE-COMPLETED-EVENT", "DCM-I-PDU", "DDS-DOMAIN-RANGE", "DDS-EVENT-DEPLOYMENT", "DDS-FIELD-DEPLOYMENT", "DDS-METHOD-DEPLOYMENT", "DDS-PROVIDED-SERVICE-INSTANCE", "DDS-REQUIRED-SERVICE-INSTANCE", "DDS-RPC-SERVICE-DEPLOYMENT", "DDS-SECURE-COM-PROPS", "DDS-SECURE-GOVERNANCE", "DDS-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "DDS-SERVICE-INTERFACE-DEPLOYMENT", "DDS-TOPIC-ACCESS-RULE", "DE", "DEADLINE-SUPERVISION", "DEBOUNCE-DATA", "DEBUG", "DECREASING", "DEDICATED", "DEF-ITEM", "DEFAULT", "DEFAULT-ERROR-TRACER", "DEFAULT-IF-REVISION-UPDATE", "DEFAULT-IF-UNDEFINED", "DEFAULT-MODE", "DEFAULT-TRACE-STATE-DISABLED", "DEFAULT-TRACE-STATE-ENABLED", "DEFAULT-TRIGGER", "DEFERRED", "DEFICIT-ROUND-ROBIN", "DEFINE-BY-IDENTIFIER", "DEFINE-BY-MEMORY-ADDRESS", "DEFLATE", "DELEGATION-SW-CONNECTOR", "DELETE", "DEPENDANT", "DEPENDENCY-ON-ARTIFACT", "DERIVED-FROM", "DESCENDANT", "DESELECTED", "DETAILED", "DETAILED-BYPASSING-FILTERS", "DETERMINISTIC-CLIENT", "DETERMINISTIC-CLIENT-RESOURCE-NEEDS", "DEVELOPMENT", "DEVELOPMENT-ERROR", "DEVELOPMENT-ERROR-TRACER", "DHCPV-4", "DHCPV-6", "DIAG-EVENT-DEBOUNCE-ALGORITHM", "DIAG-EVENT-DEBOUNCE-COUNTER-BASED", "DIAG-EVENT-DEBOUNCE-MONITOR-INTERNAL", "DIAG-EVENT-DEBOUNCE-TIME-BASED", "DIAG-REQUEST", "DIAG-RESPONSE", "DIAGNOSTIC-ABSTRACT-ALIAS-EVENT", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER", "DIAGNOSTIC-ABSTRACT-DATA-IDENTIFIER-INTERFACE", "DIAGNOSTIC-ABSTRACT-ROUTINE-INTERFACE", "DIAGNOSTIC-ACCESS-PERMISSION", "DIAGNOSTIC-AGING", "DIAGNOSTIC-AUTH-ROLE", "DIAGNOSTIC-AUTHENTICATION", "DIAGNOSTIC-AUTHENTICATION-CLASS", "DIAGNOSTIC-AUTHENTICATION-CONFIGURATION", "DIAGNOSTIC-AUTHENTICATION-INTERFACE", "DIAGNOSTIC-AUTHENTICATION-PORT-MAPPING", "DIAGNOSTIC-CAPABILITY-ELEMENT", "DIAGNOSTIC-CLEAR-CONDITION", "DIAGNOSTIC-CLEAR-CONDITION-GROUP", "DIAGNOSTIC-CLEAR-CONDITION-NEEDS", "DIAGNOSTIC-CLEAR-CONDITION-PORT-MAPPING", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION", "DIAGNOSTIC-CLEAR-DIAGNOSTIC-INFORMATION-CLASS", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO", "DIAGNOSTIC-CLEAR-RESET-EMISSION-RELATED-INFO-CLASS", "DIAGNOSTIC-COM-CONTROL", "DIAGNOSTIC-COM-CONTROL-CLASS", "DIAGNOSTIC-COM-CONTROL-INTERFACE", "DIAGNOSTIC-COMMON-ELEMENT", "DIAGNOSTIC-COMMUNICATION-MANAGER", "DIAGNOSTIC-COMMUNICATION-MANAGER-NEEDS", "DIAGNOSTIC-COMPONENT-NEEDS", "DIAGNOSTIC-CONDITION", "DIAGNOSTIC-CONDITION-GROUP", "DIAGNOSTIC-CONDITION-INTERFACE", "DIAGNOSTIC-CONNECTED-INDICATOR", "DIAGNOSTIC-CONNECTION", "DIAGNOSTIC-CONTRIBUTION-SET", "DIAGNOSTIC-CONTROL-DTC-SETTING", "DIAGNOSTIC-CONTROL-DTC-SETTING-CLASS", "DIAGNOSTIC-CONTROL-NEEDS", "DIAGNOSTIC-CUSTOM-SERVICE-CLASS", "DIAGNOSTIC-CUSTOM-SERVICE-INSTANCE", "DIAGNOSTIC-DATA-BY-IDENTIFIER", "DIAGNOSTIC-DATA-ELEMENT", "DIAGNOSTIC-DATA-ELEMENT-INTERFACE", "DIAGNOSTIC-DATA-IDENTIFIER", "DIAGNOSTIC-DATA-IDENTIFIER-GENERIC-INTERFACE", "DIAGNOSTIC-DATA-IDENTIFIER-INTERFACE", "DIAGNOSTIC-DATA-IDENTIFIER-SET", "DIAGNOSTIC-DATA-PORT-MAPPING", "DIAGNOSTIC-DATA-TRANSFER", "DIAGNOSTIC-DATA-TRANSFER-CLASS", "DIAGNOSTIC-DE-AUTHENTICATION", "DIAGNOSTIC-DEBOUNCE-ALGORITHM-PROPS", "DIAGNOSTIC-DEM-PROVIDED-DATA-MAPPING", "DIAGNOSTIC-DO-IP-ACTIVATION-LINE-INTERFACE", "DIAGNOSTIC-DO-IP-GROUP-IDENTIFICATION-INTERFACE", "DIAGNOSTIC-DO-IP-POWER-MODE-INTERFACE", "DIAGNOSTIC-DO-IP-TRIGGER-VEHICLE-ANNOUNCEMENT-INTERFACE", "DIAGNOSTIC-DOWNLOAD-INTERFACE", "DIAGNOSTIC-DTC-INFORMATION-INTERFACE", "DIAGNOSTIC-DYNAMIC-DATA-IDENTIFIER", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER", "DIAGNOSTIC-DYNAMICALLY-DEFINE-DATA-IDENTIFIER-CLASS", "DIAGNOSTIC-ECU-INSTANCE-PROPS", "DIAGNOSTIC-ECU-RESET", "DIAGNOSTIC-ECU-RESET-CLASS", "DIAGNOSTIC-ECU-RESET-INTERFACE", "DIAGNOSTIC-ENABLE-CONDITION", "DIAGNOSTIC-ENABLE-CONDITION-GROUP", "DIAGNOSTIC-ENABLE-CONDITION-NEEDS", "DIAGNOSTIC-ENABLE-CONDITION-PORT-MAPPING", "DIAGNOSTIC-ENV-BSW-MODE-ELEMENT", "DIAGNOSTIC-ENV-MODE-ELEMENT", "DIAGNOSTIC-ENV-SWC-MODE-ELEMENT", "DIAGNOSTIC-ENVIRONMENTAL-CONDITION", "DIAGNOSTIC-EVENT", "DIAGNOSTIC-EVENT-INFO-NEEDS", "DIAGNOSTIC-EVENT-INTERFACE", "DIAGNOSTIC-EVENT-MANAGER", "DIAGNOSTIC-EVENT-MANAGER-NEEDS", "DIAGNOSTIC-EVENT-NEEDS", "DIAGNOSTIC-EVENT-PORT-MAPPING", "DIAGNOSTIC-EVENT-TO-DEBOUNCE-ALGORITHM-MAPPING", "DIAGNOSTIC-EVENT-TO-ENABLE-CONDITION-GROUP-MAPPING", "DIAGNOSTIC-EVENT-TO-OPERATION-CYCLE-MAPPING", "DIAGNOSTIC-EVENT-TO-SECURITY-EVENT-MAPPING", "DIAGNOSTIC-EVENT-TO-STORAGE-CONDITION-GROUP-MAPPING", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-J-1939-MAPPING", "DIAGNOSTIC-EVENT-TO-TROUBLE-CODE-UDS-MAPPING", "DIAGNOSTIC-EXTENDED-DATA-RECORD", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-INTERFACE", "DIAGNOSTIC-EXTERNAL-AUTHENTICATION-PORT-MAPPING", "DIAGNOSTIC-FIM-ALIAS-EVENT", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP", "DIAGNOSTIC-FIM-ALIAS-EVENT-GROUP-MAPPING", "DIAGNOSTIC-FIM-ALIAS-EVENT-MAPPING", "DIAGNOSTIC-FIM-EVENT-GROUP", "DIAGNOSTIC-FIM-FUNCTION-MAPPING", "DIAGNOSTIC-FREEZE-FRAME", "DIAGNOSTIC-FUNCTION-IDENTIFIER", "DIAGNOSTIC-FUNCTION-IDENTIFIER-INHIBIT", "DIAGNOSTIC-FUNCTION-INHIBIT-SOURCE", "DIAGNOSTIC-GENERIC-UDS-INTERFACE", "DIAGNOSTIC-GENERIC-UDS-NEEDS", "DIAGNOSTIC-GENERIC-UDS-PORT-MAPPING", "DIAGNOSTIC-INDICATOR", "DIAGNOSTIC-INDICATOR-INTERFACE", "DIAGNOSTIC-INDICATOR-NEEDS", "DIAGNOSTIC-INDICATOR-PORT-MAPPING", "DIAGNOSTIC-INFO-TYPE", "DIAGNOSTIC-INHIBIT-SOURCE-EVENT-MAPPING", "DIAGNOSTIC-IO-CONTROL", "DIAGNOSTIC-IO-CONTROL-CLASS", "DIAGNOSTIC-IO-CONTROL-NEEDS", "DIAGNOSTIC-IUMPR", "DIAGNOSTIC-IUMPR-DENOMINATOR-GROUP", "DIAGNOSTIC-IUMPR-GROUP", "DIAGNOSTIC-IUMPR-TO-FUNCTION-IDENTIFIER-MAPPING", "DIAGNOSTIC-J-1939-EXPANDED-FREEZE-FRAME", "DIAGNOSTIC-J-1939-FREEZE-FRAME", "DIAGNOSTIC-J-1939-NODE", "DIAGNOSTIC-J-1939-SPN", "DIAGNOSTIC-J-1939-SPN-MAPPING", "DIAGNOSTIC-J-1939-SW-MAPPING", "DIAGNOSTIC-LOG-AND-TRACE", "DIAGNOSTIC-MAPPING", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING", "DIAGNOSTIC-MASTER-TO-SLAVE-EVENT-MAPPING-SET", "DIAGNOSTIC-MEASUREMENT-IDENTIFIER", "DIAGNOSTIC-MEMORY-ADDRESSABLE-RANGE-ACCESS", "DIAGNOSTIC-MEMORY-BY-ADDRESS", "DIAGNOSTIC-MEMORY-DESTINATION", "DIAGNOSTIC-MEMORY-DESTINATION-MIRROR", "DIAGNOSTIC-MEMORY-DESTINATION-PORT-MAPPING", "DIAGNOSTIC-MEMORY-DESTINATION-PRIMARY", "DIAGNOSTIC-MEMORY-DESTINATION-USER-DEFINED", "DIAGNOSTIC-MEMORY-IDENTIFIER", "DIAGNOSTIC-MONITOR-INTERFACE", "DIAGNOSTIC-MONITOR-PORT-MAPPING", "DIAGNOSTIC-OPERATION-CYCLE", "DIAGNOSTIC-OPERATION-CYCLE-INTERFACE", "DIAGNOSTIC-OPERATION-CYCLE-NEEDS", "DIAGNOSTIC-OPERATION-CYCLE-PORT-MAPPING", "DIAGNOSTIC-PARAMETER-IDENTIFIER", "DIAGNOSTIC-PORT-INTERFACE", "DIAGNOSTIC-POWERTRAIN-FREEZE-FRAME", "DIAGNOSTIC-PROOF-OF-OWNERSHIP", "DIAGNOSTIC-PROTOCOL", "DIAGNOSTIC-PROVIDED-DATA-MAPPING", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER", "DIAGNOSTIC-READ-DATA-BY-IDENTIFIER-CLASS", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID", "DIAGNOSTIC-READ-DATA-BY-PERIODIC-ID-CLASS", "DIAGNOSTIC-READ-DTC-INFORMATION", "DIAGNOSTIC-READ-DTC-INFORMATION-CLASS", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS", "DIAGNOSTIC-READ-MEMORY-BY-ADDRESS-CLASS", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER", "DIAGNOSTIC-READ-SCALING-DATA-BY-IDENTIFIER-CLASS", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE", "DIAGNOSTIC-REQUEST-CONTROL-OF-ON-BOARD-DEVICE-CLASS", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA", "DIAGNOSTIC-REQUEST-CURRENT-POWERTRAIN-DATA-CLASS", "DIAGNOSTIC-REQUEST-DOWNLOAD", "DIAGNOSTIC-REQUEST-DOWNLOAD-CLASS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-CLASS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS", "DIAGNOSTIC-REQUEST-EMISSION-RELATED-DTC-PERMANENT-STATUS-CLASS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-CLASS", "DIAGNOSTIC-REQUEST-FILE-TRANSFER-NEEDS", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS", "DIAGNOSTIC-REQUEST-ON-BOARD-MONITORING-TEST-RESULTS-CLASS", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA", "DIAGNOSTIC-REQUEST-POWERTRAIN-FREEZE-FRAME-DATA-CLASS", "DIAGNOSTIC-REQUEST-ROUTINE-RESULTS", "DIAGNOSTIC-REQUEST-UPLOAD", "DIAGNOSTIC-REQUEST-UPLOAD-CLASS", "DIAGNOSTIC-REQUEST-VEHICLE-INFO", "DIAGNOSTIC-REQUEST-VEHICLE-INFO-CLASS", "DIAGNOSTIC-RESPONSE-ON-EVENT", "DIAGNOSTIC-RESPONSE-ON-EVENT-CLASS", "DIAGNOSTIC-RESPONSE-ON-EVENT-NEEDS", "DIAGNOSTIC-ROUTINE", "DIAGNOSTIC-ROUTINE-CONTROL", "DIAGNOSTIC-ROUTINE-CONTROL-CLASS", "DIAGNOSTIC-ROUTINE-GENERIC-INTERFACE", "DIAGNOSTIC-ROUTINE-INTERFACE", "DIAGNOSTIC-ROUTINE-NEEDS", "DIAGNOSTIC-ROUTINE-SUBFUNCTION", "DIAGNOSTIC-SECURITY-ACCESS", "DIAGNOSTIC-SECURITY-ACCESS-CLASS", "DIAGNOSTIC-SECURITY-EVENT-REPORTING-MODE-MAPPING", "DIAGNOSTIC-SECURITY-LEVEL", "DIAGNOSTIC-SECURITY-LEVEL-INTERFACE", "DIAGNOSTIC-SECURITY-LEVEL-PORT-MAPPING", "DIAGNOSTIC-SERVICE-CLASS", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-MAPPING", "DIAGNOSTIC-SERVICE-DATA-IDENTIFIER-PORT-MAPPING", "DIAGNOSTIC-SERVICE-DATA-MAPPING", "DIAGNOSTIC-SERVICE-GENERIC-MAPPING", "DIAGNOSTIC-SERVICE-INSTANCE", "DIAGNOSTIC-SERVICE-SW-MAPPING", "DIAGNOSTIC-SERVICE-TABLE", "DIAGNOSTIC-SERVICE-VALIDATION-INTERFACE", "DIAGNOSTIC-SERVICE-VALIDATION-MAPPING", "DIAGNOSTIC-SESSION", "DIAGNOSTIC-SESSION-CONTROL", "DIAGNOSTIC-SESSION-CONTROL-CLASS", "DIAGNOSTIC-SOFTWARE-CLUSTER-PROPS", "DIAGNOSTIC-START-ROUTINE", "DIAGNOSTIC-STOP-ROUTINE", "DIAGNOSTIC-STORAGE-CONDITION", "DIAGNOSTIC-STORAGE-CONDITION-GROUP", "DIAGNOSTIC-STORAGE-CONDITION-NEEDS", "DIAGNOSTIC-STORAGE-CONDITION-PORT-MAPPING", "DIAGNOSTIC-SW-MAPPING", "DIAGNOSTIC-TEST-RESULT", "DIAGNOSTIC-TEST-ROUTINE-IDENTIFIER", "DIAGNOSTIC-TRANSFER-EXIT", "DIAGNOSTIC-TRANSFER-EXIT-CLASS", "DIAGNOSTIC-TROUBLE-CODE", "DIAGNOSTIC-TROUBLE-CODE-GROUP", "DIAGNOSTIC-TROUBLE-CODE-J-1939", "DIAGNOSTIC-TROUBLE-CODE-OBD", "DIAGNOSTIC-TROUBLE-CODE-PROPS", "DIAGNOSTIC-TROUBLE-CODE-UDS", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-CLEAR-CONDITION-GROUP-MAPPING", "DIAGNOSTIC-TROUBLE-CODE-UDS-TO-TROUBLE-CODE-OBD-MAPPING", "DIAGNOSTIC-UPLOAD-DOWNLOAD-NEEDS", "DIAGNOSTIC-UPLOAD-DOWNLOAD-PORT-MAPPING", "DIAGNOSTIC-UPLOAD-INTERFACE", "DIAGNOSTIC-VALUE-NEEDS", "DIAGNOSTIC-VERIFY-CERTIFICATE-BIDIRECTIONAL", "DIAGNOSTIC-VERIFY-CERTIFICATE-UNIDIRECTIONAL", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER", "DIAGNOSTIC-WRITE-DATA-BY-IDENTIFIER-CLASS", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS", "DIAGNOSTIC-WRITE-MEMORY-BY-ADDRESS-CLASS", "DIAGNOSTICS-COMMUNICATION-SECURITY-NEEDS", "DISABLE", "DLNA", "DLT-APPLICATION", "DLT-APPLICATION-TO-PROCESS-MAPPING", "DLT-ARGUMENT", "DLT-CONTEXT", "DLT-ECU", "DLT-LOG-CHANNEL", "DLT-LOG-CHANNEL-DESIGN", "DLT-LOG-CHANNEL-DESIGN-TO-PROCESS-DESIGN-MAPPING", "DLT-LOG-CHANNEL-TO-PROCESS-MAPPING", "DLT-LOG-SINK", "DLT-LOG-SINK-TO-PORT-PROTOTYPE-MAPPING", "DLT-MESSAGE", "DLT-MESSAGE-COLLECTION-SET", "DLT-USER-NEEDS", "DO-IP", "DO-IP-ACTIVATION-LINE-NEEDS", "DO-IP-GID-NEEDS", "DO-IP-GID-SYNCHRONIZATION-NEEDS", "DO-IP-INSTANTIATION", "DO-IP-INTERFACE", "DO-IP-LOGIC-ADDRESS", "DO-IP-LOGIC-TARGET-ADDRESS-PROPS", "DO-IP-LOGIC-TESTER-ADDRESS-PROPS", "DO-IP-POWER-MODE-STATUS-NEEDS", "DO-IP-ROUTING-ACTIVATION", "DO-IP-ROUTING-ACTIVATION-AUTHENTICATION-NEEDS", "DO-IP-ROUTING-ACTIVATION-CONFIRMATION-NEEDS", "DO-IP-SERVICE-NEEDS", "DO-IP-TP-CONFIG", "DOCUMENT-ELEMENT-SCOPE", "DOCUMENTATION", "DOCUMENTATION-CONTEXT", "DOES-NOT-REPORT-EXECUTION-STATE", "DOES-NOT-SUPPORT-BUFFER-LOCKING", "DOES-NOT-USE-LOGGING", "DOMAIN-PARTICIPANT-USER-DATA-QOS", "DONT-INVALIDATE", "DROP", "DROP-FRAME", "DROP-UNTAGGED", "DSA", "DTC-STATUS-CHANGE-NOTIFICATION-NEEDS", "DYNAMIC-PART-TRIGGER", "DZ", "E-2-E-PROFILE-COMPATIBILITY-PROPS", "E-2-E-PROFILE-CONFIGURATION", "E-2-E-PROFILE-CONFIGURATION-SET", "ECC", "ECU", "ECU-ABSTRACTION-SW-COMPONENT-TYPE", "ECU-INSTANCE", "ECU-MANAGER", "ECU-MAPPING", "ECU-PARTITION", "ECU-STATE-MGR-USER-NEEDS", "ECU-TIMING", "ECUC-ABSTRACT-EXTERNAL-REFERENCE-DEF", "ECUC-ABSTRACT-INTERNAL-REFERENCE-DEF", "ECUC-ABSTRACT-REFERENCE-DEF", "ECUC-ABSTRACT-STRING-PARAM-DEF", "ECUC-ADD-INFO-PARAM-DEF", "ECUC-BOOLEAN-PARAM-DEF", "ECUC-CHOICE-CONTAINER-DEF", "ECUC-CHOICE-REFERENCE-DEF", "ECUC-COMMON-ATTRIBUTES", "ECUC-CONTAINER-DEF", "ECUC-CONTAINER-VALUE", "ECUC-DEFINITION-COLLECTION", "ECUC-DEFINITION-ELEMENT", "ECUC-DESTINATION-URI-DEF", "ECUC-DESTINATION-URI-DEF-SET", "ECUC-ENUMERATION-LITERAL-DEF", "ECUC-ENUMERATION-PARAM-DEF", "ECUC-FLOAT-PARAM-DEF", "ECUC-FOREIGN-REFERENCE-DEF", "ECUC-FUNCTION-NAME-DEF", "ECUC-INSTANCE-REFERENCE-DEF", "ECUC-INTEGER-PARAM-DEF", "ECUC-LINKER-SYMBOL-DEF", "ECUC-MODULE-CONFIGURATION-VALUES", "ECUC-MODULE-DEF", "ECUC-MULTILINE-STRING-PARAM-DEF", "ECUC-PARAM-CONF-CONTAINER-DEF", "ECUC-PARAMETER-DEF", "ECUC-QUERY", "ECUC-QUERY-EXPRESSION", "ECUC-REFERENCE-DEF", "ECUC-STRING-PARAM-DEF", "ECUC-SYMBOLIC-NAME-REFERENCE-DEF", "ECUC-URI-REFERENCE-DEF", "ECUC-VALIDATION-CONDITION", "ECUC-VALUE-COLLECTION", "EDGE-NODE", "EL", "EMISSION-RELATED-DTC", "EN", "ENABLE", "ENABLED", "ENCRYPT-AND-SIGN", "ENCRYPT-AND-SIGN-WITH-ORIGIN-AUTHENTICATION", "ENCRYPTION", "END-2-END-EVENT-PROTECTION-PROPS", "END-2-END-METHOD-PROTECTION-PROPS", "END-TO-END-PROTECTION", "END-TO-END-PROTECTION-I-SIGNAL-I-PDU", "END-TO-END-PROTECTION-SET", "END-TO-END-PROTECTION-VARIABLE-PROTOTYPE", "ENHANCED", "ENUMERATION-MAPPING-TABLE", "EO", "EOC-EVENT-REF", "EOC-EXECUTABLE-ENTITY-REF", "EOC-EXECUTABLE-ENTITY-REF-ABSTRACT", "EOC-EXECUTABLE-ENTITY-REF-GROUP", "EPS", "EQUAL", "ERROR", "ERROR-CORRECTION", "ERROR-DETECTION", "ERROR-TRACER", "ERROR-TRACER-NEEDS", "ES", "ESP", "ET", "ETH-IP-PROPS", "ETH-TCP-IP-ICMP-PROPS", "ETH-TCP-IP-PROPS", "ETH-TP-CONFIG", "ETHERNET-CLUSTER", "ETHERNET-COMMUNICATION-CONNECTOR", "ETHERNET-COMMUNICATION-CONTROLLER", "ETHERNET-FRAME", "ETHERNET-FRAME-TRIGGERING", "ETHERNET-NETWORK-CONFIGURATION", "ETHERNET-PHYSICAL-CHANNEL", "ETHERNET-PRIORITY-REGENERATION", "ETHERNET-RAW-DATA-STREAM-CLIENT-MAPPING", "ETHERNET-RAW-DATA-STREAM-GRANT", "ETHERNET-RAW-DATA-STREAM-MAPPING", "ETHERNET-RAW-DATA-STREAM-SERVER-MAPPING", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG", "ETHERNET-WAKEUP-SLEEP-ON-DATALINE-CONFIG-SET", "EU", "EVALUATED-VARIANT-SET", "EVAP", "EVENT-ACCEPTANCE-DISABLED", "EVENT-ACCEPTANCE-ENABLED", "EVENT-COMBINATION-ON-RETRIEVAL", "EVENT-COMBINATION-ON-STORAGE", "EVENT-HANDLER", "EVENT-MAPPING", "EVENT-STORAGE-DISABLED", "EVENT-STORAGE-ENABLED", "EVENT-TRIGGERING-CONSTRAINT", "EVENT-WINDOW-CURRENT-AND-FOLLOWING-CYCLE", "EVENT-WINDOW-CURRENT-CYCLE", "EVENT-WINDOW-INFINITE", "EXACT-OR-ANY-MINOR-VERSION", "EXAMPLE", "EXCLUDE-FROM-FLASH", "EXCLUSIVE", "EXCLUSIVE-AREA", "EXCLUSIVE-AREA-NESTING-ORDER", "EXECUTABLE", "EXECUTABLE-ENTITY", "EXECUTABLE-ENTITY-ACTIVATION-REASON", "EXECUTABLE-GROUP", "EXECUTABLE-TIMING", "EXECUTE", "EXECUTION-ORDER-CONSTRAINT", "EXECUTION-TIME", "EXECUTION-TIME-CONSTRAINT", "EXERCISE", "EXPLICIT", "EXTENDED", "EXTERNAL-REPLACEMENT", "EXTERNAL-TRIGGER-OCCURRED-EVENT", "EXTERNAL-TRIGGERING-POINT-IDENT", "FA", "FAILURE-AND-SUCCESS", "FAILURE-ONLY", "FALSE", "FAST-FLASHING-MODE", "FATAL", "FAULT", "FDC-THRESHOLD", "FI", "FIBEX-ELEMENT", "FIELD", "FIELD-MAPPING", "FILE", "FILTERED", "FINISH", "FIRE-AND-FORGET-MAPPING", "FIRST-CONTAINED-TRIGGER", "FIRST-TO-SECOND", "FIT-TO-PAGE", "FIT-TO-TEXT", "FIX-AXIS", "FIXED", "FIXED-SIZE", "FIX_AXIS", "FJ", "FLAT-INSTANCE-DESCRIPTOR", "FLAT-MAP", "FLEXRAY-AR-TP-CONFIG", "FLEXRAY-AR-TP-NODE", "FLEXRAY-CLUSTER", "FLEXRAY-COMMUNICATION-CONNECTOR", "FLEXRAY-COMMUNICATION-CONTROLLER", "FLEXRAY-FRAME", "FLEXRAY-FRAME-TRIGGERING", "FLEXRAY-NM-CLUSTER", "FLEXRAY-NM-NODE", "FLEXRAY-PHYSICAL-CHANNEL", "FLEXRAY-TP-CONFIG", "FLEXRAY-TP-CONNECTION-CONTROL", "FLEXRAY-TP-NODE", "FLEXRAY-TP-PDU-POOL", "FLOAT", "FM-ATTRIBUTE-DEF", "FM-FEATURE", "FM-FEATURE-MAP", "FM-FEATURE-MAP-ASSERTION", "FM-FEATURE-MAP-CONDITION", "FM-FEATURE-MAP-ELEMENT", "FM-FEATURE-MODEL", "FM-FEATURE-RELATION", "FM-FEATURE-RESTRICTION", "FM-FEATURE-SELECTION", "FM-FEATURE-SELECTION-SET", "FO", "FOR-ALL", "FORGET", "FORWARD-AS-IS", "FR", "FRAME", "FRAME-ETHERNET-QUEUED-FOR-TRANSMISSION", "FRAME-ETHERNET-RECEIVED-BY-IF", "FRAME-ETHERNET-RECEIVED-ON-BUS", "FRAME-ETHERNET-SENT-ON-BUS", "FRAME-PORT", "FRAME-QUEUED-FOR-TRANSMISSION", "FRAME-RECEIVED-BY-IF", "FRAME-TRANSMITTED-ON-BUS", "FRAME-TRIGGERING", "FULL", "FULL-DUPLEX-MODE", "FUNCTION-GROUP-MODE-REQUEST-PHM-ACTION-ITEM", "FUNCTION-GROUP-SET", "FUNCTION-INHIBITION-AVAILABILITY-NEEDS", "FUNCTION-INHIBITION-MANAGER", "FUNCTION-INHIBITION-NEEDS", "FUNCTIONAL", "FUNCTIONAL-ADDRESS", "FUNCTIONAL-CAN-FD", "FUNCTIONAL-CLUSTER-INTERACTS-WITH-FUNCTIONAL-CLUSTER-MAPPING", "FURTHER-ACTION-BYTE-NEEDS", "FY", "GA", "GATEWAY", "GD", "GENERAL-PARAMETER", "GENERAL-PURPOSE-CONNECTION", "GENERAL-PURPOSE-I-PDU", "GENERAL-PURPOSE-PDU", "GENERIC-ETHERNET-FRAME", "GENERIC-MODULE-INSTANTIATION", "GET", "GETTER", "GETTER-SETTER", "GIF", "GL", "GLOBAL-SUPERVISION", "GLOBAL-SUPERVISION-ENTITY", "GLOBAL-SUPERVISION-NEEDS", "GLOBAL-TIME-CAN-MASTER", "GLOBAL-TIME-CAN-SLAVE", "GLOBAL-TIME-DOMAIN", "GLOBAL-TIME-ETH-MASTER", "GLOBAL-TIME-ETH-SLAVE", "GLOBAL-TIME-FR-MASTER", "GLOBAL-TIME-FR-SLAVE", "GLOBAL-TIME-GATEWAY", "GLOBAL-TIME-MASTER", "GLOBAL-TIME-SLAVE", "GN", "GRANT", "GRANT-DESIGN", "GROSS", "GU", "GZIP", "HA", "HALF-DUPLEX-MODE", "HARDWARE-TEST-MANAGER", "HARDWARE-TEST-NEEDS", "HEAD", "HEALTH-CHANNEL", "HEALTH-CHANNEL-EXTERNAL-MODE", "HEALTH-CHANNEL-EXTERNAL-STATUS", "HEALTH-CHANNEL-SUPERVISION", "HEAP-USAGE", "HI", "HIERARCHICAL-EOC", "HIGH", "HINT", "HOOK", "HOST-PORT", "HR", "HU", "HUB", "HW-ATTRIBUTE-DEF", "HW-ATTRIBUTE-LITERAL-DEF", "HW-CATEGORY", "HW-DESCRIPTION-ENTITY", "HW-ELEMENT", "HW-PIN", "HW-PIN-GROUP", "HW-TYPE", "HY", "I-4-G", "I-PDU", "I-PDU-PORT", "I-PDU-RECEIVED-BY-COM", "I-PDU-SENT-TO-IF", "I-PDU-TRIGGERING", "I-PV-6-EXT-HEADER-FILTER-LIST", "I-PV-6-EXT-HEADER-FILTER-SET", "I-SIGNAL", "I-SIGNAL-AVAILABLE-FOR-RTE", "I-SIGNAL-GROUP", "I-SIGNAL-I-PDU", "I-SIGNAL-I-PDU-GROUP", "I-SIGNAL-PORT", "I-SIGNAL-SENT-TO-COM", "I-SIGNAL-TO-I-PDU-MAPPING", "I-SIGNAL-TRIGGERING", "IA", "IAM-MODULE-INSTANTIATION", "ICMP", "IDENT-CAPTION", "IDENTIFIABLE", "IDS-COMMON-ELEMENT", "IDS-DESIGN", "IDS-MAPPING", "IDS-MGR-CUSTOM-TIMESTAMP-NEEDS", "IDS-MGR-NEEDS", "IDS-PLATFORM-INSTANTIATION", "IDSM-INSTANCE", "IDSM-MODULE-INSTANTIATION", "IDSM-PROPERTIES", "IDSM-RATE-LIMITATION", "IDSM-TRAFFIC-LIMITATION", "IE", "IEEE-1722-TP-ETHERNET-FRAME", "IEEE802-11P", "IEEE802-1AS", "IEEE802-1AS-AUTOSAR", "IGNITION", "IGNORE", "IK", "IMMEDIATE", "IMMEDIATELY", "IMPLEMENTATION", "IMPLEMENTATION-DATA-TYPE", "IMPLEMENTATION-DATA-TYPE-ELEMENT", "IMPLEMENTATION-DATA-TYPE-ELEMENT-EXTENSION", "IMPLEMENTATION-DATA-TYPE-EXTENSION", "IMPLEMENTATION-PROPS", "IN", "INCREASING", "INDICATE", "INDICATOR-STATUS-NEEDS", "INDIVIDUAL", "INFINITE", "INFINITE-TIME-TO-RESPONSE", "INFO", "INHERITED-FROM-ARRAY-ELEMENT-TYPE-SIZE", "INIT-EVENT", "INLINE", "INLINE-CONDITIONAL", "INOUT", "INSTALL", "INSTANCE-ID", "INSTRUCTION", "INTER-PARTITION-INTRA-ECU", "INTERFACE-MAPPING", "INTERFACE-MAPPING-SET", "INTERNAL-BEHAVIOR", "INTERNAL-TRIGGER-OCCURRED-EVENT", "INTERNAL-TRIGGERING-POINT", "INTERPOLATION-ROUTINE-MAPPING-SET", "INTERRUPT", "INTERRUPT-CAT-1", "INTERRUPT-CAT-2", "INTRUSION-DETECTION-SECURITY-MANAGEMENT", "INVALID", "IP-IAM-REMOTE-SUBJECT", "IP-SEC-CONFIG-PROPS", "IP-SEC-IAM-REMOTE-SUBJECT", "IP-SEC-RULE", "IPSEC", "IS", "IS-EQUAL", "IS-EXPIRED", "IS-FAILED", "IS-GREATER-OR-EQUAL", "IS-GREATER-THAN", "IS-GREATER-THAN-OR-EQUAL", "IS-LESS-OR-EQUAL", "IS-LESS-THAN", "IS-LESS-THAN-OR-EQUAL", "IS-NOT-EQUAL", "IS-NOT-RELEVANT", "IS-OK", "IS-RELEVANT", "IS-STOPPED", "ISO", "ISO-11992--4", "ISO-14229--1", "ISO-15031--6", "ISO-6", "IT", "ITALIC", "IW", "J-1939", "J-1939-CLUSTER", "J-1939-CONTROLLER-APPLICATION", "J-1939-DCM", "J-1939-DCM-DM-19-SUPPORT", "J-1939-DCM-I-PDU", "J-1939-NM-CLUSTER", "J-1939-NM-NODE", "J-1939-REQUEST-MANAGER", "J-1939-RM-INCOMING-REQUEST-SERVICE-NEEDS", "J-1939-RM-OUTGOING-REQUEST-SERVICE-NEEDS", "J-1939-SHARED-ADDRESS-CLUSTER", "J-1939-TP-CONFIG", "J-1939-TP-NODE", "JA", "JAVA", "JI", "JPG", "JUSTIFY", "JW", "KA", "KEEP", "KEEP-EXISTING", "KEY-DERIVATION", "KEY-STORAGE", "KEYWORD", "KEYWORD-SET", "KK", "KL", "KM", "KN", "KO", "KS", "KU", "KY", "LA", "LAND", "LAST-FAILED", "LAST-IS-BEST", "LAST-MODE", "LATENCY-TIMING-CONSTRAINT", "LEAF-OF-TARGET-CONTAINER", "LEFT", "LEGACY", "LIFE-CYCLE-INFO-SET", "LIFE-CYCLE-STATE", "LIFE-CYCLE-STATE-DEFINITION-GROUP", "LIMIT-TO-PAGE", "LIMIT-TO-TEXT", "LIN-CLUSTER", "LIN-COMMUNICATION-CONNECTOR", "LIN-COMMUNICATION-CONTROLLER", "LIN-EVENT-TRIGGERED-FRAME", "LIN-FRAME", "LIN-FRAME-TRIGGERING", "LIN-MASTER", "LIN-NM-CLUSTER", "LIN-PHYSICAL-CHANNEL", "LIN-SCHEDULE-TABLE", "LIN-SLAVE", "LIN-SLAVE-CONFIG-IDENT", "LIN-SPORADIC-FRAME", "LIN-TP-CONFIG", "LIN-TP-NODE", "LIN-UNCONDITIONAL-FRAME", "LINK", "LINK-LOCAL", "LINK-LOCAL--DOIP", "LINK-TIME", "LINKER", "LISTEN", "LN", "LO", "LOCAL", "LOCAL-SUPERVISION", "LOG-AND-TRACE-INSTANTIATION", "LOG-AND-TRACE-INTERFACE", "LOG-AND-TRACE-MESSAGE-COLLECTION-SET", "LOGIC-ADDRESS", "LOGICAL-AND", "LOGICAL-EXPRESSION", "LOGICAL-OR", "LOGICAL-SUPERVISION", "LONG-HEADER", "LOW", "LOWER-12-BIT", "LOWER-8-BIT", "LT", "LT-AFFECTS-PB", "LTS-13", "LV", "MAC-MULTICAST-GROUP", "MACHINE", "MACHINE-DESIGN", "MACHINE-MODE-REQUEST-PHM-ACTION-ITEM", "MACHINE-TIMING", "MACRO", "MAINTENANCE-ONLY", "MALFUNCTION", "MANUFACTURING", "MAPPING-SCOPE-CORE", "MAPPING-SCOPE-ECU", "MAPPING-SCOPE-PARTITION", "MASEKD-NEW-EQUALS-MASKED-OLD", "MASEKD-NEW-EQUALS-X", "MASKED-NEW-DIFFERS-MASKED-OLD", "MASKED-NEW-DIFFERS-X", "MASKED-NEW-EQUALS-MASKED-OLD", "MASKED-NEW-EQUALS-X", "MASTER", "MASTER-ECU", "MAX", "MC-DATA-INSTANCE", "MC-FUNCTION", "MC-GROUP", "MEASURED-EXECUTION-TIME", "MEASURED-HEAP-USAGE", "MEASURED-STACK-USAGE", "MEASUREMENT-POINT", "MEDIUM", "MEMORY-SECTION", "METHOD-MAPPING", "MG", "MI", "MIDDLE", "MIN", "MINIMUM-MINOR-VERSION", "MIXED", "MIXED-29-BIT", "MK", "ML", "MN", "MO", "MODE-ACCESS-POINT-IDENT", "MODE-DECLARATION", "MODE-DECLARATION-GROUP", "MODE-DECLARATION-GROUP-PROTOTYPE", "MODE-DECLARATION-MAPPING", "MODE-DECLARATION-MAPPING-SET", "MODE-DECLARATION-REQUESTED", "MODE-DECLARATION-SWITCH-COMPLETED", "MODE-DECLARATION-SWITCH-INITIATED", "MODE-INTERFACE-MAPPING", "MODE-SWITCH-INTERFACE", "MODE-SWITCH-POINT", "MODE-SWITCHED-ACK-EVENT", "MODE-TRANSITION", "MODELED", "MONITOR-MODE", "MONO", "MONOTONOUS", "MOST-SIGNIFICANT-BYTE-FIRST", "MOST-SIGNIFICANT-BYTE-LAST", "MR", "MS", "MT", "MULTICORE-REENTRANT", "MULTILANGUAGE-REFERRABLE", "MULTIPLE", "MULTIPLE-OCCURRENCES", "MULTIPLEXED-I-PDU", "MY", "N-PDU", "NA", "NAND", "NE", "NET", "NETWORK", "NETWORK-CONFIGURATION", "NETWORK-ENDPOINT", "NETWORK-REPRESENTATION-FROM-COM-SPEC", "NEVER", "NEW-IS-DIFFERENT", "NEW-IS-EQUAL", "NEW-IS-GREATER", "NEW-IS-GREATER-OR-EQUAL", "NEW-IS-LESS", "NEW-IS-LESS-OR-EQUAL", "NEW-IS-OUTSIDE", "NEW-IS-WITHIN", "NEWLINE", "NEWLINE-IF-NECESSARY", "NFOLD", "NL", "NM-CLUSTER", "NM-CONFIG", "NM-ECU", "NM-HANDLE-TO-FUNCTION-GROUP-STATE-MAPPING", "NM-INSTANTIATION", "NM-NETWORK-HANDLE", "NM-NODE", "NM-PDU", "NO", "NO-ACK", "NO-AFFECT", "NO-BOOT", "NO-BREAK", "NO-DEFAULT", "NO-FLOAT", "NO-HEADER", "NO-KEEP", "NO-MONOTONY", "NO-NEWLINE", "NO-OBD-SUPPORT", "NO-PGWIDE", "NO-PROTECTION", "NO-RETURN-VALUE-PROVIDED", "NO-SEVERITY", "NO-SHOW-ALIAS-NAME", "NO-SHOW-CATEGORY", "NO-SHOW-CONTENT", "NO-SHOW-LONG-NAME", "NO-SHOW-NUMBER", "NO-SHOW-PAGE", "NO-SHOW-SEE", "NO-SHOW-SHORT-NAME", "NO-SHOW-TYPE", "NO-SLOPPY", "NO-STATUS-BYTE-CHANGE", "NO-STORE-EVENT", "NO-SUPERVISION", "NO-SUPPORT", "NO-TRANSFORMER-ERROR-HANDLING", "NO-TRANSFORMER-STATUS-FORWARDING", "NO-TRUSTED-PLATFORM-SUPPORT", "NODE", "NOHREF", "NON-EMMISSION-RELATED-DTC", "NON-OS-MODULE-INSTANTIATION", "NON-REENTRANT", "NON-VOLATILE", "NON-VOLATILE-RAM-MANAGER", "NONE", "NORMALFIXED", "NOT", "NOT-ACCESSIBLE", "NOT-AVAILABLE", "NOT-DEFINED", "NOT-EQUAL", "NOT-SENT", "NOT-TESTED", "NOT-VALID", "NOTHING", "NOTIFICATION", "NTP--RFC-958", "NUMBER", "NV-BLOCK-DESCRIPTOR", "NV-BLOCK-NEEDS", "NV-BLOCK-SW-COMPONENT-TYPE", "NV-DATA-INTERFACE", "NV-RAM-MANAGER", "OBD", "OBD-CONTROL-SERVICE-NEEDS", "OBD-DCY", "OBD-DRIVING-CYCLE", "OBD-INFO-SERVICE-NEEDS", "OBD-MONITOR-SERVICE-NEEDS", "OBD-PID-SERVICE-NEEDS", "OBD-RATIO-DENOMINATOR-NEEDS", "OBD-RATIO-SERVICE-NEEDS", "OBSERVER", "OBSERVER-BASED", "OC", "OCCURENCE", "OEM-BOOT", "OEM-BOOT-RESP-APP", "OFF", "OFFSET", "OFFSET-TIMING-CONSTRAINT", "OM", "ON-CHANGE-OF-DATA-IDENTIFIER", "ON-COMPARISON-OF-VALUES", "ON-DTC-STATUS-CHANGE", "ON-ENTRY", "ON-EXIT", "ON-TRANSITION", "ONE-EVERY-N", "ONLY-THIS-CYCLE-AND-READINESS", "OPAQUE", "OPEN", "OPERATING-SYSTEM", "OPERATION-CALL-RECEIVED", "OPERATION-CALL-RESPONSE-RECEIVED", "OPERATION-CALL-RESPONSE-SENT", "OPERATION-CALLED", "OPERATION-INVOKED-EVENT", "OPTIONS", "OR", "ORDINARY-EOC", "OS-MODULE-INSTANTIATION", "OS-TASK-EXECUTION-EVENT", "OS-TASK-PROXY", "OTHER", "OUT", "OVERRIDE", "OVERWRITE", "P-PORT-PROTOTYPE", "PA", "PACKAGEABLE-ELEMENT", "PARAMETER-ACCESS", "PARAMETER-DATA-PROTOTYPE", "PARAMETER-INTERFACE", "PARAMETER-SW-COMPONENT-TYPE", "PARTIAL-NETWORK", "PARTITION", "PASS-THROUGH-SW-CONNECTOR", "PASSIVE", "PASSTHROUGH", "PAYLOAD-AS-ARRAY", "PAYLOAD-AS-POINTER-TO-ARRAY", "PC-AFFECTS-LT", "PC-AFFECTS-LT-AND-PB", "PC-AFFECTS-PB", "PDF", "PDU", "PDU-ACTIVATION-ROUTING-GROUP", "PDU-R", "PDU-TO-FRAME-MAPPING", "PDU-TRIGGERING", "PDUR-I-PDU-GROUP", "PENDING", "PER-EXECUTABLE", "PER-INSTANCE-MEMORY", "PERIODIC-EVENT-TRIGGERING", "PERIODIC-RATE-FAST", "PERIODIC-RATE-MEDIUM", "PERIODIC-RATE-SLOW", "PERSISTENCY-DATA-ELEMENT", "PERSISTENCY-DEPLOYMENT", "PERSISTENCY-DEPLOYMENT-ELEMENT", "PERSISTENCY-DEPLOYMENT-ELEMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "PERSISTENCY-DEPLOYMENT-TO-CRYPTO-KEY-SLOT-MAPPING", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-CHANNEL-MAPPING", "PERSISTENCY-DEPLOYMENT-TO-DLT-LOG-SINK-MAPPING", "PERSISTENCY-FILE", "PERSISTENCY-FILE-ARRAY", "PERSISTENCY-FILE-ELEMENT", "PERSISTENCY-FILE-PROXY", "PERSISTENCY-FILE-PROXY-INTERFACE", "PERSISTENCY-FILE-PROXY-TO-FILE-MAPPING", "PERSISTENCY-FILE-STORAGE", "PERSISTENCY-FILE-STORAGE-INTERFACE", "PERSISTENCY-INTERFACE", "PERSISTENCY-INTERFACE-ELEMENT", "PERSISTENCY-KEY-VALUE-DATABASE", "PERSISTENCY-KEY-VALUE-DATABASE-INTERFACE", "PERSISTENCY-KEY-VALUE-PAIR", "PERSISTENCY-KEY-VALUE-STORAGE", "PERSISTENCY-KEY-VALUE-STORAGE-INTERFACE", "PERSISTENCY-PORT-PROTOTYPE-TO-DEPLOYMENT-MAPPING", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-ARRAY-MAPPING", "PERSISTENCY-PORT-PROTOTYPE-TO-FILE-STORAGE-MAPPING", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-DATABASE-MAPPING", "PERSISTENCY-PORT-PROTOTYPE-TO-KEY-VALUE-STORAGE-MAPPING", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-DATABASE", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-ELEMENT", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-FILE", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-KEY", "PERSISTENCY-REDUNDANCY-HANDLING-SCOPE-STORAGE", "PGWIDE", "PHM-ABSTRACT-RECOVERY-NOTIFICATION-INTERFACE", "PHM-ACTION", "PHM-ACTION-ITEM", "PHM-ACTION-LIST", "PHM-ARBITRATION", "PHM-CHECKPOINT", "PHM-CONTRIBUTION-TO-MACHINE-MAPPING", "PHM-HEALTH-CHANNEL-INTERFACE", "PHM-HEALTH-CHANNEL-RECOVERY-NOTIFICATION-INTERFACE", "PHM-HEALTH-CHANNEL-STATUS", "PHM-LOGICAL-EXPRESSION", "PHM-RECOVERY-ACTION-INTERFACE", "PHM-RULE", "PHM-SUPERVISED-ENTITY-INTERFACE", "PHM-SUPERVISION", "PHM-SUPERVISION-RECOVERY-NOTIFICATION-INTERFACE", "PHYSICAL", "PHYSICAL-ADDRESS", "PHYSICAL-CAN-FD", "PHYSICAL-CHANNEL", "PHYSICAL-DIMENSION", "PHYSICAL-DIMENSION-MAPPING-SET", "PL", "PLAIN", "PLATFORM-ACTION-ITEM", "PLATFORM-HEALTH-MANAGEMENT-CONTRIBUTION", "PLATFORM-HEALTH-MANAGEMENT-INTERFACE", "PLATFORM-MODULE-ENDPOINT-CONFIGURATION", "PLATFORM-MODULE-ETHERNET-ENDPOINT-CONFIGURATION", "PLATFORM-PHM-ACTION-ITEM", "PNC-MAPPING-IDENT", "PNG", "POLY", "PORT", "PORT-BLUEPRINT", "PORT-ELEMENT-TO-COMMUNICATION-RESOURCE-MAPPING", "PORT-GROUP", "PORT-INTERFACE", "PORT-INTERFACE-DEFINITION", "PORT-INTERFACE-MAPPING", "PORT-INTERFACE-MAPPING-SET", "PORT-INTERFACE-TO-DATA-TYPE-MAPPING", "PORT-PROTOTYPE", "PORT-PROTOTYPE-BLUEPRINT", "POSSIBLE-ERROR-REACTION", "POST", "POST-BUILD", "POST-BUILD-VARIANT-CRITERION", "POST-BUILD-VARIANT-CRITERION-VALUE-SET", "POWER", "POWER-WINDOW-TIME", "PR-PORT-PROTOTYPE", "PRE--R-4--2", "PRE-COMPILE", "PRE-COMPILE-TIME", "PRECONFIGURED-CONFIGURATION", "PREDEFINED-VARIANT", "PRESENTATION-CONTINUOUS", "PRESENTATION-DISCRETE", "PRESHARED-KEY-IDENTITY", "PRIMARY-ECU", "PRIMITIVE", "PRIMITIVE-ATTRIBUTE-TAILORING", "PRIO-OCC", "PRIVATE-KEY", "PROCESS", "PROCESS-DESIGN", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING", "PROCESS-DESIGN-TO-MACHINE-DESIGN-MAPPING-SET", "PROCESS-EXECUTION-ERROR", "PROCESS-IS-NOT-SELF-TERMINATING", "PROCESS-IS-SELF-TERMINATING", "PROCESS-PHM-ACTION-ITEM", "PROCESS-TO-MACHINE-MAPPING", "PROCESS-TO-MACHINE-MAPPING-SET", "PROCESSING-STYLE-ASYNCHRONOUS", "PROCESSING-STYLE-ASYNCHRONOUS-WITH-ERROR", "PROCESSING-STYLE-SYNCHRONOUS", "PROCESSOR", "PROCESSOR-CORE", "PRODUCER", "PROTECT-LAMP", "PROTECTED", "PROVIDED-AP-SERVICE-INSTANCE", "PROVIDED-DDS-SERVICE-INSTANCE", "PROVIDED-SERVICE-INSTANCE", "PROVIDED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-P-PORT-PROTOTYPE-MAPPING", "PROVIDED-SOMEIP-SERVICE-INSTANCE", "PROVIDED-USER-DEFINED-SERVICE-INSTANCE", "PS", "PSK", "PSK-IDENTITY-TO-KEY-SLOT-MAPPING", "PT", "PTP--IEEE-1588--2002", "PTP--IEEE-1588--2008", "PUBLIC-KEY", "PUBLISHED-INFORMATION", "PURE-LOCAL-TIME-BASE", "PUT", "QU", "QUEUED", "R-4--2", "R-PORT-PROTOTYPE", "RAPID-PROTOTYPING-SCENARIO", "RAW", "RAW-DATA", "RAW-DATA-STREAM-CLIENT-INTERFACE", "RAW-DATA-STREAM-DEPLOYMENT", "RAW-DATA-STREAM-GRANT", "RAW-DATA-STREAM-GRANT-DESIGN", "RAW-DATA-STREAM-INTERFACE", "RAW-DATA-STREAM-MAPPING", "RAW-DATA-STREAM-METHOD-DEPLOYMENT", "RAW-DATA-STREAM-SERVER-INTERFACE", "REACTION", "READ-ONLY", "READ-WRITE", "REBOOT", "RECOMMENDED-CONFIGURATION", "RECORD-VALUE-FIELD", "RECOVERY-NOTIFICATION", "RECOVERY-NOTIFICATION-TO-P-PORT-PROTOTYPE-MAPPING", "RECOVERY-VIA-APPLICATION-ACTION", "RECOVERY-VIA-APPLICATION-ACTION-TO-CLIENT-SERVER-OPERATION-MAPPING", "RECT", "RED-STOP-LAMP", "REDUNDANT", "REDUNDANT-PER-ELEMENT", "REDUNDANT-PER-KEY", "REF-ALL", "REF-NON-STANDARD", "REF-NONE", "REFERENCE-TAILORING", "REFERRABLE", "REGULAR", "REJECT", "REMOVE", "REPETITIVE-EOC", "REPLACE", "REPLACE-BY-TIMEOUT-SUBSTITUTION-VALUE", "REPORT", "REPORT-AFTER-INIT", "REPORT-BEFORE-INIT", "REPORT-DTC-RECORD-INFORMATION-ON-DTC-STATUS-CHANGE", "REPORT-MOST-RECENT-DTC-ON-STATUS-CHANGE", "REPORTING-IN-CHRONLOGICAL-ORDER-OLDEST-FIRST", "REPORTS-EXECUTION-STATE", "REQUEST", "REQUEST-CALLBACK-TYPE-MANUFACTURER", "REQUEST-CALLBACK-TYPE-SUPPLIER", "REQUEST-NO-RETURN", "REQUIRED-AP-SERVICE-INSTANCE", "REQUIRED-DDS-SERVICE-INSTANCE", "REQUIRED-SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-R-PORT-PROTOTYPE-MAPPING", "REQUIRED-SOMEIP-SERVICE-INSTANCE", "REQUIRED-USER-DEFINED-SERVICE-INSTANCE", "REQUIRES-CALLBACK-EXECUTION", "RES-AXIS", "RESET-ECU", "RESET-MACHINE", "RESET-MCU", "RESET-VM", "RESOURCE-CONSUMPTION", "RESOURCE-GROUP", "RESPOND-AFTER-RESET", "RESPOND-BEFORE-RESET", "RESPONSE", "RESPONSE-SYNCHRONIZATION", "REST-ABSTRACT-ENDPOINT", "REST-ABSTRACT-NUMERICAL-PROPERTY-DEF", "REST-ABSTRACT-PROPERTY-DEF", "REST-ARRAY-PROPERTY-DEF", "REST-BOOLEAN-PROPERTY-DEF", "REST-ELEMENT-DEF", "REST-ENDPOINT-DELETE", "REST-ENDPOINT-GET", "REST-ENDPOINT-POST", "REST-ENDPOINT-PUT", "REST-HTTP-PORT-PROTOTYPE-MAPPING", "REST-INTEGER-PROPERTY-DEF", "REST-NUMBER-PROPERTY-DEF", "REST-OBJECT-REF", "REST-PRIMITIVE-PROPERTY-DEF", "REST-RESOURCE-DEF", "REST-SERVICE-INTERFACE", "REST-STRING-PROPERTY-DEF", "RESTART", "RESTART-APPLICATION", "RES_AXIS", "RETURN-ON-EVENT-CLEARED", "RETURN-ON-EVENT-STOPPED", "RETURN-VALUE-PROVIDED", "RIGHT", "RM", "RN", "RO", "ROLL-BACK", "ROOT-SW-CLUSTER-DESIGN-COMPONENT-PROTOTYPE", "ROOT-SW-COMPONENT-PROTOTYPE", "ROOT-SW-COMPOSITION-PROTOTYPE", "ROTATE-180", "ROTATE-180-LIMIT-TO-TEXT", "ROTATE-90-CCW", "ROTATE-90-CCW-FIT-TO-TEXT", "ROTATE-90-CCW-LIMIT-TO-TEXT", "ROTATE-90-CW", "ROTATE-90-CW-FIT-TO-TEXT", "ROTATE-90-CW-LIMIT-TO-TEXT", "ROUGH-ESTIMATE-HEAP-USAGE", "ROUGH-ESTIMATE-OF-EXECUTION-TIME", "ROUGH-ESTIMATE-STACK-USAGE", "ROUTER", "ROUTER-ADVERTISEMENT", "RPT-COMPONENT", "RPT-CONTAINER", "RPT-ENABLER-RAM", "RPT-ENABLER-RAM-AND-ROM", "RPT-ENABLER-ROM", "RPT-EXECUTABLE-ENTITY", "RPT-EXECUTABLE-ENTITY-EVENT", "RPT-EXECUTION-CONTEXT", "RPT-LEVEL-1", "RPT-LEVEL-2", "RPT-LEVEL-3", "RPT-PROFILE", "RPT-SERVICE-POINT", "RSA", "RTE-EVENT", "RTE-EVENT-IN-COMPOSITION-SEPARATION", "RTE-EVENT-IN-COMPOSITION-TO-OS-TASK-PROXY-MAPPING", "RTE-EVENT-IN-SYSTEM-SEPARATION", "RTE-EVENT-IN-SYSTEM-TO-OS-TASK-PROXY-MAPPING", "RTPGE", "RU", "RULE", "RUN-CONTINUOUS", "RUN-ONCE", "RUNNABLE-ENTITY", "RUNNABLE-ENTITY-ACTIVATED", "RUNNABLE-ENTITY-GROUP", "RUNNABLE-ENTITY-STARTED", "RUNNABLE-ENTITY-TERMINATED", "RUNNABLE-ENTITY-VARIABLE-ACCESS", "RUNTIME-ERROR", "RW", "RX-TRIGGER", "SA", "SAE-J-1939--73", "SAE-J-2012--DA", "SAFETY", "SATURATE", "SCHEDULE-VARIANT-1", "SCHEDULE-VARIANT-2", "SCHEDULE-VARIANT-3", "SCHEDULE-VARIANT-4", "SCHEDULE-VARIANT-5", "SCHEDULE-VARIANT-6", "SCHEDULE-VARIANT-7", "SCHEDULED", "SD", "SDG-ABSTRACT-FOREIGN-REFERENCE", "SDG-ABSTRACT-PRIMITIVE-ATTRIBUTE", "SDG-AGGREGATION-WITH-VARIATION", "SDG-ATTRIBUTE", "SDG-CAPTION", "SDG-CLASS", "SDG-DEF", "SDG-FOREIGN-REFERENCE", "SDG-FOREIGN-REFERENCE-WITH-VARIATION", "SDG-PRIMITIVE-ATTRIBUTE", "SDG-PRIMITIVE-ATTRIBUTE-WITH-VARIATION", "SDG-REFERENCE", "SDG-TAILORING", "SEARCH-FOR-ALL", "SEARCH-FOR-ALL-INSTANCES", "SEARCH-FOR-ANY", "SEARCH-FOR-ID", "SEARCH-FOR-SPECIFIC-INSTANCE", "SEC-OC-CRYPTO-SERVICE-MAPPING", "SEC-OC-DEPLOYMENT", "SEC-OC-JOB-MAPPING", "SEC-OC-JOB-REQUIREMENT", "SEC-OC-SECURE-COM-PROPS", "SECOND-TO-FIRST", "SECONDARY-ECU", "SECRET-SEED", "SECTION-NAME-PREFIX", "SECURE-COM-PROPS", "SECURE-COM-PROPS-SET", "SECURE-COMMUNICATION-AUTHENTICATION-PROPS", "SECURE-COMMUNICATION-DEPLOYMENT", "SECURE-COMMUNICATION-FRESHNESS-PROPS", "SECURE-COMMUNICATION-PROPS-SET", "SECURE-ON-BOARD-COMMUNICATION", "SECURE-ON-BOARD-COMMUNICATION-NEEDS", "SECURED-I-PDU", "SECURED-PDU-HEADER-08-BIT", "SECURED-PDU-HEADER-16-BIT", "SECURED-PDU-HEADER-32-BIT", "SECURITY", "SECURITY-EVENT-AGGREGATION-FILTER", "SECURITY-EVENT-CONTEXT-MAPPING", "SECURITY-EVENT-CONTEXT-MAPPING-APPLICATION", "SECURITY-EVENT-CONTEXT-MAPPING-BSW-MODULE", "SECURITY-EVENT-CONTEXT-MAPPING-COMM-CONNECTOR", "SECURITY-EVENT-CONTEXT-MAPPING-FUNCTIONAL-CLUSTER", "SECURITY-EVENT-CONTEXT-PROPS", "SECURITY-EVENT-DEFINITION", "SECURITY-EVENT-FILTER-CHAIN", "SECURITY-EVENT-MAPPING", "SECURITY-EVENT-ONE-EVERY-N-FILTER", "SECURITY-EVENT-REPORT-INTERFACE", "SECURITY-EVENT-REPORT-TO-SECURITY-EVENT-DEFINITION-MAPPING", "SECURITY-EVENT-STATE-FILTER", "SECURITY-EVENT-THRESHOLD-FILTER", "SELECTED", "SENDER-RECEIVER-INTERFACE", "SENSOR-ACTUATOR-SW-COMPONENT-TYPE", "SENT-TAGGED", "SENT-UNTAGGED", "SERIALIZATION-TECHNOLOGY", "SERIALIZER", "SERVER-AUTHENTICATE", "SERVER-CALL-POINT", "SERVER-DECRYPT", "SERVER-ENCRYPT", "SERVER-MAC-GENERATE", "SERVER-MAC-VERIFY", "SERVER-VERIFY", "SERVICE-DISCOVERY", "SERVICE-EVENT-DEPLOYMENT", "SERVICE-FIELD-DEPLOYMENT", "SERVICE-INSTANCE-COLLECTION-SET", "SERVICE-INSTANCE-TO-APPLICATION-ENDPOINT-MAPPING", "SERVICE-INSTANCE-TO-MACHINE-MAPPING", "SERVICE-INSTANCE-TO-PORT-PROTOTYPE-MAPPING", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING", "SERVICE-INSTANCE-TO-SIGNAL-MAPPING-SET", "SERVICE-INSTANCE-TO-SW-CLUSTER-DESIGN-PORT-PROTOTYPE-MAPPING", "SERVICE-INTERFACE", "SERVICE-INTERFACE-APPLICATION-ERROR-MAPPING", "SERVICE-INTERFACE-DEPLOYMENT", "SERVICE-INTERFACE-ELEMENT-MAPPING", "SERVICE-INTERFACE-ELEMENT-SECURE-COM-CONFIG", "SERVICE-INTERFACE-EVENT-MAPPING", "SERVICE-INTERFACE-FIELD-MAPPING", "SERVICE-INTERFACE-MAPPING", "SERVICE-INTERFACE-MAPPING-SET", "SERVICE-INTERFACE-METHOD-MAPPING", "SERVICE-INTERFACE-PEDIGREE", "SERVICE-INTERFACE-TRIGGER-MAPPING", "SERVICE-METHOD-DEPLOYMENT", "SERVICE-NEEDS", "SERVICE-ONLY", "SERVICE-PROXY-SW-COMPONENT-TYPE", "SERVICE-SW-COMPONENT-TYPE", "SERVICE-TIMING", "SESSION-HANDLING-ACTIVE", "SESSION-HANDLING-INACTIVE", "SETTER", "SG", "SH", "SHORT-HEADER", "SHOW-ALIAS-NAME", "SHOW-CATEGORY", "SHOW-CONTENT", "SHOW-LONG-NAME", "SHOW-NUMBER", "SHOW-PAGE", "SHOW-SEE", "SHOW-SHORT-NAME", "SHOW-TYPE", "SI", "SIDES", "SIGN", "SIGN-WITH-ORIGIN-AUTHENTICATION", "SIGNAL-BASED", "SIGNAL-BASED-EVENT-DEPLOYMENT", "SIGNAL-BASED-EVENT-ELEMENT-TO-I-SIGNAL-TRIGGERING-MAPPING", "SIGNAL-BASED-FIELD-DEPLOYMENT", "SIGNAL-BASED-FIELD-TO-I-SIGNAL-TRIGGERING-MAPPING", "SIGNAL-BASED-METHOD-DEPLOYMENT", "SIGNAL-BASED-METHOD-TO-I-SIGNAL-TRIGGERING-MAPPING", "SIGNAL-BASED-SERVICE-INTERFACE-DEPLOYMENT", "SIGNAL-BASED-TRIGGER-TO-I-SIGNAL-TRIGGERING-MAPPING", "SIGNAL-SERVICE-TRANSLATION-ELEMENT-PROPS", "SIGNAL-SERVICE-TRANSLATION-EVENT-PROPS", "SIGNAL-SERVICE-TRANSLATION-PROPS", "SIGNAL-SERVICE-TRANSLATION-PROPS-SET", "SIGNATURE", "SILENT", "SIMULATED-EXECUTION-TIME", "SINGLE", "SINGLE-CORE-REENTRANT", "SINGLE-LANGUAGE-REFERRABLE", "SINGLE-OCCURRENCE", "SK", "SL", "SLAVE", "SLOPPY", "SLOW-FLASHING-MODE", "SLP", "SM", "SN", "SO", "SO-AD-ROUTING-GROUP", "SO-CON-I-PDU-IDENTIFIER", "SOCKET-ADDRESS", "SOCKET-CONNECTION-BUNDLE", "SOCKET-CONNECTION-IPDU-IDENTIFIER-SET", "SOFTWARE-ACTIVATION-DEPENDENCY", "SOFTWARE-CLUSTER", "SOFTWARE-CLUSTER-DESIGN", "SOFTWARE-CLUSTER-REQUIREMENT", "SOFTWARE-PACKAGE", "SOFTWARE-PACKAGE-STEP", "SOMEIP", "SOMEIP-DATA-PROTOTYPE-TRANSFORMATION-PROPS", "SOMEIP-EVENT", "SOMEIP-EVENT-DEPLOYMENT", "SOMEIP-EVENT-GROUP", "SOMEIP-FIELD", "SOMEIP-FIELD-DEPLOYMENT", "SOMEIP-METHOD", "SOMEIP-METHOD-DEPLOYMENT", "SOMEIP-PROVIDED-EVENT-GROUP", "SOMEIP-REQUIRED-EVENT-GROUP", "SOMEIP-SD-CLIENT-EVENT-GROUP-TIMING-CONFIG", "SOMEIP-SD-CLIENT-SERVICE-INSTANCE-CONFIG", "SOMEIP-SD-SERVER-EVENT-GROUP-TIMING-CONFIG", "SOMEIP-SD-SERVER-SERVICE-INSTANCE-CONFIG", "SOMEIP-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "SOMEIP-SERVICE-INTERFACE", "SOMEIP-SERVICE-INTERFACE-DEPLOYMENT", "SOMEIP-TP-CHANNEL", "SOMEIP-TP-CONFIG", "SOMEIP-TRANSFORMATION-PROPS", "SPEC-ELEMENT-REFERENCE", "SPEC-ELEMENT-SCOPE", "SPECIFICATION-DOCUMENT-SCOPE", "SPORADIC-EVENT-TRIGGERING", "SQ", "SR", "SS", "SSDP", "ST", "STACK-USAGE", "STANDARD", "STANDARD-PORT", "START", "START-FROM-BEGINNING", "STARTUP-CONFIG", "STARTUP-CONFIG-SET", "STATIC-OR-DYNAMIC-PART-TRIGGER", "STATIC-PART-TRIGGER", "STATIC-SOCKET-CONNECTION", "STATUS-BIT-AGING-AND-DISPLACEMENT", "STATUS-BIT-NORMAL", "STD", "STD-AXIS", "STD-CPP-IMPLEMENTATION-DATA-TYPE", "STD_AXIS", "STEADY", "STIMULUS-SYNCHRONIZATION", "STOP", "STOP-TRIGGER", "STORE-EVENT", "STORE-PERSISTENTLY", "STRICT-MODE", "STRICT-MONOTONOUS", "STRICT-PRIORITY", "STRICTLY-DECREASING", "STRICTLY-INCREASING", "STRUCTURED-REQ", "SU", "SUPERVISED-ENTITY-CHECKPOINT-NEEDS", "SUPERVISED-ENTITY-NEEDS", "SUPERVISION-CHECKPOINT", "SUPERVISION-ENTITY", "SUPERVISION-MODE", "SUPERVISION-MODE-CONDITION", "SUPPLIER", "SUPPORTS-BUFFER-LOCKING", "SV", "SVG", "SW", "SW-ADDR-METHOD", "SW-AXIS-TYPE", "SW-BASE-TYPE", "SW-CALIBRATION-METHOD", "SW-CALPRM-PROTOTYPE", "SW-CLASS-ATTR-IMPL", "SW-CLASS-INSTANCE", "SW-CLASS-PROTOTYPE", "SW-CODE-SYNTAX", "SW-COMPONENT-MAPPING-CONSTRAINTS", "SW-COMPONENT-PROTOTYPE", "SW-COMPONENT-TYPE", "SW-CONNECTOR", "SW-GENERIC-AXIS-PARAM-TYPE", "SW-INSTANCE", "SW-MC-BASE-TYPE", "SW-MC-FRAME", "SW-MC-INTERFACE", "SW-MC-INTERFACE-SOURCE", "SW-RECORD-LAYOUT", "SW-SERVICE-ARG", "SW-SERVICE-PROTOTYPE", "SW-SYSTEMCONST", "SW-SYSTEMCONSTANT-VALUE-SET", "SW-VARIABLE-PROTOTYPE", "SWC", "SWC-BSW-MAPPING", "SWC-IMPLEMENTATION", "SWC-INTERNAL-BEHAVIOR", "SWC-MODE-MANAGER-ERROR-EVENT", "SWC-MODE-SWITCH-EVENT", "SWC-SERVICE-DEPENDENCY", "SWC-TIMING", "SWC-TO-APPLICATION-PARTITION-MAPPING", "SWC-TO-ECU-MAPPING", "SWC-TO-IMPL-MAPPING", "SWITCH", "SYMBOL-PROPS", "SYMBOLIC-NAME-PROPS", "SYMMETRIC", "SYMMETRIC-KEY", "SYNC-BASE-TIME-MANAGER", "SYNC-TIME-BASE-MGR-USER-NEEDS", "SYNCHRONIZATION-POINT-CONSTRAINT", "SYNCHRONIZATION-TIMING-CONSTRAINT", "SYNCHRONIZED", "SYNCHRONIZED-MASTER-TIME-BASE", "SYNCHRONIZED-SLAVE-TIME-BASE", "SYNCHRONIZED-TIME-BASE-CONSUMER", "SYNCHRONIZED-TIME-BASE-CONSUMER-INTERFACE", "SYNCHRONIZED-TIME-BASE-PROVIDER", "SYNCHRONIZED-TIME-BASE-PROVIDER-INTERFACE", "SYNCHRONOUS", "SYNCHRONOUS-SERVER-CALL-POINT", "SYSTEM", "SYSTEM-DESIGN-TIME", "SYSTEM-MAPPING", "SYSTEM-MEMORY-USAGE", "SYSTEM-SIGNAL", "SYSTEM-SIGNAL-GROUP", "SYSTEM-SUPPLIER-BOOT", "SYSTEM-SUPPLIER-BOOT-RESP-APP", "SYSTEM-TIMING", "TA", "TARGET-CONTAINER", "TASK", "TC", "TCP", "TCP-OPTION-FILTER-LIST", "TCP-OPTION-FILTER-SET", "TD-CP-SOFTWARE-CLUSTER-MAPPING", "TD-CP-SOFTWARE-CLUSTER-MAPPING-SET", "TD-CP-SOFTWARE-CLUSTER-RESOURCE-MAPPING", "TD-EVENT-BSW", "TD-EVENT-BSW-INTERNAL-BEHAVIOR", "TD-EVENT-BSW-MODE-DECLARATION", "TD-EVENT-BSW-MODULE", "TD-EVENT-COM", "TD-EVENT-COMPLEX", "TD-EVENT-CYCLE-START", "TD-EVENT-FR-CLUSTER-CYCLE-START", "TD-EVENT-FRAME", "TD-EVENT-FRAME-ETHERNET", "TD-EVENT-I-PDU", "TD-EVENT-I-SIGNAL", "TD-EVENT-MODE-DECLARATION", "TD-EVENT-OPERATION", "TD-EVENT-SERVICE-INSTANCE", "TD-EVENT-SERVICE-INSTANCE-DISCOVERY", "TD-EVENT-SERVICE-INSTANCE-EVENT", "TD-EVENT-SERVICE-INSTANCE-FIELD", "TD-EVENT-SERVICE-INSTANCE-METHOD", "TD-EVENT-SWC", "TD-EVENT-SWC-INTERNAL-BEHAVIOR", "TD-EVENT-SWC-INTERNAL-BEHAVIOR-REFERENCE", "TD-EVENT-TRIGGER", "TD-EVENT-TT-CAN-CYCLE-START", "TD-EVENT-VARIABLE-DATA-PROTOTYPE", "TD-EVENT-VFB", "TD-EVENT-VFB-PORT", "TD-EVENT-VFB-REFERENCE", "TE", "TERMINATE", "TEST-FAILED", "TEST-FAILED-BIT", "TEST-FAILED-THIS-OPERATION-CYCLE", "TESTED", "TESTED-AND-FAILED", "TG", "TH", "TI", "TIFF", "TIME", "TIME-BASE-PROVIDER-TO-PERSISTENCY-MAPPING", "TIME-BASE-RESOURCE", "TIME-SYNC-MODULE-INSTANTIATION", "TIME-SYNC-PORT-PROTOTYPE-TO-TIME-BASE-MAPPING", "TIME-SYNC-SERVER-CONFIGURATION", "TIME-SYNCHRONIZATION-INTERFACE", "TIME-SYNCHRONIZATION-MASTER-INTERFACE", "TIME-SYNCHRONIZATION-PURE-LOCAL-INTERFACE", "TIME-SYNCHRONIZATION-SLAVE-INTERFACE", "TIMING-CONDITION", "TIMING-CONSTRAINT", "TIMING-DESCRIPTION", "TIMING-DESCRIPTION-EVENT", "TIMING-DESCRIPTION-EVENT-CHAIN", "TIMING-EVENT", "TIMING-EXTENSION", "TIMING-EXTENSION-RESOURCE", "TIMING-MODE-INSTANCE", "TIP", "TK", "TL", "TLS-12", "TLS-13", "TLS-CONNECTION-GROUP", "TLS-CRYPTO-CIPHER-SUITE", "TLS-CRYPTO-CIPHER-SUITE-PROPS", "TLS-CRYPTO-SERVICE-MAPPING", "TLS-DEPLOYMENT", "TLS-IAM-REMOTE-SUBJECT", "TLS-JOB-MAPPING", "TLS-JOB-REQUIREMENT", "TLS-SECURE-COM-PROPS", "TLV-DATA-ID-DEFINITION-SET", "TN", "TO", "TOP", "TOPBOT", "TOPIC", "TOPIC-1", "TOPIC-PREFIX", "TP-ADDRESS", "TP-CONFIG", "TP-CONNECTION-IDENT", "TR", "TRACE", "TRACE-REFERRABLE", "TRACEABLE", "TRACEABLE-TABLE", "TRACEABLE-TEXT", "TRACED-FAILURE", "TRANSFER", "TRANSFORMATION-PROPS", "TRANSFORMATION-PROPS-SET", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-ELEMENT-MAPPING-SET", "TRANSFORMATION-PROPS-TO-SERVICE-INTERFACE-MAPPING-SET", "TRANSFORMATION-TECHNOLOGY", "TRANSFORMER-ERROR-HANDLING", "TRANSFORMER-HARD-ERROR-EVENT", "TRANSFORMER-STATUS-FORWARDING", "TRANSFORMING-I-SIGNAL", "TRANSIENT-FAULT", "TRANSLATION-START", "TRANSPORT", "TRANSPORT-LAYER-INDEPENDENT-ID-COLLECTION-SET", "TRANSPORT-LAYER-INDEPENDENT-INSTANCE-ID", "TRAP", "TRIGGER", "TRIGGER-ACTIVATED", "TRIGGER-INTERFACE", "TRIGGER-INTERFACE-MAPPING", "TRIGGER-RELEASED", "TRIGGER-UNICAST", "TRIGGERED", "TRIGGERED-ON-CHANGE", "TRIGGERED-ON-CHANGE-WITHOUT-REPETITION", "TRIGGERED-ON-EVALUATION", "TRIGGERED-WITHOUT-REPETITION", "TRUE", "TS", "TT", "TTCAN-CLUSTER", "TTCAN-COMMUNICATION-CONNECTOR", "TTCAN-COMMUNICATION-CONTROLLER", "TTCAN-PHYSICAL-CHANNEL", "TUNNEL", "TW", "TX-REF-TRIGGER", "TX-REF-TRIGGER-GAP", "TX-TRIGGER-MERGED", "TX-TRIGGER-SINGLE", "UCM", "UCM-DESCRIPTION", "UCM-MASTER", "UCM-MODULE-INSTANTIATION", "UCM-STEP", "UDP", "UDP-CHECKSUM-DISABLED", "UDP-CHECKSUM-ENABLED", "UDP-NM", "UDP-NM-CLUSTER", "UDP-NM-NODE", "UDS", "UK", "UNDECIDED", "UNDEFINED", "UNIT", "UNIT-GROUP", "UNNUMBER", "UNSPECIFIED", "UP-LINK-PORT", "UPDATE", "UPLOADABLE-EXCLUSIVE-PACKAGE-ELEMENT", "UPLOADABLE-PACKAGE-ELEMENT", "UR", "USE-ARGUMENT-TYPE", "USE-ARRAY-BASE-TYPE", "USE-FIRST-CONTEXT-DATA", "USE-LAST-CONTEXT-DATA", "USE-VOID", "USER-DEFINED", "USER-DEFINED-CLUSTER", "USER-DEFINED-COMMUNICATION-CONNECTOR", "USER-DEFINED-COMMUNICATION-CONTROLLER", "USER-DEFINED-ETHERNET-FRAME", "USER-DEFINED-EVENT-DEPLOYMENT", "USER-DEFINED-FIELD-DEPLOYMENT", "USER-DEFINED-GLOBAL-TIME-MASTER", "USER-DEFINED-GLOBAL-TIME-SLAVE", "USER-DEFINED-I-PDU", "USER-DEFINED-METHOD-DEPLOYMENT", "USER-DEFINED-PDU", "USER-DEFINED-PHYSICAL-CHANNEL", "USER-DEFINED-SERVICE-INSTANCE-TO-MACHINE-MAPPING", "USER-DEFINED-SERVICE-INTERFACE-DEPLOYMENT", "USER-DEFINED-TRANSFORMATION-PROPS", "USES-LOGGING", "UZ", "V-2-X-ACTIVE-SUPPORTED", "V-2-X-FAC-USER-NEEDS", "V-2-X-FACILITIES", "V-2-X-M-USER-NEEDS", "V-2-X-MANAGEMENT", "V-2-X-NOT-SUPPORTED", "VALID", "VAR", "VAR-FAST", "VAR-NO-INIT", "VAR-POWER-ON-INIT", "VARIABLE-ACCESS", "VARIABLE-AND-PARAMETER-INTERFACE-MAPPING", "VARIABLE-DATA-PROTOTYPE", "VARIABLE-DATA-PROTOTYPE-RECEIVED", "VARIABLE-DATA-PROTOTYPE-SENT", "VARIABLE-SIZE", "VARIANT-LINK-TIME", "VARIANT-POST-BUILD", "VARIANT-POST-BUILD-LOADABLE", "VARIANT-POST-BUILD-SELECTABLE", "VARIANT-PRE-COMPILE", "VARIATION-POINT-PROXY", "VEHICLE-PACKAGE", "VEHICLE-ROLLOUT-STEP", "VENDOR-SPECIFIC", "VENDOR-SPECIFIC-SERVICE-NEEDS", "VERBOSE", "VERIFICATION", "VERIFY", "VERTEX-OF-TARGET-CONTAINER", "VFB-TIMING", "VI", "VIEW-MAP", "VIEW-MAP-SET", "VLAN-CONFIG", "VO", "VOLATILE", "WAIT-POINT", "WAIT-TIME-DATE", "WARMUP", "WARN", "WARNING", "WARNING-INDICATOR-REQUESTED-BIT-NEEDS", "WATCH-DOG-MANAGER", "WATCH-TRIGGER", "WATCH-TRIGGER-GAP", "WATCHDOG-ACTION-ITEM", "WATCHDOG-PHM-ACTION-ITEM", "WEIGHTED-ROUND-ROBIN", "WILL-CALL", "WILL-RECEIVE", "WILL-SEND", "WO", "WONT-CALL", "WONT-RECEIVE", "WONT-SEND", "WORST-CASE-HEAP-USAGE", "WORST-CASE-STACK-USAGE", "WRITE-ONLY", "WRONG-TRIGGER", "X-509", "X-MII", "X-MMI", "XCP", "XCP-PDU", "XDOC", "XFILE", "XG-MII", "XH", "XOR", "XREF-TARGET", "XXG-MII", "YO", "ZH", "ZU", "default", "preserve"];

    /// derive an enum entry from an input string using a perfect hash function
    pub fn from_bytes(input: &[u8]) -> Result<Self, ParseEnumItemError> {
        // here, hashfunc(input, <param>) is an ordinary hash function which may produce collisions
        // it is possible to create two tables so that
        //     table1[hashfunc(input, param1)] + table2[hashfunc(input, param2)] == desired enumeration value
        // these tables are pre-built and included here as constants, since the values to be hashed don't change
        static HASH_TABLE_1: [u16; 1869] = [65535, 65535, 221, 65535, 650, 2238, 1282, 1007, 2190, 65535, 65535, 2266, 65535, 65535, 65535, 1894, 727, 1465, 1598, 65535, 1206, 65535, 2007, 65535, 1845, 65535, 931, 722, 248, 1000, 908, 1286, 2043, 14, 1575, 135, 65535, 1033, 413, 1795, 78, 514, 687, 502, 1874, 2308, 65535, 65535, 688, 1042, 65535, 791, 419, 188, 1704, 1948, 65535, 65535, 924, 2272, 1089, 65535, 1622, 2023, 1154, 65535, 65535, 220, 65535, 426, 1068, 181, 568, 1694, 998, 1043, 1069, 672, 121, 566, 1682, 1691, 650, 65535, 1009, 1435, 1378, 65535, 65535, 1570, 582, 2366, 1185, 780, 65535, 65535, 65535, 65535, 65535, 425, 65535, 1744, 568, 65535, 1890, 1346, 758, 2074, 1113, 1945, 830, 623, 1452, 172, 842, 13, 2040, 1157, 2396, 2209, 1894, 1637, 1513, 1011, 671, 1751, 2385, 1479, 1784, 65535, 65535, 62, 1340, 1085, 65535, 900, 498, 2317, 1076, 1227, 65535, 187, 65535, 1253, 1820, 2148, 1333, 65535, 363, 704, 1268, 65535, 330, 1430, 515, 65535, 65535, 1145, 65535, 65535, 45, 1356, 2137, 376, 2075, 418, 1560, 65535, 1786, 231, 2153, 702, 2295, 65535, 65535, 665, 1547, 65535, 171, 267, 65535, 65535, 197, 123, 2170, 920, 65535, 1959, 65535, 65535, 1692, 1480, 1245, 1304, 2231, 2020, 2351, 466, 1583, 1659, 65535, 1476, 1446, 2082, 65535, 611, 1324, 65535, 1002, 65535, 65535, 2250, 65535, 2036, 1324, 2372, 1342, 2318, 2198, 1386, 65535, 1072, 1519, 796, 2122, 65535, 65535, 520, 1564, 901, 164, 2233, 990, 360, 223, 1303, 65535, 1640, 65535, 379, 65535, 65535, 1823, 1213, 939, 65535, 1614, 1695, 65535, 1217, 1138, 1047, 2227, 65535, 65535, 1598, 65535, 543, 1519, 2088, 65535, 65535, 1068, 793, 65535, 1674, 777, 65535, 65535, 65535, 65535, 65535, 2276, 1708, 65535, 675, 1748, 1466, 1527, 65535, 1998, 500, 11, 65535, 1378, 402, 65535, 65535, 65535, 65535, 2058, 65535, 999, 1647, 65535, 2114, 1340, 1335, 2082, 2035, 354, 639, 342, 1456, 65535, 909, 65535, 613, 65535, 2366, 1171, 351, 65535, 1437, 597, 2181, 1569, 1494, 462, 892, 1547, 2274, 20, 802, 1496, 2258, 1070, 65535, 325, 65535, 2078, 2044, 1211, 65535, 65535, 875, 1595, 2333, 2374, 2199, 65535, 65535, 1339, 65535, 1159, 365, 1018, 1810, 1297, 46, 620, 65535, 1580, 488, 381, 1522, 590, 1487, 2291, 368, 1180, 559, 2145, 65535, 766, 65535, 2387, 2097, 65535, 1030, 65535, 2059, 219, 1409, 788, 1475, 1431, 268, 764, 1412, 65535, 65535, 286, 65535, 65535, 136, 65535, 65535, 2154, 701, 1252, 1761, 719, 2146, 65535, 65535, 1265, 1223, 972, 218, 38, 65535, 1770, 1079, 2064, 841, 2258, 65535, 329, 1196, 446, 1846, 1653, 1618, 1147, 550, 1041, 410, 676, 65535, 982, 65535, 65535, 86, 2308, 1428, 282, 2394, 1075, 1772, 123, 1880, 1905, 65535, 752, 179, 559, 2192, 65535, 2385, 65535, 5, 1860, 65535, 269, 65535, 65535, 563, 1623, 65535, 892, 65535, 2257, 1933, 65535, 843, 429, 1078, 1028, 792, 1340, 65535, 1754, 65535, 65535, 1129, 1743, 65535, 898, 1207, 1697, 1808, 65535, 877, 2276, 65535, 2298, 136, 1765, 1746, 1444, 420, 1161, 209, 1412, 1701, 686, 469, 65535, 65535, 65535, 746, 2253, 318, 1940, 1074, 1300, 2142, 895, 65535, 453, 1707, 1223, 1352, 397, 65535, 1714, 898, 250, 65535, 1344, 380, 974, 1305, 694, 299, 669, 539, 2158, 65535, 1106, 841, 25, 26, 65535, 1045, 1460, 1835, 510, 856, 263, 366, 1539, 1668, 1807, 1746, 65535, 1970, 1419, 65535, 65535, 2005, 65535, 1458, 1549, 1100, 1513, 379, 2064, 2035, 1080, 820, 370, 224, 1489, 1512, 320, 347, 65535, 519, 340, 1569, 810, 412, 2270, 763, 324, 1682, 1233, 65535, 1349, 65535, 1574, 65535, 2238, 259, 466, 1484, 1954, 65535, 1253, 1251, 323, 1943, 0, 1955, 1609, 450, 1980, 1584, 198, 1987, 631, 2095, 695, 2026, 1210, 657, 2214, 2018, 65535, 2246, 65535, 2341, 2096, 65535, 1633, 1392, 1338, 1809, 138, 1308, 65535, 1883, 632, 1092, 1751, 65535, 1851, 65535, 65535, 1134, 750, 65535, 1662, 8, 65535, 65535, 37, 1105, 65535, 1104, 96, 65535, 65535, 2177, 1035, 1190, 65535, 65535, 65535, 2219, 541, 1076, 1416, 1753, 65535, 1164, 1812, 704, 65535, 390, 543, 65535, 1859, 2338, 65535, 1792, 1230, 885, 100, 1742, 2125, 1543, 505, 65535, 65535, 65535, 669, 65535, 1656, 1020, 1526, 65535, 275, 581, 2179, 1016, 1138, 1910, 1727, 960, 2115, 1004, 1511, 420, 632, 1501, 1752, 1984, 2274, 65535, 1028, 330, 1666, 1438, 1403, 406, 46, 65535, 65535, 1382, 2036, 2154, 1563, 109, 2279, 734, 65535, 893, 833, 714, 1711, 102, 1825, 1968, 2094, 1571, 65535, 65535, 1292, 2081, 2235, 65535, 1913, 830, 65535, 517, 786, 2214, 1032, 65535, 2039, 690, 62, 238, 1923, 1684, 65535, 1018, 340, 65535, 1637, 1025, 65535, 1738, 65535, 1202, 1749, 2258, 805, 511, 1769, 468, 1672, 91, 1129, 1247, 1096, 1227, 65535, 1412, 65535, 2312, 525, 552, 588, 65535, 65535, 2062, 65535, 65535, 14, 2020, 1195, 1416, 1848, 1470, 1542, 2008, 684, 65535, 2329, 65535, 1168, 285, 1828, 65535, 65535, 301, 1067, 414, 65535, 1627, 2332, 1446, 190, 616, 1988, 2213, 1097, 2069, 65535, 65535, 1223, 1819, 1774, 560, 1148, 1781, 826, 1529, 469, 59, 1615, 2043, 65535, 327, 983, 343, 855, 1727, 1990, 65535, 65535, 1025, 1913, 65535, 1054, 65535, 882, 65535, 2144, 1418, 65535, 1556, 2203, 549, 2370, 1874, 65535, 577, 65535, 2076, 65535, 883, 1261, 65535, 1294, 2207, 65535, 1839, 1202, 65535, 202, 1572, 2165, 1210, 2313, 398, 646, 1854, 1619, 1672, 1893, 2033, 319, 65535, 595, 2114, 65535, 65535, 1584, 65535, 811, 65535, 464, 65535, 65535, 715, 898, 1691, 1761, 348, 735, 65535, 166, 1770, 53, 2188, 65535, 848, 1940, 65535, 1686, 2321, 357, 1340, 2189, 65535, 1970, 102, 1361, 844, 936, 23, 1353, 1641, 2181, 1084, 642, 1610, 1247, 2297, 1359, 346, 1165, 708, 651, 1896, 65535, 702, 564, 65535, 1491, 65535, 1309, 124, 314, 935, 65535, 65535, 65535, 513, 789, 65535, 1461, 910, 1626, 1303, 2103, 971, 140, 1495, 2129, 583, 65535, 1185, 65535, 65535, 1762, 2093, 2290, 743, 949, 477, 2155, 1865, 682, 235, 65535, 1818, 1907, 65535, 207, 891, 591, 987, 1081, 65535, 1069, 1064, 640, 65535, 65535, 705, 580, 65535, 65535, 65535, 1104, 65535, 346, 1, 65535, 2006, 656, 362, 619, 65535, 65535, 1496, 2091, 389, 2162, 301, 65535, 1533, 267, 670, 1574, 89, 65535, 2108, 118, 848, 2382, 1748, 1849, 65535, 1690, 736, 849, 2136, 2224, 65535, 134, 561, 1514, 2246, 294, 65535, 65535, 1344, 1144, 809, 1882, 1035, 65535, 65535, 644, 230, 251, 42, 65535, 39, 1520, 1505, 65535, 65535, 65535, 2194, 1389, 2028, 113, 65535, 1741, 1784, 1945, 1883, 1487, 65535, 177, 2182, 418, 1279, 467, 1122, 2314, 923, 65535, 435, 1916, 65535, 65535, 536, 65535, 752, 65535, 1839, 1886, 65535, 461, 2047, 1357, 1430, 845, 1487, 2245, 65535, 1697, 1650, 65535, 694, 65535, 354, 65535, 14, 953, 1352, 1743, 205, 139, 1121, 846, 131, 2004, 118, 1252, 1950, 2194, 891, 2212, 65535, 2280, 65535, 170, 65535, 1525, 551, 2152, 2296, 2026, 65535, 1348, 65535, 2316, 496, 727, 687, 311, 301, 251, 789, 1074, 213, 1586, 65535, 1245, 179, 1040, 65535, 65535, 1783, 2203, 1561, 960, 65535, 65535, 1304, 1839, 65535, 1550, 65535, 575, 720, 65535, 2356, 1415, 65535, 516, 993, 2040, 1198, 65535, 371, 1093, 2159, 1581, 2132, 375, 65535, 1894, 581, 1525, 1738, 283, 1094, 677, 68, 436, 65535, 65535, 65535, 607, 1139, 146, 1489, 775, 1710, 197, 65535, 65535, 110, 65535, 2003, 65535, 2133, 2058, 1942, 65535, 65535, 2346, 65535, 389, 1190, 589, 2348, 65535, 65535, 65535, 65535, 65535, 65535, 65535, 793, 430, 1658, 65535, 1755, 65535, 65535, 46, 1962, 65535, 2295, 1888, 193, 65535, 340, 1264, 2018, 941, 1921, 1070, 1683, 1084, 1679, 222, 612, 65535, 504, 65535, 1088, 65535, 2305, 312, 2322, 65535, 65535, 633, 2065, 65535, 1300, 65535, 2239, 1538, 65535, 2347, 65535, 65535, 65535, 1960, 65535, 65535, 738, 65535, 1799, 65535, 1118, 2309, 65535, 1483, 229, 1278, 159, 1469, 1404, 292, 1271, 1275, 1769, 1602, 1515, 2008, 1827, 1803, 1046, 427, 65535, 65535, 5, 2004, 1103, 1895, 1934, 2019, 1908, 725, 1510, 65535, 638, 246, 1513, 195, 1221, 65535, 65535, 2207, 1271, 65535, 65535, 65535, 1259, 471, 65535, 65535, 65535, 1271, 65535, 874, 308, 1969, 728, 787, 490, 1466, 662, 1586, 2243, 65535, 569, 65535, 360, 1455, 1445, 1998, 65535, 65535, 2236, 373, 1616, 2165, 65535, 1136, 2326, 612, 1710, 1722, 65535, 75, 1881, 1959, 334, 369, 1841, 1285, 423, 1724, 65535, 65535, 1862, 2247, 932, 1813, 534, 446, 1973, 668, 65535, 65535, 2137, 143, 726, 117, 65535, 65535, 21, 65535, 1711, 65535, 957, 487, 67, 65535, 564, 2219, 2024, 1935, 1895, 1043, 65535, 2017, 122, 1829, 1997, 2264, 282, 239, 917, 2352, 1855, 2158, 279, 65535, 886, 2213, 841, 1536, 561, 1679, 65535, 65535, 1497, 64, 234, 65535, 2267, 65535, 65535, 1301, 1947, 1429, 1537, 703, 1026, 1891, 65535, 65535, 991, 65535, 1195, 207, 975, 612, 2106, 887, 229, 65535, 1209, 1300, 1355, 824, 2115, 347, 1392, 65535, 2157, 709, 590, 964, 1671, 1425, 65535, 65535, 2172, 65535, 65535, 44, 103, 65535, 65535, 81, 1271, 1950, 244, 1543, 236, 65535, 1173, 2033, 65535, 570, 65535, 971, 952, 65535, 2308, 950, 1270, 1923, 65535, 1048, 2237, 2038, 488, 510, 345, 65535, 65535, 1276, 937, 65535, 2078, 65535, 1289, 496, 636, 65535, 2340, 33, 1122, 1701, 65535, 1060, 1249, 65535, 759, 2260, 2333, 65535, 659, 65535, 1500, 65535, 482, 2320, 1821, 1215, 65535, 579, 65535, 65535, 193, 65535, 1539, 870, 1976, 65535, 65535, 2167, 65535, 65535, 11, 65535, 957, 2007, 65535, 1003, 680, 54, 1950, 7, 2238, 2063, 1130, 1681, 65535, 777, 545, 75, 65535, 669, 98, 65535, 65535, 389, 168, 1474, 146, 65535, 1450, 65535, 211, 2117, 430, 177, 722, 2180, 65535, 65535, 1586, 65535, 65535, 65535, 2107, 1736, 393, 93, 174, 2032, 1747, 65535, 435, 167, 698, 65535, 65535, 529, 193, 887, 159, 1195, 65535, 325, 790, 65535, 65535, 2259, 172, 23, 65535, 226, 2280, 65535, 1007, 2210, 65535, 1737, 1571, 893, 1473, 1200, 65535, 2279, 635, 65535, 15, 2197, 65535, 65535, 65535, 956, 413, 1668, 60, 65535, 439, 1262, 601, 2070, 1998, 65535, 1175, 2034, 2211, 141, 1019, 65535, 65535, 2013, 474, 65535, 1475, 65535, 424, 65535, 1753, 65535, 65535, 2235, 1462, 1187, 65535, 65535, 1254, 65535, 98, 1831, 1730, 1471, 507, 65535, 261, 1144, 2340, 851, 1854, 2137, 1592, 2086, 114, 1172, 402, 148, 1078, 65535, 2370, 1841, 65535, 2014, 65535, 1597, 65535, 2324, 65535, 65535, 65535, 239, 1156, 2152, 1533, 65535, 65535, 65535, 65535, 764, 237, 65535, 65535, 1680, 1111, 7, 1700, 1832, 65535, 2331, 1761, 1530, 2241, 65535, 65535, 416, 116, 2228, 1594, 1045, 1766, 2375, 1005, 65535, 648, 1522, 2325, 65535, 2340, 530, 322, 1499, 826, 629, 1449, 954, 65535, 1746, 85, 65535, 65535, 1923, 445, 437, 828, 1629, 659, 2308, 788, 1522, 1626, 859, 2247, 2040, 654, 65535, 2052, 157, 1451, 594, 65535, 2112, 1907, 1364, 946, 1857, 65535, 60, 649, 307, 1183, 1712, 305, 65535, 625, 225, 1669, 65535, 2192, 1905, 65535, 297, 1465, 2132, 1696, 2135, 1998, 2015, 298, 2113, 1801, 2179, 65535, 1706, 847, 65535, 1787, 1064, 1646, 411, 138, 1462, 2262, 909, 949, 678, 1844, 1280, 2343, 65535, 1965, 1306, 2242, 857, 65535, 1538, 385, 1649, 229, 163, 65535, 1662, 1420, 2379, 347, 65535, 1559, 65535, 1332, 65535, 2386, 880, 2099, 1773, 35, 1729, 382, 1518, 1974, 1164, 357, 2134, 542, 65535, 1331, 995, 1527, 1521, 2364, 65535, 1510, 65535, 1499, 257, 65535, 1545, 661, 17, 939, 2269, 453, 1616, 411, 1661, 668, 278, 1027, 402, 65535, 478, 2284, 902, 1445, 463, 2027, 1146, 974, 1512, 1260, 65535, 2310, 1161];
        static HASH_TABLE_2: [u16; 1869] = [1615, 249, 65535, 0, 1507, 2144, 0, 87, 1738, 2061, 0, 1101, 65535, 65535, 622, 588, 802, 65535, 65535, 270, 972, 65535, 882, 65535, 260, 2369, 1909, 1533, 0, 484, 881, 1427, 1137, 1853, 65535, 266, 2302, 1797, 0, 969, 505, 1045, 65535, 1675, 0, 1287, 65535, 1087, 394, 327, 339, 0, 884, 596, 65535, 925, 65535, 41, 1167, 1003, 0, 1709, 0, 0, 65535, 724, 65535, 65535, 690, 0, 65535, 296, 2104, 1874, 0, 1203, 65535, 0, 0, 955, 982, 0, 11, 1612, 1356, 162, 65535, 90, 65535, 1231, 65535, 65535, 65535, 342, 0, 1681, 1962, 292, 1539, 1193, 2236, 858, 0, 2362, 65535, 65535, 0, 2315, 65535, 0, 2390, 187, 550, 2126, 65535, 0, 65535, 65535, 355, 1741, 65535, 0, 2184, 235, 0, 289, 65535, 0, 0, 694, 65535, 800, 910, 864, 1240, 39, 637, 1179, 1044, 358, 1644, 0, 0, 0, 1754, 0, 1395, 1229, 65535, 268, 1483, 424, 673, 680, 2087, 1004, 120, 1413, 65535, 2183, 65535, 728, 964, 912, 0, 65535, 0, 632, 2006, 488, 0, 65535, 1023, 199, 851, 65535, 1816, 0, 65535, 1207, 0, 1319, 2048, 65535, 2231, 831, 0, 65535, 65535, 2062, 65535, 0, 2115, 0, 1645, 436, 2130, 1769, 2298, 0, 2384, 0, 65535, 65535, 65535, 0, 1243, 65535, 502, 0, 1329, 0, 0, 1571, 271, 0, 65535, 129, 1478, 19, 2197, 65535, 1468, 65535, 1886, 0, 65535, 1479, 1838, 0, 65535, 1876, 434, 0, 65535, 0, 65535, 0, 583, 0, 0, 1327, 1870, 65535, 997, 65535, 2314, 0, 516, 65535, 65535, 1225, 2249, 1206, 2235, 65535, 1841, 65535, 0, 1511, 65535, 96, 65535, 955, 65535, 283, 1784, 503, 0, 0, 6, 65535, 65535, 557, 1412, 2210, 1823, 68, 65535, 2042, 0, 786, 71, 292, 65535, 1154, 0, 1908, 1983, 65535, 0, 2208, 1330, 0, 0, 826, 0, 65535, 65535, 0, 65535, 44, 832, 80, 1949, 991, 1755, 65535, 0, 65535, 865, 1782, 2184, 1726, 65535, 1091, 2052, 65535, 65535, 233, 251, 65535, 65535, 72, 1115, 65535, 0, 868, 1815, 6, 2285, 65535, 65535, 65535, 1154, 1965, 524, 0, 0, 65535, 370, 0, 1331, 1426, 576, 65535, 65535, 0, 65535, 456, 65535, 65535, 0, 788, 0, 1015, 1243, 0, 1916, 0, 65535, 1128, 0, 65535, 0, 221, 65535, 1692, 65535, 65535, 218, 65535, 0, 0, 0, 65535, 65, 0, 592, 65535, 434, 0, 1420, 1218, 0, 1242, 632, 1498, 1055, 0, 0, 1870, 1697, 401, 490, 1503, 364, 413, 0, 1281, 1315, 65535, 665, 0, 65535, 1772, 463, 0, 0, 1533, 65535, 65535, 0, 65535, 65535, 1716, 0, 65535, 65535, 2382, 2356, 600, 1816, 1355, 1235, 268, 65535, 0, 847, 172, 65535, 0, 0, 685, 366, 1389, 1056, 0, 65535, 1955, 0, 559, 297, 0, 65535, 1369, 65535, 65535, 1389, 65535, 1964, 629, 496, 65535, 420, 2170, 65535, 474, 87, 0, 1636, 905, 1884, 1757, 65535, 262, 0, 2159, 0, 2372, 0, 0, 1437, 0, 65535, 680, 0, 1785, 5, 1471, 267, 76, 108, 2063, 0, 65535, 65535, 1345, 1844, 0, 1207, 65535, 1108, 65535, 1262, 400, 237, 0, 1467, 0, 1074, 585, 1665, 0, 540, 65535, 701, 694, 1499, 1242, 65535, 2057, 65535, 1759, 724, 266, 65535, 2147, 1684, 1950, 65535, 65535, 65535, 586, 1853, 65535, 0, 0, 0, 65535, 65535, 2227, 0, 0, 0, 65535, 2041, 0, 1101, 1209, 0, 0, 65535, 2029, 2379, 0, 1507, 65535, 567, 120, 782, 65535, 578, 879, 0, 149, 0, 512, 0, 2166, 880, 1102, 1807, 65535, 1309, 301, 0, 65535, 65535, 65535, 65535, 886, 1726, 697, 896, 521, 2165, 530, 65535, 1055, 65535, 65535, 0, 2059, 1256, 65535, 0, 1420, 65535, 1351, 631, 830, 815, 1590, 65535, 65535, 1873, 1272, 65535, 0, 1221, 1441, 65535, 0, 0, 0, 2151, 65535, 0, 65535, 65535, 594, 166, 1347, 1523, 632, 930, 65535, 511, 0, 1776, 1499, 0, 65535, 1960, 427, 274, 505, 65535, 0, 1200, 65535, 65535, 0, 1000, 0, 65535, 65535, 0, 65535, 864, 1160, 1068, 0, 1327, 0, 600, 0, 65535, 0, 65535, 1142, 91, 1167, 65535, 65535, 0, 38, 673, 1657, 0, 1046, 865, 21, 0, 1817, 65535, 65535, 65535, 1462, 2084, 294, 1596, 0, 778, 295, 2048, 65535, 2192, 1853, 65535, 717, 65535, 552, 65535, 65535, 65535, 65535, 157, 729, 0, 74, 65535, 1154, 30, 1492, 1351, 1244, 65535, 617, 374, 65535, 1560, 1464, 65535, 0, 194, 164, 65535, 1320, 2142, 707, 65535, 1831, 1697, 0, 794, 112, 2203, 65535, 65535, 0, 1995, 353, 65535, 1839, 0, 65535, 10, 12, 65535, 0, 2028, 403, 65535, 0, 0, 65535, 65535, 2268, 0, 1584, 65535, 65535, 65535, 0, 597, 555, 65535, 626, 0, 65535, 1379, 65535, 968, 154, 65535, 1070, 0, 1289, 65535, 0, 65535, 65535, 1115, 0, 493, 676, 2003, 0, 65535, 65535, 280, 2171, 65535, 1460, 0, 65535, 2394, 1740, 1152, 1586, 65535, 65535, 213, 1795, 0, 65535, 65535, 2113, 2270, 0, 0, 0, 1226, 1644, 65535, 411, 0, 65535, 869, 0, 65535, 1234, 65535, 65535, 65535, 1043, 0, 476, 2396, 0, 158, 0, 0, 1733, 1316, 1770, 0, 0, 1164, 1666, 164, 1323, 0, 1839, 1247, 1490, 159, 1117, 1772, 0, 1800, 1365, 0, 1760, 0, 485, 0, 0, 495, 65535, 65535, 65535, 0, 65535, 65535, 975, 65535, 0, 1127, 2168, 1705, 65535, 208, 65535, 600, 65535, 541, 736, 0, 0, 65535, 489, 65535, 1852, 65535, 65535, 0, 0, 65535, 65535, 65535, 1282, 385, 1580, 391, 65535, 1754, 0, 1227, 0, 65535, 65535, 65535, 65535, 146, 65535, 2309, 0, 65535, 0, 0, 2368, 2030, 65535, 65535, 1842, 65535, 65535, 65535, 65535, 59, 0, 0, 65535, 531, 2131, 1932, 65535, 2257, 65535, 0, 688, 184, 21, 0, 2073, 0, 1643, 0, 0, 65535, 883, 1931, 65535, 154, 1223, 0, 55, 65535, 2140, 1859, 65535, 792, 65535, 65535, 2019, 1772, 1075, 0, 65535, 65535, 1473, 65535, 2093, 988, 0, 1313, 827, 65535, 1586, 65535, 65535, 1267, 302, 1624, 189, 341, 107, 0, 894, 65535, 65535, 1384, 1503, 821, 197, 65535, 306, 1184, 0, 1867, 784, 1100, 647, 1489, 65535, 0, 119, 65535, 1204, 0, 75, 2191, 511, 1829, 685, 787, 65535, 624, 65535, 0, 1947, 0, 0, 65535, 0, 571, 1735, 1910, 1887, 530, 789, 446, 1450, 65535, 1697, 1321, 65535, 1343, 1127, 0, 0, 1988, 495, 65535, 65535, 0, 761, 83, 65535, 65535, 1489, 1936, 1493, 1179, 0, 1569, 2311, 2324, 65535, 65535, 1724, 0, 1855, 0, 65535, 0, 816, 65535, 0, 65535, 1365, 65535, 0, 65535, 2118, 0, 0, 1758, 0, 65535, 65535, 1246, 2388, 1731, 1471, 65535, 65535, 1853, 65535, 111, 65535, 65535, 65535, 1168, 804, 386, 65535, 146, 0, 1668, 65535, 0, 97, 65535, 478, 1172, 2111, 65535, 1824, 1617, 308, 374, 0, 1326, 981, 1575, 1473, 301, 1600, 0, 65535, 2050, 203, 0, 65535, 65535, 65535, 65535, 1688, 0, 0, 65535, 65535, 1204, 0, 473, 135, 308, 65535, 65535, 200, 1798, 65535, 0, 680, 65535, 0, 2296, 1192, 65535, 65535, 289, 1758, 65535, 65535, 0, 65535, 541, 1686, 348, 65535, 0, 65535, 0, 0, 1440, 2317, 2203, 0, 65535, 0, 65535, 1932, 65535, 65535, 1797, 65535, 1453, 679, 106, 65535, 0, 0, 313, 1577, 798, 0, 65535, 1439, 35, 1419, 185, 65535, 1914, 867, 415, 1874, 2199, 1302, 65535, 477, 1295, 239, 65535, 572, 65535, 1306, 176, 612, 20, 65535, 1126, 1366, 65535, 1548, 0, 65535, 65535, 65535, 567, 65535, 65535, 483, 65535, 0, 0, 1518, 8, 1467, 1156, 2037, 0, 65535, 1678, 0, 1263, 0, 0, 0, 65535, 65535, 65535, 65535, 2015, 65535, 1101, 760, 0, 890, 0, 1946, 0, 65535, 2340, 1936, 1778, 65535, 0, 65535, 0, 65535, 933, 0, 897, 2253, 784, 2213, 1231, 2186, 65535, 65535, 345, 706, 2070, 589, 2012, 1223, 0, 1944, 0, 1831, 1529, 0, 401, 1788, 0, 2332, 2327, 1330, 0, 818, 1400, 2250, 50, 65535, 0, 65535, 0, 161, 19, 238, 600, 65535, 1120, 65535, 65535, 65535, 1505, 65535, 814, 1002, 0, 1534, 65535, 1677, 2354, 1722, 1970, 1575, 65535, 0, 65535, 689, 282, 1254, 2107, 1212, 0, 680, 1207, 65535, 184, 0, 0, 2140, 238, 1934, 0, 2366, 65535, 2300, 1803, 1860, 65535, 804, 65535, 0, 0, 1979, 1915, 65535, 65535, 2094, 0, 773, 0, 0, 1655, 0, 1026, 0, 99, 1375, 1747, 0, 65535, 0, 65535, 862, 65535, 2035, 1910, 179, 266, 65535, 65535, 1311, 65535, 0, 1657, 1358, 0, 2143, 65535, 65535, 65535, 2274, 65535, 1839, 1056, 62, 65535, 1245, 65535, 65535, 1480, 65535, 1536, 0, 65535, 698, 2350, 2006, 1668, 1241, 636, 1616, 1296, 65535, 0, 1516, 1299, 0, 1513, 0, 281, 0, 1854, 1486, 1711, 0, 284, 1763, 0, 65535, 65535, 65535, 65535, 0, 1699, 1104, 65535, 303, 0, 65535, 0, 28, 2003, 65535, 330, 1190, 1069, 182, 0, 0, 0, 584, 0, 212, 155, 65535, 64, 377, 0, 0, 495, 1355, 65535, 428, 1152, 1772, 1948, 2087, 799, 0, 919, 0, 1590, 0, 310, 65535, 0, 1235, 745, 1654, 260, 65535, 65535, 65535, 0, 570, 832, 1210, 1740, 1082, 1711, 919, 410, 0, 0, 2113, 65535, 146, 65535, 0, 65535, 279, 2073, 0, 65535, 65535, 1662, 656, 0, 1682, 0, 65535, 1428, 2250, 65535, 1978, 0, 1456, 65535, 0, 65535, 0, 820, 375, 1252, 65535, 1676, 0, 738, 0, 65535, 549, 2243, 1681, 0, 0, 1784, 447, 575, 0, 2252, 65535, 2055, 0, 65535, 65535, 0, 936, 865, 65535, 65535, 78, 0, 2059, 65535, 566, 65535, 0, 1544, 65535, 65535, 0, 65535, 1271, 595, 7, 569, 65535, 65535, 0, 65535, 2305, 593, 687, 740, 0, 0, 1409, 1551, 819, 1379, 65535, 2192, 0, 65535, 65535, 65535, 1721, 1507, 555, 1304, 828, 65535, 0, 0, 872, 65535, 65535, 65535, 1735, 1110, 397, 199, 61, 860, 0, 724, 1466, 1693, 1339, 1041, 1434, 238, 1257, 1304, 504, 1102, 432, 2047, 88, 65535, 65535, 1949, 20, 0, 610, 65535, 0, 1031, 1893, 65535, 2096, 1483, 515, 5, 2368, 435, 65535, 0, 65535, 65535, 0, 1436, 65535, 532, 392, 65535, 1408, 65535, 1632, 2187, 0, 2376, 2180, 1225, 0, 0, 2219, 0, 65535, 1259, 65535, 2151, 65535, 65535, 0, 1916, 2378, 65535, 65535, 518, 65535, 65535, 354, 1756, 0, 65535, 65535, 0, 1600, 0, 1858, 65535, 0, 1992, 418, 0, 467, 65535, 0, 1030, 65535, 468, 347, 0, 65535, 65535, 2240, 0, 0, 65535, 1992, 65535, 65535, 1641, 0, 0, 0, 65535, 1549, 0, 0, 1290, 2348, 65535, 65535, 0, 1601, 65535, 884, 1720, 0, 65535, 1793, 1055, 65535, 322, 1105, 2239, 707, 1657, 0, 65535, 0, 1082, 65535, 0, 0, 1325, 65535, 967, 506, 1446, 103, 65535, 65535, 65535, 0, 0, 433, 0, 2189, 65535, 479, 1447, 0, 903, 0, 65535, 1604, 1683, 1528, 65535, 1494, 550, 0, 1890, 535, 65535, 868, 1164, 2388, 65535, 284, 27, 335, 1146, 1885, 179, 0, 65535, 0, 65535, 65535, 436, 1760, 0, 65535, 65535, 65535, 0, 0, 0, 0, 2153, 2273, 413, 1974, 754, 65535, 1659, 835, 65535, 0, 65535, 22, 2070, 65535, 65535, 956, 2361, 1115, 451, 65535, 1660, 237, 65535, 1651, 1466, 613, 0, 65535, 0, 65535, 1830, 65535, 0, 873, 1422, 1882, 1576, 1325, 0, 1947, 0, 1998, 1432, 481, 749, 0, 2143, 65535, 0, 1041, 65535, 1818, 385, 423, 65535, 65535, 65535, 451, 0, 2207, 65535, 576, 65535, 0, 155, 1099, 65535, 1872, 2080, 65535, 269, 1963, 0, 1973, 0, 1048, 0, 65535, 0, 493, 631, 65535, 1552, 1978, 1053, 65535, 65535, 273, 454, 0, 65535, 399, 1454, 796, 65535, 1708, 204, 1495, 951, 65535, 0, 0, 838, 2114, 1985, 1060, 0];
        let hashval1: usize = hashfunc(input, 27359);
        let hashval2: usize = hashfunc(input, 20733);
        let val1 = HASH_TABLE_1[hashval1 % 1869];
        let val2 = HASH_TABLE_2[hashval2 % 1869];
        if val1 == u16::MAX || val2 == u16::MAX {
            return Err(ParseEnumItemError);
        }
        let item_idx = (val1 + val2) as usize % 2398;
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

