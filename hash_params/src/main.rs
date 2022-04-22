use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args[1..].iter() {
        let hashed = rco_utils::calculate_hash(arg);
        println!("{arg} --> {hashed:x}");
    }
}
