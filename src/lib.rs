//! Encode/Decode for ASTERIX protocol using the deku library

use deku::prelude::*;

pub mod types;
pub use crate::types::*;

mod custom_read_write;

mod fourty_eight;
use fourty_eight::Cat48;

mod thirty_four;
use thirty_four::Cat34;

pub mod data_item;
mod fspec;

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct AsterixPacket {
    #[deku(bytes = "1")]
    pub category: u8,
    #[deku(bytes = "2", update = "Self::update_len(&mut self.messages)")]
    pub length: u16,
    #[deku(
        reader = "Self::read_messages(rest, *category, *length)",
        writer = "Self::write_messages(&self.messages, *category)"
    )]
    pub messages: Vec<AsterixMessage>,
}

impl AsterixPacket {
    /// Parse as messages until the length of bits read matches `length`
    fn read_messages(
        rest: &BitSlice<Msb0, u8>,
        category: u8,
        length: u16,
    ) -> Result<(&BitSlice<Msb0, u8>, Vec<AsterixMessage>), DekuError> {
        let mut inside_rest = rest;
        let mut messages = vec![];

        // The finish len is the bytes subtracted by the length - 3 (header bytes)
        let finish_len = (inside_rest.len() / 8) - usize::from(length - 3);

        // loop until the correct number of bytes have been read, then return Vec
        loop {
            let (new_rest, value) =
                DekuRead::read(inside_rest, (deku::ctx::Endian::Big, category))?;
            let value: AsterixMessage = Result::<_, DekuError>::Ok(value)?;
            messages.push(value);
            inside_rest = new_rest;
            if inside_rest.len() / 8 == finish_len {
                break;
            }
        }
        Ok((inside_rest, messages))
    }

    fn write_messages(
        messages: &Vec<AsterixMessage>,
        category: u8,
    ) -> Result<BitVec<Msb0, u8>, DekuError> {
        let mut acc: BitVec<Msb0, u8> = BitVec::new();
        for message in messages {
            let bits = message.write((deku::ctx::Endian::Big, category))?;
            acc.extend(bits);
        }
        Ok(acc)
    }

    fn update_len(messages: &mut Vec<AsterixMessage>) -> u16 {
        let mut len: u16 = 0;
        for message in messages.iter_mut() {
            message.update().unwrap();
            let bits = message.write((deku::ctx::Endian::Big, 0)).unwrap();
            len += (bits.len() / 8) as u16 + 3
        }
        len
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "category", ctx = "_: deku::ctx::Endian, category: u8")]
/// Union of Asterix categories
pub enum AsterixMessage {
    #[deku(id = "48")]
    Cat48(Cat48),
    #[deku(id = "34")]
    Cat34(Cat34),
}
