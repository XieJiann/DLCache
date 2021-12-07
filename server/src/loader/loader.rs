use std::collections::HashMap;

use super::channel::{self as loader_channel, LoaderReceiver, LoaderSender};
// Loader store the information of schema, dataset and filter

pub type IdxSender = LoaderSender<u32>;
pub type IdxReceiver = LoaderReceiver<u32>;
pub type DataSender = LoaderSender<u64>;
pub type DataReceiver = LoaderReceiver<u64>;
#[derive(Debug, Default)]
struct Cursor {
    v: Vec<u64>,
    idx: usize,
}

impl Cursor {
    fn push(&mut self, host_id: u64) {
        self.v.push(host_id);
    }

    fn next(&mut self) -> u64 {
        self.idx = (self.idx + 1) % (self.v.len());
        self.v[self.idx]
    }
}

#[derive(Debug)]
pub struct Loader {
    loader_id: u64,
    // store all hosts except the local ones
    hosts: HashMap<u64, IdxSender>,
    cursor: Cursor,
    data_addr_s: Option<DataSender>,
}

pub fn create_idx_channel(loader_id: u64) -> (LoaderSender<u32>, LoaderReceiver<u32>) {
    loader_channel::new::<u32>(loader_id)
}

pub fn create_data_channel(loader_id: u64) -> (LoaderSender<u64>, LoaderReceiver<u64>) {
    loader_channel::new::<u64>(loader_id)
}

impl Loader {
    pub fn new(loader_id: u64) -> Self {
        Loader {
            loader_id,
            hosts: HashMap::new(),
            cursor: Default::default(),
            data_addr_s: None,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.loader_id
    }

    pub async fn send_data(&self, addr: u64) {
        self.data_addr_s.as_ref().unwrap().send(addr).await;
    }

    pub async fn send_idx(&mut self, idx: u32, host_id: u64) {
        if self.hosts.contains_key(&host_id) {
            self.hosts[&host_id].send(idx).await;
        } else {
            self.hosts[&self.cursor.next()].send(idx).await;
        }
    }

    pub fn add_idx_sender(&mut self, idx_sender: IdxSender, host_id: u64) {
        self.hosts.insert(host_id, idx_sender);
        self.cursor.push(host_id);
    }
    pub fn del_idx_sender(&mut self, host_id: u64) {
        self.hosts.remove(&host_id);
    }
    pub fn add_data_sender(&mut self, data_sender: DataSender) {
        self.data_addr_s = Some(data_sender);
    }
    pub fn del_data_sender(&mut self) {
        self.data_addr_s = None;
    }

    pub fn is_empty(&self) -> bool {
        self.data_addr_s.is_none() && self.hosts.is_empty()
    }
}
