use prost::Message;

pub mod rat {
    include!(concat!(env!("OUT_DIR"), "/rat.items.rs"));
}

fn main() {
    let mut test = rat::Test::default();
    test.words = "word string".to_string();
    test.thoughts = "thought string".to_string();
    let mut buf = Vec::new();
    buf.reserve(test.encoded_len());
    test.encode(&mut buf).unwrap();
    println!("{test:?}");
    println!("{buf:?}");
}
