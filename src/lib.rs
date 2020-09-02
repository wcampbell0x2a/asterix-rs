//! Encode/Decode for ASTERIX protocol using the deku library
//!
//! # Creating an Asterix packet
//! There are currently two ways of creating an AsterixPacket:
//!
//! ## From `&[u8]`
//! ```
//! use deku::prelude::*;
//! use asterix::AsterixPacket;
//!
//! let bytes = &[0x22, 0x00, 0x0b, 0xf0, 0x19, 0x0d, 0x02, 0x35, 0x6d, 0xfa, 0x60];
//! let (_, mut packet) = AsterixPacket::from_bytes((bytes, 0)).unwrap();
//! ```
//!
//! ## Packet Creation
//! Create an CAT34 Asterix packet.
//!
//! ```rust
//! use deku::prelude::*;
//! use asterix::{AsterixPacket, Cat34};
//! use asterix::data_item::*;
//! use asterix::types::*;
//!
//! let mut thirty_eight = Cat34::default();
//! thirty_eight.data_source_identifier = Some(DataSourceIdentifier { sac: 25, sic: 13 });
//! thirty_eight.message_type = Some(MessageType {
//!     t: MTYPE::SectorCrossing,
//! });
//! thirty_eight.time_of_day = Some(TimeOfDay { time: 27355.953 });
//! thirty_eight.sector_number = Some(SectorNumber { num: 135 });
//!
//! let mut packet = AsterixPacket::default();
//! packet.category = 34;
//! packet.messages = vec![asterix::AsterixMessage::Cat34(thirty_eight)];
//! ```
//!
//! # Encoding Packets
//! ```rust
//! use deku::prelude::*;
//! use asterix::AsterixPacket;
//!
//! // Create / Mutate a packet
//! let mut packet = AsterixPacket::default();
//!
//! // finalize(): Updates fspec for all packet messages, as well as setting the length as per
//! //             the protocol.
//! packet.finalize().unwrap();
//!
//! // serialize
//! packet.to_bytes().unwrap();
//! ```

use deku::prelude::*;

pub mod types;

mod custom_read_write;
mod modifier;

mod fourty_eight;
pub use fourty_eight::Cat48;

mod thirty_four;
pub use thirty_four::Cat34;

pub mod data_item;
mod fspec;

#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct AsterixPacket {
    /// Category of all `messages`
    #[deku(bytes = "1")]
    pub category: u8,
    /// Total length of `AsterixPacket`
    #[deku(bytes = "2", update = "Self::update_len(&mut self.messages)")]
    pub length: u16,
    /// Asterix Messages
    #[deku(
        reader = "Self::read_messages(rest, *category, *length)",
        writer = "Self::write_messages(&self.messages, *category)"
    )]
    pub messages: Vec<AsterixMessage>,
}

impl AsterixPacket {
    /// Update fspec and len
    pub fn finalize(&mut self) -> Result<(), DekuError> {
        for message in &mut self.messages {
            message.update_fspec();
        }
        self.update()?;
        Ok(())
    }

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
                AsterixMessage::read(inside_rest, (deku::ctx::Endian::Big, category))?;
            messages.push(value);
            inside_rest = new_rest;
            if inside_rest.len() / 8 == finish_len {
                break;
            }
        }
        Ok((inside_rest, messages))
    }

    fn write_messages(
        messages: &[AsterixMessage],
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

impl AsterixMessage {
    pub fn update_fspec(&mut self) {
        match self {
            Self::Cat48(c) => c.update_fspec(),
            Self::Cat34(c) => c.update_fspec(),
        }
    }
}
