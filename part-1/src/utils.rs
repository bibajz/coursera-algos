use rand;

pub fn generate_random_vector(n: u64) -> Vec<u8> {
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(rand::random::<u8>());
    }
    return vec;
}
