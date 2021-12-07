use crate::joader::joader_table::JoaderTable;
use crate::loader::sampler::{self, Rsampler, Ssampler};
use crate::loader::{Loader, Rloader};
use crate::proto::distributed::distributed_svc_server::DistributedSvc;
use crate::proto::distributed::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};

use super::GlobalID;

#[derive(Debug)]
struct Host {}

#[derive(Debug)]
pub struct DistributedSvcImpl {
    ip_port: String,
    joader_table: Arc<Mutex<JoaderTable>>,
    host_table: Arc<Mutex<HashMap<u64, Host>>>,
    loader_id: GlobalID,
    host_id: GlobalID,
    loader_id_table: Arc<Mutex<HashMap<String, u64>>>,
    host_id_table: Arc<Mutex<HashMap<String, u64>>>,
    host_addr_table: Arc<Mutex<HashMap<String, String>>>,
}

#[async_trait]
impl DistributedSvc for DistributedSvcImpl {
    async fn register_host(
        &self,
        request: Request<RegisterHostRequest>,
    ) -> Result<Response<RegisterHostResponse>, Status> {
        todo!()
    }
    async fn delete_host(
        &self,
        request: Request<DeleteHostRequest>,
    ) -> Result<Response<DeleteHostResponse>, Status> {
        todo!()
    }
    async fn create_sampler(
        &self,
        request: Request<CreateSamplerRequest>,
    ) -> Result<Response<CreateSamplerResponse>, Status> {
        let request = request.into_inner();
        let host_id = *self
            .host_id_table
            .lock()
            .await
            .get(&request.ip)
            .ok_or_else(|| Status::not_found(format!("{} not exited", request.ip)))?;
        let loader_id_table = self.loader_id_table.lock().await;
        let mut jt = self.joader_table.lock().await;
        let loader_id;
        if loader_id_table.contains_key(&request.name) {
            loader_id = loader_id_table[&request.name];
        } else {
            loader_id = self.loader_id.get_id().await;
        }
        let (ss, rs) = sampler::new(&request.dataset_name, loader_id, host_id);
        let joader = jt
            .get_mut(&request.dataset_name)
            .map_err(|x| Status::not_found(x))?;
        joader.add_sampler(ss);
        let length = joader.len();
        Ok(Response::new(CreateSamplerResponse { length, loader_id }))
    }

    async fn delete_sampler(
        &self,
        request: Request<DeleteSamplerRequest>,
    ) -> Result<Response<DeleteSamplerResponse>, Status> {
        todo!()
    }

    async fn query_host(
        &self,
        request: Request<QueryHostRequest>,
    ) -> Result<Response<QueryHostResponse>, Status> {
        todo!()
        // let request = request.into_inner();
        // let host_id = self
        //     .host_id_table
        //     .lock()
        //     .await
        //     .get(&request.host_addr)
        //     .ok_or_else(|| Status::not_found(format!("{} not exited", request.host_addr)))?;
    }

    async fn sample(
        &self,
        request: Request<SampleRequest>,
    ) -> Result<Response<SampleResponse>, Status> {
        todo!()
    }
}
