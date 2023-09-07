use std::fmt::Write;

use bytes::BytesMut;
use tokio::net::TcpListener;
pub mod session;
pub mod telnet;

use session::Session;

pub async fn listen(port: usize) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    println!("Listening on port: {}", port);
    let mut buffer: BytesMut = BytesMut::with_capacity(4096);

    buffer.write_str("test").unwrap();

    println!("{:#?}", buffer.split_to(buffer.len()));
    println!("{:#?}", buffer);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);

        tokio::spawn(async move {
            let mut session = Session::new(socket, addr);

            session
                 .client
                 .send(b"\xFF\xFB\x55\xFF\xFB\x03\xFF\xFD\x18\xFF\xFA\x18\x01\xFF\xF0\x1b[6n\x1b[?1000;1002;1006;1015h")
                 .await
                 .unwrap();

            loop {
                match session.client.read_frame().await {
                    Ok(Some(frame)) => println!("{:#?}", frame),
                    Ok(None) => {
                        println!("Connection closed by remote");
                        break;
                    }
                    Err(e) => {
                        println!("Failed to read from socket; err = {:?}", e);
                        break;
                    }
                }
            }
        });

        //     session
        //         .client
        //         .send(b"\xFF\xFB\x55\xFF\xFB\x03\xFF\xFD\x18\xFF\xFA\x18\x01\xFF\xF0\x1b[6n\x1b[?1000;1002;1006;1015h")
        //         .await
        //         .unwrap();
        //     session.client.read().await.unwrap();
        //     println!("Closing connection from: {}", addr);
        // });
    }
}
