use super::to_status;
use crate::loader;
use crate::loader::Rloader;
use crate::proto::dataloader::data_loader_svc_server::DataLoaderSvc;
use crate::proto::dataloader::*;
use crate::{joader::joader_table::JoaderTable};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};



#[derive(Debug)]
pub struct DataLoaderSvcImpl {
    joader_table: Arc<Mutex<JoaderTable>>,
    loader_id_table: Arc<Mutex<HashMap<String, u64>>>,
    loader_id: Arc<Mutex<u64>>,
    host_id: Arc<Mutex<u64>>,
}

impl DataLoaderSvcImpl {
    pub fn new(joader_table: Arc<Mutex<JoaderTable>>) -> Self {
        // Self {
        //     joader_table,
        //     loader_table: Default::default(),
        //     loader_id: Default::default(),
        //     host_id: Default::default(),
        // }
        todo!()
    }
}

impl DataLoaderSvcImpl {
    #[inline]
    async fn get_loader_id(&self) -> u64 {
        let mut id = self.loader_id.lock().await;
        *id += 1;
        *id
    }

    #[inline]
    async fn get_host_id(&self) -> u64 {
        let mut id = self.host_id.lock().await;
        *id += 1;
        *id
    }
}

#[async_trait]
impl DataLoaderSvc for DataLoaderSvcImpl {
    async fn create_dataloader(
        &self,
        request: Request<CreateDataloaderRequest>,
    ) -> Result<Response<CreateDataloaderResponse>, Status> {
        // log::info!("call create loader {:?}", request);
        // let host_id = self.get_host_id().await;
        // let loader_id = self.get_loader_id().await;
        // let request = request.into_inner();
        // let name = request.name.clone();
        // let host_addr = request.host_addr.clone();
        // let (shost, rhost) = host::new(host_id);

        // let dataset_name = request.dataset_name.clone();
        // let mut loader_table = self.loader_table.lock().await;

        // let mut joader_table = self.joader_table.lock().await;
        // let shm_path = joader_table.get_shm_path();
        // let joader = joader_table
        //     .get(&dataset_name)
        //     .map_err(|x| Status::not_found(x))?;

        // let length;
        // if joader.contains(&name) {
        //     length = joader
        //         .attach(&name, shost, host_addr)
        //         .map_err(|x| Status::not_found(x))?;
        // } else {
        //     let (s, r) = loader::from_proto(request, loader_id);
        //     length = joader_table
        //         .add_loader(s)
        //         .map_err(|x| Status::already_exists(x))?;
        // }
        // loader_table
        //         .get_mut(&host_addr)
        //         .unwrap()
        //         .insert(r)
        //         .map_err(|x| Status::not_found(x))?;
        // Ok(Response::new(CreateDataloaderResponse {
        //     length,
        //     shm_path,
        //     loader_id,
        //     status: None,
        // }))
        todo!()
    }
    async fn next(&self, request: Request<NextRequest>) -> Result<Response<NextResponse>, Status> {
        // let loader_id = request.into_inner().loader_id;
        // let mut loader_table = self.loader_table.lock().await;
        // let mut address = Vec::new();
        // let r = loader_table.get_mut(&loader_id).unwrap();
        // while let Ok(addr) = r.try_next().await {
        //     address.push(addr);
        // }
        // Ok(Response::new(NextResponse { address }))
        todo!()
    }
    async fn delete_dataloader(
        &self,
        request: Request<DeleteDataloaderRequest>,
    ) -> Result<Response<DeleteDataloaderResponse>, Status> {
        todo!()
        // log::info!("call delete loader {:?}", request);

        // let mut status = None;
        // let mut loader_table = self.loader_table.lock().await;
        // if let Some(loader) = loader_table.remove(&request.into_inner().loader_id) {
        //     let mut joader_table = self.joader_table.lock().await;
        //     status = Some(to_status(&joader_table.del_loader(loader)));
        // }
        // Ok(Response::new(DeleteDataloaderResponse { status }))
    }
}
