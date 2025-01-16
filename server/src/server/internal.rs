use anyhow::Ok;
use futures::SinkExt;
use log;
use uuid::Uuid;

use super::servers_table::ServerContext;

pub async fn process(context: &mut ServerContext) -> anyhow::Result<()> {
    // TODO: consider refactoring this due to immutable borrowing in between mutable ones
    let receiver = context.receiver.clone();
    let (_, mut sink) = context.borrow_and_split_stream();

    let id = Uuid::new_v4();

    log::info!(target: "internal_connection_events", "Internal client Connected {}", id);
    sink.send("connected").await?;

    while let Result::Ok(msg) = receiver.recv().await {
        let received = msg.as_str();
        
        sink.send(received).await?;
    }

    log::info!(target: "internal_connection_events", "Client Disonnected {}", id);

    Ok(())
}