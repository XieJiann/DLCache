use crate::proto::dataloader::CreateDataloaderRequest;
use tokio::sync::mpsc::{channel, error::TryRecvError, Receiver, Sender};
// Loader store the information of schema, dataset and filter
#[derive(Default, Debug, Clone)]
struct Loader {
    dataset_name: String,
    id: u64,
}

#[derive(Debug)]
pub struct Sloader {
    loader: Loader,
    data_addr_s: Sender<u64>,
}
#[derive(Debug)]
pub struct Rloader {
    loader: Loader,
    data_addr_r: Receiver<u64>,
}
pub fn from_proto(request: CreateDataloaderRequest, id: u64) -> (Sloader, Rloader) {
    let loader = Loader {
        dataset_name: request.name,
        id,
    };
    let (data_addr_s, data_addr_r) = channel::<u64>(4096);
    (
        Sloader {
            loader: loader.clone(),
            data_addr_s,
        },
        Rloader { loader, data_addr_r },
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
        self.loader.id
    }

    pub fn get_name(&self) -> &str {
        &self.loader.dataset_name
    }
}

impl Sloader {
    pub fn get_id(&self) -> u64 {
        self.loader.id
    }

    pub async fn send_data(&self, addr: u64) {
        self.data_addr_s.send(addr).await.unwrap();
    }

    pub fn get_name(&self) -> &str {
        &self.loader.dataset_name
    }
}
