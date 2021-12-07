use tokio::sync::mpsc::{channel, error::TryRecvError, Receiver, Sender};

#[derive(Debug)]
pub struct LoaderSender<T> {
    loader_id: u64,
    sender: Sender<T>,
}

#[derive(Debug)]
pub struct LoaderReceiver<T> {
    loader_id: u64,
    recv: Receiver<T>,
}

pub fn new<T>(loader_id: u64) -> (LoaderSender<T>, LoaderReceiver<T>) {
    let (sender, recv) = channel::<T>(4098);
    (
        LoaderSender { loader_id, sender },
        LoaderReceiver { loader_id, recv },
    )
}

impl<T> LoaderSender<T> {
    pub fn get_loader_id(&self) -> u64 {
        self.loader_id
    }
}

impl<T> LoaderReceiver<T> {
    pub fn get_loader_id(&self) -> u64 {
        self.loader_id
    }

    pub fn recv_all(&mut self) -> Vec<T> {
        todo!()
    }
}
