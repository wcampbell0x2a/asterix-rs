mod types;

pub use crate::types::*;
use deku::prelude::*;

//TODO add Units for read/write

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
    #[deku(reader = "read_fspec(rest)")]
    pub fspec: Vec<u8>,
    #[deku(skip, cond = "is_fspec(0b1000_0000, fspec, 0)")]
    pub data_source_identifier: Option<DataSourceIdentifier>,
    #[deku(skip, cond = "is_fspec(0b100_0000, fspec, 0)")]
    pub time_of_day: Option<TimeOfDay>,
    #[deku(skip, cond = "is_fspec(0b10_0000, fspec, 0)")]
    pub target_report_descriptor: Option<TargetReportDescriptor>,
    #[deku(skip, cond = "is_fspec(0b1_0000, fspec, 0)")]
    pub measured_position_in_polar_coordinates: Option<MeasuredPositionInPolarCoordinates>,
    #[deku(skip, cond = "is_fspec(0b1000, fspec, 0)")]
    pub mode_3_a_code_in_octal_representation: Option<Mode3ACodeInOctalRepresentation>,
    #[deku(skip, cond = "is_fspec(0b100, fspec, 0)")]
    pub flight_level_in_binary_repre: Option<FlightLevelInBinaryRepresentation>,
    #[deku(skip, cond = "is_fspec(0b10, fspec, 0)")]
    pub radar_plot_characteristics: Option<RadarPlotCharacteristics>,
    #[deku(skip, cond = "is_fspec(0b1000_0000, fspec, 1)")]
    pub aircraft_address: Option<AircraftAddress>,
    #[deku(skip, cond = "is_fspec(0b100_0000, fspec, 1)")]
    pub aircraft_identification: Option<AircraftIdentification>,
    #[deku(skip, cond = "is_fspec(0b10_0000, fspec, 1)")]
    pub mode_smb_data: Option<ModeSMBData>,
    #[deku(skip, cond = "is_fspec(0b1_0000, fspec, 1)")]
    pub track_number: Option<TrackNumber>,
    #[deku(skip, cond = "is_fspec(0b1000, fspec, 1)")]
    pub calculated_position_cartesian_coor: Option<CalculatedPositionCartesianCorr>,
    #[deku(skip, cond = "is_fspec(0b100, fspec, 1)")]
    pub calculated_track_velocity: Option<CalculatedTrackVelocity>,
    #[deku(skip, cond = "is_fspec(0b10, fspec, 1)")]
    pub track_status: Option<TrackStatus>,
    #[deku(skip, cond = "is_fspec(0b10, fspec, 2)")]
    pub communications_capability_flight_status: Option<CommunicationsCapabilityFlightStatus>,
}

/// Read fspec until last bit is == 0
fn read_fspec(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {
    let mut v = vec![];
    let mut inner_rest = rest;
    loop {
        let (rest, value) = u8::read(
            inner_rest,
            (deku::ctx::Endian::Big, deku::ctx::BitSize(8_usize)),
        )?;
        inner_rest = rest;
        v.push(value);
        if value & 0x01 == 0 {
            break;
        }
    }
    Ok((inner_rest, v))
}

/// Usage in cond for checking if dataitem is to be read
fn is_fspec(dataitem_fspec: u8, fspec: &[u8], pos: usize) -> bool {
    if pos < fspec.len() {
        dataitem_fspec & fspec[pos] != dataitem_fspec
    } else {
        true
    }
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
        (deku::ctx::Endian::Big, deku::ctx::BitSize(24_usize));

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
        (deku::ctx::Endian::Big, deku::ctx::BitSize(16_usize));

    fn read_rho(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, f32::from(value) * (1.0 / 256.0)))
    }

    fn write_rho(rho: &f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (*rho / (1.0 / 256.0)) as u16;
        value.write(Self::CTX)
    }

    fn read_theta(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, f32::from(value) * (360.0 / 65536.0)))
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
        reader = "Self::read(rest)",
        writer = "Self::write(&self.identification)"
    )]
    pub identification: String,
}

impl AircraftIdentification {
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
pub struct CalculatedPositionCartesianCorr {
    #[deku(reader = "Self::read(rest)", writer = "Self::write(&self.x)")]
    x: f32,
    #[deku(reader = "Self::read(rest)", writer = "Self::write(&self.y)")]
    y: f32,
}

impl CalculatedPositionCartesianCorr {
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(16_usize));

    fn read(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = i16::read(rest, Self::CTX)?;
        Ok((rest, f32::from(value) * (1.0 / 128.0)))
    }

    fn write(val: &f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (*val / (1.0 / 128.0)) as i16;
        value.write(Self::CTX)
    }
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
        writer = "Self::write_heading(self.heading)"
    )]
    pub heading: f32,
}

