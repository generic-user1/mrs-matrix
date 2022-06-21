//! The Charset trait and a variety of pre-made character sets

use std::ops::RangeInclusive;

pub trait Charset {
    ///Return the character set as a vector of chars
    fn get_charset(&self) -> Vec<char>;
}

/// ASCII letter and number characters
pub struct Alphanumeric();
impl Charset for Alphanumeric {
    fn get_charset(&self) -> Vec<char> 
    {
        //generate and return vector of all alphanumeric chars
        //alphanumerics make up ASCII (or UTF-8) codes 0x30 through 0x39 for digits,
        //0x41 to 0x5A for capitals, and 0x61 to 0x7A for lowercase
        const CHARCODE_RANGES: [RangeInclusive<u8>; 3] = [
            0x30..=0x39,
            0x41..=0x5a,
            0x61..=0x7a
        ];
        //the length of these three combined is 62 chars; hence capacity of 62
        let mut outvec: Vec<char> = Vec::with_capacity(62);
        for charcode_range in CHARCODE_RANGES {
            for charcode in charcode_range {
                outvec.push(charcode as char);
            }
        }
        outvec
    }
}

/// All printable ASCII characters
pub struct PrintableAscii();
impl Charset for PrintableAscii {
    fn get_charset(&self) -> Vec<char> 
    {
        //generate and return vector of all printable ascii chars (sans the space character)
        //printable ascii chars make up codes 0x21 through 0x7E
        //note that 0x20 is the space character
        let mut outvec: Vec<char> = Vec::with_capacity(93);
        for charcode in 0x21..=0x7E_u8 {
            outvec.push(charcode as char);
        }

        outvec
    }
}

/// All printable ASCII characters plus some fun unicode symbols
pub struct AsciiAndSymbols();
impl Charset for AsciiAndSymbols {
    fn get_charset(&self) -> Vec<char> 
    {
        //generate vector of all printable ascii chars (sans the space character)
        //then add some fun unicode symbols and return the result
        let mut outvec = PrintableAscii().get_charset();

        const CHARCODE_RANGES: [RangeInclusive<u32>; 3] = [
            0x2100..=0x2138, //exclude U+2139 which doesn't always style properly on Windows
            0x213A..=0x214F, 
            0x2A00..=0x2AFF];

        for charcode_range in CHARCODE_RANGES {
            for charcode in charcode_range {
                outvec.push(char::from_u32(charcode).expect("tried to add invalid char to AsciiAndSymbols"));
            }
        }

        outvec
    }
}