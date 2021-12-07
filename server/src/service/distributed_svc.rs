use crate::joader::joader_table::JoaderTable;
use crate::loader::{create_idx_channel, IdxReceiver, Loader};
use crate::proto::distributed::distributed_svc_server::DistributedSvc;
use crate::proto::distributed::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{async_trait, Request, Response, Status};

use super::GlobalID;

#[derive(Debug, Default)]
struct Host {
    recv: HashMap<u64, IdxReceiver>,
}

impl Host {
    fn add(&mut self, r: IdxReceiver) {
        self.recv.insert(r.get_loader_id(), r);
    }
}

#[derive(Debug)]
pub struct DistributedSvcImpl {
    ip_port: String,
    loader_id: GlobalID,
    host_id: GlobalID,
    loader_id_table: Arc<Mutex<HashMap<String, u64>>>,
    host_id_table: Arc<Mutex<HashMap<String, u64>>>,
    host_table: Arc<Mutex<HashMap<u64, Host>>>,
    host_port_table: Arc<Mutex<HashMap<String, u64>>>,
    joader_table: Arc<Mutex<JoaderTable>>,
}

#[async_trait]
impl DistributedSvc for DistributedSvcImpl {
    async fn register_host(
        &self,
        request: Request<RegisterHostRequest>,
    ) -> Result<Response<RegisterHostResponse>, Status> {
        let request = request.into_inner();
        let mut table = self.host_id_table.lock().await;
        if table.contains_key(&request.ip) {
            return Err(Status::already_exists(format!("{}", request.ip)));
        }
        let id = self.host_id.get_id().await;
        table.insert(request.ip.clone(), id);
        self.host_port_table
            .lock()
            .await
            .insert(request.ip, request.port);
        self.host_table.lock().await.insert(id, Host::default());
        Ok(Response::new(RegisterHostResponse { host_id: id }))
    }

    async fn delete_host(
        &self,
        request: Request<DeleteHostRequest>,
    ) -> Result<Response<DeleteHostResponse>, Status> {
        Err(Status::unimplemented(
            "Delete host has not been implemented",
        ))
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
        let joader = jt
            .get_mut(&request.dataset_name)
            .map_err(|x| Status::not_found(x))?;
        let loader_id;
        if loader_id_table.contains_key(&request.name) {
            loader_id = loader_id_table[&request.name];
        } else {
            loader_id = self.loader_id.get_id().await;
            let loader = Loader::new(loader_id);
            joader.add_loader(loader);
        }
        let (is, ir) = create_idx_channel(loader_id);
        let loader = joader
            .get_mut(loader_id)
            .map_err(|x| Status::not_found(x))?;
        loader.add_idx_sender(is, host_id);
        let length = joader.len();
        Ok(Response::new(CreateSamplerResponse { length, loader_id }))
    }

    async fn delete_sampler(
        &self,
        request: Request<DeleteSamplerRequest>,
    ) -> Result<Response<DeleteSamplerResponse>, Status> {
        let request = request.into_inner();
        let host_id = *self
            .host_id_table
            .lock()
            .await
            .get(&request.ip)
            .ok_or_else(|| Status::not_found(format!("{} not exited", request.ip)))?;
        let loader_id_table = self.loader_id_table.lock().await;
        let mut jt = self.joader_table.lock().await;
        let joader = jt
            .get_mut(&request.dataset_name)
            .map_err(|x| Status::not_found(x))?;
        let loader_id = loader_id_table
            .get(&request.name)
            .ok_or_else(|| Status::not_found(format!("{} not exited", request.name)))?;
        let loader = joader.get_mut(*loader_id).unwrap();
        loader.del_idx_sender(host_id);
        Ok(Response::new(DeleteSamplerResponse {}))
    }

    async fn query_host(
        &self,
        request: Request<QueryHostRequest>,
    ) -> Result<Response<QueryHostResponse>, Status> {
        let req = request.into_inner();
        let port = *self
            .host_port_table
            .lock()
            .await
            .get(&req.ip)
            .ok_or_else(|| Status::not_found(format!("Host {} not exist", req.ip)))?;
        Ok(Response::new(QueryHostResponse { port }))
    }

    async fn sample(
        &self,
        request: Request<SampleRequest>,
    ) -> Result<Response<SampleResponse>, Status> {
        todo!()
    }
}
