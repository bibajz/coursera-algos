use rand;

pub fn generate_random_vector(n: u64) -> Vec<u8> {
    let mut vec = Vec::new();
    for _ in 0..n {
        vec.push(rand::random::<u8>());
    }
    return vec;
}

fn prepend_zeros(n: usize, s: &str) -> String {
    let zeros: &str = &"0".repeat(n);
    return [zeros, s].join("");
}

fn append_zeros(n: usize, s: &str) -> String {
    let zeros: &str = &"0".repeat(n);
    return [s, zeros].join("");
}

#[test]
fn test_int_from_str() {
    assert_eq!(123, u8::from_str_radix("123", 10).unwrap())
}

#[test]
fn test_prepend_empty() {
    assert_eq!("00", prepend_zeros(2, ""))
}

#[test]
fn test_prepend_zero() {
    assert_eq!("", prepend_zeros(0, ""))
}

#[test]
fn test_prepend_nonempty_zero() {
    assert_eq!("abc", prepend_zeros(0, "abc"))
}

#[test]
fn test_prepend_ok() {
    assert_eq!("00abc", prepend_zeros(2, "abc"))
}

#[test]
fn test_prepend_append() {
    assert_eq!("00abc00", append_zeros(2, &prepend_zeros(2, "abc")))
}

#[test]
fn test_append_prepend() {
    assert_eq!("00abc00", prepend_zeros(2, &append_zeros(2, "abc")))
}
