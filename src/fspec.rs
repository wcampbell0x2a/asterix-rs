use deku::prelude::*;

/// Read fspec until last bit is == 0
pub fn read_fspec(rest: &BitSlice<Msb0, u8>) -> Result<(&BitSlice<Msb0, u8>, Vec<u8>), DekuError> {
    let mut v = vec![];
    let mut inner_rest = rest;
    loop {
        let (rest, value) = u8::read(
            inner_rest,
            (deku::ctx::Endian::Big, deku::ctx::BitSize(8_usize)),
        )?;
        inner_rest = rest;
        v.push(value);
        if value & 0x01 == 0 {
            break;
        }
    }
    Ok((inner_rest, v))
}

/// Usage in cond for checking if dataitem is to be read
pub fn is_fspec(dataitem_fspec: u8, fspec: &[u8], pos: usize) -> bool {
    if pos < fspec.len() {
        dataitem_fspec & fspec[pos] != dataitem_fspec
    } else {
        true
    }
}

/// Remove trailing empty fspec entries
pub fn trim_fspec(fspec: &mut Vec<u8>) {
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
}

/// Add FX bits
pub fn add_fx(fspec: &mut Vec<u8>) {
    let fspec_len = fspec.len();
    for f in fspec.iter_mut().take(fspec_len - 1) {
        *f |= 0b0000_0001
    }
}
