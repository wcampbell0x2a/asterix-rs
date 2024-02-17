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
    use std::io::Read;

    use super::*;

    /// Read in big-endian bits to u32, multiply by f32, return f32
    pub(crate) fn bits_to_f32<R: Read>(
        reader: &mut Reader<R>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<f32, DekuError> {
        let value =
            u32::from_reader_with_ctx(reader, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))?;
        Ok(op(value as f32, modifier, modifier_op))
    }

    /// Read in big-endian bits to i16, multiply by f32, return f32
    pub(crate) fn bits_i16_to_f32<R: Read>(
        reader: &mut Reader<R>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<f32, DekuError> {
        let value =
            i16::from_reader_with_ctx(reader, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))?;
        Ok(op(f32::from(value), modifier, modifier_op))
    }

    pub(crate) fn op(value: f32, modifier: f32, modifier_op: Op) -> f32 {
        modifier_op.calculate(value as f32, modifier)
    }

    /// Read in big-endian bits, multiply by f32, return Some(f32)
    pub(crate) fn bits_to_optionf32<R: Read>(
        reader: &mut Reader<R>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
    ) -> Result<Option<f32>, DekuError> {
        bits_to_f32(reader, bits, modifier, modifier_op).map(|f| Some(f))
    }
}

pub mod write {
    use std::io::Write;

    use super::*;

    pub(crate) fn f32_u32<W: Write>(
        value: &f32,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
        writer: &mut Writer<W>,
    ) -> Result<(), DekuError> {
        // TODO this should be function for this and the other one
        let value = modifier_op.calculate(*value, modifier);
        (value as u32).to_writer(writer, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))
    }

    pub(crate) fn f32_optionu32<W: Write>(
        value: &Option<f32>,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
        writer: &mut Writer<W>,
    ) -> Result<(), DekuError> {
        value.map_or(Ok(()), |value| f32_u32(&value, bits, modifier, modifier_op, writer))
    }

    pub(crate) fn f32_i32<W: Write>(
        value: &f32,
        bits: usize,
        modifier: f32,
        modifier_op: Op,
        writer: &mut Writer<W>,
    ) -> Result<(), DekuError> {
        let value = modifier_op.calculate(*value, modifier);
        (value as i32).to_writer(writer, (deku::ctx::Endian::Big, deku::ctx::BitSize(bits)))
    }
}
