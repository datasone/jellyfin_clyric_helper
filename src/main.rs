#![windows_subsystem = "windows"]

use futures_util::StreamExt;
use ipipe::Pipe;
use std::io;
use std::io::Write;
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let ws_addr = "127.0.0.1:7117";
    let try_socket = TcpListener::bind(&ws_addr).await;
    let listener = try_socket.expect("Failed to bind");
    println!("Listening on: {}", ws_addr);

    let pipe = Pipe::with_name("clyricsocket").unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let writer = pipe.clone();
        tokio::spawn(accept_connection(stream, writer));
    }

    Ok(())
}

async fn accept_connection(stream: TcpStream, writer: Pipe) {
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    let (_, mut receiver) = ws_stream.split();

    loop {
        if let Some(Ok(msg)) = receiver.next().await {
            if msg.is_text() {
                println!("{}", msg);
                let _ = write!(writer.clone(), "{}", msg);
            }
        } else {
            break;
        }
    }
}
