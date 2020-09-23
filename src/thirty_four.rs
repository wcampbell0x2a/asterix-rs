use crate::data_item::{
    AntennaRotationSpeed, DataSourceIdentifier, MessageType, SectorNumber, TimeOfDay,
};
use crate::fspec::{add_fx, is_fspec, read_fspec, trim_fspec};
use asterix_derive::UpdateFspec;
use deku::prelude::*;

/// Transmission of Monoradar Service Messages
#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite, UpdateFspec)]
#[deku(endian = "big")]
pub struct Cat34 {
    #[deku(reader = "read_fspec(rest)")]
    pub fspec: Vec<u8>,
    /// FRN 1
    #[deku(skip, cond = "is_fspec(DataSourceIdentifier::FRN_34, fspec, 0)")]
    pub data_source_identifier: Option<DataSourceIdentifier>,
    /// FRN 2
    #[deku(skip, cond = "is_fspec(MessageType::FRN_34, fspec, 0)")]
    pub message_type: Option<MessageType>,
    /// FRN 3
    #[deku(skip, cond = "is_fspec(TimeOfDay::FRN_34, fspec, 0)")]
    pub time_of_day: Option<TimeOfDay>,
    /// FRN 4
    #[deku(skip, cond = "is_fspec(SectorNumber::FRN_34, fspec, 0)")]
    pub sector_number: Option<SectorNumber>,
    /// FRN 5
    #[deku(skip, cond = "is_fspec(AntennaRotationSpeed::FRN_34, fspec, 0)")]
    pub antenna_rotation_speed: Option<AntennaRotationSpeed>,
}
