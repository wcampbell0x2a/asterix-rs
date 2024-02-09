//! Enums used for providing common meaning for bits in a `data_item`

use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "3")]
pub enum TYP {
    NoDetection = 0x00,
    SinglePSRDetection = 0x01,
    SingleSSRDetection = 0x02,
    SSRPlusPSRDetection = 0x03,
    SingleModeSAllCall = 0x04,
    SingleModeSRollCall = 0x05,
    ModeSAllCallPlusPSR = 0x06,
    ModeSRollCallPlusPSR = 0x07,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum SIM {
    ActualTargetReport = 0x00,
    SimulatedTargetReport = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum RDP {
    ReportFromRDPChain1 = 0x00,
    ReportFromRDPChain2 = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum SPI {
    AbsenceOfSPI = 0x00,
    SpecialPositionIdentification = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum RAB {
    ReportFromAircraftTransponder = 0x00,
    ReportFromFieldMonitor = 0x01,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum FX {
    EndOfDataItem = 0x00,
    ExtensionIntoFirstExtent = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum V {
    CodeValidated = 0x00,
    CodeNotValidated = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum G {
    Default = 0x00,
    GarbledCode = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum L {
    Mode3CodeDerivedFromTheReplyOfTheTransponder = 0x00,
    Mode3CodeNotExtractedDuringTheLastScan = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum CNF {
    ConfirmedTrack = 0x00,
    TentativeTrack = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "2")]
pub enum RAD {
    CombinedTrack = 0x00,
    PSRTrack = 0x01,
    SSRModeSTrack = 0x02,
    Invalid = 0x03,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum DOU {
    NormalConfidence = 0x00,
    LowConfidence = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum MAH {
    NoHorizontalManSensed = 0x00,
    HorizontalManSensed = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "2")]
pub enum CDM {
    Maintaining = 0x00,
    Climbing = 0x01,
    Descending = 0x02,
    Unknown = 0x03,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum TRE {
    TrackStillAlive = 0x00,
    EndOfTrackLifetime = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum GHO {
    TrueTargetTrack = 0x00,
    GhostTargetTrack = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum SUP {
    No = 0x00,
    Yes = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum TCC {
    RadarPlanePlotTransformation = 0x00,
    SlantRangePlotTransformation = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "3")]
pub enum COM {
    NoCommunicationsSurveillanceOnly = 0x00,
    CommACommB = 0x01,
    CommACommBUplinkELM = 0x02,
    CommACommBUplinkELMDownlinkELM = 0x03,
    Top5TransponderCapability = 0x04,
    #[deku(id_pat = "0x05..=0x07")]
    NoAssigned,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "3")]
pub enum STAT {
    NoAlertNoSPIAircraftAirborne = 0x00,
    NoAlertNoSPIAircraftOnGround = 0x01,
    AlertNoSPIAircraftAirborne = 0x02,
    AlertNoSPIAircraftOnGround = 0x03,
    AlertSPIAircraftAirborneOrOnGround = 0x04,
    NoAlertSPIAircraftAirborneOrOnGround = 0x05,
    NotAssigned = 0x06,
    Unknown = 0x07,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum SI {
    SICodeCapable = 0x00,
    IICodeCapable = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum MSSC {
    No = 0x00,
    Yes = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum ARC {
    Resolution100ft = 0x00,
    Resolution25ft = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum AIC {
    No = 0x00,
    Yes = 0x01,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "8")]
pub enum MTYPE {
    NorthMarker = 0x01,
    SectorCrossing = 0x02,
    GeographicaFiltering = 0x03,
    JammingStrobe = 0x04,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u16", bits = "7")]
pub enum CODE {
    #[deku(id = "0")]
    NotDefined,
    #[deku(id = "1")]
    MultipathReply,
    #[deku(id = "2")]
    ReplySidelobeInterrogationReception,
    #[deku(id = "3")]
    SplitPlot,
    #[deku(id = "4")]
    SecondTimeAroundReply,
    #[deku(id = "5")]
    Angel,
    #[deku(id = "6")]
    SlowMovingTarget,
    #[deku(id = "7")]
    FixedPSRPlot,
    #[deku(id = "8")]
    SlowPSRPlot,
    #[deku(id = "9")]
    LowQualityPSRPlot,
    #[deku(id = "10")]
    PhantomSSRPlot,
    #[deku(id = "11")]
    NonMatchingMode3ACode,
    #[deku(id = "12")]
    ModeCCodeModeSAbnormal,
    #[deku(id = "13")]
    TargetInClutter,
    #[deku(id = "14")]
    MaximumDopplerREsponseInZeroFilter,
    #[deku(id = "15")]
    TransponderAnomalyDetected,
    #[deku(id = "16")]
    DuplicatedOrIllegalModeSAircraftAddress,
    #[deku(id = "17")]
    ModeSErrorCorrectionApplied,
    #[deku(id = "18")]
    UndecodableModeCSCode,
    #[deku(id = "19")]
    Birds,
    #[deku(id = "20")]
    FlockOfBirds,
    #[deku(id = "21")]
    Mode1PresentOriginalReply,
    #[deku(id = "22")]
    Mode2PresentOriginalReply,
    #[deku(id = "23")]
    PlotCausedByWindTurbine,
    #[deku(id = "24")]
    Helicopter,
    #[deku(id = "25")]
    MaxiumumNumberInterrogationsSurveillance,
    #[deku(id = "26")]
    MaxiumumNumberInterrogationsBDS,
    #[deku(id = "27")]
    BDSOverlayIncoherence,
    #[deku(id = "28")]
    PotentialBDSSwapDetected,
    #[deku(id = "29")]
    TrackUpdateZenithalGap,
    #[deku(id = "30")]
    ModeSTrackReAquired,
    #[deku(id = "31")]
    DuplicatedMode5PairNoPinDetected,
    #[deku(id = "32")]
    WrongDFReplyFormatDetected,
    #[deku(id = "33")]
    TransponderAnomalyMs,
    #[deku(id = "34")]
    TransponderAnomalySI,
    #[deku(id = "35")]
    PotentialICConflict,
    #[deku(id = "36")]
    ICConflictDetectionPossible,
    #[deku(id_pat = "37..=63")]
    AMGAllocated(u16),
    #[deku(id_pat = "64..=127")]
    Manufacturer(u16),
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum D {
    Valid = 0,
    Doubtful = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Operational Release Status of the System
pub enum NOGO {
    SystemIsReleasedForOperationalUse = 0,
    OperationalUseOfSystemIsInhibited = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Radar Data Processor Chain Selection Status
pub enum RDPC {
    RDPC1Selected = 0,
    RDPC2Selected = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Event to signal a reset/restart of the selected Radar Data Processor Chain,
/// i.e. expect a new assignment of track numbers
pub enum RDPR {
    DefaultSituation = 0,
    ResetOfRDPC = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Monitoring System Connected Status
pub enum MSC {
    MonitoringSystemConnected = 0,
    MonitoringSystemDisconnected = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Time Source Validity
pub enum TSV {
    Valid = 0,
    Invalid = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Selected Antenna
pub enum ANT {
    Antenna1 = 0,
    Antenna2 = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "2")]
/// Channel A/B Selection Status
pub enum CHAB {
    NoChannelSelected = 0b00,
    ChannelAOnlySelected = 0b01,
    ChannelBOnlySelected = 0b10,
    DiversityMode = 0b11,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Overload Condition
pub enum OVL {
    NoOverload = 0,
    Overload = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Channel A/B selection status for Surveillance Co-ordination Function
pub enum SCF {
    ChannelAInUse = 0,
    ChannelBInUse = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
/// Channel A/B selection status for Data Link Function
pub enum DLF {
    ChannelAInUse = 0,
    ChannelBInUse = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "3")]
pub enum RED {
    NoReductionActive = 0b000,
    ReductionStep1Active = 0b001,
    ReductionStep2Active = 0b010,
    ReductionStep3Active = 0b011,
    ReductionStep4Active = 0b100,
    ReductionStep5Active = 0b101,
    ReductionStep6Active = 0b110,
    ReductionStep7Active = 0b111,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum POL {
    LinearPolarization = 0,
    CircularPolarization = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum STC {
    STCMap1 = 0b00,
    STCMap2 = 0b01,
    STCMap3 = 0b10,
    STCMap4 = 0b11,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum CLU {
    Autonomous = 0,
    #[deku(id = "1")]
    NotAutonomous = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "5")]
pub enum MessageCounterTYP {
    NoDetection = 0,
    SinglePSRTargetReports = 1,
    SingleSSRTargetReports = 2,
    SSRPSRTargetReports = 3,
    SingleAllCallTargetReports = 4,
    SingleRollCallTargetReports = 5,
    AllCallPSRModeSTargetReports = 6,
    RollCallPSRModeSTargetReports = 7,
    FilterForWeatherData = 8,
    FilterForJammingStrobe = 9,
    FilterPSRData = 10,
    FilterSSRModeSData = 11,
    FilterSSRModeSPSRData = 12,
    FilterForEnhancedSuveillanceData = 13,
    FilterForPSREnhancedSurveillance = 14,
    FilterForPSREnhancedSurveillancePlusSSRModeSDataNotInAreaOfPrimeInterest = 15,
    FilterForPSREnhancedSurveillancePlusAllSSRModeSData = 16,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "8")]
pub enum DataFilterTYP {
    InvalidValue = 0,
    FilterWeatherData = 1,
    FilterJammingStrobe = 2,
    FilterPSRData = 3,
    FilterSSRModeSData = 4,
    FilterSSRModeSPSRData = 5,
    EnhancedSurveillanceData = 6,
    FilterPSREnhancedSurveillanceData = 7,
    FilterPSREnhancedSurveillanceSSRModeSDataNotInAreaOfPrimeInterest = 8,
    FilterPSREnhancedSurveillanceAllSSRModeSData = 9,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum TST {
    RealTargetReport = 0,
    TestTargetReport = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum ERR {
    NoExtendedRange = 0,
    ExtendedRangePresent = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum XPP {
    NoXPulsePresent = 0,
    XPulsePresent = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum ME {
    NoMilitaryEmergency = 0,
    MilitaryEmergency = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "1")]
pub enum MI {
    NoMilitaryIdentification = 0,
    MilitaryIdentification = 1,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(type = "u8", bits = "2")]
pub enum FOEFRI {
    NoMode4Interrogation = 0b00,
    FriendlyTarget = 0b01,
    UnknownTarget = 0b10,
    NoReply = 0b11,
}
