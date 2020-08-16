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
    #[deku(id = "34")]
    Cat34(Cat34),
}
