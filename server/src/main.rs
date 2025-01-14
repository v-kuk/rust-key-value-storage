mod key_value_store;

use futures::{SinkExt, StreamExt};
use key_value_store::KeyValueStore;
use tokio::net::TcpListener;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let address = "127.0.0.1:9876";
    let server = TcpListener::bind(address).await?;
    let kvs = KeyValueStore::new();

    println!("Server listening on {address}");

    loop {
        let (mut tcp, _) = server.accept().await?;
        let (reader, writer) = tcp.split();
        let mut stream = FramedRead::new(reader, LinesCodec::new());
        let mut sink = FramedWrite::new(writer, LinesCodec::new());

        println!("Client connected");
        sink.send("hello").await?;
        while let Some(Ok(msg)) = stream.next().await {
            let received = msg.as_str();
            let args: Vec<&str> = received.split_whitespace().collect();
            let command = args[0];

            match command {
                "set" => {
                    if args.len() != 3 {
                        sink.send(
                            "Invalid set syntax. Use: set <key> <value>".to_string(),
                        ).await?;
                        continue;
                    }

                    let key = args[1].to_string();
                    let value = args[2].to_string();
                    kvs.set(key, value);
                    sink.send("Value added!".to_string()).await?;
                },
                "del" => {
                    if args.len() != 2 {
                        sink.send(
                            "Invalid del syntax. Use: del <key>".to_string(),
                        ).await?;
                        continue;
                    }

                    let key = args[1].to_string();
                    kvs.del(&key);
                    sink.send(
                        "Key deleted".to_string(),
                    ).await?;
                },
                "get" => {
                    if args.len() != 2 {
                        sink.send(
                            "Invalid del syntax. Use: del <key>".to_string(),
                        ).await?;
                        continue;
                    }

                    let key = args[1].to_string();

                    let value = kvs.get(key.as_str()).unwrap_or("No such key".to_string());
                    sink.send(
                        value.to_string()
                    ).await?;
                },
                "ping" => {
                    sink.send("pong".to_string()).await?;
                },
                "exit" => {
                    sink.send("exit".to_string()).await?;
                    break;
                },
                _ => {
                    sink.send(format!("Unknown command: {}", received)).await?;
                },
                
            }
        }

        println!("Server stopped");
    }
}