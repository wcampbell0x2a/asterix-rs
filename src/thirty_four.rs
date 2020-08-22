use crate::data_item::{DataSourceIdentifier, MessageType, SectorNumber, TimeOfDay};
use crate::fspec::{add_fx, is_fspec, read_fspec, trim_fspec};
use deku::prelude::*;

#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
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

impl Cat34 {
    pub fn update_fspec(&mut self) {
        // Start with max fpsec
        let mut fspec = vec![0x00];
        // add Data Items fspecs where they are Some
        if self.data_source_identifier.is_some() {
            fspec[0] |= DataSourceIdentifier::FRN_34;
        }
        if self.message_type.is_some() {
            fspec[0] |= MessageType::FRN_34;
        }
        if self.time_of_day.is_some() {
            fspec[0] |= TimeOfDay::FRN_34;
        }
        if self.sector_number.is_some() {
            fspec[0] |= SectorNumber::FRN_34;
        }
        trim_fspec(&mut fspec);
        add_fx(&mut fspec);
        self.fspec = fspec;
    }
}
