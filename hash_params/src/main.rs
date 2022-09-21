use std::collections::hash_map::DefaultHasher;
use std::env;

// Yeah all of this stuff is already in the lib, but it's gated by a feature and I haven't figured out
// How best to handle that yet

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args[1..].iter() {
        let hashed = calculate_hash(arg);
        println!("{arg} --> {hashed:x}");
    }
}
