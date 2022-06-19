//! The Charset enum containing a variety of pre-made character sets


pub enum Charset {
    /// ASCII letter and number characters
    Alphanumeric,
    /// All printable ASCII characters
    PrintableAscii
}

impl Charset {

    ///Return the associated charset for a given variant of `Charset`
    pub fn value(&self) -> Vec<char>
    {
        match self {
            Charset::Alphanumeric => Charset::gen_alphanumeric(),
            Charset::PrintableAscii => Charset::gen_printable_ascii()
        }
    }

    ///The function used to generate the Alphanumeric charset
    fn gen_alphanumeric() -> Vec<char> 
    {
        //generate and return vector of all alphanumeric chars
        //alphanumerics make up ASCII (or UTF-8) codes 0x30 through 0x39 for digits,
        //0x41 to 0x5A for capitals, and 0x61 to 0x7A for lowercase
        //this set is 62 chars long; hence capacity of 62
        let mut outvec: Vec<char> = Vec::with_capacity(62);
        for charcode in 0x30..=0x39_u8 {outvec.push(charcode as char);}
        for charcode in 0x41..=0x5a_u8 {outvec.push(charcode as char);}
        for charcode in 0x61..=0x7a_u8 {outvec.push(charcode as char);}
        outvec
    }

    ///The function used to generate the PrintableAscii charset
    fn gen_printable_ascii() -> Vec<char> 
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