mod types;

use deku::prelude::*;
pub use crate::types::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct AsterixPacket {
    #[deku(bytes = "1")]
    pub category: u8,
    #[deku(bytes = "2")]
    pub length: u16,
    // TODO Update to Vec<T> till length is read
    #[deku(ctx = "*category")]
    pub message: AsterixMessage,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "category", ctx = "_: deku::ctx::Endian, category: u8")]
pub enum AsterixMessage {
    #[deku(id = "48")]
    Cat48(Cat48),
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Cat48 {
    // TODO use fx as last bit in fspec for extended fspec fields
    #[deku(bytes = "1")]
    pub fspec1: u8,
    #[deku(bytes = "1")]
    pub fspec2: u8,
    #[deku(bytes = "1")]
    pub fspec3: u8,
    #[deku(skip, cond = "is_fspec(0b1000_0000, *fspec1)")]
    pub data_source_identifier: Option<DataSourceIdentifier>,
    #[deku(skip, cond = "is_fspec(0b100_0000, *fspec1)")]
    pub time_of_day: Option<TimeOfDay>,
    #[deku(skip, cond = "is_fspec(0b10_0000, *fspec1)")]
    pub target_report_descriptor: Option<TargetReportDescriptor>,
    #[deku(skip, cond = "is_fspec(0b1_0000, *fspec1)")]
    pub measured_position_in_polar_coordinates: Option<MeasuredPositionInPolarCoordinates>,
    #[deku(skip, cond = "is_fspec(0b1000, *fspec1)")]
    pub mode_3_a_code_in_octal_representation: Option<Mode3ACodeInOctalRepresentation>,
    #[deku(skip, cond = "is_fspec(0b100, *fspec1)")]
    pub flight_level_in_binary_repre: Option<FlightLevelInBinaryRepresentation>,
    // TODO check fspec
    #[deku(skip, cond = "is_fspec(0b100_0000, *fspec2)")]
    pub aircraft_address: Option<AircraftAddress>,
    // TODO check fspec
    #[deku(skip, cond = "is_fspec(0b10_0000, *fspec2)")]
    pub aircraft_identification: Option<AircraftIdentification>,
    // TODO check fspec
    #[deku(skip, cond = "is_fspec(0b100_0000, *fspec2)")]
    pub mode_smb_data: Option<ModeSMBData>,
    #[deku(skip, cond = "is_fspec(0b1_0000, *fspec2)")]
    pub track_number: Option<TrackNumber>,
    #[deku(skip, cond = "is_fspec(0b100, *fspec2)")]
    // TODO handle special float
    pub calculated_track_velocity: Option<CalculatedTrackVelocity>,
    #[deku(skip, cond = "is_fspec(0b10, *fspec2)")]
    pub track_status: Option<TrackStatus>,
    #[deku(skip, cond = "is_fspec(0b10, *fspec3)")]
    pub communications_capability_flight_status: Option<CommunicationsCapabilityFlightStatus>,
}

fn is_fspec(dataitem_fspec: u8, fspec: u8) -> bool {
    dataitem_fspec & fspec != dataitem_fspec
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct DataSourceIdentifier {
    #[deku(bytes = "1")]
    pub sac: u8,
    #[deku(bytes = "1")]
    pub sic: u8,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct TimeOfDay {
    #[deku(reader = "Self::read(rest)", writer = "Self::write(&self.time)")]
    pub time: f32,
}

impl TimeOfDay {
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(24usize));

    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u32::read(rest, Self::CTX)?;
        Ok((rest, value as f32 / 128.0))
    }

    fn write(time: &f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (*time * 128.0) as u32;
        value.write(Self::CTX)
    }
}

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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct MeasuredPositionInPolarCoordinates {
    #[deku(reader = "Self::read_rho(rest)", writer = "Self::write_rho(&self.rho)")]
    pub rho: f32,
    #[deku(
        reader = "Self::read_theta(rest)",
        writer = "Self::write_theta(&self.theta)"
    )]
    pub theta: f32,
}

