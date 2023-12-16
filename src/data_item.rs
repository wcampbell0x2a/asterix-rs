//! Defined Data Items that are used for formal parsing of data structs in categories

use crate::custom_read_write::{read, write, Op};
use crate::fspec::is_fspec;
use crate::modifier;
use crate::types::{
    DataFilterTYP, MessageCounterTYP, AIC, ANT, ARC, CDM, CHAB, CLU, CNF, CODE, COM, D, DLF, DOU,
    ERR, FOEFRI, FX, G, GHO, L, MAH, ME, MI, MSC, MSSC, MTYPE, NOGO, OVL, POL, RAB, RAD, RDP, RDPC,
    RDPR, RED, SCF, SI, SIM, SPI, STAT, STC, SUP, TCC, TRE, TST, TSV, TYP, V, XPP,
};
use deku::bitvec::{BitSlice, BitVec, Msb0};
use deku::prelude::*;

const RHO_MODIFIER: f32 = 1.0 / 256.0;
const THETA_MODIFIER: f32 = 360.0 / 65536.0;

/// Identification of the radar station from which the data is received
///
/// Data Item I048/010
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct DataSourceIdentifier {
    /// System Area Code
    pub sac: u8,
    /// System Identification Code
    pub sic: u8,
}

impl DataSourceIdentifier {
    pub const FRN_34: u8 = 0b1000_0000;
    pub const FRN_48: u8 = 0b1000_0000;
}

/// Absolute time stamping expressed as Co-ordinated Universal Time (UTC)
///
/// Data Item I048/140
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct TimeOfDay {
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 24, Self::MODIFIER, Op::Divide)",
        writer = "write::f32_u32(&self.time, 24, Self::MODIFIER, Op::Multiply, deku::output)"
    )]
    pub time: f32,
}

impl TimeOfDay {
    pub const FRN_34: u8 = 0b10_0000;
    pub const FRN_48: u8 = 0b100_0000;
    const MODIFIER: f32 = 128.0;
}

/// Type and properties of the target report
///
/// Data Item I048/020
///
/// TODO: This can extend with FX bit.
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct TargetReportDescriptor {
    pub typ: TYP,
    pub sim: SIM,
    pub rdp: RDP,
    pub spi: SPI,
    pub rab: RAB,
    pub fx1: FX,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub tst: Option<TST>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub err: Option<ERR>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub xpp: Option<XPP>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub me: Option<ME>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub mi: Option<MI>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub foe_fri: Option<FOEFRI>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub fx2: Option<FX>,
}

impl TargetReportDescriptor {
    pub const FRN_48: u8 = 0b10_0000;
}

/// Measured position of an aircraft in local polar co-ordinates
///
/// Data Item I048/040
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct MeasuredPositionInPolarCoordinates {
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, RHO_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.rho, 16, RHO_MODIFIER, Op::Divide, deku::output)"
    )]
    pub rho: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, THETA_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.theta, 16, THETA_MODIFIER, Op::Divide, deku::output)"
    )]
    pub theta: f32,
}

impl MeasuredPositionInPolarCoordinates {
    pub const FRN_48: u8 = 0b1_0000;
}

/// Mode-3/A code converted into octal representation
///
/// Data Item I048/070
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct Mode3ACodeInOctalRepresentation {
    pub v: V,
    pub g: G,
    pub l: L,
    #[deku(bits = "1")]
    pub reserved: u8,
    /// Mode-3/A reply in octal representation
    #[deku(bits = "12", endian = "big")]
    pub reply: u16,
}

impl Mode3ACodeInOctalRepresentation {
    pub const FRN_48: u8 = 0b1000;
}

/// Flight Level converted into binary representation
///
/// Data Item I048/090
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct FlightLevelInBinaryRepresentation {
    pub v: V,
    pub g: G,
    #[deku(
        reader = "Self::read(deku::rest)",
        writer = "Self::write(&self.flight_level, deku::output)"
    )]
    pub flight_level: u16,
}

