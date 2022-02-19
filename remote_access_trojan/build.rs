fn main() {
    let proto_file = "./rat.proto";
    
    tonic_build::configure()
                .build_server(true)
                .compile(&[proto_file], &["."])
                .unwrap();
}