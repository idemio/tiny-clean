use crate::common::{HEX, HEX_MASK, HEX_SHIFT, char_bucket, char_mask};
use fnv::{FnvHashMap, FnvHashSet};

/// Defines the range of ASCII characters considered valid during processing.
///
/// This enum provides different levels of character range restrictions, allowing control over
/// whether only standard ASCII, extended ASCII, or all characters are acceptable. It is primarily used
/// in scenarios requiring validation or filtering of character inputs to match specific requirements.
#[derive(Debug)]
pub enum ValidAsciiRange {
    /// Represents a state where only standard ASCII characters (values 0-127) are considered valid.
    ///
    /// # Usage
    /// - This variant restricts processing to the standard ASCII range, commonly used for basic text handling
    ///   and ensuring compatibility with systems limited to ASCII characters.
    /// - Useful when stricter validation of input is required to exclude extended or non-ASCII characters.
    ASCII,

    /// Represents a state where characters within the extended ASCII range are considered valid.
    ///
    /// # Usage
    /// - This variant allows for characters in the extended ASCII set (values 128-255) to be included in processing.
    /// - Typically used when handling text that may contain symbols, accented characters, or other extended ASCII values.
    ASCIIExtended,

    /// Represents a state where no restrictions are imposed on the range of valid ASCII characters.
    ///
    /// # Usage
    /// - This variant allows all characters to pass through unmodified, regardless of whether they belong
    ///   to the ASCII range or beyond.
    /// - Typically used in scenarios where character encoding or transformation is not required, and the input
    ///   should be preserved as-is.
    NoRestrict,
}

/// Defines the type of encoding rule applied to characters during processing.
///
/// This enum provides variants for handling different character transformation strategies,
/// such as escaping or substitution. Each variant includes its own parameters for fine-grained control
/// over how specific characters are processed to ensure the output meets required formatting or constraints.
///
/// # Variants
/// - `Esc`: Represents escape-based rules for encoding characters.
/// - `Sub`: Represents substitution-based rules for replacing characters with predefined strings.
///
/// # General Use
/// The `RuleType` enum is used in contexts where custom character encoding is required, allowing
/// for precise behavior through either direct substitution or specific escape sequences.
#[derive(Clone, Debug)]
pub enum RuleType {
    Inv,
    /// Represents an escape rule defining how a character should be escaped, with additional metadata.
    ///
    /// # Fields
    /// - `simple: bool`: Specifies whether this is a "simple" escape, where the escape character
    ///   is directly followed by the character being escaped (e.g., `\n` or `\t`), instead of a more complex encoding
    ///   (e.g., `\xHH` or `\uHHHH`).
    ///
    /// # Usage
    /// - The `Esc` variant is commonly used when certain characters need to be escaped based on specific rules
    ///   in order to ensure the output adheres to the expected format or specification.
    Esc(bool),

    /// Represents a substitution rule where the target character(s) are replaced with the specified string.
    ///
    /// # Fields
    /// - `&'static str`: The string that will replace the character(s) this rule applies to. This substitution
    ///   string is static and known at compile time, enabling efficient memory usage.
    ///
    /// # Usage
    /// - The `Sub` variant is typically used when a character or range of characters needs to be replaced
    ///   with a predefined sequence of characters. i.e. replace '<' with "&lt;"
    /// - It can handle cases where encoding requires direct substitution rather than escaping.
    Sub(&'static str),
}

/// Defines a set of rules for encoding or substituting characters, either as part of a range
/// or for individual Unicode characters.
///
/// # Variants
/// - `Range`: Applies a rule to a range of characters, potentially excluding specific characters.
///   Useful when working with contiguous groups of characters.
/// - `Uni`: Targets a single specific character with a defined encoding or substitution rule.
///
/// # Usage
/// This enum allows flexible definition of character processing rules, supporting both batch
/// operations with ranges and individual handling for special cases or exceptions.
///
/// # Remarks
/// Ensure to consider performance implications when using large ranges and exclusion lists,
/// as they may impact encoding speed.
#[derive(Clone, Debug)]
pub enum Rule {
    /// Represents a range of characters and associated rules for encoding or substitution.
    /// If a large range is selected, this can slow the encoder speed significantly.
    ///
    /// # Fields
    /// - `start_character: char`: The starting character of the range (inclusive).
    /// - `end_character: char`: The ending character of the range (exclusive). This defines
    ///   the upper bound for the range, meaning the characters within `[start_character, end_character)`
    ///   will be processed by the specified rule.
    /// - `exclude: Option<Vec<char>>`: An optional list of characters to be excluded from the range.
    ///   Any characters contained within this list will not be affected by the associated rule, even if
    ///   they fall within the range.
    /// - `rule_type: RuleType`: The rule that specifies how the characters within the range should be
    ///   handled. This could define escaping, substitution, or other transformations.
    ///
    /// # Notes
    /// - Characters within the specified range are processed according to `rule_type`, provided they
    ///   are not excluded by the `exclude` list.
    /// - The `end_character` is non-inclusive, so it is not included in the range processing.
    ///
    /// # Example
    /// ```rust
    /// use tiny_clean::encoder::Rule::Range;
    /// use tiny_clean::encoder::RuleType;
    ///
    /// // all characters from a-z would be replaced with 'replacement'
    /// // except for 'e', 'i', 'o'
    /// let range_rule = Range {
    ///     start_character: 'a',
    ///     end_character: 'z',
    ///     exclude: Some(vec!['e', 'i', 'o']),
    ///     rule_type: RuleType::Sub("replacement"),
    /// };
    /// ```
    Range {
        start_character: char,
        end_character: char,
        exclude: Option<Vec<char>>,
        rule_type: RuleType,
    },

