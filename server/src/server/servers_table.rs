use std::{collections::HashMap, sync::Arc};
use async_channel::{Receiver, Sender};
use futures::lock::Mutex;
use tokio::net::{tcp::{ReadHalf, WriteHalf}, TcpStream};
use tokio_util::codec::{FramedRead, FramedWrite, LinesCodec};

use crate::key_value_store::KeyValueStore;

struct ServersTableRecord {
    pub is_master: bool,
    pub id: u32,
}

pub struct ServersTable {
    table: HashMap<u32, ServersTableRecord>,
    last_id: u32,
}

impl ServersTable {
    pub fn new() -> Self {
        ServersTable {
            table: HashMap::new(),
            last_id: 0,
        }
    }
    
    pub fn add_new(self: &mut Self, is_master: bool) -> u32 {
        self.last_id += 1;
        
        self.table.insert(self.last_id, ServersTableRecord {
            is_master,
            id: self.last_id,
        });
        
        self.last_id
    }

    pub fn get_all(self: Self) -> Vec<(u32, ServersTableRecord)> {
        self.table.into_iter().map(|entry| entry).collect()
    }

    pub fn get_other(self: Self, omit_id: u32) -> Vec<(u32, ServersTableRecord)> {
        self.table
            .into_iter()
            .filter(| entry | entry.0 != omit_id)
            .map(|entry| entry)
            .collect()
    }
}

pub struct ServerContext {
    pub receiver: Receiver<String>,
    pub sender: Sender<String>,
    pub servers_map: Arc<Mutex<ServersTable>>,
    pub kvs: Arc<Mutex<KeyValueStore>>,
    pub tcp_stream: TcpStream,
}

impl ServerContext{
    pub fn borrow_and_split_stream(&mut self) -> (FramedRead<ReadHalf, LinesCodec>, FramedWrite<WriteHalf, LinesCodec>) {
        let (reader, writer) = self.tcp_stream.split();
        let stream = FramedRead::new(reader, LinesCodec::new());
        let sink = FramedWrite::new(writer, LinesCodec::new());

        (stream, sink)
    }
}