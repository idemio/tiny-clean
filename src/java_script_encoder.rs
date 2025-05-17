use crate::common::{HEX, HEX_MASK, HEX_SHIFT, char_bucket, char_mask, encode_as_hex_byte, encode_as_unicode, dump_masks_to_ascii};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum JavaScriptEncoderMode {
    Source,
    Block,
    Html,
    Attribute,
}

pub struct JavaScriptEncoder {
    ascii_only: bool,
    valid_masks: [u32; 4],
    hex_encode_quotes: bool,
}

impl JavaScriptEncoder {
    pub fn new(mode: JavaScriptEncoderMode, ascii_only: bool) -> Self {
        let mut valid_masks = [
            0,
            u32::MAX & !(char_mask('\'') | char_mask('"')),
            u32::MAX & !char_mask('\\'),
            if ascii_only {
                u32::MAX & !char_mask(127 as char)
            } else {
                u32::MAX
            },
        ];
        // For BLOCK or HTML mode, also escape '/' and '-'
        if mode == JavaScriptEncoderMode::Block || mode == JavaScriptEncoderMode::Html {
            valid_masks[1] &= !(char_mask('/') | char_mask('-'));
        }

        // For all modes except SOURCE, escape '&'
        if mode != JavaScriptEncoderMode::Source {
            valid_masks[1] &= !char_mask('&');
        }

        if cfg!(debug_assertions) {
            dump_masks_to_ascii(&valid_masks);
        }

        let hex_encode_quotes = mode == JavaScriptEncoderMode::Attribute || mode == JavaScriptEncoderMode::Html;
        JavaScriptEncoder {
            ascii_only,
            valid_masks,
            hex_encode_quotes,
        }
    }
    const LINE_SEPARATOR: char = '\u{2028}';
    const PARAGRAPH_SEPARATOR: char = '\u{2029}';

    pub fn encode(&self, input: &str) -> String {
        let starting_capacity = (u32::MAX / 2).min((input.len() * 6) as u32) as usize;
        let mut result = String::with_capacity(starting_capacity);
        for c in input.chars() {
            if c as u32 <= 127 {
                let mask_index = char_bucket(c);
                let character_mask = char_mask(c);

                if (self.valid_masks[mask_index] & character_mask) == 0 {
                    match c {
                        '\u{0008}' => {
                            result.push_str("\\b");
                            continue;
                        }
                        '\u{0009}' => {
                            result.push_str("\\t");
                            continue;
                        }
                        '\u{000a}' => {
                            result.push_str("\\n");
                            continue;
                        }
                        '\u{000c}' => {
                            result.push_str("\\f");
                            continue;
                        }
                        '\u{000d}' => {
                            result.push_str("\\r");
                            continue;
                        }
                        '\'' | '"' => {
                            if self.hex_encode_quotes {
                                encode_as_hex_byte('\\', &mut result, c);
                                continue;
                            } else {
                                result.push('\\');
                                result.push(c);
                                continue;
                            }
                        }
                        '\\' | '/' | '-' => {
                            result.push('\\');
                            result.push(c);
                            continue;
                        }
                        _ => {
                            encode_as_hex_byte('\\', &mut result, c);
                            continue;
                        }
                    }
                }
            } else if self.ascii_only || c == Self::LINE_SEPARATOR || c == Self::PARAGRAPH_SEPARATOR
            {
                if c as u32 <= 0xFF {
                    encode_as_hex_byte('\\', &mut result, c);
                    continue;
                } else {
                    encode_as_unicode('\\', &mut result, c);
                    continue;
                }
            }
            result.push(c);
        }

        result.shrink_to_fit();
        result
    }
}

#[cfg(test)]
mod test {

    use crate::java_script_encoder::{JavaScriptEncoder, JavaScriptEncoderMode};
    fn generic_tests(encoder: &JavaScriptEncoder) {
        assert_eq!("\\b", encoder.encode("\u{8}"));
        assert_eq!("\\t", encoder.encode("\t"));
        assert_eq!("\\n", encoder.encode("\n"));
        assert_eq!("\\r", encoder.encode("\r"));
        assert_eq!("\\x00", encoder.encode("\u{0000}"));
        assert_eq!("\\u2028", encoder.encode("\u{2028}"));
        assert_eq!("\\u2029", encoder.encode("\u{2029}"));
        assert_eq!("abcd", encoder.encode("abcd"));
        assert_eq!("ABCD", encoder.encode("ABCD"));
    }

    fn ascii_only_tests(encoder: &JavaScriptEncoder) {
        assert_eq!("\\u1234", encoder.encode("\u{1234}"));
        assert_eq!("\\xff", encoder.encode("\u{ff}"));
    }

    fn ascii_extended_tests(encoder: &JavaScriptEncoder) {
        assert_eq!("\u{00ff}", encoder.encode("\u{00ff}"));
    }
    #[test]
    fn t_java_script_block_ascii_only() {
        let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Block, true);
        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));
        assert_eq!("\\-", encoder.encode("-"));
        assert_eq!("\\x26", encoder.encode("&"));
        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn t_java_script_block_ascii_extended() {
        let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Block, false);
        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\x26", encoder.encode("&"));
        assert_eq!("\\/", encoder.encode("/"));
        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn t_java_script_source_ascii_only() {
        let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Source, true);
        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));
        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn t_java_script_source_ascii_extended() {
        let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Source, false);
        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));
        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn t_java_script_html_ascii_only() {
        let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Html, true);
        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));
        assert_eq!("\\-", encoder.encode("-"));
        assert_eq!("\\x26", encoder.encode("&"));
        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn t_java_script_html_ascii_extended() {
        let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Html, false);
        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));
        assert_eq!("\\-", encoder.encode("-"));
        assert_eq!("\\x26", encoder.encode("&"));
        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn t_java_script_attribute_ascii_only() {
        let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Attribute, true);
        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));
        assert_eq!("\\x26", encoder.encode("&"));
        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn t_java_script_attribute_ascii_extended() {
        let encoder = JavaScriptEncoder::new(JavaScriptEncoderMode::Attribute, false);
        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));
        assert_eq!("\\x26", encoder.encode("&"));
        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

}
