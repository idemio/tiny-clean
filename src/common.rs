pub(crate) const HEX_SHIFT: u32 = 4;
pub(crate) const HEX_MASK: u32 = 0x0F;
pub(crate) const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];

/// Generates a bitmask for the given character `c`.
///
/// # Explanation
/// - Converts the character `c` into its Unicode scalar value (`u32`).
/// - Computes the bitwise AND with `31` (`0b11111`) to ensure the value is within the range `[0, 31]`
///   (only the lower 5 bits are used).
/// - Shifts the value `1` to the left by the result of the calculation, creating a bitmask where
///   only one specific bit is set.
///
#[inline]
pub(crate) const fn char_mask(c: char) -> u32 {
    1 << (c as u32 & 31)
}

/// Calculates the "bucket" index for the given character `c` by dividing its Unicode scalar value by 32.
///
/// # Explanation
/// - Converts the character `c` into its Unicode scalar value (`u32`).
/// - Performs a bitwise right shift by 5 (`c as u32 >> 5`), which is equivalent to integer division by 32.
/// - Casts the resulting value to `usize` for use as an index or bucket identifier in further operations.
#[inline]
pub(crate) const fn char_bucket(c: char) -> usize {
    (c as u32 >> 5) as usize
}