impl FlightLevelInBinaryRepresentation {
    pub const FRN_48: u8 = 0b100;
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(14_usize));

    fn read(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, u16), DekuError> {
        u16::read(rest, Self::CTX).map(|(rest, value)| (rest, value / 4))
    }

    fn write(flight_level: &u16, output: &mut BitVec<u8, Msb0>) -> Result<(), DekuError> {
        let value = *flight_level * 4;
        value.write(output, Self::CTX)
    }
}

/// Aircraft address (24-bits Mode S address) assigned uniquely to
/// each aircraft
///
/// Data Item I048/220
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct AircraftAddress {
    #[deku(bytes = "3", endian = "big")]
    pub address: u32,
}

impl AircraftAddress {
    pub const FRN_48: u8 = 0b1000_0000;
}

/// Aircraft identification (in 8 characters) obtained from an aircraft
/// equipped with a Mode S transponder
///
/// Data Item I048/240
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct AircraftIdentification {
    /// IA5 char array
    #[deku(
        reader = "Self::read(deku::rest)",
        writer = "Self::write(&self.identification, deku::output)"
    )]
    pub identification: String,
}

impl AircraftIdentification {
    pub const FRN_48: u8 = 0b100_0000;
    /// Read and convert to String
    fn read(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, String), DekuError> {
        let (rest, one) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        let (rest, two) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        let (rest, three) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        let (rest, four) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        let (rest, five) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        let (rest, six) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        let (rest, seven) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        let (rest, _) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        let value = format!(
            "{}{}{}{}{}{}{}",
            Self::asterix_char_to_ascii(one) as char,
            Self::asterix_char_to_ascii(two) as char,
            Self::asterix_char_to_ascii(three) as char,
            Self::asterix_char_to_ascii(four) as char,
            Self::asterix_char_to_ascii(five) as char,
            Self::asterix_char_to_ascii(six) as char,
            Self::asterix_char_to_ascii(seven) as char
        );
        Ok((rest, value))
    }

    /// Parse from String to u8 and write
    fn write(field_a: &str, output: &mut BitVec<u8, Msb0>) -> Result<(), DekuError> {
        for c in field_a.chars() {
            Self::asterix_ascii_to_ia5_char(c as u8)
                .write(output, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        }
        0_u8.write(output, (deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))
    }

    const IA5_ALPHA: u8 = 0x01;
    const IA5_SPACE: u8 = 0x20;
    const IA5_DIGIT: u8 = 0x30;
    const ASC_DIGIT: u8 = b'0';
    const ASC_ALPHA: u8 = b'A';
    const ASC_SPACE: u8 = b' ';
    const ASC_ERROR: u8 = b'?';

    /// parse into ascii from IA5 char array
    const fn asterix_char_to_ascii(code: u8) -> u8 {
        // space
        if code == Self::IA5_SPACE {
            Self::ASC_SPACE
        }
        // digit
        else if Self::IA5_DIGIT <= code && code < Self::IA5_DIGIT + 10 {
            Self::ASC_DIGIT + (code - Self::IA5_DIGIT)
        }
        // letter
        else if Self::IA5_ALPHA <= code && code < Self::IA5_ALPHA + 26 {
            Self::ASC_ALPHA + (code - Self::IA5_ALPHA)
        } else {
            Self::ASC_ERROR
        }
    }

    /// parse from IA5 char as u8 to u8 value
    const fn asterix_ascii_to_ia5_char(code: u8) -> u8 {
        // space
        if code == Self::ASC_SPACE {
            Self::IA5_SPACE
        }
        // digit
        else if Self::ASC_DIGIT <= code && code < Self::ASC_DIGIT + 10 {
            Self::IA5_DIGIT + (code - Self::ASC_DIGIT)
        }
        // letter
        else if Self::ASC_ALPHA <= code && code < Self::ASC_ALPHA + 26 {
            Self::IA5_ALPHA + (code - Self::ASC_ALPHA)
        } else {
            Self::ASC_ERROR
        }
    }
}

/// Mode S Comm B data as extracted from the aircraft
/// transponder
///
/// Data Item I048/250, Mode S MB Data
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct ModeSMBData {
    #[deku(update = "self.mb_data.len()")]
    pub count: u8,
    #[deku(count = "count")]
    pub mb_data: Vec<MBData>,
    #[deku(bits = "4")]
    pub bds1: u8,
    #[deku(bits = "4")]
    pub bds2: u8,
}

impl ModeSMBData {
    pub const FRN_48: u8 = 0b10_0000;
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MBData {
    #[deku(count = "7")]
    pub data: Vec<u8>,
}

/// An integer value representing a unique reference to a track
/// record within a particular track file
///
/// Data Item I048/161
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct TrackNumber {
    #[deku(bits = "4")]
    pub reserved: u8,
    #[deku(bits = "12", endian = "big")]
    pub number: u16,
}

impl TrackNumber {
    pub const FRN_48: u8 = 0b1_0000;
}

/// Calculated position of an aircraft in Cartesian co-ordinates
///
/// Data Item I048/042
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct CalculatedPositionCartesianCorr {
    #[deku(
        reader = "read::bits_i16_to_f32(deku::rest, 16, Self::MODIFIER, Op::Multiply)",
        writer = "write::f32_i32(&self.x, 16, Self::MODIFIER, Op::Divide, deku::output)"
    )]
    pub x: f32,
    #[deku(
        reader = "read::bits_i16_to_f32(deku::rest, 16, Self::MODIFIER, Op::Multiply)",
        writer = "write::f32_i32(&self.y, 16, Self::MODIFIER, Op::Divide, deku::output)"
    )]
    pub y: f32,
}

