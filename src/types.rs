//! Enums used for providing common meaning for bits in a `data_item`

use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "3")]
pub enum TYP {
    #[deku(id = "0x00")]
    NoDetection,
    #[deku(id = "0x01")]
    SinglePSRDetection,
    #[deku(id = "0x02")]
    SingleSSRDetection,
    #[deku(id = "0x03")]
    SSRPlusPSRDetection,
    #[deku(id = "0x04")]
    SingleModeSAllCall,
    #[deku(id = "0x05")]
    SingleModeSRollCall,
    #[deku(id = "0x06")]
    ModeSAllCallPlusPSR,
    #[deku(id = "0x07")]
    ModeSRollCallPlusPSR,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum SIM {
    #[deku(id = "0x00")]
    ActualTargetReport,
    #[deku(id = "0x01")]
    SimulatedTargetReport,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum RDP {
    #[deku(id = "0x00")]
    ReportFromRDPChain1,
    #[deku(id = "0x01")]
    ReportFromRDPChain2,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum SPI {
    #[deku(id = "0x00")]
    AbsenceOfSPI,
    #[deku(id = "0x01")]
    SpecialPositionIdentification,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum RAB {
    #[deku(id = "0x00")]
    ReportFromAircraftTransponder,
    #[deku(id = "0x01")]
    ReportFromFieldMonitor,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum FX {
    #[deku(id = "0x00")]
    EndOfDataItem = 0,
    #[deku(id = "0x01")]
    ExtensionIntoFirstExtent = 1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum V {
    #[deku(id = "0x00")]
    CodeValidated = 0,
    #[deku(id = "0x01")]
    CodeNotValidated = 1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum G {
    #[deku(id = "0x00")]
    Default = 0,
    #[deku(id = "0x01")]
    GarbledCode = 1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum L {
    #[deku(id = "0x00")]
    Mode3CodeDerivedFromTheReplyOfTheTransponder = 0,
    #[deku(id = "0x01")]
    Mode3CodeNotExtractedDuringTheLastScan = 1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum CNF {
    #[deku(id = "0x00")]
    ConfirmedTrack,
    #[deku(id = "0x01")]
    TentativeTrack,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "2")]
pub enum RAD {
    #[deku(id = "0x00")]
    CombinedTrack,
    #[deku(id = "0x01")]
    PSRTrack,
    #[deku(id = "0x02")]
    SSRModeSTrack,
    #[deku(id = "0x03")]
    Invalid,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum DOU {
    #[deku(id = "0x00")]
    NormalConfidence,
    #[deku(id = "0x01")]
    LowConfidence,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum MAH {
    #[deku(id = "0x00")]
    NoHorizontalManSensed,
    #[deku(id = "0x01")]
    HorizontalManSensed,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "2")]
pub enum CDM {
    #[deku(id = "0x00")]
    Maintaining,
    #[deku(id = "0x01")]
    Climbing,
    #[deku(id = "0x02")]
    Descending,
    #[deku(id = "0x03")]
    Unknown,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum TRE {
    #[deku(id = "0x00")]
    TrackStillAlive,
    #[deku(id = "0x01")]
    EndOfTrackLifetime,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum GHO {
    #[deku(id = "0x00")]
    TrueTargetTrack,
    #[deku(id = "0x01")]
    GhostTargetTrack,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum SUP {
    #[deku(id = "0x00")]
    No,
    #[deku(id = "0x01")]
    Yes,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum TCC {
    #[deku(id = "0x00")]
    RadarPlanePlotTransformation,
    #[deku(id = "0x01")]
    SlantRangePlotTransformation,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "3")]
pub enum COM {
    #[deku(id = "0x00")]
    NoCommunicationsSurveillanceOnly,
    #[deku(id = "0x01")]
    CommACommB,
    #[deku(id = "0x02")]
    CommACommBUplinkELM,
    #[deku(id = "0x03")]
    CommACommBUplinkELMDownlinkELM,
    #[deku(id = "0x04")]
    Top5TransponderCapability,
    #[deku(id_pat = "0x05..=0x07")]
    NoAssigned,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "3")]
pub enum STAT {
    #[deku(id = "0x00")]
    NoAlertNoSPIAircraftAirborne,
    #[deku(id = "0x01")]
    NoAlertNoSPIAircraftOnGround,
    #[deku(id = "0x02")]
    AlertNoSPIAircraftAirborne,
    #[deku(id = "0x03")]
    AlertNoSPIAircraftOnGround,
    #[deku(id = "0x04")]
    AlertSPIAircraftAirborneOrOnGround,
    #[deku(id = "0x05")]
    NoAlertSPIAircraftAirborneOrOnGround,
    #[deku(id = "0x06")]
    NotAssigned,
    #[deku(id = "0x07")]
    Unknown,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum SI {
    #[deku(id = "0x00")]
    SICodeCapable,
    #[deku(id = "0x01")]
    IICodeCapable,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum MSSC {
    #[deku(id = "0x00")]
    No,
    #[deku(id = "0x01")]
    Yes,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum ARC {
    #[deku(id = "0x00")]
    Resolution100ft,
    #[deku(id = "0x01")]
    Resolution25ft,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum AIC {
    #[deku(id = "0x00")]
    No,
    #[deku(id = "0x01")]
    Yes,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "8")]
pub enum MTYPE {
    #[deku(id = "0x01")]
    NorthMarker,
    #[deku(id = "0x02")]
    SectorCrossing,
    #[deku(id = "0x03")]
    GeographicaFiltering,
    #[deku(id = "0x04")]
    JammingStrobe,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u16", id_bits = "7")]
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
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum D {
    #[deku(id = "0")]
    Valid,
    #[deku(id = "1")]
    Doubtful,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Operational Release Status of the System
pub enum NOGO {
    #[deku(id = "0")]
    SystemIsReleasedForOperationalUse,
    #[deku(id = "1")]
    OperationalUseOfSystemIsInhibited,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Radar Data Processor Chain Selection Status
pub enum RDPC {
    #[deku(id = "0")]
    RDPC1Selected,
    #[deku(id = "1")]
    RDPC2Selected,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Event to signal a reset/restart of the selected Radar Data Processor Chain,
/// i.e. expect a new assignment of track numbers
pub enum RDPR {
    #[deku(id = "0")]
    DefaultSituation,
    #[deku(id = "1")]
    ResetOfRDPC,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Monitoring System Connected Status
pub enum MSC {
    #[deku(id = "0")]
    MonitoringSystemConnected,
    #[deku(id = "1")]
    MonitoringSystemDisconnected,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Time Source Validity
pub enum TSV {
    #[deku(id = "0")]
    Valid,
    #[deku(id = "1")]
    Invalid,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Selected Antenna
pub enum ANT {
    #[deku(id = "0")]
    Antenna1,
    #[deku(id = "1")]
    Antenna2,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "2")]
/// Channel A/B Selection Status
pub enum CHAB {
    #[deku(id = "0b00")]
    NoChannelSelected,
    #[deku(id = "0b01")]
    ChannelAOnlySelected,
    #[deku(id = "0b10")]
    ChannelBOnlySelected,
    #[deku(id = "0b11")]
    DiversityMode,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Overload Condition
pub enum OVL {
    #[deku(id = "0")]
    NoOverload,
    #[deku(id = "1")]
    Overload,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Channel A/B selection status for Surveillance Co-ordination Function
pub enum SCF {
    #[deku(id = "0")]
    ChannelAInUse,
    #[deku(id = "1")]
    ChannelBInUse,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
/// Channel A/B selection status for Data Link Function
pub enum DLF {
    #[deku(id = "0")]
    ChannelAInUse,
    #[deku(id = "1")]
    ChannelBInUse,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "3")]
pub enum RED {
    #[deku(id = "0b000")]
    NoReductionActive,
    #[deku(id = "0b001")]
    ReductionStep1Active,
    #[deku(id = "0b010")]
    ReductionStep2Active,
    #[deku(id = "0b011")]
    ReductionStep3Active,
    #[deku(id = "0b100")]
    ReductionStep4Active,
    #[deku(id = "0b101")]
    ReductionStep5Active,
    #[deku(id = "0b110")]
    ReductionStep6Active,
    #[deku(id = "0b111")]
    ReductionStep7Active,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum POL {
    #[deku(id = "0")]
    LinearPolarization,
    #[deku(id = "1")]
    CircularPolarization,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum STC {
    #[deku(id = "0b00")]
    STCMap1,
    #[deku(id = "0b01")]
    STCMap2,
    #[deku(id = "0b10")]
    STCMap3,
    #[deku(id = "0b11")]
    STCMap4,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1")]
pub enum CLU {
    #[deku(id = "0")]
    Autonomous,
    #[deku(id = "1")]
    NotAutonomous,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "5")]
pub enum MessageCounterTYP {
    #[deku(id = "0")]
    NoDetection,
    #[deku(id = "1")]
    SinglePSRTargetReports,
    #[deku(id = "2")]
    SingleSSRTargetReports,
    #[deku(id = "3")]
    SSRPSRTargetReports,
    #[deku(id = "4")]
    SingleAllCallTargetReports,
    #[deku(id = "5")]
    SingleRollCallTargetReports,
    #[deku(id = "6")]
    AllCallPSRModeSTargetReports,
    #[deku(id = "7")]
    RollCallPSRModeSTargetReports,
    #[deku(id = "8")]
    FilterForWeatherData,
    #[deku(id = "9")]
    FilterForJammingStrobe,
    #[deku(id = "10")]
    FilterPSRData,
    #[deku(id = "11")]
    FilterSSRModeSData,
    #[deku(id = "12")]
    FilterSSRModeSPSRData,
    #[deku(id = "13")]
    FilterForEnhancedSuveillanceData,
    #[deku(id = "14")]
    FilterForPSREnhancedSurveillance,
    #[deku(id = "15")]
    FilterForPSREnhancedSurveillancePlusSSRModeSDataNotInAreaOfPrimeInterest,
    #[deku(id = "16")]
    FilterForPSREnhancedSurveillancePlusAllSSRModeSData,
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "8")]
pub enum DataFilterTYP {
    #[deku(id = "0")]
    InvalidValue,
    #[deku(id = "1")]
    FilterWeatherData,
    #[deku(id = "2")]
    FilterJammingStrobe,
    #[deku(id = "3")]
    FilterPSRData,
    #[deku(id = "4")]
    FilterSSRModeSData,
    #[deku(id = "5")]
    FilterSSRModeSPSRData,
    #[deku(id = "6")]
    EnhancedSurveillanceData,
    #[deku(id = "7")]
    FilterPSREnhancedSurveillanceData,
    #[deku(id = "8")]
    FilterPSREnhancedSurveillanceSSRModeSDataNotInAreaOfPrimeInterest,
    #[deku(id = "9")]
    FilterPSREnhancedSurveillanceAllSSRModeSData,
}