impl MeasuredPositionInPolarCoordinates {
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(16usize));

    fn read_rho(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, value as f32 * (1.0 / 256.0)))
    }

    fn write_rho(rho: &f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (*rho / (1.0 / 256.0)) as u16;
        value.write(Self::CTX)
    }

    fn read_theta(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, value as f32 * (360.0 / 65536.0)))
    }

    fn write_theta(rho: &f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (*rho / (360.0 / 65536.0)) as u16;
        value.write(Self::CTX)
    }
}

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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct FlightLevelInBinaryRepresentation {
    pub v: V,
    pub g: G,
    // TODO check wireshark
    #[deku(bits = "14")]
    pub flight_level: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct AircraftAddress {
    #[deku(bytes = "3")]
    pub address: u32,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct AircraftIdentification {
    /// IA5 char array
    #[deku(
        reader = "AircraftIdentification::read(rest)",
        writer = "AircraftIdentification::write(&self.identification)"
    )]
    pub identification: String,
}

impl AircraftIdentification {
    /// Read and convert to String
    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, String), DekuError> {
        let (rest, one) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        let (rest, two) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        let (rest, three) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        let (rest, four) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        let (rest, five) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        let (rest, six) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        let (rest, seven) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        let (rest, _) = u8::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        let value = format!(
            "{}{}{}{}{}{}{}",
            to_ascii(one) as char,
            to_ascii(two) as char,
            to_ascii(three) as char,
            to_ascii(four) as char,
            to_ascii(five) as char,
            to_ascii(six) as char,
            to_ascii(seven) as char
        );
        Ok((rest, value))
    }

    /// Parse from String to u8 and write
    fn write(field_a: &str) -> Result<BitVec<Msb0, u8>, DekuError> {
        let mut acc: BitVec<Msb0, u8> = BitVec::new();
        let mut chars = field_a.chars();
        for c in field_a.chars() {
            let bits =
                from_ascii(c as u8).write((deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
            acc.extend(bits);
        }
        let bits = 0_u8.write((deku::ctx::Endian::Big, deku::ctx::BitSize(6usize)))?;
        acc.extend(bits);
        Ok(acc)
    }
}

const ia5_alpha: u8 = 0x01;
const ia5_space: u8 = 0x20;
const ia5_digit: u8 = 0x30;
const asc_digit: u8 = b'0';
const asc_alpha: u8 = b'A';
const asc_space: u8 = b' ';
const asc_error: u8 = b'?';

/// parse into ascii from IA5 char array
const fn to_ascii(code: u8) -> u8 {
    // space
    if code == ia5_space {
        return asc_space;
    }

    // digit
    if ia5_digit <= code && code < ia5_digit + 10 {
        return asc_digit + (code - ia5_digit);
    }

    // letter
    if ia5_alpha <= code && code < ia5_alpha + 26 {
        return asc_alpha + (code - ia5_alpha);
    }

    asc_error
}

/// parse from IA5 char as u8 to u8 value
const fn from_ascii(code: u8) -> u8 {
    // space
    if code == asc_space {
        return ia5_space;
    }

    // digit
    if asc_digit <= code && code < asc_digit + 10 {
        return ia5_digit + (code - asc_digit);
    }

    // letter
    if asc_alpha <= code && code < asc_alpha + 26 {
        return ia5_alpha + (code - asc_alpha);
    }

    asc_error
}

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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
pub struct MBData {
    #[deku(count = "7")]
    pub data: Vec<u8>,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct TrackNumber {
    #[deku(bits = "4")]
    pub reserved: u8,
    #[deku(bits = "12", endian = "big")]
    pub number: u16,
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct CalculatedTrackVelocity {
    #[deku(
        reader = "Self::read_groundspeed(rest)",
        writer = "Self::write_groundspeed(&self.groundspeed)"
    )]
    pub groundspeed: f32,
    #[deku(
        reader = "Self::read_heading(rest)",
        writer = "Self::write_heading(&self.heading)"
    )]
    pub heading: f32,
}

impl CalculatedTrackVelocity {
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(16usize));

    fn read_groundspeed(
        rest: &BitSlice<Msb0, u8>,
    ) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, value as f32 * (1.0 / 16384.0)))
    }

    fn write_groundspeed(groundspeed: &f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (*groundspeed / (1.0 / 16384.0)) as u16;
        value.write(Self::CTX)
    }

    fn read_heading(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, value as f32 * (360.0 / 65536.0)))
    }

    fn write_heading(heading: &f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (*heading / (360.0 / 65536.0)) as u16;
        value.write(Self::CTX)
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
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
    #[deku(bits = "3")]
    pub reserved: u32,
    pub fx2: FX,
}

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
