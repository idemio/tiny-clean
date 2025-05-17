use crate::common::{char_bucket, char_mask, encode_as_hex_byte, encode_as_unicode};
use crate::encoder::{CompiledEncoderRules, Encoder, Rule, RuleType, ValidAsciiRange};

pub enum JavaScriptEncoderMode {
    Source,
    Block,
    Html,
    Attribute,
}

pub struct JavaScriptEncoderSettings {
    pub mode: JavaScriptEncoderMode,
}

pub struct JavaScriptEncoder {
    pub(crate) rules: CompiledEncoderRules,
    pub(crate) escape_char: char,
    pub(crate) invalid_char: char,
    pub(crate) ascii_properties: ValidAsciiRange,
}

impl Encoder<JavaScriptEncoderSettings> for JavaScriptEncoder {
    fn create(
        escape_char: char,
        valid_ascii_range: ValidAsciiRange,
        config: JavaScriptEncoderSettings,
    ) -> Self {
        match config.mode {
            JavaScriptEncoderMode::Source => {
                let rules = [
                    Rule::Uni {
                        ch: '\u{0008}',
                        rt: RuleType::Sub("\\b"),
                    },
                    Rule::Uni {
                        ch: '\u{0009}',
                        rt: RuleType::Sub("\\t"),
                    },
                    Rule::Uni {
                        ch: '\u{000A}',
                        rt: RuleType::Sub("\\n"),
                    },
                    Rule::Uni {
                        ch: '\u{000C}',
                        rt: RuleType::Sub("\\f"),
                    },
                    Rule::Uni {
                        ch: '\u{000D}',
                        rt: RuleType::Sub("\\r"),
                    },
                    Rule::Uni {
                        ch: '\u{2028}',
                        rt: RuleType::Esc(true)
                    },
                    Rule::Uni {
                        ch: '\u{2029}',
                        rt: RuleType::Esc(true)
                    },
                    Rule::Uni {
                        ch: '\u{0000}',
                        rt: RuleType::Esc(false),
                    },
                    Rule::Uni {
                        ch: '"',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '\'',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '\\',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Esc(false),
                    },
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                Self {
                    rules: compiled_rules,
                    escape_char,
                    invalid_char: ' ',
                    ascii_properties: valid_ascii_range,
                }
            }
            JavaScriptEncoderMode::Block => {
                let rules = [
                    Rule::Uni {
                        ch: '\u{0008}',
                        rt: RuleType::Sub("\\b"),
                    },
                    Rule::Uni {
                        ch: '\u{0009}',
                        rt: RuleType::Sub("\\t"),
                    },
                    Rule::Uni {
                        ch: '\u{000A}',
                        rt: RuleType::Sub("\\n"),
                    },
                    Rule::Uni {
                        ch: '\u{000C}',
                        rt: RuleType::Sub("\\f"),
                    },
                    Rule::Uni {
                        ch: '\u{000D}',
                        rt: RuleType::Sub("\\r"),
                    },
                    Rule::Uni {
                        ch: '\u{0000}',
                        rt: RuleType::Esc(false),
                    },
                    Rule::Uni {
                        ch: '/',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                      ch: '\u{2028}',
                        rt: RuleType::Esc(true)
                    },
                    Rule::Uni {
                        ch: '\u{2029}',
                        rt: RuleType::Esc(true)
                    },
                    Rule::Uni {
                        ch: '-',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '"',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '\'',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '\\',
                        rt: RuleType::Esc(true),
                    },
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                Self {
                    rules: compiled_rules,
                    escape_char,
                    invalid_char: ' ',
                    ascii_properties: valid_ascii_range,
                }
            }
            JavaScriptEncoderMode::Html => {
                let rules = [
                    Rule::Uni {
                        ch: '\u{0008}',
                        rt: RuleType::Sub("\\b"),
                    },
                    Rule::Uni {
                        ch: '\u{0009}',
                        rt: RuleType::Sub("\\t"),
                    },
                    Rule::Uni {
                        ch: '\u{000A}',
                        rt: RuleType::Sub("\\n"),
                    },
                    Rule::Uni {
                        ch: '\u{000C}',
                        rt: RuleType::Sub("\\f"),
                    },
                    Rule::Uni {
                        ch: '\u{000D}',
                        rt: RuleType::Sub("\\r"),
                    },
                    Rule::Uni {
                        ch: '\u{0000}',
                        rt: RuleType::Esc(false),
                    },
                    Rule::Uni {
                        ch: '\u{2028}',
                        rt: RuleType::Esc(true)
                    },
                    Rule::Uni {
                        ch: '\u{2029}',
                        rt: RuleType::Esc(true)
                    },
                    Rule::Uni {
                        ch: '/',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '-',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '"',
                        rt: RuleType::Esc(false),
                    },
                    Rule::Uni {
                        ch: '\'',
                        rt: RuleType::Esc(false),
                    },
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '\\',
                        rt: RuleType::Esc(true),
                    },
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                Self {
                    rules: compiled_rules,
                    escape_char,
                    invalid_char: ' ',
                    ascii_properties: valid_ascii_range,
                }
            }
            JavaScriptEncoderMode::Attribute => {
                let rules = [
                    Rule::Uni {
                        ch: '\u{0008}',
                        rt: RuleType::Sub("\\b"),
                    },
                    Rule::Uni {
                        ch: '\u{0009}',
                        rt: RuleType::Sub("\\t"),
                    },
                    Rule::Uni {
                        ch: '\u{000A}',
                        rt: RuleType::Sub("\\n"),
                    },
                    Rule::Uni {
                        ch: '\u{000C}',
                        rt: RuleType::Sub("\\f"),
                    },
                    Rule::Uni {
                        ch: '\u{000D}',
                        rt: RuleType::Sub("\\r"),
                    },
                    Rule::Uni {
                        ch: '\u{2028}',
                        rt: RuleType::Esc(true)
                    },
                    Rule::Uni {
                        ch: '\u{2029}',
                        rt: RuleType::Esc(true)
                    },
                    Rule::Uni {
                        ch: '\u{0000}',
                        rt: RuleType::Esc(false),
                    },
                    Rule::Uni {
                        ch: '"',
                        rt: RuleType::Esc(false),
                    },
                    Rule::Uni {
                        ch: '\'',
                        rt: RuleType::Esc(false),
                    },
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Esc(true),
                    },
                    Rule::Uni {
                        ch: '\\',
                        rt: RuleType::Esc(true),
                    },
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                Self {
                    rules: compiled_rules,
                    escape_char,
                    invalid_char: ' ',
                    ascii_properties: valid_ascii_range,
                }
            }
        }
    }

    fn compiled_rules(&self) -> &CompiledEncoderRules {
        &self.rules
    }

    fn escape_char(&self) -> char {
        self.escape_char
    }

    fn invalid_char(&self) -> char {
        self.invalid_char
    }

    fn ascii_properties(&self) -> &ValidAsciiRange {
        &self.ascii_properties
    }

    fn encode(&self, input: &str) -> String {
        // TODO bench mark this to find best ratio
        let init_capacity = (u16::MAX / 2)
            .min((self.compiled_rules().output_buffer_max_len * input.len()) as u16)
            as usize;
        let mut output = String::with_capacity(init_capacity);

        for character in input.chars() {
            // Fast route with ascii mask
            if (character as u32) < 256 {
                let bucket = char_bucket(character);
                let mask = char_mask(character);

                // Check if there is a rule defined for the character
                if self.compiled_rules().valid_ascii_mask[bucket] & mask == 0 {
                    if self.compiled_rules().encode_map.contains_key(&character) {
                        let encode_option =
                            self.compiled_rules().encode_map.get(&character).unwrap();

                        if *encode_option {
                            output.push(self.escape_char());
                            output.push(character);
                            continue;
                        }

                        if character as u32 <= 0xFF {
                            encode_as_hex_byte(self.escape_char(), &mut output, character);
                            continue;
                        } else {
                            encode_as_unicode(self.escape_char(), &mut output, character);
                            continue;
                        }
                    } else if self.compiled_rules().replace_map.contains_key(&character) {
                        let replace = self.compiled_rules().replace_map.get(&character).unwrap();
                        output.push_str(replace);
                        continue;
                    } else if self.compiled_rules().invalid_set.contains(&character) {
                        output.push(self.invalid_char());
                        continue;
                    }
                } else {
                    match self.ascii_properties() {
                        ValidAsciiRange::ASCII => {
                            if (character as u32) < 127 {
                                output.push(character);
                                continue;
                            }
                        }
                        _ => {}
                    }
                }
            } else if self.compiled_rules().replace_map.contains_key(&character) {
                let replace = self.compiled_rules().replace_map.get(&character).unwrap();
                output.push_str(replace);
                continue;
            } else if self.compiled_rules().encode_map.contains_key(&character) {
                if character as u32 <= 0xFF {
                    encode_as_hex_byte(self.escape_char(), &mut output, character);
                    continue;
                } else {
                    encode_as_unicode(self.escape_char(), &mut output, character);
                    continue;
                }
            } else if self.compiled_rules().invalid_set.contains(&character) {
                output.push(self.invalid_char());
                continue;
            }

            match self.ascii_properties() {
                ValidAsciiRange::NoRestrict => {
                    output.push(character);
                }
                ValidAsciiRange::ASCIIExtended => {
                    if (character as u32) > 0xFF {
                        encode_as_unicode(self.escape_char(), &mut output, character);
                    } else {
                        output.push(character);
                    }
                }
                ValidAsciiRange::ASCII => {
                    if character as u32 <= 0xFF {
                        encode_as_hex_byte(self.escape_char(), &mut output, character);
                    } else {
                        encode_as_unicode(self.escape_char(), &mut output, character);
                    }
                }
            }
        }
        output.shrink_to_fit();
        output
    }
}

#[cfg(test)]
mod test {
    use crate::encoder::*;
    use crate::java_script_encoder::{
        JavaScriptEncoder, JavaScriptEncoderMode, JavaScriptEncoderSettings,
    };

    fn unicode_test<T: Sized>(encoder: &impl Encoder<T>) {
        let simple_unicode_test = encoder.encode(&'\u{1234}'.to_string());
        assert_eq!("\u{1234}", simple_unicode_test);
    }

    fn generic_tests<T: Sized>(encoder: &impl Encoder<T>) {
        let backspace_test = encoder.encode(&'\u{0008}'.to_string());
        assert_eq!("\\b", backspace_test);

        let tab_test = encoder.encode(&'\t'.to_string());
        assert_eq!("\\t", tab_test);

        let newline_test = encoder.encode(&'\n'.to_string());
        assert_eq!("\\n", newline_test);

        let carriage_return_test = encoder.encode(&'\r'.to_string());
        assert_eq!("\\r", carriage_return_test);

        let nul_test = encoder.encode(&'\u{0000}'.to_string());
        assert_eq!("\\x00", nul_test);

        let line_separator_test = encoder.encode(&'\u{2028}'.to_string());
        let line_separator_assertion = "\\u2028".to_string();
        assert_eq!(line_separator_assertion, line_separator_test);

        let paragraph_separator_test = encoder.encode(&'\u{2029}'.to_string());
        let paragraph_separator_assertion = "\\u2029".to_string();
        assert_eq!(paragraph_separator_assertion, paragraph_separator_test);

        let simple_lower_case_test = encoder.encode(&"abcd".to_string());
        assert_eq!("abcd", simple_lower_case_test);

        let simple_upper_case_test = encoder.encode(&"ABCD".to_string());
        assert_eq!("ABCD", simple_upper_case_test);
    }

    fn ascii_only_tests<T: Sized>(encoder: &impl Encoder<T>) {
        let simple_unicode_test = encoder.encode(&'\u{1234}'.to_string());
        assert_eq!("\\u1234", simple_unicode_test);

        let high_ascii_test = encoder.encode(&'\u{ff}'.to_string());
        assert_eq!("\\xff", high_ascii_test);
    }

    fn ascii_extended_tests<T: Sized>(encoder: &impl Encoder<T>) {
        let high_ascii_test = encoder.encode(&'\u{00ff}'.to_string());
        assert_eq!("\u{00ff}", high_ascii_test);
    }
    #[test]
    fn java_script_block_ascii_only() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::ASCII,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Block,
            },
        );

        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));
        assert_eq!("\\-", encoder.encode("-"));

        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn java_script_block_ascii_extended() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::ASCIIExtended,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Block,
            },
        );

        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));

        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn java_script_block_no_restrict() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::NoRestrict,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Block,
            },
        );

        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));

        generic_tests(&encoder);
        unicode_test(&encoder);
    }

    #[test]
    fn java_script_source_ascii_only() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::ASCII,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Source,
            },
        );

        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\x26", encoder.encode("&"));
        assert_eq!("/", encoder.encode("/"));

        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn java_script_source_ascii_extended() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::ASCIIExtended,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Source,
            },
        );

        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\x26", encoder.encode("&"));
        assert_eq!("/", encoder.encode("/"));

        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn java_script_source_no_restrict() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::NoRestrict,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Source,
            },
        );

        assert_eq!("\\\"", encoder.encode("\""));
        assert_eq!("\\\'", encoder.encode("\'"));
        assert_eq!("\\x26", encoder.encode("&"));
        assert_eq!("/", encoder.encode("/"));

        generic_tests(&encoder);
        unicode_test(&encoder);
    }

    #[test]
    fn java_script_html_ascii_only() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::ASCII,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Html,
            },
        );

        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));
        assert_eq!("\\-", encoder.encode("-"));


        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn java_script_html_ascii_extended() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::ASCIIExtended,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Html,
            },
        );

        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));
        assert_eq!("\\-", encoder.encode("-"));

        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn java_script_html_no_restrict() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::NoRestrict,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Html,
            },
        );

        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("\\/", encoder.encode("/"));
        assert_eq!("\\-", encoder.encode("-"));

        generic_tests(&encoder);
        unicode_test(&encoder);
    }

    #[test]
    fn java_script_attribute_ascii_only() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::ASCII,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Attribute,
            },
        );

        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));


        generic_tests(&encoder);
        ascii_only_tests(&encoder);
    }

    #[test]
    fn java_script_attribute_ascii_extended() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::ASCIIExtended,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Attribute,
            },
        );

        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));

        generic_tests(&encoder);
        ascii_extended_tests(&encoder);
    }

    #[test]
    fn java_script_attribute_no_restrict() {
        let encoder = JavaScriptEncoder::create(
            '\\',
            ValidAsciiRange::NoRestrict,
            JavaScriptEncoderSettings {
                mode: JavaScriptEncoderMode::Attribute,
            },
        );

        assert_eq!("\\x22", encoder.encode("\""));
        assert_eq!("\\x27", encoder.encode("\'"));
        assert_eq!("/", encoder.encode("/"));

        generic_tests(&encoder);
        unicode_test(&encoder);
    }

}