impl CalculatedPositionCartesianCorr {
    pub const FRN_48: u8 = 0b1000;
    const MODIFIER: f32 = 1.0 / 128.0;
}

/// Calculated track velocity expressed in polar co-ordinates
///
/// Data Item I048/200
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct CalculatedTrackVelocity {
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, modifier::groundspeed(), Op::Multiply)",
        writer = "write::f32_u32(&self.groundspeed, 16, modifier::groundspeed(), Op::Divide, deku::output)"
    )]
    pub groundspeed: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, modifier::heading1(), Op::Multiply)",
        writer = "write::f32_u32(&self.heading, 16, modifier::heading1(), Op::Divide, deku::output)"
    )]
    pub heading: f32,
}

impl CalculatedTrackVelocity {
    pub const FRN_48: u8 = 0b100;
}

/// Status of monoradar track (PSR and/or SSR updated)
///
/// Data Item I048/170
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct TrackStatus {
    pub cnf: CNF,
    pub rad: RAD,
    pub dou: DOU,
    pub mah: MAH,
    pub cdm: CDM,
    pub fx1: FX,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub tre: Option<TRE>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub gho: Option<GHO>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub sup: Option<SUP>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub tcc: Option<TCC>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent", bits = "3")]
    pub reserved: Option<u8>,
    #[deku(skip, cond = "*fx1 != FX::ExtensionIntoFirstExtent")]
    pub fx2: Option<FX>,
}

impl TrackStatus {
    pub const FRN_48: u8 = 0b10;
}

/// Track quality in the form of a vector of standard deviations
///
/// Data Item I048/210
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct TrackQuality {
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 8, Self::MODIFIER, Op::Divide)",
        writer = "write::f32_u32(&self.horizontal_stddev, 8, Self::MODIFIER, Op::Multiply, deku::output)"
    )]
    pub horizontal_stddev: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 8, Self::MODIFIER, Op::Divide)",
        writer = "write::f32_u32(&self.vertical_stddev, 8, Self::MODIFIER, Op::Multiply, deku::output)"
    )]
    pub vertical_stddev: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 8, modifier::groundspeed(), Op::Multiply)",
        writer = "write::f32_u32(&self.groundspeed_stddev, 8, modifier::groundspeed(), Op::Divide, deku::output)"
    )]
    pub groundspeed_stddev: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 8, modifier::heading2(), Op::Multiply)",
        writer = "write::f32_u32(&self.heading_stddev, 8, modifier::heading2(), Op::Divide, deku::output)"
    )]
    pub heading_stddev: f32,
}

