use deku::prelude::*;

//TODO use top level endian = "big"
//TODO separate into packet/message

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8")]
enum Asterix {
    #[deku(id = "48")]
    Cat48(Cat48),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
struct Cat48 {
    #[deku(bytes = "2", endian = "big")]
    length: u16,
    #[deku(bytes = "1", endian = "big")]
    fspec1: u8,
    #[deku(bytes = "1", endian = "big")]
    fspec2: u8,
    #[deku(bytes = "1", endian = "big")]
    fspec3: u8,
    #[deku(skip, cond = "0b1000_0000 & *fspec1 != 0b1000_0000")]
    data_source_identifier: Option<DataSourceIdentifier>,
    #[deku(skip, cond = "0b100_0000 & *fspec1 != 0b100_0000")]
    time_of_day: Option<TimeOfDay>,
    #[deku(skip, cond = "0b10_0000 & *fspec1 != 0b10_0000")]
    target_report_descriptor: Option<TargetReportDescriptor>,
    #[deku(skip, cond = "0b1_0000 & *fspec1 != 0b1_0000")]
    measured_position_in_polar_coordinates: Option<MeasuredPositionInPolarCoordinates>,
    #[deku(skip, cond = "0b1000 & *fspec1 != 0b1000")]
    mode_3_a_code_in_octal_representation: Option<Mode3ACodeInOctalRepresentation>,
    #[deku(skip, cond = "0b100 & *fspec1 != 0b100")]
    flight_level_in_binary_repre: Option<FlightLevelInBinaryRepresentation>,
    // TODO check fspec
    #[deku(skip, cond = "0b100_0000 & *fspec2 != 0b100_0000")]
    aircraft_address: Option<AircraftAddress>,
    // TODO check fspec
    // TODO use map to_string()
    #[deku(skip, cond = "0b10_0000 & *fspec2 != 0b10_0000")]
    aircraft_identification: Option<AircraftIdentification>,
    // TODO check fspec
    // TODO handle counter
    #[deku(skip, cond = "0b100_0000 & *fspec2 != 0b100_0000", bytes = "1", endian = "big")]
    counter: u8,
    #[deku(skip, cond = "0b100_0000 & *fspec2 != 0b100_0000")]
    mode_smb_data: Option<ModeSMBData>,
    #[deku(skip, cond = "0b1_0000 & *fspec2 != 0b1_0000")]
    track_number: Option<TrackNumber>,
    #[deku(skip, cond = "0b100 & *fspec2 != 0b100")]
    // TODO handle special float
    calculated_track_velocity: Option<CalculatedTrackVelocity>,
    #[deku(skip, cond = "0b10 & *fspec2 != 0b10")]
    track_status: Option<TrackStatus>,
    #[deku(skip, cond = "0b10 & *fspec3 != 0b10")]
    communications_capability_flight_status: Option<CommunicationsCapabilityFlightStatus>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct DataSourceIdentifier {
    #[deku(bytes = "1", endian = "big")]
    pub sac: u8,
    #[deku(bytes = "1", endian = "big")]
    pub sic: u8,
}

//TODO fix display of f32
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct TimeOfDay {
    #[deku(bytes = "3", endian = "big")]
    pub time: f32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct TargetReportDescriptor {
    pub typ: TYP,
    pub sim: SIM,
    pub rdp: RDP,
    pub spi: SPI,
    pub rab: RAB,
    pub fx: FX,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "3", endian = "big")]
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
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum SIM {
    #[deku(id = "0x00")]
    ActualTargetReport,
    #[deku(id = "0x01")]
    SimulatedTargetReport,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum RDP {
    #[deku(id = "0x00")]
    ReportFromRDPChain1,
    #[deku(id = "0x01")]
    ReportFromRDPChain2,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum SPI {
    #[deku(id = "0x00")]
    AbsenceOfSPI,
    #[deku(id = "0x01")]
    SpecialPositionIdentification,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum RAB {
    #[deku(id = "0x00")]
    ReportFromAircraftTransponder,
    #[deku(id = "0x01")]
    ReportFromFieldMonitor,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum FX {
    #[deku(id = "0x00")]
    EndOfDataItem = 0,
    #[deku(id = "0x01")]
    ExtensionIntoFirstExtent = 1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MeasuredPositionInPolarCoordinates {
    pub rho: u16,
    pub theta: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Mode3ACodeInOctalRepresentation {
    pub v: V,
    pub g: G,
    pub l: L,
    #[deku(bits = "12", endian = "big")]
    pub reply: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum V {
    #[deku(id = "0x00")]
    CodeValidated = 0,
    #[deku(id = "0x01")]
    CodeNotValidated = 1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum G {
    #[deku(id = "0x00")]
    Default = 0,
    #[deku(id = "0x01")]
    GarbledCode = 1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum L {
    #[deku(id = "0x00")]
    Mode3CodeDerivedFromTheReplyOfTheTransponder = 0,
    #[deku(id = "0x01")]
    Mode3CodeNotExtractedDuringTheLastScan = 1,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct FlightLevelInBinaryRepresentation {
    pub v: V,
    pub g: G,
    // TODO check wireshark
    #[deku(bits = "15", endian = "big")]
    pub flight_level: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct AircraftAddress {
    #[deku(bytes = "3", endian = "big")]
    pub address: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct AircraftIdentification {
    /// IA5 char array
    #[deku(bytes = "6", endian = "big")]
    pub identification: u64,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct ModeSMBData {
    #[deku(count = "7", endian = "big")]
    pub mb_data: Vec<u8>,
    #[deku(bits = "4", endian = "big")]
    pub bds1: u8,
    #[deku(bits = "4", endian = "big")]
    pub bds2: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct TrackNumber {
    #[deku(bytes = "2", endian = "big")]
    pub number: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct CalculatedTrackVelocity {
    #[deku(bytes = "2", endian = "big")]
    pub groundspeed: f32,
    #[deku(bytes = "2", endian = "big")]
    pub heading: f32,
}







#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct TrackStatus {
    pub cnf: CNF,
    pub rad: RAD,
    pub dou: DOU,
    pub mah: MAH,
    pub cdm: CDM,
    pub fx1: FX,
    pub tre: TRE,
    pub gho: GHO,
    pub sup: SUP,
    pub tcc: TCC,
    #[deku(bits = "3", endian = "big")]
    pub reserved: u32,
    pub fx2: FX,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum CNF {
    #[deku(id = "0x00")]
    ConfirmedTrack,
    #[deku(id = "0x01")]
    TentativeTrack,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "2", endian = "big")]
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
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum DOU {
    #[deku(id = "0x00")]
    NormalConfidence,
    #[deku(id = "0x01")]
    LowConfidence,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum MAH {
    #[deku(id = "0x00")]
    NoHorizontalManSensed,
    #[deku(id = "0x01")]
    HorizontalManSensed,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "2", endian = "big")]
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
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum TRE {
    #[deku(id = "0x00")]
    TrackStillAlive,
    #[deku(id = "0x01")]
    EndOfTrackLifetime,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum GHO {
    #[deku(id = "0x00")]
    TrueTargetTrack,
    #[deku(id = "0x01")]
    GhostTargetTrack,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum SUP {
    #[deku(id = "0x00")]
    No,
    #[deku(id = "0x01")]
    Yes,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum TCC {
    #[deku(id = "0x00")]
    RadarPlanePlotTransformation,
    #[deku(id = "0x01")]
    SlantRangePlotTransformation,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct CommunicationsCapabilityFlightStatus {
    pub com: COM,
    pub stat: STAT,
    pub si: SI,
    #[deku(bits = "1", endian = "big")]
    pub reserved: u8,
    pub mssc: MSSC,
    pub arc: ARC,
    pub aic: AIC,
    #[deku(bits = "1", endian = "big")]
    pub b1a: u8,
    #[deku(bits = "4", endian = "big")]
    pub b1b: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "3", endian = "big")]
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
#[deku(id_type = "u8", id_bits = "3", endian = "big")]
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
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum SI {
    #[deku(id = "0x00")]
    SICodeCapable,
    #[deku(id = "0x01")]
    IICodeCapable,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum MSSC {
    #[deku(id = "0x00")]
    No,
    #[deku(id = "0x01")]
    Yes,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum ARC {
    #[deku(id = "0x00")]
    Resolution100ft,
    #[deku(id = "0x01")]
    Resolution25ft,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id_type = "u8", id_bits = "1", endian = "big")]
pub enum AIC {
    #[deku(id = "0x00")]
    No,
    #[deku(id = "0x01")]
    Yes,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let bytes = vec![
          0x30, 0x00, 0x30, 0xfd, 0xf7, 0x02, 0x19, 0xc9,
          0x35, 0x6d, 0x4d, 0xa0, 0xc5, 0xaf, 0xf1, 0xe0,
          0x02, 0x00, 0x05, 0x28, 0x3c, 0x66, 0x0c, 0x10,
          0xc2, 0x36, 0xd4, 0x18, 0x20, 0x01, 0xc0, 0x78,
          0x00, 0x31, 0xbc, 0x00, 0x00, 0x40, 0x0d, 0xeb,
          0x07, 0xb9, 0x58, 0x2e, 0x41, 0x00, 0x20, 0xf5,
        ];
        let (_, ass) = Asterix::from_bytes((&bytes, 0)).unwrap();
        println!("{:#?}", ass);
    }
}
