pub mod command;
pub mod frame;
pub mod negotiation;

use std::io::Error;

use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use self::{
    command::{Command, NegotationOption},
    frame::Frame,
    negotiation::SubnegotiationOption,
};

pub struct Client {
    stream: TcpStream,
    buffer: BytesMut,
}

impl Client {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,

            // Allocate a 4k buffer
            buffer: BytesMut::with_capacity(4096),
        }
    }

    /**
     * Read a frame from the client.
     *
     * If the client has not sent enough data to form a frame, this method will
     * block until enough data has been received.
     */
    pub async fn read_frame(&mut self) -> Result<Option<Frame>, Error> {
        loop {
            // Attemp to parse a frame from the currently buffered data. If
            // enough data has been buffered, the frame is returned.

            if let Some(frame) = frame::parse(&mut self.buffer) {
                // A frame was parsed from the buffer, return it
                return Ok(Some(frame));
            }

            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // The remote closed the connection, there will be no more data
                // to read
                return Ok(None);
            }
        }
    }

    /**
     * Send data to the client.
     *
     * This method will block (asynchronously) until all of the data has been sent.
     */
    pub async fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        self.stream.write_all(data).await?;
        self.stream.flush().await?;
        Ok(())
    }

    /**
     * Send a negotiation to the client.
     *
     * A negotiation is a command that is sent to the client that is part of
     * the standard telnet protocol. It is used to negotiate the state of the
     * connection.
     *
     * For example, the client may request that the server enable the echo
     * option using the following command:
     *
     *   IAC DO ECHO
     *
     * The server would then respond with the following command:
     *
     *  IAC WILL ECHO
     *
     * The server would then enable the echo option.
     */
    pub async fn send_negotiation(
        &mut self,
        command: Command,
        option: NegotationOption,
    ) -> Result<(), Error> {
        let bytes = [0xFF, command.into(), option.into()];
        self.send(&bytes).await
    }

    /**
     * Send a subnegotiation to the client.
     *
     * A subnegotiation is a command that is sent to the client that is not
     * part of the standard telnet protocol. It is used to send additional
     * information to the client.
     *
     * For example, the server may request the terminal type from the client
     * using the following command:
     *
     *    IAC SB TERMINAL-TYPE SEND IAC SE
     *
     * The client would then respond with the following command:
     *
     *   IAC WILL TERMINAL-TYPE
     *   IAC SB TERMINAL-TYPE IS <terminal-type> IAC SE
     */
    pub async fn send_subnegotiation(&mut self, option: SubnegotiationOption) -> Result<(), Error> {
        let mut bytes = vec![0xFF, 0xFA];

        bytes.extend_from_slice(Vec::<u8>::from(option).as_slice());
        bytes.push(0xFF);
        bytes.push(0xF0);

        self.send(&bytes).await
    }
}