impl TrackQuality {
    pub const FRN_48: u8 = 0b1000_0000;
    const MODIFIER: f32 = 1.0 / 128.0;
}

/// Communications capability of the transponder, capability of the onboard ACAS equipment and
/// flight status
///
/// Data Item I048/230
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct CommunicationsCapabilityFlightStatus {
    pub com: COM,
    pub stat: STAT,
    pub si: SI,
    #[deku(bits = "1")]
    pub reserved: u8,
    pub mssc: MSSC,
    pub arc: ARC,
    pub aic: AIC,
    #[deku(bits = "1")]
    pub b1a: u8,
    #[deku(bits = "4")]
    pub b1b: u8,
}

impl CommunicationsCapabilityFlightStatus {
    pub const FRN_48: u8 = 0b10;
}

/// Additional information on the quality of the target report
///
/// Data Item I048/130
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct RadarPlotCharacteristics {
    #[deku(until = "|b: &u8| *b & 0b0000_0001 == 0")]
    pub fspec: Vec<u8>,
    #[deku(
        skip,
        cond = "is_fspec(0b1000_0000, fspec, 0)",
        reader = "read::bits_to_optionf32(deku::rest, 8, Self::runlength_modifier(), Op::Multiply)",
        writer = "write::f32_optionu32(&self.srl, 8, Self::runlength_modifier(), Op::Divide, deku::output)"
    )]
    pub srl: Option<f32>,
    #[deku(skip, cond = "is_fspec(0b100_0000, fspec, 0)")]
    pub srr: Option<u8>,
    #[deku(skip, cond = "is_fspec(0b10_0000, fspec, 0)")]
    pub sam: Option<i8>,
    #[deku(
        skip,
        cond = "is_fspec(0b1_0000, fspec, 0)",
        reader = "read::bits_to_optionf32(deku::rest, 8, Self::runlength_modifier(), Op::Multiply)",
        writer = "write::f32_optionu32(&self.prl, 8, Self::runlength_modifier(), Op::Divide, deku::output)"
    )]
    pub prl: Option<f32>,
    #[deku(skip, cond = "is_fspec(0b1000, fspec, 0)")]
    pub pam: Option<u8>,
    #[deku(
        skip,
        cond = "is_fspec(0b100, fspec, 0)",
        reader = "read::bits_to_optionf32(deku::rest, 8, Self::nm_modifier(), Op::Multiply)",
        writer = "write::f32_optionu32(&self.rpd, 8, Self::nm_modifier(), Op::Divide, deku::output)"
    )]
    pub rpd: Option<f32>,
    #[deku(
        skip,
        cond = "is_fspec(0b100, fspec, 0)",
        reader = "read::bits_to_optionf32(deku::rest, 8, Self::apd_modifier(), Op::Multiply)",
        writer = "write::f32_optionu32(&self.apd, 8, Self::apd_modifier(), Op::Divide, deku::output)"
    )]
    pub apd: Option<f32>,
}

impl RadarPlotCharacteristics {
    pub const FRN_48: u8 = 0b10;

    fn runlength_modifier() -> f32 {
        360.0 / f32::from(2_u16.pow(13))
    }

    fn nm_modifier() -> f32 {
        1.0 / 256.0
    }

    fn apd_modifier() -> f32 {
        360.0 / f32::from(2_u16.pow(14))
    }
}

/// This Data Item allows for a more convenient handling of the
/// messages at the receiver side by further defining the type of
/// transaction
///
/// Data Item I034/000
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct MessageType {
    pub t: MTYPE,
}

impl MessageType {
    pub const FRN_34: u8 = 0b100_0000;
}

/// Eight most significant bits of the antenna azimuth defining a
/// particular azimuth sector
///
/// Data Item I034/020
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct SectorNumber {
    #[deku(reader = "Self::read(deku::rest)", writer = "Self::write(&self.num, deku::output)")]
    pub num: u16,
}

