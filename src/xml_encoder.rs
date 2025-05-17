use crate::common::{char_bucket, char_mask};

pub enum XmlEncoderMode {
    All,
    Content,
    Attribute,
    SingleQuotedAttribute,
    DoubleQuotedAttribute,
}

pub struct XmlEncoder {
    valid_masks: [u32; 4],
}

impl XmlEncoder {
    pub fn new(mode: XmlEncoderMode) -> Self {
        let base_mask = char_mask('\r') | char_mask('\t') | char_mask('\n');
        match mode {
            XmlEncoderMode::All => {
                let to_be_encoded = ['&', '<', '>', '\'', '"'];
                let mut to_be_encoded_mask = 0u32;
                for char in to_be_encoded {
                    to_be_encoded_mask |= char_mask(char);
                }
                let valid_masks = [
                    base_mask,
                    u32::MAX & !to_be_encoded_mask,
                    u32::MAX,
                    u32::MAX
                ];
                Self {valid_masks}
            }
            XmlEncoderMode::Content => {
                let to_be_encoded = ['&', '<', '>'];
                let mut to_be_encoded_mask = 0u32;
                for char in to_be_encoded {
                    to_be_encoded_mask |= char_mask(char);
                }
                let valid_masks = [
                    base_mask,
                    u32::MAX & !to_be_encoded_mask,
                    u32::MAX,
                    u32::MAX
                ];
                Self {valid_masks}
            }
            XmlEncoderMode::Attribute => {
                let to_be_encoded = ['&', '<', '\'', '"'];
                let mut to_be_encoded_mask = 0u32;
                for char in to_be_encoded {
                    to_be_encoded_mask |= char_mask(char);
                }
                let valid_masks = [
                    base_mask,
                    u32::MAX & !to_be_encoded_mask,
                    u32::MAX,
                    u32::MAX
                ];
                Self {valid_masks}
            }
            XmlEncoderMode::SingleQuotedAttribute => {
                let to_be_encoded = ['&', '<', '\''];
                let mut to_be_encoded_mask = 0u32;
                for char in to_be_encoded {
                    to_be_encoded_mask |= char_mask(char);
                }
                let valid_masks = [
                    base_mask,
                    u32::MAX & !to_be_encoded_mask,
                    u32::MAX,
                    u32::MAX
                ];
                Self {valid_masks}
            }
            XmlEncoderMode::DoubleQuotedAttribute => {
                let to_be_encoded = ['&', '<', '"'];
                let mut to_be_encoded_mask = 0u32;
                for char in to_be_encoded {
                    to_be_encoded_mask |= char_mask(char);
                }
                let valid_masks = [
                    base_mask,
                    u32::MAX & !to_be_encoded_mask,
                    u32::MAX,
                    u32::MAX
                ];
                Self {valid_masks}
            }
        }
    }

    pub fn encode(&self, input: &str) -> String {
        let max_capacity = (u32::MAX / 2).min((input.len() * 5) as u32) as usize;
        let mut result = String::with_capacity(max_capacity);
        for c in input.chars() {
            if (c as u32) < 127 {
                let bucket = char_bucket(c);
                let mask = char_mask(c);
                if c > '>' || self.valid_masks[bucket] & mask != 0 {
                    result.push(c);
                } else {
                    match c {
                        '&' => {
                            result.push('&');
                            result.push('a');
                            result.push('m');
                            result.push('p');
                            result.push(';');
                        }
                        '<' => {
                            result.push('&');
                            result.push('l');
                            result.push('t');
                            result.push(';');
                        }
                        '>' => {
                            result.push('&');
                            result.push('g');
                            result.push('t');
                            result.push(';');
                        }
                        '\'' => {
                            result.push('&');
                            result.push('#');
                            result.push('3');
                            result.push('9');
                            result.push(';');
                        }
                        '\"' => {
                            result.push('&');
                            result.push('#');
                            result.push('3');
                            result.push('4');
                            result.push(';');
                        }
                        _ => result.push(' '),
                    }
                }
            } else if c > '\u{fffd}' || (c >= '\u{fdd0}' && c <= '\u{fdef}') {
                result.push(' ');
            } else {
                result.push(c);
            }
        }
        result.shrink_to_fit();
        result
    }
}

#[cfg(test)]
mod test {
    use crate::xml_encoder::{XmlEncoder, XmlEncoderMode};

    fn generic_tests(encoder: &XmlEncoder) {
        assert_eq!("\u{fffd}", encoder.encode("\u{fffd}"));
        assert_eq!(" ", encoder.encode("\u{ffff}"));
    }
    #[test]
    fn test_all_encode() {
        let encoder = XmlEncoder::new(XmlEncoderMode::All);
        assert_eq!("&amp;", encoder.encode("&"));
        assert_eq!("&gt;", encoder.encode(">"));
        assert_eq!("&lt;", encoder.encode("<"));
        assert_eq!("&#39;", encoder.encode("\'"));
        assert_eq!("&#34;", encoder.encode("\""));
        generic_tests(&encoder);
    }

    #[test]
    fn test_content_encode() {
        let encoder = XmlEncoder::new(XmlEncoderMode::Content);
        assert_eq!("&amp;", encoder.encode("&"));
        assert_eq!("&gt;", encoder.encode(">"));
        assert_eq!("&lt;", encoder.encode("<"));
        assert_eq!("\'", encoder.encode("\'"));
        assert_eq!("\"", encoder.encode("\""));
        generic_tests(&encoder);
    }

    #[test]
    fn test_attribute_encode() {
        let encoder = XmlEncoder::new(XmlEncoderMode::Attribute);
        assert_eq!("&amp;", encoder.encode("&"));
        assert_eq!(">", encoder.encode(">"));
        assert_eq!("&lt;", encoder.encode("<"));
        assert_eq!("&#39;", encoder.encode("\'"));
        assert_eq!("&#34;", encoder.encode("\""));
        generic_tests(&encoder);
    }

    #[test]
    fn test_single_quoted_encode() {
        let encoder = XmlEncoder::new(XmlEncoderMode::SingleQuotedAttribute);
        assert_eq!("&amp;", encoder.encode("&"));
        assert_eq!(">", encoder.encode(">"));
        assert_eq!("&lt;", encoder.encode("<"));
        assert_eq!("&#39;", encoder.encode("\'"));

        assert_eq!("\"", encoder.encode("\""));

        generic_tests(&encoder);
    }

    #[test]
    fn test_double_quoted_encode() {
        let encoder = XmlEncoder::new(XmlEncoderMode::DoubleQuotedAttribute);
        assert_eq!("&amp;", encoder.encode("&"));
        assert_eq!(">", encoder.encode(">"));
        assert_eq!("&lt;", encoder.encode("<"));
        assert_eq!("&#34;", encoder.encode("\""));
        assert_eq!("\'", encoder.encode("\'"));

        generic_tests(&encoder);
    }
}
