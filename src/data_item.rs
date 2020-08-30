//! Defined Data Items that are used for formal parsing of data structs in categories

use crate::custom_read_write::{read, write, Op};
use crate::fspec::{is_fspec, read_fspec};
use crate::modifier;
use crate::types::{
    AIC, ARC, CDM, CNF, CODE, COM, D, DOU, FX, G, GHO, L, MAH, MSSC, MTYPE, RAB, RAD, RDP, SI, SIM,
    SPI, STAT, SUP, TCC, TRE, TYP, V,
};
use deku::prelude::*;

/// Identification of the radar station from which the data is received
///
/// Data Item I048/010
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct DataSourceIdentifier {
    /// System Area Code
    #[deku(bytes = "1")]
    pub sac: u8,
    /// System Identification Code
    #[deku(bytes = "1")]
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
        reader = "read::bits_to_f32(rest, 24, Self::MODIFIER, Op::Divide)",
        writer = "write::f32_u32(&self.time, 24, Self::MODIFIER, Op::Multiply)"
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
/// Data Item I048/040
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
    pub fx: FX,
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
        reader = "read::bits_to_f32(rest, 16, Self::RHO_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.rho, 16, Self::RHO_MODIFIER, Op::Divide)"
    )]
    pub rho: f32,
    #[deku(
        reader = "read::bits_to_f32(rest, 16, Self::THETA_MODIFIER, Op::Multiply)",
        writer = "write::f32_u32(&self.theta, 16, Self::THETA_MODIFIER, Op::Divide)"
    )]
    pub theta: f32,
}

impl MeasuredPositionInPolarCoordinates {
    pub const FRN_48: u8 = 0b1_0000;
    const RHO_MODIFIER: f32 = 1.0 / 256.0;
    const THETA_MODIFIER: f32 = 360.0 / 65536.0;
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
        reader = "Self::read(rest)",
        writer = "Self::write(&self.flight_level)"
    )]
    pub flight_level: u16,
}

impl FlightLevelInBinaryRepresentation {
    pub const FRN_48: u8 = 0b100;
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(14_usize));

    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, u16), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, value / 4))
    }

    fn write(flight_level: &u16) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = *flight_level * 4;
        value.write(Self::CTX)
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
        reader = "Self::read(rest)",
        writer = "Self::write(&self.identification)"
    )]
    pub identification: String,
}