impl SectorNumber {
    pub const FRN_34: u8 = 0b1_0000;
    pub const FRN_48: u8 = 0b1_0000;
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(8_usize));

    fn modifier() -> f32 {
        360.0 / 2_f32.powi(8)
    }

    fn read(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, u16), DekuError> {
        u16::read(rest, Self::CTX)
            .map(|(rest, value)| (rest, (f32::from(value) * Self::modifier()) as u16))
    }

    fn write(num: &u16, output: &mut BitVec<u8, Msb0>) -> Result<(), DekuError> {
        let value = (f32::from(*num) / Self::modifier()) as u8;
        value.write(output, Self::CTX)
    }
}

/// Warning/error conditions detected by a radar station for the target
/// report involved. Target Classification information for the target
/// involved
///
/// Data Item I048/030
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct WarningErrorConditionsTargetClass {
    #[deku(until = "|codefx: &CodeFx| codefx.fx == FX::EndOfDataItem")]
    pub codefxs: Vec<CodeFx>,
}

impl WarningErrorConditionsTargetClass {
    pub const FRN_48: u8 = 0b100_0000;
}

#[derive(Debug, PartialEq, Clone, Copy, DekuRead, DekuWrite)]
pub struct CodeFx {
    pub code: CODE,
    pub fx: FX,
}

/// Confidence level for each bit of a Mode-3/A reply as provided
/// by a monopulse SSR station
///
/// Data Item I048/080
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct Mode3ACodeConfidenceIndicator {
    #[deku(bits = "4", endian = "big")]
    pub reserved: u8,
    #[deku(bits = "12", endian = "big")]
    pub confidence: u16,
}

impl Mode3ACodeConfidenceIndicator {
    pub const FRN_48: u8 = 0b10_0000;
}

/// Mode-C height in Gray notation as received from the
/// transponder together with the confidence level for each reply bit
/// as provided by a MSSR/Mode S station
///
/// Data Item I048/100
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct ModeCCodeAndConfidenceIndicator {
    pub v: V,
    pub g: G,
    #[deku(bits = "2", endian = "big")]
    pub reserved0: u8,
    #[deku(bits = "12", endian = "big")]
    pub mode_c_gray_notation: u16,
    #[deku(bits = "4", endian = "big")]
    pub reserved1: u8,
    #[deku(bits = "12", endian = "big")]
    pub confidence: u16,
}

impl ModeCCodeAndConfidenceIndicator {
    pub const FRN_48: u8 = 0b1_0000;
}

/// Height of a target as measured by a 3D radar. The height shall
/// use mean sea level as the zero reference level
///
/// Data Item I048/110
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct HeightMeasuredBy3dRadar {
    #[deku(bits = "2", endian = "big")]
    pub reserved: u8,
    #[deku(reader = "Self::read(deku::rest)", writer = "Self::write(&self.height, deku::output)")]
    pub height: i32,
}

impl HeightMeasuredBy3dRadar {
    pub const FRN_48: u8 = 0b1000;
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(14_usize));
    pub const MODIFIER: i32 = 25;

    fn read(rest: &BitSlice<u8, Msb0>) -> Result<(&BitSlice<u8, Msb0>, i32), DekuError> {
        i32::read(rest, Self::CTX).map(|(rest, value)| (rest, value * Self::MODIFIER))
    }

    fn write(height: &i32, output: &mut BitVec<u8, Msb0>) -> Result<(), DekuError> {
        let value = height / Self::MODIFIER;
        value.write(output, Self::CTX)
    }
}

/// Information on the Doppler Speed of the target report
///
/// Data Item I048/120
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct RadialDopplerSpeed {
    #[deku(bits = "1", endian = "big")]
    pub cal: u8,
    #[deku(bits = "1", endian = "big")]
    pub rds: u8,
    #[deku(bits = "6", endian = "big")]
    pub spare: u8,
    #[deku(skip, endian = "big", cond = "*cal == 0")]
    pub calculated_doppler_speed: Option<CalculatedDopplerSpeed>,
    #[deku(skip, endian = "big", cond = "*rds == 0")]
    pub raw_doppler_speed: Option<RawDopplerSpeed>,
}

