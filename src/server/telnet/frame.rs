use bytes::{Buf, BytesMut};

use super::{
    command::{Command, NegotationOption},
    negotiation::SubnegotiationOption,
};

#[derive(Debug, PartialEq)]
pub enum Frame {
    Content(Vec<u8>),
    Negotiation(Command, NegotationOption),
    Subnegotiation(SubnegotiationOption),
    UnknownSubnegotion(NegotationOption, Vec<u8>),
}

// Parse the next frame from the buffer
pub fn parse(buffer: &mut BytesMut) -> Option<Frame> {
    match buffer.get(0)? {
        0xFF => parse_iac_frame(buffer),
        _ => {
            // Pull all the content up to the first IAC byte off the buffer
            let iac_index = buffer
                .iter()
                .position(|b| *b == 0xFF)
                .unwrap_or(buffer.len());

            // Copy the content into the frame
            let content = buffer.split_to(iac_index).to_vec();

            Some(Frame::Content(content))
        }
    }
}

fn parse_iac_frame(buffer: &mut BytesMut) -> Option<Frame> {
    match buffer.get(1)? {
        // IAC IAC
        0xFF => {
            buffer.advance(2);
            Some(Frame::Content(vec![0xFF]))
        }

        // IAC SB <command> <data> IAC SE
        0xFA => parse_subnegotiation(buffer),

        // IAC WILL
        0xFB => parse_action(buffer, Command::Will),

        // IAC WONT
        0xFC => parse_action(buffer, Command::Wont),

        // IAC DO
        0xFD => parse_action(buffer, Command::Do),

        // IAC DONT
        0xFE => parse_action(buffer, Command::Dont),

        // The IAC byte was not followed by a valid command
        _ => {
            // Advance past the IAC byte
            buffer.advance(1);

            println!("Unknown IAC command: {:#?}", buffer);
            None
        }
    }
}

fn parse_action(buffer: &mut BytesMut, command: Command) -> Option<Frame> {
    let option_byte = *buffer.get(2)?;

    // Advance past the IAC <command> <option>
    buffer.advance(3);

    Some(Frame::Negotiation(
        command,
        NegotationOption::from(option_byte),
    ))
}

fn parse_subnegotiation(buffer: &mut BytesMut) -> Option<Frame> {
    // Find the sequence 0xFF 0xF0 to determine the end of the subnegotiation

    let pos = buffer
        .windows(2)
        .position(|window| window == [0xFF, 0xF0])?;

    let option = NegotationOption::from(*buffer.get(2)?);

    // Advance past the header of IAC SB <option>
    buffer.advance(3);

    // Copy the subnegotiation data into the frame
    let mut data = buffer.split_to(pos - 3).to_vec();

    // Replace any 0xFF 0xFF sequences with a single 0xFF byte
    while let Some(pos) = data.windows(2).position(|w| w == [0xFF, 0xFF]) {
        // Remove only 1 of the 0xFF bytes
        data.remove(pos);
    }

    // Advance past the 0xFF 0xF0 sequence
    buffer.advance(2);

    let subnegotiation_option = SubnegotiationOption::parse(option, &data)?;
    let subnegotiation = Frame::Subnegotiation(subnegotiation_option);

    Some(subnegotiation)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_content() {
        let mut buffer = BytesMut::from(&b"test"[..]);
        assert_eq!(parse(&mut buffer), Some(Frame::Content(b"test".to_vec())));
    }

    #[test]
    fn test_parse_iac_iac() {
        let mut buffer = BytesMut::from(&b"\xFF\xFF"[..]);
        assert_eq!(parse(&mut buffer), Some(Frame::Content(vec![0xFF])));
    }

    #[test]
    fn test_parse_content_until_iac() {
        let mut buffer = BytesMut::from(&b"test\xFF"[..]);
        assert_eq!(parse(&mut buffer), Some(Frame::Content(b"test".to_vec())));
        assert_eq!(buffer, BytesMut::from(&b"\xFF"[..]));
    }

    #[test]
    fn test_parse_iac_sb() {
        let mut buffer = BytesMut::from(&b"\xFF\xFA\x18\x00Hello World!\xFF\xF0"[..]);
        assert_eq!(
            parse(&mut buffer),
            Some(Frame::Subnegotiation(SubnegotiationOption::TerminalType(
                "Hello World!".to_string()
            )))
        );
    }

    #[test]
    fn test_parse_iac_will() {
        let mut buffer = BytesMut::from(&b"\xFF\xFB\x55"[..]);
        assert_eq!(
            parse(&mut buffer),
            Some(Frame::Negotiation(
                Command::Will,
                NegotationOption::Compress
            ))
        );
    }

    #[test]
    fn test_parse_iac_wont() {
        let mut buffer = BytesMut::from(&b"\xFF\xFC\x55"[..]);
        assert_eq!(
            parse(&mut buffer),
            Some(Frame::Negotiation(
                Command::Wont,
                NegotationOption::Compress
            ))
        );
    }

    #[test]
    fn test_parse_iac_do() {
        let mut buffer = BytesMut::from(&b"\xFF\xFD\x55"[..]);
        assert_eq!(
            parse(&mut buffer),
            Some(Frame::Negotiation(Command::Do, NegotationOption::Compress))
        );
    }

    #[test]
    fn test_parse_iac_dont() {
        let mut buffer = BytesMut::from(&b"\xFF\xFE\x55"[..]);
        assert_eq!(
            parse(&mut buffer),
            Some(Frame::Negotiation(
                Command::Dont,
                NegotationOption::Compress
            ))
        );
    }
}