    /// Represents a single Unicode character and its associated encoding or substitution rule.
    ///
    /// # Fields
    /// - `ch: char`: The individual Unicode character to which the specified rule applies.
    /// - `rt: RuleType`: The rule that defines how the character `ch` should be handled. This can specify
    ///   escaping, substitution, or other transformations.
    ///
    /// # Notes
    /// - The `Uni` variant is used when a rule applies to a specific character, as opposed to a range of characters.
    /// - This is particularly useful for handling special cases or exceptions that fall outside general ranges.
    ///
    /// # Example
    /// ```rust
    /// use tiny_clean::encoder::Rule::Uni;
    /// use tiny_clean::encoder::RuleType;
    ///
    /// // escapes the character '©' in the simple way (with the defined escape character) instead of encoding.
    /// let single_char_rule = Uni {
    ///     ch: '©',
    ///     rt: RuleType::Esc(true),
    /// };
    /// ```
    Uni {
        ch: char,
        rt: RuleType,
    },
}

/// A collection of precompiled rules and data structures used to configure encoding behavior.
///
/// # Overview
/// - This struct aggregates all necessary information to perform character encoding efficiently.
/// - Precomputed data like bitmasks and mappings ensure fast lookups and minimal processing overhead.
///
/// # Key Features
/// - Supports character validation, replacement, and encoding rules through specialized maps and masks.
/// - Limits output size to ensure memory safety and manageable encoding results.
///
/// # Usage
/// - Designed for internal use within the crate to streamline encoding operations.
pub struct CompiledEncoderRules {
    /// A bitmask used to validate ASCII character ranges efficiently.
    ///
    /// # Purpose
    /// - Represents a compact encoding of valid ASCII characters using a fixed-size array of bits.
    /// - Each bit indicates whether a corresponding ASCII character (0-255) is valid based on its position.
    /// - Optimized for quick validation checks during processing.
    ///
    /// # Structure
    /// - Fixed array of 8 `u32` values, where each bit corresponds to a specific ASCII character.
    pub(crate) valid_ascii_mask: [u32; 8],

    /// A mapping of characters to their replacement string representations.
    pub(crate) replace_map: FnvHashMap<char, &'static str>,

    /// A mapping of characters to their corresponding encoding options.
    pub(crate) encode_map: FnvHashMap<char, bool>,

    /// set of all characters to be replaced with the chosen 'invalid' character.
    pub(crate) invalid_set: FnvHashSet<char>,

