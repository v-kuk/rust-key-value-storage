mod key_value_store;

use futures::{SinkExt, StreamExt};
use key_value_store::KeyValueStore;
use log;
use uuid::Uuid;
use tokio::net::TcpListener;
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = log4rs::init_file("logger.yaml", Default::default());

    let address = "127.0.0.1:9876";
    let server = TcpListener::bind(address).await?;
    let kvs = KeyValueStore::new();

    log::info!("Server listening on {address}");

    loop {
        let (mut tcp, _) = server.accept().await?;
        let (reader, writer) = tcp.split();
        let mut stream = FramedRead::new(reader, LinesCodec::new());
        let mut sink = FramedWrite::new(writer, LinesCodec::new());

        let id = Uuid::new_v4();

        log::info!(target: "connection_events", "Client Connected {}", id);
        sink.send("connected").await?;

        while let Some(Ok(msg)) = stream.next().await {
            let received = msg.as_str();
            let args: Vec<&str> = received.split_whitespace().collect();
            let command = args[0];

            match command {
                "set" => {
                    if args.len() != 3 {
                        log::warn!("Invalid set syntax, command = {:?}", args);
                        
                        sink.send(
                            "Invalid set syntax. Use: set <key> <value>".to_string(),
                        ).await?;
                        continue;
                    }
                    
                    
                    let key = args[1].to_string();
                    log::info!(target: "commands_event", "user - {}, set {}", id, key);

                    let value = args[2].to_string();

                    let res = match kvs.set(key, value) {
                        Some(_) => 1,
                        None => 0
                    };
                    sink.send(res.to_string()).await?;
                },
                "del" => {
                    if args.len() != 2 {
                        log::warn!("Invalid del syntax, command = {:?}", args);

                        sink.send(
                            "Invalid del syntax. Use: del <key>".to_string(),
                        ).await?;
                        continue;
                    }

                    let key = args[1].to_string();
                    log::info!(target: "commands_event", "user - {}, del {}", id, key);

                    let res = match kvs.del(&key) {
                        Some(_) => 1,
                        None => 0
                    };
                    sink.send(res.to_string()).await?;
                },
                "get" => {
                    if args.len() != 2 {
                        log::warn!("Invalid get syntax, command = {:?}", args);

                        sink.send(
                            "Invalid del syntax. Use: del <key>".to_string(),
                        ).await?;
                        continue;
                    }

                    let key = args[1].to_string();

                    log::info!(target: "commands_event", "user - {}, del {}", id, key);

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
                    log::info!(target: "commands_event", "user - {}, invalid_command - {:?}", id, args);

                    sink.send(format!("Unknown command: {}", received)).await?;
                },
                
            }
        }
        log::info!(target: "connection_events", "Client Connected {}", id);

        log::info!("Server stopped");
    }
}