impl RadialDopplerSpeed {
    pub const FRN_48: u8 = 0b100;
}

/// Subfield of `HeightMeasuredBy3dRadar`
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct CalculatedDopplerSpeed {
    pub d: D,
    #[deku(bits = "5", endian = "big")]
    pub spare: u8,
    #[deku(bits = "10", endian = "big")]
    pub cal: u16,
}

/// Subfield of `HeightMeasuredBy3dRadar`
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct RawDopplerSpeed {
    /// Repetition Factor
    #[deku(endian = "big")]
    pub rep: u8,
    /// Doppler Speed: 1 m/sec
    #[deku(endian = "big")]
    pub dop: u16,
    /// Ambiquity Range: 1 m/sec
    #[deku(endian = "big")]
    pub amb: u16,
    /// Transmitter Frequency: 1 Mhz
    #[deku(endian = "big")]
    pub frq: u16,
}

/// Currently active Resolution Advisory (RA), if any, generated by the
/// ACAS associated with the transponder transmitting the report and
/// threat identity data
///
/// Data Item I048/260
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct ACASResolutionAdvisoryReport {
    pub mb_data: [u8; 7],
}

impl ACASResolutionAdvisoryReport {
    pub const FRN_48: u8 = 0b1000_0000;
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct Mode1CodeOctalRepresentation {
    pub v: V,
    pub g: G,
    pub l: L,
    #[deku(bits = "5", endian = "big")]
    pub data: u8,
}

impl Mode1CodeOctalRepresentation {
    pub const FRN_48: u8 = 0b100_0000;
}

/// Reply to Mode-2 interrogation
///
/// Data Item I048/050
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct Mode2CodeOctalRepresentation {
    pub v: V,
    pub g: G,
    pub l: L,
    #[deku(bits = "1", endian = "big")]
    pub spare: u8,
    #[deku(bits = "12", endian = "big")]
    pub data: u16,
}

impl Mode2CodeOctalRepresentation {
    pub const FRN_48: u8 = 0b10_0000;
}

/// Confidence level for each bit of a Mode-1 reply as provided by
/// a monopulse SSR station
///
/// Data Item I048/065
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct Mode1CodeConfidenceIndicator {
    #[deku(bits = "3", endian = "big")]
    pub spare: u8,
    #[deku(bits = "5", endian = "big")]
    pub data: u8,
}

impl Mode1CodeConfidenceIndicator {
    pub const FRN_48: u8 = 0b1_0000;
}

/// Confidence level for each bit of a Mode-2 reply as provided by
/// a monopulse SSR station
///
/// Data Item I048/060
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct Mode2CodeConfidenceIndicator {
    #[deku(bits = "4", endian = "big")]
    pub spare: u8,
    #[deku(bits = "12", endian = "big")]
    pub data: u16,
}

impl Mode2CodeConfidenceIndicator {
    pub const FRN_48: u8 = 0b1000;
}

/// Antenna rotation period as measured between two consecutive
/// North crossings or as averaged during a period of time
///
/// Data Item I034/041
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct AntennaRotationSpeed {
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, Self::MODIFIER, Op::Divide)",
        writer = "write::f32_u32(&self.period, 16, Self::MODIFIER, Op::Multiply, deku::output)"
    )]
    pub period: f32,
}

impl AntennaRotationSpeed {
    pub const FRN_34: u8 = 0b1000;
    const MODIFIER: f32 = 128.0;
}

/// Information concerning the configuration and status of a System
///
/// Data Item I034/050
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct SystemConfigurationAndStatus {
    #[deku(bits = "1")]
    pub com_bit: u8,
    #[deku(bits = "2")]
    pub spare_bit0: u8,
    #[deku(bits = "1")]
    pub psr_bit: u8,
    #[deku(bits = "1")]
    pub ssr_bit: u8,
    #[deku(bits = "1")]
    pub mds_bit: u8,
    #[deku(bits = "1")]
    pub spare_bit1: u8,
    pub fx_bit: FX,
    #[deku(skip, cond = "*com_bit != 1")]
    pub com: Option<ComSubField>,
    #[deku(skip, cond = "*psr_bit != 1")]
    pub psr: Option<Sensor>,
    #[deku(skip, cond = "*ssr_bit != 1")]
    pub ssr: Option<Sensor>,
    #[deku(skip, cond = "*mds_bit != 1")]
    pub mds: Option<MdsSubField>,
}

