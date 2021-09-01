use tonic::{transport::Server, Request, Response, Status};

use ddddb::ddddb_server::{Ddddb, DdddbServer};
use ddddb::{CreateReply, CreateRequest};

pub mod ddddb {
    tonic::include_proto!("ddddb"); // The string specified here must match the proto package name
}

#[derive(Debug, Default)]
pub struct MyDdddb {}

#[tonic::async_trait]
impl Ddddb for MyDdddb {
    async fn create(
        &self,
        request: Request<CreateRequest>, // Accept request of type CreateRequest
    ) -> Result<Response<CreateReply>, Status> { // Return an instance of type CreateReply
        
        println!("Got a request: {:?}", request);


        let reply = ddddb::CreateReply {
            id: format!("Create {}!", request.into_inner().id).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };
        
        // find table

        // create table?

        // Put file in table, aw

        // use a crazy b tree


        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let ddddb = MyDdddb::default();


    Server::builder()
        .add_service(DdddbServer::new(ddddb))
        .serve(addr)
        .await?;
        
    Ok(())
}