use num_enum::{FromPrimitive, IntoPrimitive};

#[derive(Debug, IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum Command {
    Will = 0xFB,
    Wont = 0xFC,
    Do = 0xFD,
    Dont = 0xFE,

    #[num_enum(default)]
    Unknown = 0,
}

#[derive(Debug, IntoPrimitive, FromPrimitive, Clone, Copy, PartialEq)]
#[repr(u8)]
pub enum NegotationOption {
    Echo = 0x01,
    SuppressGoAhead = 0x03,
    TerminalType = 0x18,
    Compress = 0x55,
    Compress2 = 0x56,

    #[num_enum(default)]
    Unknown = 0xFF,
}