    /// Specifies the maximum allowable length for the output buffer in bytes.
    pub(crate) output_buffer_max_len: usize
}

pub trait Encoder<C>
where
    C: Sized,
{
    /// Creates a new instance of the encoder/processor with the given configuration settings.
    ///
    /// # Parameters
    /// - `escape_char: char`: The character used as the escape prefix in encoded output.
    ///   This is typically used to introduce escape sequences.
    /// - `valid_ascii_range: ValidAsciiRange`: A struct that defines the valid range of ASCII characters
    ///   that do not require escaping or substitution. Characters outside this range will be checked
    ///   against additional encoding rules.
    /// - `config: C`: A generic configuration parameter that provides additional settings, rules, or
    ///   context required to initialize the instance. The type `C` is determined by the caller.
    ///
    /// # Returns
    /// - `Self`: An instance of the type implementing this function, initialized with the provided escape
    ///   character, valid ASCII range, and additional configuration settings.
    ///
    /// # Usage
    /// This function is typically used to create and configure an encoder or processor for text-based
    /// operations, such as escaping or replacing characters in a string. The configuration determines
    /// the specific behavior of the instance.
    fn create(escape_char: char, valid_ascii_range: ValidAsciiRange, config: C) -> Self;

    /// Compiles a set of encoding/escaping rules into an optimized representation for later use.
    /// The most commonly used ascii and extended ascii are placed into bitmask buckets.
    ///
    /// # Type Parameter
    /// - `B`: The number of input rules, specified at compile-time as a constant.
    ///
    /// # Parameters
    /// - `rules: [Rule; B]`: An array of `Rule` structures defining the encoding or substitution behavior.
    ///   Each rule can specify character ranges or individual characters alongside their transformation logic.
    ///
    /// # Returns
    /// - `CompiledEncoderRules`: A precomputed structure that contains:
    ///     - `valid_ascii_mask`: A mask representing valid ranges for fast checks on ASCII and extended ASCII characters.
    ///     - `replace_map`: A mapping of characters to corresponding substitution strings for quick lookup.
    ///     - `encode_map`: A mapping of characters to encoding options (e.g., escape sequences and minimum lengths).
    ///     - `output_buffer_max_len`: The length of the largest substitution string, used for output buffering.
    ///
    /// # Behavior
    /// - Based on the rules provided, characters in a given range are processed as either:
    ///   - Escaped with a defined `RuleType::Esc`.
    ///   - Replaced by a given string with `RuleType::Sub`.
    ///
    /// - For ASCII and extended ASCII characters (`c < 256`):
    ///     - Calculations involve determining the "bucket" index using `char_bucket(c)`
    ///       and masking out invalid characters using `char_mask(c)`.
    ///     - Efficient masking logic is applied to `valid_ascii_mask` for quick subsequent checks.
    ///     - `encode_map` and `replace_map` store encoding options or replacement strings as appropriate.
    ///     - Excluded characters within a range are skipped.
    ///
    /// - For Unicode characters beyond ASCII (`c >= 256`):
    ///     - Directly adds values to `encode_map` or `replace_map` without bucket optimizations.
    fn compile_encoder_rules<const B: usize>(rules: [Rule; B]) -> CompiledEncoderRules {
        let mut valid_mask = [u32::MAX; 8];
        let mut replace_chars: FnvHashMap<char, &'static str> = FnvHashMap::default();
        let mut encode_chars: FnvHashMap<char, bool> = FnvHashMap::default();
        let mut invalid_chars: FnvHashSet<char> = FnvHashSet::default();

        // Track the longest replacement string. This ensures the output buffer is appropriately sized.
        let mut largest_replace = 1usize;

        for rule in rules {
            match rule {
                Rule::Range {
                    start_character: start,
                    end_character: end,
                    exclude,
                    rule_type,
                } => {
                    assert!(
                        (start as u32) < (end as u32),
                        "End character should be larger than the starting character."
                    );

                    // all ascii and extended ascii
                    if (start as u32) < 256 && (end as u32) < 256 {
                        match rule_type {
                            RuleType::Esc(simple) => {
                                // Process all characters in the range, unless they are in the exclusion list.
                                for char in start..=end {
                                    if exclude.as_ref().is_some_and(|excl| excl.contains(&char)) {
                                        continue;
                                    }

                                    // Determine which bucket (segment) this character belongs to.
                                    // Generate a bitmask for the character.
                                    // Update the valid_mask to mark this character as encoded.
                                    let bucket: usize = char_bucket(char);
                                    let char_mask: u32 = !char_mask(char);
                                    valid_mask[bucket] &= char_mask;
                                    encode_chars.insert(char, simple);
                                }
                            }
                            RuleType::Sub(replace_string) => {
                                for char in start..=end {
                                    if exclude.as_ref().is_some_and(|excl| excl.contains(&char)) {
                                        continue;
                                    }

                                    // Update the maximum length of a replacement string for buffer allocation.
                                    largest_replace = largest_replace.max(replace_string.len());

                                    // Determine which bucket (segment) this character belongs to.
                                    // Generate a bitmask for the character.
                                    // Update the valid_mask to mark this character as encoded.
                                    let bucket: usize = char_bucket(char);
                                    let char_mask: u32 = !char_mask(char);
                                    valid_mask[bucket] &= char_mask;
                                    replace_chars.insert(char, replace_string);
                                }
                            }
                            RuleType::Inv => {
                                for char in start..=end {
                                    if exclude.as_ref().is_some_and(|exl| exl.contains(&char)) {
                                        continue;
                                    }
                                    let bucket: usize = char_bucket(char);
                                    let char_mask: u32 = !char_mask(char);
                                    valid_mask[bucket] &= char_mask;
                                    invalid_chars.insert(char);
                                }
                            }
                        }
                    } else {
                        match rule_type {
                            RuleType::Esc(simple) => {
                                for char in start..=end {
                                    if exclude.as_ref().is_some_and(|excl| excl.contains(&char)) {
                                        continue;
                                    }
                                    encode_chars.insert(char, simple);
                                }
                            }
                            RuleType::Sub(replace_string) => {
                                for char in start..=end {
                                    if exclude.as_ref().is_some_and(|excl| excl.contains(&char)) {
                                        continue;
                                    }

                                    // Update the maximum length of a replacement string for buffer allocation.
                                    largest_replace = largest_replace.max(replace_string.len());
                                    replace_chars.insert(char, replace_string);
                                }
                            }
                            RuleType::Inv => {
                                for char in start..=end {
                                    if exclude.as_ref().is_some_and(|exl| exl.contains(&char)) {
                                        continue;
                                    }
                                    invalid_chars.insert(char);
                                }
                            }
                        }
                    }
                }
                Rule::Uni {
                    ch: c,
                    rt: rule_type,
                } => {
                    if (c as u32) <= 256 {
                        match rule_type {
                            RuleType::Esc(simple) => {
                                let bucket: usize = char_bucket(c);
                                let char_mask: u32 = !char_mask(c);
                                valid_mask[bucket] &= char_mask;
                                encode_chars.insert(c, simple);
                            }
                            RuleType::Sub(replace_string) => {
                                largest_replace = largest_replace.max(replace_string.len());
                                let bucket: usize = char_bucket(c);
                                let char_mask: u32 = !char_mask(c);
                                valid_mask[bucket] &= char_mask;
                                replace_chars.insert(c, replace_string);
                            }
                            RuleType::Inv => {
                                let bucket: usize = char_bucket(c);
                                let char_mask: u32 = !char_mask(c);
                                valid_mask[bucket] &= char_mask;
                                invalid_chars.insert(c);
                            }
                        }
                    } else {
                        match rule_type {
                            RuleType::Esc(simple) => {
                                encode_chars.insert(c, simple);
                            }
                            RuleType::Sub(replace_string) => {
                                largest_replace = largest_replace.max(replace_string.len());
                                replace_chars.insert(c, replace_string);
                            }
                            RuleType::Inv => {
                                invalid_chars.insert(c);
                            }
                        }
                    }
                }
            }
        }

        CompiledEncoderRules {
            valid_ascii_mask: valid_mask,
            replace_map: replace_chars,
            encode_map: encode_chars,
            invalid_set: invalid_chars,
            output_buffer_max_len: largest_replace
        }
    }
    fn compiled_rules(&self) -> &CompiledEncoderRules;
    fn escape_char(&self) -> char;
    fn invalid_char(&self) -> char;
    fn ascii_properties(&self) -> &ValidAsciiRange;

    /// Encodes the input string according to the defined character encoding rules and returns the encoded result.
    ///
    /// # Parameters
    /// - `input: &str`: The input string that needs to be encoded.
    ///
    /// # Returns
    /// `String`: The encoded string based on the specified rules, preserving or transforming characters as necessary.
    ///
    /// # Behavior
    /// - The function iterates over each character in the input string and applies character-specific rules
    ///   for encoding or substitution as defined in the `compiled_rules`.
    ///
    /// ## Encoding Process:
    /// 1. **ASCII Fast Path**:
    ///    - If the character is part of the ASCII set (code points below 256), it utilizes a quick lookup for
    ///      encoding logic using a combination of a precomputed valid ASCII mask and buckets.
    ///    - ASCII characters may be:
    ///      - Simply escaped (`\x` or `\u` escape sequences).
    ///      - Transformed based on the `encode_map` or directly pushed into the output if valid.
    /// 2. **Replacement Map**:
    ///    - If a replacement rule exists for the character (defined in `replace_map`), the replacement string
    ///      is appended to the output.
    /// 3. **Non-ASCII Characters**:
    ///    - Characters beyond the ASCII range are encoded as either:
    ///      - A hexadecimal byte escape sequence (`\xHH` for characters ≤ `0xFF`).
    ///      - A Unicode escape sequence (`\uHHHH` for characters beyond `0xFF`).
    ///
    /// ## Special Handling:
    /// - Uses `ValidAsciiRange` to control how ASCII characters are processed. For instance:
    ///   - Characters under 127 may bypass encoding based on range settings.
    /// - Handles both simple escapes (e.g., prepending an escape character) and complex transformations for
    ///   characters that require substitution or escaping.
    ///
    /// ## Performance:
    /// - The output buffer is pre-allocated based on a heuristic to minimize reallocation during encoding:
    ///   - `self.compiled_rules().output_buffer_max_len * input.len()`.
    /// - The function utilizes efficient bitwise operations for fast path lookup of ASCII characters.
    /// - Uses inline functions like `encode_as_hex_byte` and `encode_as_unicode` for escape sequence construction.
    ///
    /// ## Cleanup:
    /// - At the end, the output string is shrunk to fit the actual encoded content using `shrink_to_fit`.
    fn encode(&self, input: &str) -> String;


}
