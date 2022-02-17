use prost::Message;

pub mod rat {
    include!(concat!(env!("OUT_DIR"), "/rat.items.rs"));
}

fn main() {
    println!("implant");
    let mut example = rat::Test::default();
    example.words = "implant words".to_string();
    example.thoughts = "implant thoughts".to_string();
    let mut buf = Vec::new();
    buf.reserve(example.encoded_len());
    example.encode(&mut buf).unwrap();
    println!("{example:?}");
    println!("{buf:?}");
}