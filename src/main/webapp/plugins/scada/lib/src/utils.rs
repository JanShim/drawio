use std::hash::{Hash,DefaultHasher, Hasher};


pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

pub fn compare_eq_options<T: PartialEq>(a: &Option<T>, b: &Option<T> ) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a.eq(b),
        (None, None) => true,
        _ => false,
    }    
}