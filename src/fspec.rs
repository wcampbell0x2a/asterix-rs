use crate::FSPEC_IDENT;

/// Usage in cond for checking if dataitem is to be read, by checking the fspec for the data item
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
pub fn add_fx(fspec: &mut [u8]) {
    let fspec_len = fspec.len();
    for f in fspec.iter_mut().take(fspec_len - 1) {
        *f |= FSPEC_IDENT
    }
}
