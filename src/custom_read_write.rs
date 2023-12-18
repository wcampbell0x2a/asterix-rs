use deku::bitvec::{BitSlice, BitVec, Msb0};
/// Several helpers for deku reading of certain types into certain types
use deku::prelude::*;

#[allow(dead_code)]
pub(crate) enum Op {
    Multiply,
    Divide,
    Add,
    Subtract,
}

impl Op {
    pub(crate) fn calculate(&self, value: f32, modifier: f32) -> f32 {
        match self {
            Op::Multiply => value * modifier,
            Op::Divide => value / modifier,
            Op::Add => value + modifier,
            Op::Subtract => value - modifier,
        }
    }
}

pub(crate) mod read {
    use super::*;

    /// Read in big-endian bits to u32, multiply by f32, return f32
    pub(crate) fn bits_to_f32(
        rest: &BitSlice<u8, Msb0>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<(&BitSlice<u8, Msb0>, f32), DekuError> {
        let (rest, value) = u32::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))?;
        Ok(op(rest, value as f32, modifier, modifier_op))
    }

    /// Read in big-endian bits to i16, multiply by f32, return f32
    pub(crate) fn bits_i16_to_f32(
        rest: &BitSlice<u8, Msb0>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<(&BitSlice<u8, Msb0>, f32), DekuError> {
        let (rest, value) = i16::read(rest, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))?;
        Ok(op(rest, f32::from(value), modifier, modifier_op))
    }

    pub(crate) fn op(
        rest: &BitSlice<u8, Msb0>,
        value: f32,
        modifier: f32,
        modifier_op: Op,
    ) -> (&BitSlice<u8, Msb0>, f32) {
        (rest, modifier_op.calculate(value, modifier))
    }

    /// Read in big-endian bits, multiply by f32, return Some(f32)
    pub(crate) fn bits_to_optionf32(
        rest: &BitSlice<u8, Msb0>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<(&BitSlice<u8, Msb0>, Option<f32>), DekuError> {
        bits_to_f32(rest, bits, modifier, modifier_op).map(|(rest, f)| (rest, Some(f)))
    }
}

pub mod write {
    use super::*;

    pub(crate) fn f32_u32(
        value: &f32,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
        output: &mut BitVec<u8, Msb0>,
    ) -> Result<(), DekuError> {
        // TODO this should be function for this and the other one
        let value = modifier_op.calculate(*value, modifier);
        (value as u32).write(output, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))
    }

    pub(crate) fn f32_optionu32(
        value: &Option<f32>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
        output: &mut BitVec<u8, Msb0>,
    ) -> Result<(), DekuError> {
        value.map_or(Ok(()), |value| f32_u32(&value, bits, modifier, modifier_op, output))
    }

    pub(crate) fn f32_i32(
        value: &f32,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
        output: &mut BitVec<u8, Msb0>,
    ) -> Result<(), DekuError> {
        let value = modifier_op.calculate(*value, modifier);
        (value as i32).write(output, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))
    }
}
