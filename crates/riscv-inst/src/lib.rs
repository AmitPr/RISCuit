mod macros;
pub mod rv32;
pub mod rv64;

#[cfg(feature = "c")]
pub mod rv_c;

pub(crate) use macros::*;

#[cfg(test)]
mod test;

#[inline(always)]
#[must_use = "this returns the result of the operation, without modifying the original"]
/// Get the bits in range [start..end) and place them at dst_pos
pub(crate) const fn bits32(val: u32, range: std::ops::Range<u8>, dst_pos: u8) -> u32 {
    let width = range.end - range.start;
    let mask = ((1u32 << width) - 1) & (val >> range.start);
    mask << dst_pos
}

// #[inline(always)]
// #[must_use = "this returns the result of the operation, without modifying the original"]
// /// Get the bits in range [start..end) and place them at dst_pos
// pub(crate) const fn bits16(val: u16, range: std::ops::Range<u8>, dst_pos: u8) -> u16 {
//     let width = range.end - range.start;
//     let mask = ((1u16 << width) - 1) & (val >> range.start);
//     mask << dst_pos
// }
