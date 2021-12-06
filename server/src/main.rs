use ::joader::cache::cache::Cache;
use ::joader::joader::joader_table::JoaderTable;
use clap::{load_yaml, App};
use joader::proto::dataloader::data_loader_svc_server::DataLoaderSvcServer;
use joader::proto::dataset::dataset_svc_server::DatasetSvcServer;
use joader::service::{DataLoaderSvcImpl, DatasetSvcImpl};
use libc::shm_unlink;
use std::net::SocketAddr;
use std::process;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};
use tonic::transport::Server;

async fn start(joader_table: Arc<Mutex<JoaderTable>>) {
    println!("start joader loop ....");
    loop {
        let mut joader_table = joader_table.lock().await;
        if joader_table.is_empty() {
            log::debug!("sleep ....");
            sleep(Duration::from_millis(1000)).await;
            continue;
        }
        joader_table.next().await;
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();
    let log4rs_config = matches.value_of("log4rs_config").unwrap();
    let ip_port = matches.value_of("ip_port").unwrap();
    let head_num: u64 = matches.value_of("head_num").unwrap().parse().unwrap();
    let cache_capacity: usize = matches.value_of("cache_capacity").unwrap().parse().unwrap();
    let shm_path = matches.value_of("shm_path").unwrap().to_string();
    log4rs::init_file(log4rs_config, Default::default()).unwrap();
    // //start joader_table
    let cache = Cache::new(cache_capacity, &shm_path, head_num);
    let joader_table = Arc::new(Mutex::new(JoaderTable::new(cache, ip_port)));

    ctrlc::set_handler(move || {
        unsafe {
            let shmpath = shm_path.as_ptr() as *const i8;
            shm_unlink(shmpath);
        };
        println!("Close {:?} successfully", shm_path);
        process::exit(1);
    })
    .expect("Error setting Ctrl-C handler");
    // start server
    let addr: SocketAddr = ip_port.parse()?;
    let dataset_svc = DatasetSvcImpl::new(joader_table.clone());
    let data_loader_svc = DataLoaderSvcImpl::new(joader_table.clone());

    // start joader
    tokio::spawn(async move { start(joader_table).await });

    println!("start joader at {:?}......\n", addr);
    Server::builder()
        .add_service(DatasetSvcServer::new(dataset_svc))
        .add_service(DataLoaderSvcServer::new(data_loader_svc))
        .serve(addr)
        .await?;
    Ok(())
}
