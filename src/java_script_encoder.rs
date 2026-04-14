use crate::common::{char_bucket, char_mask, encode_as_hex_byte, encode_as_unicode};
use type_state_builder::TypeStateBuilder;

#[derive(TypeStateBuilder)]
#[builder(impl_into)]
pub struct JavaScriptEncoderConfig {
    #[builder(required)]
    mode: JavaScriptEncoderMode,

    #[builder(required)]
    ascii_only: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum JavaScriptEncoderMode {
    #[default]
    Source,
    Block,
    Html,
    Attribute,
}

#[non_exhaustive]
pub struct JavaScriptEncoder {
    pub ascii_only: bool,
    pub valid_masks: [u32; 4],
    pub hex_encode_quotes: bool,
}

impl JavaScriptEncoder {
    pub fn new(config: JavaScriptEncoderConfig) -> Self {
        let mut valid_masks = [
            0,
            u32::MAX & !(char_mask('\'') | char_mask('"')),
            u32::MAX & !char_mask('\\'),
            if config.ascii_only {
                u32::MAX & !char_mask(127 as char)
            } else {
                u32::MAX
            },
        ];
        // For BLOCK or HTML mode, also escape '/' and '-'
        if config.mode == JavaScriptEncoderMode::Block || config.mode == JavaScriptEncoderMode::Html
        {
            valid_masks[1] &= !(char_mask('/') | char_mask('-'));
        }

        // For all modes except SOURCE, escape '&'
        if config.mode != JavaScriptEncoderMode::Source {
            valid_masks[1] &= !char_mask('&');
        }

        #[cfg(debug_assertions)]
        crate::common::dump_masks_to_ascii(&valid_masks);

        let hex_encode_quotes = config.mode == JavaScriptEncoderMode::Attribute
            || config.mode == JavaScriptEncoderMode::Html;
        JavaScriptEncoder {
            ascii_only: config.ascii_only,
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
    use crate::JavaScriptEncoderConfig;

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
        let encoder = JavaScriptEncoder::new(
            JavaScriptEncoderConfig::builder()
                .mode(JavaScriptEncoderMode::Block)
                .ascii_only(true)
                .build(),
        );
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
        let encoder = JavaScriptEncoder::new(
            JavaScriptEncoderConfig::builder()
                .mode(JavaScriptEncoderMode::Block)
                .ascii_only(false)
                .build(),
        );
        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\x26", encoder.encode("&"));
        assert_eq!("\\/", encoder.encode("/"));
        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn t_java_script_source_ascii_only() {
        let encoder = JavaScriptEncoder::new(
            JavaScriptEncoderConfig::builder()
                .mode(JavaScriptEncoderMode::Source)
                .ascii_only(true)
                .build(),
        );
        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));
        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn t_java_script_source_ascii_extended() {
        let encoder = JavaScriptEncoder::new(
            JavaScriptEncoderConfig::builder()
                .mode(JavaScriptEncoderMode::Source)
                .ascii_only(false)
                .build(),
        );
        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));
        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn t_java_script_html_ascii_only() {
        let encoder = JavaScriptEncoder::new(
            JavaScriptEncoderConfig::builder()
                .mode(JavaScriptEncoderMode::Html)
                .ascii_only(true)
                .build(),
        );
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
        let encoder = JavaScriptEncoder::new(
            JavaScriptEncoderConfig::builder()
                .mode(JavaScriptEncoderMode::Html)
                .ascii_only(false)
                .build(),
        );
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
        let encoder = JavaScriptEncoder::new(
            JavaScriptEncoderConfig::builder()
                .mode(JavaScriptEncoderMode::Attribute)
                .ascii_only(true)
                .build(),
        );
        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));
        assert_eq!("\\x26", encoder.encode("&"));
        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn t_java_script_attribute_ascii_extended() {
        let encoder = JavaScriptEncoder::new(
            JavaScriptEncoderConfig::builder()
                .mode(JavaScriptEncoderMode::Attribute)
                .ascii_only(false)
                .build(),
        );
        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));
        assert_eq!("\\x26", encoder.encode("&"));
        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }
}
