pub(crate) const HEX_SHIFT: u32 = 4;
pub(crate) const HEX_MASK: u32 = 0x0F;

pub(crate) const HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];
pub(crate) const U_HEX: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
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

/// Encodes a single character as a hexadecimal escape sequence and appends it to the output string.
///
/// # Parameters
/// - `escape_char: char`: The prefix character used to denote the beginning of an escape sequence.
///   Commonly, this would be a backslash (`'\\'`).
/// - `output: &mut String`: A mutable reference to the output string where the encoded hex sequence
///   will be appended.
/// - `character: char`: The character to be encoded as a hexadecimal byte.
///
/// # Behavior
/// - The function reserves enough space in the output string to accommodate the escape sequence (`4` characters).
/// - The escape sequence format is `"{escape_char}xHH"`, where `HH` represents the two-digit hexadecimal
///   value of the input character.
/// - This function uses the constants `HEX`, `HEX_SHIFT`, and `HEX_MASK` to efficiently extract and format
///   the hexadecimal digits.
#[inline]
pub(crate) fn encode_as_hex_byte(escape_char: char, output: &mut String, character: char) {
    output.push(escape_char);
    output.push('x');
    output.push(HEX[(character as u32 >> HEX_SHIFT) as usize]);
    output.push(HEX[(character as u32 & HEX_MASK) as usize]);
}

/// Encodes a single character as a Unicode escape sequence and appends it to the output string.
///
/// # Parameters
/// - `escape_char: char`: The prefix character used to indicate the beginning of the escape sequence.
///   Commonly, this would be a backslash (`'\\'`).
/// - `output: &mut String`: A mutable reference to the output string where the Unicode escape sequence
///   will be appended.
/// - `character: char`: The character to be encoded as a Unicode escape sequence.
///
/// # Behavior
/// - The function reserves enough space in the output string to accommodate the escape sequence (`6` characters).
/// - The escape sequence format is `"{escape_char}uHHHH"`, where `HHHH` represents the four-digit
///   hexadecimal Unicode code point of the input character.
/// - Hexadecimal digits are efficiently calculated and appended to the output using bitwise operations
///   and the `HEX` lookup table.
#[inline]
pub(crate) fn encode_as_unicode(escape_char: char, output: &mut String, character: char) {
    output.push(escape_char);
    output.push('u');
    output.push(HEX[(character as u32 >> (3 * HEX_SHIFT)) as usize & HEX_MASK as usize]);
    output.push(HEX[(character as u32 >> (2 * HEX_SHIFT)) as usize & HEX_MASK as usize]);
    output.push(HEX[(character as u32 >> (1 * HEX_SHIFT)) as usize & HEX_MASK as usize]);
    output.push(HEX[(character as u32 & HEX_MASK) as usize]);
}

pub(crate) fn dump_masks_to_ascii(masks: &[u32; 4]) {
    println!("Dumping Mask Values (0-127)");
    for char in '\u{0000}'..='\u{007F}' {
        let bucket = char_bucket(char);
        let mask = char_mask(char);
        if (masks[bucket] & mask) != 0 {
            println!("{char:?} {}: VALID", char as u32);
        } else {
            println!("{char:?} {}: INVALID", char as u32);
        }
    }
}
