use crate::common::{char_bucket, char_mask, encode_as_unicode, HEX_MASK, HEX_SHIFT, U_HEX};
use crate::encoder::Encoder;

const MAX_UTF8_2_BYTE: u32 = 0x7FF;
const UTF8_2_BYTE_FIRST_MSB: u32 = 0xC0;
 const UTF8_3_BYTE_FIRST_MSB: u32 = 0xE0;
 const UTF8_4_BYTE_FIRST_MSB: u32 = 0xF0;
const UTF8_BYTE_MSB: u32 = 0x80;
const UTF8_SHIFT: u32 = 6;
const UTF8_MASK: u32 = 0x3f;

pub enum UriEncoderMode {
    Component,
    FullUri,
}

pub struct UriEncoder {
    valid_masks: [u32; 4],
}
impl UriEncoder {
    pub fn new(mode: UriEncoderMode) -> Self {
        let reserved_chars1 = ['!', '#', '$', '?', '&', '(', ')', '*', '+', ',', ':', ';', '=', '/', '\''];
        let mut uri_reserved_bucket1: u32 = 0;
        for reserved in reserved_chars1 {
            uri_reserved_bucket1 |= char_mask(reserved);
        }

        let reserved_chars2 = ['[', ']', '@'];
        let mut uri_reserved_bucket2: u32 = 0;
        for reserved in reserved_chars2 {
            uri_reserved_bucket2 |= char_mask(reserved);
        }

        //  starting from '0' + 10 bits (aka 0-9)
        let one_to_nine = ((1u32 << 10u32) - 1u32) << ('0' as u32 & 31u32);

        //  starting from 'A' + 26 bits (aka A-Z)
        let uppercase_a_z = ((1u32 << 26u32) - 1u32) << ('A' as u32 & 31u32);

        //  starting from 'a' + 26 bits (aka a-z)
        let lowercase_a_z = ((1u32 << 26u32) - 1u32) << ('a' as u32 & 31u32);

        let uri_unreserved_bucket1 =  one_to_nine | char_mask('-') | char_mask('.');
        let uri_unreserved_bucket2 = uppercase_a_z | char_mask('_');
        let uri_unreserved_bucket3 = lowercase_a_z | char_mask('~');

        match mode {
            UriEncoderMode::Component => {
                let valid_masks = [
                    0,
                    uri_unreserved_bucket1,
                    uri_unreserved_bucket2,
                    uri_unreserved_bucket3
                ];
                Self{valid_masks}
            }
            UriEncoderMode::FullUri => {
                let valid_masks = [
                    0,
                    uri_unreserved_bucket1 | uri_reserved_bucket1,
                    uri_unreserved_bucket2 | uri_reserved_bucket2,
                    uri_unreserved_bucket3
                ];
                Self {valid_masks}
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
                result.push(  U_HEX[(b1 >> HEX_SHIFT) as usize]);
                result.push( U_HEX[(b1 & HEX_MASK) as usize]);

                let b2 = UTF8_BYTE_MSB | ((c as u32 >> (2 * UTF8_SHIFT)) & UTF8_MASK);
                result.push('%');
                result.push( U_HEX[(b2 >> HEX_SHIFT) as usize]);
                result.push( U_HEX[(b2 & HEX_MASK) as usize]);

                let b3 = UTF8_BYTE_MSB | ((c as u32 >> UTF8_SHIFT) & UTF8_MASK);
                result.push('%');
                result.push( U_HEX[(b3 >> HEX_SHIFT) as usize]);
                result.push( U_HEX[(b3 & HEX_MASK) as usize]);

                let b4 = UTF8_BYTE_MSB | (c as u32 & UTF8_MASK);
                result.push('%');
                result.push( U_HEX[(b4 >> HEX_SHIFT) as usize]);
                result.push( U_HEX[(b4 & HEX_MASK) as usize]);

            }
        }
        result.shrink_to_fit();
        result
    }
}

#[cfg(test)]
mod test {
    use crate::common::char_mask;
    use crate::uri_encoder::{UriEncoder, UriEncoderMode};

    #[test]
    fn test_encode() {
        let encoder = UriEncoder::new(UriEncoderMode::FullUri);
        println!("output: {}", encoder.encode("https://www.test.website.com/myPath/?testvalue=\"invalid^char\u{fffd}"));
    }

    #[test]
    fn mask_test() {
        let one_to_nine = ((1u32 << 10u32) - 1u32) << ('0' as u32 & 31);
        println!("{:b}", one_to_nine);
        for ch in '!'..='?' {
            let mask = char_mask(ch);
            if one_to_nine & mask != 0 {
                println!("is number: {:?}", ch);
            } else {
                println!("is not number: {:?}", ch);
            }
        }

        let uppercase_a_z = ((1u32 << 26u32) - 1) << ('A' as u32 & 31u32);
        println!("{:b}", uppercase_a_z);

        for ch in '@'..='_' {
            let mask = char_mask(ch);
            if uppercase_a_z & mask != 0 {
                println!("is uppercase letter: {:?}", ch);
            } else {
                println!("is not uppercase letter: {:?}", ch);
            }
        }

        let lowercase_a_z = ((1u32 << 26u32) - 1) << ('a' as u32 & 31u32);
        for ch in '`'..='~' {
            let mask = char_mask(ch);
            if uppercase_a_z & mask != 0 {
                println!("is lowercase letter: {:?}", ch);
            } else {
                println!("is not lowercase letter: {:?}", ch);
            }
        }
    }
}