impl AircraftIdentification {
    pub const FRN_48: u8 = 0b100_0000;
    /// Read and convert to String
    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, String), DekuError> {
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
            Self::as_ascii(one) as char,
            Self::as_ascii(two) as char,
            Self::as_ascii(three) as char,
            Self::as_ascii(four) as char,
            Self::as_ascii(five) as char,
            Self::as_ascii(six) as char,
            Self::as_ascii(seven) as char
        );
        Ok((rest, value))
    }

    /// Parse from String to u8 and write
    fn write(field_a: &str) -> Result<BitVec<Msb0, u8>, DekuError> {
        let mut acc: BitVec<Msb0, u8> = BitVec::new();
        for c in field_a.chars() {
            let bits = Self::as_ia5(c as u8)
                .write((deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
            acc.extend(bits);
        }
        let bits = 0_u8.write((deku::ctx::Endian::Big, deku::ctx::BitSize(6_usize)))?;
        acc.extend(bits);
        Ok(acc)
    }

    const IA5_ALPHA: u8 = 0x01;
    const IA5_SPACE: u8 = 0x20;
    const IA5_DIGIT: u8 = 0x30;
    const ASC_DIGIT: u8 = b'0';
    const ASC_ALPHA: u8 = b'A';
    const ASC_SPACE: u8 = b' ';
    const ASC_ERROR: u8 = b'?';

    /// parse into ascii from IA5 char array
    const fn as_ascii(code: u8) -> u8 {
        // space
        if code == Self::IA5_SPACE {
            return Self::ASC_SPACE;
        }
        // digit
        if Self::IA5_DIGIT <= code && code < Self::IA5_DIGIT + 10 {
            return Self::ASC_DIGIT + (code - Self::IA5_DIGIT);
        }
        // letter
        if Self::IA5_ALPHA <= code && code < Self::IA5_ALPHA + 26 {
            return Self::ASC_ALPHA + (code - Self::IA5_ALPHA);
        }
        Self::ASC_ERROR
    }

    /// parse from IA5 char as u8 to u8 value
    const fn as_ia5(code: u8) -> u8 {
        // space
        if code == Self::ASC_SPACE {
            return Self::IA5_SPACE;
        }
        // digit
        if Self::ASC_DIGIT <= code && code < Self::ASC_DIGIT + 10 {
            return Self::IA5_DIGIT + (code - Self::ASC_DIGIT);
        }
        // letter
        if Self::ASC_ALPHA <= code && code < Self::ASC_ALPHA + 26 {
            return Self::IA5_ALPHA + (code - Self::ASC_ALPHA);
        }
        Self::ASC_ERROR
    }
}

/// Mode S Comm B data as extracted from the aircraft
/// transponder
///
/// Data Item I048/250, Mode S MB Data
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct ModeSMBData {
    #[deku(bytes = "1", update = "self.mb_data.len()")]
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
        reader = "read::bits_i16_to_f32(rest, 16, Self::MODIFIER, Op::Multiply)",
        writer = "write::f32_i32(&self.x, 16, Self::MODIFIER, Op::Divide)"
    )]
    pub x: f32,
    #[deku(
        reader = "read::bits_i16_to_f32(rest, 16, Self::MODIFIER, Op::Multiply)",
        writer = "write::f32_i32(&self.y, 16, Self::MODIFIER, Op::Divide)"
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
        reader = "read::bits_to_f32(rest, 16, modifier::groundspeed(), Op::Multiply)",
        writer = "write::f32_u32(&self.groundspeed, 16, modifier::groundspeed(), Op::Divide)"
    )]
    pub groundspeed: f32,
    #[deku(
        reader = "read::bits_to_f32(rest, 16, modifier::heading1(), Op::Multiply)",
        writer = "write::f32_u32(&self.heading, 16, modifier::heading1(), Op::Divide)"
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
    pub reserved: Option<u32>,
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
        reader = "read::bits_to_f32(rest, 8, Self::MODIFIER, Op::Divide)",
        writer = "write::f32_u32(&self.horizontal_stddev, 8, Self::MODIFIER, Op::Multiply)"
    )]
    pub horizontal_stddev: f32,
    #[deku(
        reader = "read::bits_to_f32(rest, 8, Self::MODIFIER, Op::Divide)",
        writer = "write::f32_u32(&self.vertical_stddev, 8, Self::MODIFIER, Op::Multiply)"
    )]
    pub vertical_stddev: f32,
    #[deku(
        reader = "read::bits_to_f32(rest, 8, modifier::groundspeed(), Op::Multiply)",
        writer = "write::f32_u32(&self.groundspeed_stddev, 8, modifier::groundspeed(), Op::Divide)"
    )]
    pub groundspeed_stddev: f32,
    #[deku(
        reader = "read::bits_to_f32(rest, 8, modifier::heading2(), Op::Multiply)",
        writer = "write::f32_u32(&self.heading_stddev, 8, modifier::heading2(), Op::Divide)"
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
    #[deku(reader = "read_fspec(rest)")]
    pub fspec: Vec<u8>,
    #[deku(
        skip,
        cond = "is_fspec(0b1000_0000, fspec, 0)",
        reader = "read::bits_to_optionf32(rest, 8, Self::runlength_modifier(), Op::Multiply)",
        writer = "write::f32_optionu32(&self.srl, 8, Self::runlength_modifier(), Op::Divide)"
    )]
    pub srl: Option<f32>,
    #[deku(skip, cond = "is_fspec(0b100_0000, fspec, 0)", bytes = "1")]
    pub srr: Option<u8>,
    #[deku(skip, cond = "is_fspec(0b10_0000, fspec, 0)", bytes = "1")]
    pub sam: Option<i8>,
    #[deku(
        skip,
        cond = "is_fspec(0b1_0000, fspec, 0)",
        reader = "read::bits_to_optionf32(rest, 8, Self::runlength_modifier(), Op::Multiply)",
        writer = "write::f32_optionu32(&self.prl, 8, Self::runlength_modifier(), Op::Divide)"
    )]
    pub prl: Option<f32>,
    #[deku(skip, cond = "is_fspec(0b1000, fspec, 0)", bytes = "1")]
    pub pam: Option<u8>,
    #[deku(
        skip,
        cond = "is_fspec(0b100, fspec, 0)",
        reader = "read::bits_to_optionf32(rest, 8, Self::nm_modifier(), Op::Multiply)",
        writer = "write::f32_optionu32(&self.rpd, 8, Self::nm_modifier(), Op::Divide)"
    )]
    pub rpd: Option<f32>,
    #[deku(
        skip,
        cond = "is_fspec(0b100, fspec, 0)",
        reader = "read::bits_to_optionf32(rest, 8, Self::apd_modifier(), Op::Multiply)",
        writer = "write::f32_optionu32(&self.apd, 8, Self::apd_modifier(), Op::Divide)"
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
    #[deku(reader = "Self::read(rest)", writer = "Self::write(&self.num)")]
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

    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, u16), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, (f32::from(value) * Self::modifier()) as u16))
    }

    fn write(num: &u16) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (f32::from(*num) / Self::modifier()) as u8;
        value.write(Self::CTX)
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
    #[deku(reader = "Self::read(rest)")]
    pub codefxs: Vec<CodeFx>,
}

impl WarningErrorConditionsTargetClass {
    pub const FRN_48: u8 = 0b100_0000;

    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<CodeFx>), DekuError> {
        let mut codefxs = vec![];
        let mut inner_rest = rest;
        loop {
            let (rest, codefx) = CodeFx::read(inner_rest, ()).unwrap();
            inner_rest = rest;
            codefxs.push(codefx);
            if codefx.fx == FX::EndOfDataItem {
                break;
            }
        }
        Ok((inner_rest, codefxs))
    }
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
    #[deku(reader = "Self::read(rest)", writer = "Self::write(&self.height)")]
    pub height: i32,
}

impl HeightMeasuredBy3dRadar {
    pub const FRN_48: u8 = 0b1000;
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(14_usize));
    pub const MODIFIER: i32 = 25;

    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, i32), DekuError> {
        let (rest, value) = i32::read(rest, Self::CTX)?;
        Ok((rest, (value * Self::MODIFIER) as i32))
    }

    fn write(height: &i32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = height / Self::MODIFIER;
        value.write(Self::CTX)
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
