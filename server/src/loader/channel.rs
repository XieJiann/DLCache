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

pub fn new<T: std::fmt::Debug>(loader_id: u64) -> (LoaderSender<T>, LoaderReceiver<T>) {
    let (sender, recv) = channel::<T>(4098);
    (
        LoaderSender { loader_id, sender },
        LoaderReceiver { loader_id, recv },
    )
}

impl<T: std::fmt::Debug> LoaderSender<T> {
    pub fn get_loader_id(&self) -> u64 {
        self.loader_id
    }

    pub async fn send(&self, d: T) {
        self.sender.send(d).await.unwrap();
    }
}

impl<T> LoaderReceiver<T> {
    pub fn get_loader_id(&self) -> u64 {
        self.loader_id
    }

    pub fn recv_all(&mut self) -> Vec<T> {
        let mut ret = Vec::new();
        loop {
            match self.recv.try_recv() {
                Ok(v) => ret.push(v),
                Err(err) => {
                    assert_eq!(err, TryRecvError::Empty);
                    break;
                }
            }
        }
        ret
    }
}
