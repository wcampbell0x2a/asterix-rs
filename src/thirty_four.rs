use crate::data_item::{DataSourceIdentifier, MessageType, SectorNumber, TimeOfDay};
use crate::fspec::{is_fspec, read_fspec};
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
            fspec[0] = fspec[0] | DataSourceIdentifier::FRN_34;
        }
        if self.message_type.is_some() {
            fspec[0] = fspec[0] | MessageType::FRN_34;
        }
        if self.time_of_day.is_some() {
            fspec[0] = fspec[0] | TimeOfDay::FRN_34;
        }
        if self.sector_number.is_some() {
            fspec[0] = fspec[0] | SectorNumber::FRN_34;
        }
        // Remove trailing fspecs
        // - find last item in fspec that isn't 00...
        let mut remove_indicies = vec![];
        for (n, f) in fspec.iter().rev().enumerate() {
            if *f != 0x00 {
                break;
            }
            remove_indicies.push(fspec.len() - n);
        }
        for i in &remove_indicies {
            fspec.remove(*i - 1);
        }
        // Add FX bits
        let fspec_len = fspec.len();
        for (n, f) in fspec[..fspec_len].iter_mut().enumerate() {
            if n == fspec_len - 1 {
                break;
            }
            *f = *f | 0b0000_0001
        }
        self.fspec = fspec;
    }
}
