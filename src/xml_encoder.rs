use crate::encoder::{CompiledEncoderRules, Encoder, Rule, RuleType, ValidAsciiRange};

pub enum XmlEncoderMode {
    All,
    Content,
    Attribute,
    SingleQuotedAttribute,
    DoubleQuotedAttribute,
}

pub struct XmlEncoderSettings {
    mode: XmlEncoderMode,
}

pub struct XmlEncoder {
    pub(crate) rules: CompiledEncoderRules,
    pub(crate) escape_char: char,
    pub(crate) invalid_char: char,
    pub(crate) ascii_properties: ValidAsciiRange,
}

impl Encoder<XmlEncoderSettings> for XmlEncoder {
    fn create(
        escape_char: char,
        valid_ascii_range: ValidAsciiRange,
        config: XmlEncoderSettings,
    ) -> Self {
        match config.mode {
            XmlEncoderMode::All => {
                let rules = [
                    Rule::Range {
                        start_character: '\u{0000}',
                        end_character: '\u{003E}',
                        exclude: Some(vec!['\u{0009}', '\u{000A}', '\u{000C}', '\u{000D}']),
                        rule_type: RuleType::Inv
                    },
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Sub("&amp;"),
                    },
                    Rule::Uni {
                        ch: '<',
                        rt: RuleType::Sub("&lt;"),
                    },
                    Rule::Uni {
                        ch: '>',
                        rt: RuleType::Sub("&gt;"),
                    },
                    Rule::Uni {
                        ch: '\'',
                        rt: RuleType::Sub("&#39;"),
                    },
                    Rule::Uni {
                        ch: '"',
                        rt: RuleType::Sub("&#34;"),
                    }
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                XmlEncoder {
                    rules: compiled_rules,
                    escape_char,
                    invalid_char: ' ',
                    ascii_properties: valid_ascii_range,
                }
            }
            XmlEncoderMode::Content => {
                let rules = [
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Sub("&amp;"),
                    },
                    Rule::Uni {
                        ch: '<',
                        rt: RuleType::Sub("&lt;"),
                    },
                    Rule::Uni {
                        ch: '>',
                        rt: RuleType::Sub("&gt;"),
                    },
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                XmlEncoder {
                    rules: compiled_rules,
                    escape_char,
                    invalid_char: ' ',
                    ascii_properties: valid_ascii_range,
                }
            }
            XmlEncoderMode::Attribute => {
                let rules = [
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Sub("&amp;"),
                    },
                    Rule::Uni {
                        ch: '<',
                        rt: RuleType::Sub("&lt;"),
                    },
                    Rule::Uni {
                        ch: '\'',
                        rt: RuleType::Sub("&#39;"),
                    },
                    Rule::Uni {
                        ch: '"',
                        rt: RuleType::Sub("&#34;"),
                    },
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                XmlEncoder {
                    rules: compiled_rules,
                    escape_char,
                    invalid_char: ' ',
                    ascii_properties: valid_ascii_range,
                }
            }
            XmlEncoderMode::SingleQuotedAttribute => {
                let rules = [
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Sub("&amp;"),
                    },
                    Rule::Uni {
                        ch: '<',
                        rt: RuleType::Sub("&lt;"),
                    },
                    Rule::Uni {
                        ch: '\'',
                        rt: RuleType::Sub("&#39;"),
                    },
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                XmlEncoder {
                    rules: compiled_rules,
                    escape_char,
                    invalid_char: ' ',
                    ascii_properties: valid_ascii_range,
                }
            }
            XmlEncoderMode::DoubleQuotedAttribute => {
                let rules = [
                    Rule::Uni {
                        ch: '&',
                        rt: RuleType::Sub("&amp;"),
                    },
                    Rule::Uni {
                        ch: '<',
                        rt: RuleType::Sub("&lt;"),
                    },
                    Rule::Uni {
                        ch: '"',
                        rt: RuleType::Sub("&#34;"),
                    },
                ];
                let compiled_rules = Self::compile_encoder_rules(rules);
                XmlEncoder {
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
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::encoder::{Encoder, ValidAsciiRange};
    use crate::xml_encoder::{XmlEncoder, XmlEncoderMode, XmlEncoderSettings};

    fn generic_tests<T: Sized>(encoder: &impl Encoder<T>) {
        assert_eq!("safe", encoder.encode("safe"));
        assert_eq!("unencoded &amp; encoded", encoder.encode("unencoded & encoded"));
        //assert_eq!(" b ", encoder.encode(r"\0b\26"));
        //assert_eq!("\u{fffd}", encoder.encode("\u{fffd}"));
        assert_eq!(" ", encoder.encode("\u{ffff}"));
    }

    #[test]
    fn xml_encoder_all_ascii_only() {
        let encoder = XmlEncoder::create('\\', ValidAsciiRange::ASCII, XmlEncoderSettings {
            mode: XmlEncoderMode::All
        });
        println!("char: {:?}", '\u{26}');
        generic_tests(&encoder);
    }
}
