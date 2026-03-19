use std::hash::{DefaultHasher, Hash, Hasher};

pub fn get_hash(s: &String) -> u64 {
    let mut h = DefaultHasher::new();
    s.hash(&mut h);
    return h.finish();
}