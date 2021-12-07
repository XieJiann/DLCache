use std::collections::HashMap;

use crate::proto::dataloader::CreateDataloaderRequest;
use tokio::sync::mpsc::{channel, error::TryRecvError, Receiver, Sender};

use super::sampler::Ssampler;
// Loader store the information of schema, dataset and filter

#[derive(Debug)]
pub struct Loader {
    // store all hosts except the local ones
    name: String,
    id: u64,
    hosts: HashMap<String, Ssampler>,
    data_addr_s: Option<Sender<u64>>,
}

#[derive(Debug)]
pub struct Rloader {
    name: String,
    id: u64,
    data_addr_r: Receiver<u64>,
}

pub fn new() -> Loader {
    todo!()
}

pub fn from_proto(request: CreateDataloaderRequest, id: u64) -> (Loader, Rloader) {
    let (data_addr_s, data_addr_r) = channel::<u64>(4096);
    (
        Loader {
            name: request.name.clone(),
            id,
            hosts: HashMap::new(),
            data_addr_s: Some(data_addr_s),
        },
        Rloader {
            name: request.name,
            id,
            data_addr_r,
        },
    )
}

impl Rloader {
    pub async fn next(&mut self) -> u64 {
        self.data_addr_r.recv().await.unwrap()
    }

    pub async fn try_next(&mut self) -> Result<u64, TryRecvError> {
        self.data_addr_r.try_recv()
    }

    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Loader {
    pub fn get_id(&self) -> u64 {
        self.id
    }

    pub async fn send_data(&self, addr: u64) {
        self.data_addr_s.as_ref().unwrap().send(addr).await.unwrap();
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn attach(&mut self, host: Ssampler, host_addr: String) -> Result<(), String> {
        todo!()
    }
}
