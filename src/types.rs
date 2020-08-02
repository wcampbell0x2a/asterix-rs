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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
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
