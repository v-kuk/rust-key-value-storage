use anyhow::Ok;
use futures::{SinkExt, StreamExt};
use log;
use uuid::Uuid;

use super::servers_table::ServerContext;

pub async fn process(context: &mut ServerContext) -> anyhow::Result<()> {
    let sender = context.sender.clone();
    let kvs = context.kvs.clone();
    let (mut stream, mut sink) = context.borrow_and_split_stream();

    let id = Uuid::new_v4();
    
    log::info!(target: "connection_events", "Client Connected {}", id);
    sink.send("connected").await?;

    while let Some(Result::Ok(msg)) = stream.next().await {
        sender.send(msg.clone()).await?;
        
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
                let store = kvs.lock().await;
                let res = match store.set(key, value) {
                    Some(_) => "exists",
                    None => "new"
                };
                sink.send(format!("1 {res}")).await?;
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
                let store = kvs.lock().await;
                let res = match store.del(&key) {
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
                log::info!(target: "commands_event", "user - {}, get {}", id, key);
                let store = kvs.lock().await;
                let value = store.get(key.as_str()).unwrap_or("No such key".to_string());
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

    log::info!(target: "connection_events", "Client Disonnected {}", id);

    Ok(())
}