impl CalculatedTrackVelocity {
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(16_usize));

    // TODO use 2_i16.pow(-14)
    fn read_groundspeed(
        rest: &BitSlice<Msb0, u8>,
    ) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, f32::from(value) * (1.0 / 16384.0)))
    }

    fn write_groundspeed(groundspeed: &f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (*groundspeed / (1.0 / 16384.0)) as u16;
        value.write(Self::CTX)
    }

    // TODO use 2_i16.pow(16)
    fn read_heading(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u16::read(rest, Self::CTX)?;
        Ok((rest, f32::from(value) * (360.0 / 65536.0)))
    }

    fn write_heading(heading: f32) -> Result<BitVec<Msb0, u8>, DekuError> {
        let value = (heading / (360.0 / 65536.0)) as u16;
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

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(ctx = "_: deku::ctx::Endian")]
pub struct RadarPlotCharacteristics {
    #[deku(reader = "read_fspec(rest)")]
    pub fspec: Vec<u8>,
    #[deku(
        skip,
        cond = "is_fspec(0b1000_0000, fspec, 0)",
        reader = "Self::runlength_reader(rest)",
        writer = "Self::runlength_writer(&self.srl)"
    )]
    pub srl: Option<f32>,
    #[deku(skip, cond = "is_fspec(0b100_0000, fspec, 0)", bytes = "1")]
    pub srr: Option<u8>,
    #[deku(skip, cond = "is_fspec(0b10_0000, fspec, 0)", bytes = "1")]
    pub sam: Option<i8>,
    #[deku(
        skip,
        cond = "is_fspec(0b1_0000, fspec, 0)",
        reader = "Self::runlength_reader(rest)",
        writer = "Self::runlength_writer(&self.prl)"
    )]
    pub prl: Option<f32>,
    #[deku(skip, cond = "is_fspec(0b1000, fspec, 0)", bytes = "1")]
    pub pam: Option<u8>,
    #[deku(
        skip,
        cond = "is_fspec(0b100, fspec, 0)",
        reader = "Self::nm_reader(rest)",
        writer = "Self::nm_writer(&self.rpd)"
    )]
    pub rpd: Option<f32>,
    #[deku(
        skip,
        cond = "is_fspec(0b100, fspec, 0)",
        reader = "Self::apd_reader(rest)",
        writer = "Self::apd_writer(&self.apd)"
    )]
    pub apd: Option<f32>,
}

impl RadarPlotCharacteristics {
    const CTX: (deku::ctx::Endian, deku::ctx::BitSize) =
        (deku::ctx::Endian::Big, deku::ctx::BitSize(8_usize));

    fn runlength_reader(
        rest: &BitSlice<Msb0, u8>,
    ) -> Result<(&BitSlice<Msb0, u8>, Option<f32>), DekuError> {
        let (rest, value) = u8::read(rest, Self::CTX)?;
        Ok((
            rest,
            Some(f32::from(value) * (360.0 / 2_u16.pow(13) as f32)),
        ))
    }

    fn runlength_writer(srl: &Option<f32>) -> Result<BitVec<Msb0, u8>, DekuError> {
        if let Some(srl) = srl {
            let value = (*srl / (360.0 / 2_u16.pow(13) as f32)) as u16;
            value.write(Self::CTX)
        } else {
            Ok(BitVec::new())
        }
    }

    fn nm_reader(
        rest: &BitSlice<Msb0, u8>,
    ) -> Result<(&BitSlice<Msb0, u8>, Option<f32>), DekuError> {
        let (rest, value) = u8::read(rest, Self::CTX)?;
        Ok((rest, Some(f32::from(value) * (1.0 / 256.0))))
    }

    fn nm_writer(nm: &Option<f32>) -> Result<BitVec<Msb0, u8>, DekuError> {
        if let Some(nm) = nm {
            let value = (*nm / (1.0 / 256.0)) as u16;
            value.write(Self::CTX)
        } else {
            Ok(BitVec::new())
        }
    }

    fn apd_reader(
        rest: &BitSlice<Msb0, u8>,
    ) -> Result<(&BitSlice<Msb0, u8>, Option<f32>), DekuError> {
        let (rest, value) = u8::read(rest, Self::CTX)?;
        Ok((
            rest,
            Some(f32::from(value) * (360.0 / 2_u16.pow(14) as f32)),
        ))
    }

    fn apd_writer(apd: &Option<f32>) -> Result<BitVec<Msb0, u8>, DekuError> {
        if let Some(apd) = apd {
            let value = (*apd / (360.0 / 2_u16.pow(14) as f32)) as u16;
            value.write(Self::CTX)
        } else {
            Ok(BitVec::new())
        }
    }
}
