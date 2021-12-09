use super::joader::*;
use crate::{
    cache::cache::Cache,
    dataset::new_dummy,
    loader::{create_data_channel, DataReceiver},
};

async fn read_data(mut r: DataReceiver, len: usize) -> Vec<u64> {
    let mut res = Vec::new();
    loop {
        let (mut indices, empty) = r.recv_all().await;
        res.append(&mut indices);
        if res.len() == len || empty {
            break;
        }
    }
    return res;
}

async fn write(mut joader: Joader, mut cache: Cache, len: usize) {
    for _i in 0..len {
        joader.next(&mut cache).await;
    }
}

#[tokio::test]
async fn test_1_loader() {
    let len = 10000;
    let name = "dummy".to_string();
    let dataset = new_dummy(len, name.clone());
    let mut joader = Joader::new(dataset);
    let (s, r) = create_data_channel(0);
    joader.add_loader(0);
    joader.get_mut(0).unwrap().add_data_sender(s);
    let cache = Cache::new(256, &name, 1);
    tokio::spawn(async move { write(joader, cache, len).await });
    let mut res = tokio::spawn(async move { read_data(r, len).await })
        .await
        .unwrap();
    res.sort();
    assert_eq!(res, (0..len).map(|x| x as u64).collect::<Vec<_>>());
}

// #[test]
// fn test_multiple_loader() {}
