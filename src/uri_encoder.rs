use crate::common::{HEX_MASK, HEX_SHIFT, U_HEX, char_bucket, char_mask};

/// 0111_1111_1111 --> highest 2x utf 8 bytes
/// 0000_1000_0000 --> most sig. utf8 byte
/// 0000_1100_0000 --> most sig. 2x utf8 byte
/// 0000_1110_0000 --> most sig. 3x utf8 byte
/// 0000_1111_0000 --> most sig. 4x utf8 byte
/// 0000_0000_0110 --> byte shift
/// 0000_0011_1111 --> mask
const MAX_UTF8_2_BYTE: u32 = 0b_0111_1111_1111;
const UTF8_BYTE_MSB: u32 = 0b_0000_1000_0000;
const UTF8_2_BYTE_FIRST_MSB: u32 = 0b_0000_1100_0000;
const UTF8_3_BYTE_FIRST_MSB: u32 = 0b_0000_1110_0000;
const UTF8_4_BYTE_FIRST_MSB: u32 = 0b_0000_1111_0000;

const UTF8_SHIFT: u32 = 0b_0000_0000_0110;
const UTF8_MASK: u32 = 0b_0000_0011_1111;

pub enum UriEncoderMode {
    Component,
    FullUri,
}

pub struct UriEncoder {
    valid_masks: [u32; 4],
}
impl UriEncoder {
    pub fn new(mode: UriEncoderMode) -> Self {
        //  starting from '0' + 10 bits (aka 0-9)
        let one_to_nine = ((1u32 << 10u32) - 1u32) << ('0' as u32 & 31u32);

        //  starting from 'A' + 26 bits (aka A-Z)
        let uppercase_a_z = ((1u32 << 26u32) - 1u32) << ('A' as u32 & 31u32);

        //  starting from 'a' + 26 bits (aka a-z)
        let lowercase_a_z = ((1u32 << 26u32) - 1u32) << ('a' as u32 & 31u32);

        let uri_unreserved_bucket1 = one_to_nine | char_mask('-') | char_mask('.');
        let uri_unreserved_bucket2 = uppercase_a_z | char_mask('_');
        let uri_unreserved_bucket3 = lowercase_a_z | char_mask('~');

        match mode {
            UriEncoderMode::Component => {
                let valid_masks = [
                    0,
                    uri_unreserved_bucket1,
                    uri_unreserved_bucket2,
                    uri_unreserved_bucket3,
                ];
                Self { valid_masks }
            }
            UriEncoderMode::FullUri => {
                let reserved_chars1 = [
                    '!', '#', '$', '?', '&', '(', ')', '*', '+', ',', ':', ';', '=', '/', '\'',
                ];
                let mut uri_reserved_bucket1: u32 = 0;
                for reserved in reserved_chars1 {
                    uri_reserved_bucket1 |= char_mask(reserved);
                }

                let reserved_chars2 = ['[', ']', '@'];
                let mut uri_reserved_bucket2: u32 = 0;
                for reserved in reserved_chars2 {
                    uri_reserved_bucket2 |= char_mask(reserved);
                }

                let valid_masks = [
                    0,
                    uri_unreserved_bucket1 | uri_reserved_bucket1,
                    uri_unreserved_bucket2 | uri_reserved_bucket2,
                    uri_unreserved_bucket3,
                ];
                Self { valid_masks }
            }
        }
    }

