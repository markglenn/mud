pub mod command;
pub mod frame;
pub mod negotiation;

use std::io::Error;

use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

use self::frame::Frame;

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

    pub async fn send(&mut self, data: &[u8]) -> Result<(), Error> {
        self.stream.write_all(data).await?;
        self.stream.flush().await?;
        Ok(())
    }
}
