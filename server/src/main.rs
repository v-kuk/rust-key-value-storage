mod key_value_store;
mod server;

use std::sync::Arc;
use futures::{lock::Mutex, StreamExt};
use key_value_store::KeyValueStore;
use log;
use server::servers_table::{ServerContext, ServersTable};
use tokio::net::TcpListener;
use tokio_util::codec::{FramedRead, LinesCodec};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = log4rs::init_file("logger.yaml", Default::default());

    let address = "127.0.0.1:9876";
    let internal_address = "127.0.0.1:9875";
    
    match TcpListener::bind(address).await {
        Result::Ok(listener) => {
            let internal_listener = TcpListener::bind(internal_address).await?;
            let kvs = Arc::new(Mutex::new(KeyValueStore::new()));
            let servers_map = Arc::new(Mutex::new(ServersTable::new()));

            log::info!("Server is Master!");
            log::info!("Server listening on {address}");
            log::info!("Internal server listening on {internal_address}");

            let (sender, receiver) = async_channel::unbounded::<String>();
            
            loop {
                let _ = {
                    let (tcp_stream, _) = listener.accept().await?;
                    let kvs_clone = Arc::clone(&kvs);
                    servers_map.lock().await.add_new(true);
                    let mut data_server_context = ServerContext {
                        sender: sender.clone(),
                        receiver: receiver.clone(),
                        servers_map: Arc::clone(&servers_map),
                        kvs: kvs_clone,
                        tcp_stream,
                    };
                    
                    log::info!("Starting handler");
                    tokio::spawn(async move {
                        // if let Err(e) = server::client::process(tx, tcp_stream, kvs_clone).await {
                        if let Err(e) = server::data::process(&mut data_server_context).await {
                            log::error!("Error processing connection: {}", e);
                        }
                    })
                };
                
                let _ = {
                    let (internal_tcp_stream, _) = internal_listener.accept().await?;
                    let internal_kvs_clone = Arc::clone(&kvs);
                    
                    let mut internal_server_context = ServerContext {
                        sender: sender.clone(),
                        receiver: receiver.clone(),
                        servers_map: Arc::clone(&servers_map),
                        kvs: internal_kvs_clone,
                        tcp_stream: internal_tcp_stream,
                    };

                    log::info!("Opening internal listener");
                    tokio::spawn(async move {
                        if let Err(e) = server::internal::process(&mut internal_server_context).await {
                            log::error!("Error processing connection: {}", e);
                        }
                    })
                };
            }
        },
        Err(_) => {
            log::info!("Server is slave");

            let mut stream = tokio::net::TcpStream::connect("127.0.0.1:9875").await?;
            let (reader, _) = stream.split();
            let mut stream = FramedRead::new(reader, LinesCodec::new());

            loop {
                while let Some(Result::Ok(msg)) = stream.next().await {
                    println!("{msg}");
                }
            }
        },
    };
}

