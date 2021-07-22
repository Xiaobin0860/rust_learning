mod pb;
use std::{collections::HashMap, pin::Pin, sync::Arc};

use pb::{pow_builder_server::*, *};
use tokio::sync::{mpsc, RwLock};
use tonic::{codegen::futures_core::Stream, Status};

struct Shared {
    clients: HashMap<String, mpsc::Sender<Result<BlockHash, Status>>>,
}

pub struct PowService {
    //send block to PoW engine
    tx: mpsc::Sender<Block>,
    shared: Arc<RwLock<Shared>>,
}

#[tonic::async_trait]
impl PowBuilder for PowService {
    type SubscribeStream = Pin<Box<dyn Stream<Item = Result<BlockHash, Status>> + Send + Sync>>;

    async fn subscribe(
        &self,
        request: tonic::Request<ClientInfo>,
    ) -> Result<tonic::Response<Self::SubscribeStream>, Status> {
        todo!()
    }

    async fn submit(
        &self,
        request: tonic::Request<Block>,
    ) -> Result<tonic::Response<BlockStatus>, Status> {
        todo!()
    }
}

fn main() {
    println!("Hello, world!");
}
