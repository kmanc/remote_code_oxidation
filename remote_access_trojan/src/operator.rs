use std::io::{stdin, stdout, Write};
use tonic::transport::Endpoint;

/*
TODO
    - send commands from operator to server
    - encrypt traffic from operator to server
*/

fn main() {
    println!("operator");
    print!("Command > ");
    stdout().flush().unwrap();
    let mut line = String::new();
    let input = stdin().read_line(&mut line).unwrap();
    println!("You said {line}");
}