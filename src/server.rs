use tokio::net::TcpListener;
pub mod session;
pub mod telnet;

use session::Session;

use crate::server::telnet::{
    command::{Command, NegotationOption},
    negotiation::SubnegotiationOption,
};

pub async fn listen(port: usize) -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).await?;

    println!("Listening on port: {}", port);

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("Accepted connection from: {}", addr);

        tokio::spawn(async move {
            let mut session = Session::new(socket, addr);

            //.send(b"\x1b[6n\x1b[?1000;1002;1006;1015h")

            session
                .client
                .send_negotiation(Command::Will, NegotationOption::Compress)
                .await
                .unwrap();

            session
                .client
                .send_negotiation(Command::Will, NegotationOption::SuppressGoAhead)
                .await
                .unwrap();

            session
                .client
                .send_negotiation(Command::Do, NegotationOption::TerminalType)
                .await
                .unwrap();

            session
                .client
                .send_subnegotiation(SubnegotiationOption::TerminalTypeSend)
                .await
                .unwrap();

            session.client.send(b"\x1b[6n").await.unwrap();

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
    }
}
