//! Encode/Decode for ASTERIX protocol using the deku library
//!
//! # Creating an Asterix packet
//! There are currently two ways of creating an AsterixPacket:
//!
//! ## From `&[u8]`
//! ```
//! use deku::prelude::*;
//! use asterix::*;
//! use asterix::data_item::*;
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
//! use asterix::*;
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
//! use asterix::*;
//! use asterix::data_item::*;
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

/// Size of category + length in bytes
const ASTERIX_HEADER_SIZE: u16 = 3;

#[derive(Debug, Default, PartialEq, DekuRead, DekuWrite)]
#[deku(endian = "big")]
pub struct AsterixPacket {
    /// Category of all `messages`
    #[deku(bytes = "1")]
    pub category: u8,
    /// Total length of `AsterixPacket`
    #[deku(update = "Self::update_len(&mut self.messages)")]
    pub length: u16,
    /// Asterix Messages
    #[deku(bytes_read = "length - ASTERIX_HEADER_SIZE", ctx = "*category")]
    pub messages: Vec<AsterixMessage>,
}

impl AsterixPacket {
    /// Update fspec and len
    pub fn finalize(&mut self) -> Result<(), DekuError> {
        for message in &mut self.messages {
            message.update_fspec();
        }
        self.update()
    }

    fn update_len(messages: &mut Vec<AsterixMessage>) -> u16 {
        let mut len: u16 = 0;
        for message in messages.iter_mut() {
            let mut bits: BitVec<Msb0, u8> = BitVec::new();
            message
                .write(&mut bits, (deku::ctx::Endian::Big, 0))
                .unwrap();
            len += (bits.len() / 8) as u16 + ASTERIX_HEADER_SIZE
        }
        len
    }
}

#[derive(Debug, PartialEq, DekuRead, DekuWrite)]
#[deku(id = "category", ctx = "_: deku::ctx::Endian, category: u8")]
/// Union of Asterix categories
pub enum AsterixMessage {
    #[deku(id = "34")]
    Cat34(Cat34),
    #[deku(id = "48")]
    Cat48(Cat48),
}

impl AsterixMessage {
    pub fn update_fspec(&mut self) {
        match self {
            Self::Cat34(c) => c.update_fspec(),
            Self::Cat48(c) => c.update_fspec(),
        }
    }
}
