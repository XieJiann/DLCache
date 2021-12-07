use crate::proto::distributed::CreateSamplerRequest;
use tokio::sync::mpsc::{channel, error::TryRecvError, Receiver, Sender};
#[derive(Debug)]
pub struct Ssampler {
    loader_id: u64,
    sampler_s: Sender<u32>,
}

#[derive(Debug)]
pub struct Rsampler {
    loader_id: u64,
    sampler_r: Receiver<u32>,
}

pub fn new(dataset_name: &str, loader_id: u64, host_id: u64) -> (Ssampler, Rsampler) {
    todo!()
}
