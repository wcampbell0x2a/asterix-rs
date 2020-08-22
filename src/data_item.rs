//! Defined Data Items that are used for formal parsing of data structs in categories

use crate::custom_read_write::{read, write, Op};
use crate::fspec::{is_fspec, read_fspec};
use crate::types::{
    AIC, ARC, CDM, CNF, COM, DOU, FX, G, GHO, L, MAH, MSSC, MTYPE, RAB, RAD, RDP, SI, SIM, SPI,
    STAT, SUP, TCC, TRE, TYP, V,
};
use deku::prelude::*;

/// Data Item I048/010, Data Source Identifier
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct DataSourceIdentifier {
    #[deku(bytes = "1")]
    pub sac: u8,
    #[deku(bytes = "1")]
    pub sic: u8,
}

impl DataSourceIdentifier {
    pub const FRN_34: u8 = 0b1000_0000;
    pub const FRN_48: u8 = 0b1000_0000;
}

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

/// Data Item I048/020, Target Report Descriptor
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

/// Data Item I048/040, Measured Position in Polar Co-ordinates
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

/// Data Item I048/070, Mode-3/A Code in Octal Representation.
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct Mode3ACodeInOctalRepresentation {
    pub v: V,
    pub g: G,
    pub l: L,
    #[deku(bits = "1")]
    pub reserved: u8,
    #[deku(bits = "12", endian = "big")]
    pub reply: u16,
}

impl Mode3ACodeInOctalRepresentation {
    pub const FRN_48: u8 = 0b1000;
}

/// Data Item I048/090, Flight Level in Binary Representation.
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

/// Data Item I048/220, Aircraft Address
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct AircraftAddress {
    #[deku(bytes = "3", endian = "big")]
    pub address: u32,
}

impl AircraftAddress {
    pub const FRN_48: u8 = 0b1000_0000;
}

/// Data Item I048/240, Aircraft Identification
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

/// Data Item I048/161, Track Number
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

/// Data Item I048/042, Calculated Position in Cartesian Co-ordinates
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

/// Data Item I048/200, Calculated Track Velocity in Polar Co-ordinates.
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct CalculatedTrackVelocity {
    #[deku(
        reader = "read::bits_to_f32(rest, 16, Self::groundspeed_modifier(), Op::Multiply)",
        writer = "write::f32_u32(&self.groundspeed, 16, Self::groundspeed_modifier(), Op::Divide)"
    )]
    pub groundspeed: f32,
    #[deku(
        reader = "read::bits_to_f32(rest, 16, Self::heading_modifier(), Op::Multiply)",
        writer = "write::f32_u32(&self.heading, 16, Self::heading_modifier(), Op::Divide)"
    )]
    pub heading: f32,
}

impl CalculatedTrackVelocity {
    pub const FRN_48: u8 = 0b100;
    fn groundspeed_modifier() -> f32 {
        2_f32.powi(-14)
    }

    fn heading_modifier() -> f32 {
        360.0 / 2_f32.powi(16)
    }
}

/// Data Item I048/170, Track Status
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

/// Data Item I048/210, Track Quality
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct TrackQuality {
    horizontal_stddev: u8,
    vertical_stddev: u8,
    groundspeed_stddev: u8,
    heading_stddev: f32,
}

impl TrackQuality {
    pub const FRN_48: u8 = 0b1000_0000;
}

/// Data Item I048/230, Communications/ACAS Capability and Flight Status.
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

/// Data Item I048/130, Radar Plot Characteristics
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

/// Data Item I034/000, Message Type
#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct MessageType {
    pub t: MTYPE,
}

impl MessageType {
    pub const FRN_34: u8 = 0b100_0000;
    pub const FRN_48: u8 = 0b100_0000;
}

/// Data Item I034/020, Sector Number
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