impl SystemConfigurationAndStatus {
    pub const FRN_34: u8 = 0b0100;
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct ComSubField {
    pub nogo: NOGO,
    pub rdpc: RDPC,
    pub rdpr: RDPR,
    pub ovl_rdp: OVL,
    pub olv_xmt: OVL,
    pub msc: MSC,
    pub tsv: TSV,
    #[deku(bits = "1")]
    pub spare: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct Sensor {
    pub ant: ANT,
    pub chab: CHAB,
    pub ovl: OVL,
    pub msc: MSC,
    #[deku(bits = "3")]
    pub spare: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MdsSubField {
    pub ant: ANT,
    pub chab: CHAB,
    pub ovl_sur: OVL,
    pub msc: MSC,
    pub scf: SCF,
    pub dlf: DLF,
    pub ovl_scf: OVL,
    pub ovl_dlf: OVL,
    #[deku(bits = "7")]
    pub spare: u8,
}

/// Status concerning the processing options, in use during the last antenna revolution,
/// for the various Sensors, composing the System
///
/// Data Item I034/060
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct SystemProcessingMode {
    #[deku(bits = "1")]
    pub com_bit: u8,
    #[deku(bits = "2")]
    pub spare_bit0: u8,
    #[deku(bits = "1")]
    pub psr_bit: u8,
    #[deku(bits = "1")]
    pub ssr_bit: u8,
    #[deku(bits = "1")]
    pub mds_bit: u8,
    #[deku(bits = "1")]
    pub spare_bit1: u8,
    pub fx_bit: FX,
    #[deku(skip, cond = "*com_bit != 1")]
    pub com: Option<ComSubField2>,
    #[deku(skip, cond = "*psr_bit != 1")]
    pub psr: Option<PsrSubField>,
    #[deku(skip, cond = "*ssr_bit != 1")]
    pub ssr: Option<SsrSubField>,
    #[deku(skip, cond = "*mds_bit != 1")]
    pub mds: Option<MdsSubField2>,
}

impl SystemProcessingMode {
    pub const FRN_34: u8 = 0b0010;
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct ComSubField2 {
    #[deku(bits = "1")]
    pub spare0: u8,
    pub red_rdp: RED,
    pub red_xmt: RED,
    #[deku(bits = "1")]
    pub spare1: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct PsrSubField {
    pub pol: POL,
    pub red_rad: RED,
    pub stc: STC,
    #[deku(bits = "2")]
    pub spare: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct SsrSubField {
    pub red_rad: RED,
    #[deku(bits = "5")]
    pub spare: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MdsSubField2 {
    pub red_rad: RED,
    pub clu: CLU,
    #[deku(bits = "4")]
    pub spare: u8,
}

/// Message Count values, according the various types of messages,
/// for the last completed antenna revolution, counted between two
/// North crossings
///
/// Data Item I034/070
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct MessageCountValues {
    #[deku(update = "self.counters.len()")]
    pub count: u8,
    #[deku(count = "count")]
    pub counters: Vec<MessageCounter>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MessageCounter {
    pub typ: MessageCounterTYP,
    #[deku(bits = "11")]
    pub counter: u16,
}

impl MessageCountValues {
    pub const FRN_34: u8 = 0b1000_0000;
}

/// Geographical window defined in polar co-ordinates.
///
/// Data Item I034/100
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct GenericPolarWindow {
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, RHO_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.rho_start, 16, RHO_MODIFIER, Op::Divide, deku::output)"
    )]
    pub rho_start: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, RHO_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.rho_end, 16, RHO_MODIFIER, Op::Divide, deku::output)"
    )]
    pub rho_end: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, THETA_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.theta_start, 16, THETA_MODIFIER, Op::Divide, deku::output)"
    )]
    pub theta_start: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 16, THETA_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.theta_end, 16, THETA_MODIFIER, Op::Divide, deku::output)"
    )]
    pub theta_end: f32,
}