    pub fn encode(&self, input: &str) -> String {
        let starting_capacity = (u32::MAX / 2u32).min((input.len() * 9usize) as u32) as usize;
        let mut result = String::with_capacity(starting_capacity);
        for c in input.chars() {
            if c as u32 <= 127u32 {
                let bucket = char_bucket(c);
                let mask = char_mask(c);

                if (self.valid_masks[bucket] & mask) != 0 {
                    result.push(c);
                    continue;
                } else {
                    result.push('%');
                    result.push(U_HEX[(c as u32 >> HEX_SHIFT) as usize]);
                    result.push(U_HEX[(c as u32 & HEX_MASK) as usize]);
                    continue;
                }
            } else if c as u32 <= MAX_UTF8_2_BYTE {
                let b1 = UTF8_2_BYTE_FIRST_MSB | (c as u32 >> UTF8_SHIFT);
                result.push('%');
                result.push(U_HEX[(b1 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b1 & HEX_MASK) as usize]);

                let b2 = UTF8_BYTE_MSB | (c as u32 & UTF8_MASK);
                result.push('%');
                result.push(U_HEX[(b2 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b2 & HEX_MASK) as usize]);
            } else if c as u32 <= 0xFFFF {
                let b1 = UTF8_3_BYTE_FIRST_MSB | (c as u32 >> (2 * UTF8_SHIFT));
                result.push('%');
                result.push(U_HEX[(b1 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b1 & HEX_MASK) as usize]);

                let b2 = UTF8_BYTE_MSB | ((c as u32 >> UTF8_SHIFT) & UTF8_MASK);
                result.push('%');
                result.push(U_HEX[(b2 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b2 & HEX_MASK) as usize]);

                let b3 = UTF8_BYTE_MSB | (c as u32 & UTF8_MASK);
                result.push('%');
                result.push(U_HEX[(b3 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b3 & HEX_MASK) as usize]);
            } else {
                let b1 = UTF8_4_BYTE_FIRST_MSB | (c as u32 >> (3 * UTF8_SHIFT));
                result.push('%');
                result.push(U_HEX[(b1 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b1 & HEX_MASK) as usize]);

                let b2 = UTF8_BYTE_MSB | ((c as u32 >> (2 * UTF8_SHIFT)) & UTF8_MASK);
                result.push('%');
                result.push(U_HEX[(b2 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b2 & HEX_MASK) as usize]);

                let b3 = UTF8_BYTE_MSB | ((c as u32 >> UTF8_SHIFT) & UTF8_MASK);
                result.push('%');
                result.push(U_HEX[(b3 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b3 & HEX_MASK) as usize]);

                let b4 = UTF8_BYTE_MSB | (c as u32 & UTF8_MASK);
                result.push('%');
                result.push(U_HEX[(b4 >> HEX_SHIFT) as usize]);
                result.push(U_HEX[(b4 & HEX_MASK) as usize]);
            }
        }
        result.shrink_to_fit();
        result
    }
}

#[cfg(test)]
mod test {
    use crate::uri_encoder::{UriEncoder, UriEncoderMode};

    fn shared_test_cases(encoder: &UriEncoder) {
        assert_eq!("abcABC123", encoder.encode("abcABC123"));
        assert_eq!("%20", encoder.encode(" "));
        assert_eq!("%22", encoder.encode("\""));
        assert_eq!("%25", encoder.encode("%"));
        assert_eq!("%3C", encoder.encode("<"));
        assert_eq!("%3E", encoder.encode(">"));
        assert_eq!("%5C", encoder.encode("\\"));
        assert_eq!("%5E", encoder.encode("^"));
        assert_eq!("%60", encoder.encode("`"));
        assert_eq!("%7B", encoder.encode("{"));
        assert_eq!("%7C", encoder.encode("|"));
        assert_eq!("%7D", encoder.encode("}"));
        assert_eq!("%C2%A0", encoder.encode("\u{00a0}"));
        assert_eq!("%E0%A0%80", encoder.encode("\u{0800}"));
    }

    #[test]
    fn test_component_encode() {
        let encoder = UriEncoder::new(UriEncoderMode::Component);
        assert_eq!("%3A", encoder.encode(":"));
        assert_eq!("%2F", encoder.encode("/"));
        assert_eq!("%3F", encoder.encode("?"));
        assert_eq!("%23", encoder.encode("#"));
        assert_eq!("%5B", encoder.encode("["));
        assert_eq!("%5D", encoder.encode("]"));
        assert_eq!("%40", encoder.encode("@"));
        assert_eq!("%21", encoder.encode("!"));
        assert_eq!("%24", encoder.encode("$"));
        assert_eq!("%26", encoder.encode("&"));
        assert_eq!("%27", encoder.encode("'"));
        assert_eq!("%28", encoder.encode("("));
        assert_eq!("%29", encoder.encode(")"));
        assert_eq!("%2A", encoder.encode("*"));
        assert_eq!("%2B", encoder.encode("+"));
        assert_eq!("%2C", encoder.encode(","));
        assert_eq!("%3B", encoder.encode(";"));
        assert_eq!("%3D", encoder.encode("="));
        shared_test_cases(&encoder);
    }

    #[test]
    fn test_full_uri_encode() {
        let encoder = UriEncoder::new(UriEncoderMode::FullUri);
        assert_eq!(
            "http://www.owasp.org/index.php?foo=bar&baz#fragment",
            encoder.encode("http://www.owasp.org/index.php?foo=bar&baz#fragment")
        );
        shared_test_cases(&encoder);
    }
}
