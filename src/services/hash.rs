use std::hash::{DefaultHasher, Hash, Hasher};

/// About hash.rs
/// contains little func get_hash, witch takes a &String as an arg and returns hash of this line as u64
pub fn get_hash(s: &String) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    return h.finish();
}