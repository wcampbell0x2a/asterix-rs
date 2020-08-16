use crate::data_item::{DataSourceIdentifier, MessageType, SectorNumber, TimeOfDay};
use crate::fspec::{is_fspec, read_fspec};
use deku::prelude::*;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct Cat34 {
    #[deku(reader = "read_fspec(rest)")]
    pub fspec: Vec<u8>,
    #[deku(skip, cond = "is_fspec(0b1000_0000, fspec, 0)")]
    pub data_source_identifier: Option<DataSourceIdentifier>,
    #[deku(skip, cond = "is_fspec(0b100_0000, fspec, 0)")]
    pub message_type: Option<MessageType>,
    #[deku(skip, cond = "is_fspec(0b10_0000, fspec, 0)")]
    pub time_of_day: Option<TimeOfDay>,
    #[deku(skip, cond = "is_fspec(0b1_0000, fspec, 0)")]
    pub sector_number: Option<SectorNumber>,
}
