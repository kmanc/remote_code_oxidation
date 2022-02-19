use remote_access_trojan::rat::service_tester_client::ServiceTesterClient;
use remote_access_trojan::rat::{self, RequestTester};
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("implant");

    // Does the protobuf structure work?
    let example = rat::ResponseTester {
        words:String::from("implant words"),
        thoughts:String::from("implant thoughts")
    };
    println!("{example:?}");

    // Can I send a protobuf from the client to the server?
    let addr = Endpoint::from_static("http://127.0.0.1:4444");
    let mut client = ServiceTesterClient::connect(addr).await?;
    let request = tonic::Request::new(
        RequestTester {
            words:String::from("First request"),
            thoughts:String::from("This was confusing")
        },
    );
    let response = client.send(request).await?.into_inner();
    println!("Response={response:?}");

    Ok(())
}