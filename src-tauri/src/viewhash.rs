use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use rand::Rng;
pub fn generate_hash() -> String {
    let mut rng = rand::thread_rng();
    let mut hasher = DefaultHasher::new();
    hasher.write_usize(rng.gen::<usize>());

    let r_hash: String = hasher.finish().to_string();
    //println!("# Hash generated: {}", r_hash);
    r_hash
}