use super::command::NegotationOption;

#[derive(Debug, PartialEq)]
pub enum SubnegotiationOption {
    TerminalType(String),
    TerminalTypeSend,
    Unknown,
}

impl From<SubnegotiationOption> for Vec<u8> {
    fn from(option: SubnegotiationOption) -> Self {
        match option {
            SubnegotiationOption::TerminalType(description) => {
                let mut output = vec![NegotationOption::TerminalType.into(), 0x00];
                output.extend_from_slice(description.as_bytes());

                output
            }
            SubnegotiationOption::TerminalTypeSend => {
                vec![NegotationOption::TerminalType.into(), 0x01]
            }
            SubnegotiationOption::Unknown => unimplemented!(),
        }
    }
}

impl SubnegotiationOption {
    pub fn parse(option: NegotationOption, contents: &Vec<u8>) -> Option<Self> {
        match option {
            NegotationOption::TerminalType => parse_terminal_type(contents),
            _ => None,
        }
    }
}

fn parse_terminal_type(contents: &Vec<u8>) -> Option<SubnegotiationOption> {
    match contents.get(0)? {
        // IS
        0x00 => {
            let description = String::from_utf8_lossy(&contents[1..]).into_owned();

            Some(SubnegotiationOption::TerminalType(description))
        }

        0x01 => Some(SubnegotiationOption::TerminalTypeSend),

        _ => return None,
    }
}
