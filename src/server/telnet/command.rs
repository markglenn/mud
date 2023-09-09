#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Command {
    Will = 0xFB,
    Wont = 0xFC,
    Do = 0xFD,
    Dont = 0xFE,
}

impl From<Command> for u8 {
    fn from(command: Command) -> Self {
        command as u8
    }
}

#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum NegotationOption {
    // https://datatracker.ietf.org/doc/html/rfc857
    Echo = 0x01,

    // https://datatracker.ietf.org/doc/html/rfc858
    SuppressGoAhead = 0x03,

    // https://datatracker.ietf.org/doc/html/rfc1091
    TerminalType = 0x18,
    Compress = 0x55,
    Compress2 = 0x56,
}

impl From<NegotationOption> for u8 {
    fn from(option: NegotationOption) -> Self {
        option as u8
    }
}

impl From<u8> for NegotationOption {
    fn from(byte: u8) -> Self {
        match byte {
            0x01 => NegotationOption::Echo,
            0x03 => NegotationOption::SuppressGoAhead,
            0x18 => NegotationOption::TerminalType,
            0x55 => NegotationOption::Compress,
            0x56 => NegotationOption::Compress2,
            _ => unimplemented!(),
        }
    }
}
