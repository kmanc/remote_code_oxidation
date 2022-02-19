use remote_access_trojan::rat::service_tester_server::{ServiceTester, ServiceTesterServer};
use remote_access_trojan::rat::{self, RequestTester, ResponseTester};
use tonic::{Request, Response, Status};
use tonic::transport::Server;

#[derive(Default)]
pub struct MyServiceTester {}

#[tonic::async_trait]
impl ServiceTester for MyServiceTester {
    async fn send(&self, request: Request<RequestTester>) -> Result<Response<ResponseTester>, Status> {
        println!("Request={request:?}");
        Ok(Response::new(
            ResponseTester {
                words:String::from("First response"),
                thoughts:String::from("You did it!")
            }
        ))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("server");

    // Does the protobuf structure work?
    let example = rat::ResponseTester {
        words:String::from("server words"),
        thoughts:String::from("server thoughts")
    };
    println!("{example:?}");

    // Can I send a stand up the server?
    let addr = "127.0.0.1:4444".parse()?;

    let service_server = MyServiceTester::default();
    Server::builder()
        .add_service(ServiceTesterServer::new(service_server))
        .serve(addr)
        .await?;

    Ok(())
}