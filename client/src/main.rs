use client::utils;

use futures::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

// use rust_key_value_db::utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:9876").await?;
    let (reader, writer) = stream.split();
    let mut stream = FramedRead::new(reader, LinesCodec::new());
    let mut sink = FramedWrite::new(writer, LinesCodec::new());
    
    let mut running = true;


    while running {
        while let Some(Ok(msg)) = stream.next().await {
            println!("{msg}");
            if msg == "exit" {
                running = false;
                break;
            }

            let cmd = utils::input::get_line("");
            if cmd.is_empty() {
                continue; // Skip if no input is provided
            }
    
            sink.send(cmd).await?;
        }
    }

    Ok(())
}
