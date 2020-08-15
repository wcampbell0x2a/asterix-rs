/// Several helpers for deku reading of certain types into certain types
use deku::prelude::*;

pub enum Op {
    Multiply,
    Divide,
    Add,
    Subtract,
}

pub mod read {
    use super::*;

    /// Read in big-endian bits to u32, multiply by f32, return f32
    pub fn bits_to_f32(
        rest: &BitSlice<Msb0, u8>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = u32::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))?;
        op(rest, value as f32, modifier, modifier_op)
    }

    /// Read in big-endian bits to i16, multiply by f32, return f32
    pub fn bits_i16_to_f32(
        rest: &BitSlice<Msb0, u8>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        let (rest, value) = i16::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))?;
        op(rest, value as f32, modifier, modifier_op)
    }

    fn op(
        rest: &BitSlice<Msb0, u8>,
        value: f32,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<(&BitSlice<Msb0, u8>, f32), DekuError> {
        match modifier_op {
            Op::Multiply => Ok((rest, value as f32 * modifier)),
            Op::Divide => Ok((rest, value as f32 / modifier)),
            Op::Add => Ok((rest, value as f32 + modifier)),
            Op::Subtract => Ok((rest, value as f32 - modifier)),
        }
    }

    /// Read in big-endian bits, multiply by f32, return Some(f32)
    pub fn bits_to_optionf32(
        rest: &BitSlice<Msb0, u8>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<(&BitSlice<Msb0, u8>, Option<f32>), DekuError> {
        bits_to_f32(rest, bits, modifier, modifier_op).map(|(rest, f32)| (rest, Some(f32)))
    }
}
