#![no_std]

/// Return the padded length
pub fn padded_len(len: usize) -> usize {
    let e = 63usize.saturating_sub(len.leading_zeros() as usize);
    let s = 64 - e.leading_zeros() as usize;
    let z = e - s;
    let mask = (1usize << z) - 1;
    (len + mask) & !mask
}

/// Return the padding length
#[inline]
pub fn padding_len(len: usize) -> usize {
    padded_len(len) - len
}
