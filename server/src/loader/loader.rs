use std::collections::HashMap;

use super::channel::{self as loader_channel, LoaderReceiver, LoaderSender};
// Loader store the information of schema, dataset and filter

pub type IdxSender = LoaderSender<u32>;
pub type IdxReceiver = LoaderReceiver<u32>;
pub type DataSender = LoaderSender<u64>;
pub type DataReceiver = LoaderReceiver<u64>;

#[derive(Debug)]
pub struct Loader {
    loader_id: u64,
    // store all hosts except the local ones
    hosts: HashMap<u64, IdxSender>,
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
            data_addr_s: None,
        }
    }

    pub fn get_id(&self) -> u64 {
        self.loader_id
    }

    pub async fn send_data(&self, addr: u64) {
        todo!()
    }

    pub async fn send_idx(&self, idx: u32) {
        todo!()
    }

    pub fn add_idx_sender(&mut self, idx_sender: IdxSender, host_id: u64) {
        todo!()
    }
    pub fn del_idx_sender(&mut self, host_id: u64) {
        todo!()
    }
    pub fn add_data_sender(&mut self, data_sender: DataSender) {
        todo!()
    }
    pub fn del_data_sender(&mut self) {
        todo!()
    }

    pub fn is_empty(&self) -> bool {
        todo!()
    }
}