impl GenericPolarWindow {
    pub const FRN_34: u8 = 0b0100_0000;
}

/// Data Filter, which allows suppression of individual data types.
///
/// Data Item I034/110
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct DataFilter {
    pub typ: DataFilterTYP,
}

impl DataFilter {
    pub const FRN_34: u8 = 0b0010_0000;
}

/// 3D-Position of Data Source in WGS 84 Co-ordinates
///
/// Data Item I034/120
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct ThreeDPositionOfDataSource {
    pub height_of_wgs_84: u16,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 24, Self::WGS_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.latitude_in_wgs_84, 24, Self::WGS_MODIFIER, Op::Divide, deku::output)"
    )]
    pub latitude_in_wgs_84: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 24, Self::WGS_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.longitude_in_wgs_84, 24, Self::WGS_MODIFIER, Op::Divide, deku::output)"
    )]
    pub longitude_in_wgs_84: f32,
}

impl ThreeDPositionOfDataSource {
    pub const WGS_MODIFIER: f32 = 180.0 / 8_388_608.0;
    pub const FRN_34: u8 = 0b0001_0000;
}

/// Averaged difference in range and in azimuth for the primary target
/// position with respect to the SSR target position as calculated by
/// the radar station
///
/// Data Item I034/090
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct CollimationError {
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 8, Self::MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.range_error, 8, Self::MODIFIER, Op::Divide, deku::output)"
    )]
    pub range_error: f32,
    #[deku(
        reader = "read::bits_to_f32(deku::rest, 8, Self::AZIMUTH_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.azimuth_error, 8, Self::AZIMUTH_MODIFIER, Op::Divide, deku::output)"
    )]
    pub azimuth_error: f32,
}

impl CollimationError {
    pub const MODIFIER: f32 = 1.0 / 128.0;
    // TODO #![feature(const_int_pow)]
    //pub const AZIMUTH_MODIFIER: f32 = 360.0 / f32::from(2_u16.pow(14));
    pub const AZIMUTH_MODIFIER: f32 = 360.0 / 16384.0;
    pub const FRN_34: u8 = 0b0000_1000;
}

#[cfg(test)]
mod tests {
    use super::*;
    // tests from https://github.com/wireshark/wireshark/blob/master/test/suite_dissectors/group_asterix.py

    #[test]
    fn tod_140() {
        let mut input = BitSlice::from_slice(&[0xa8, 0xbf, 0xff]);
        let item = TimeOfDay::read(&mut input, deku::ctx::Endian::Big).unwrap().1;
        assert_eq!(item.time, 86_399.99);
    }

    #[test]
    fn target_report_descriptor_020() {
        let mut input = BitSlice::from_slice(&[
            0xe0 | 0x08 | 0x04 | 0x02 | 0x01,
            0x80 | 0x40 | 0x20 | 0x10 | 0x08 | 0x06,
        ]);
        let item = TargetReportDescriptor::read(&mut input, deku::ctx::Endian::Big).unwrap().1;
        assert_eq!(item.typ, TYP::ModeSRollCallPlusPSR);
        assert_eq!(item.sim, SIM::ActualTargetReport);
        assert_eq!(item.rdp, RDP::ReportFromRDPChain2);
        assert_eq!(item.spi, SPI::SpecialPositionIdentification);
        assert_eq!(item.rab, RAB::ReportFromFieldMonitor);
        assert_eq!(item.fx1, FX::ExtensionIntoFirstExtent);
        assert_eq!(item.tst, Some(TST::TestTargetReport));
        assert_eq!(item.err, Some(ERR::ExtendedRangePresent));
        assert_eq!(item.xpp, Some(XPP::XPulsePresent));
        assert_eq!(item.me, Some(ME::MilitaryEmergency));
        assert_eq!(item.mi, Some(MI::MilitaryIdentification));
        assert_eq!(item.foe_fri, Some(FOEFRI::NoReply));
    }
}
