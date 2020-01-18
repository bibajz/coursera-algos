mod mergesort;
mod utils;

fn main() {
}

#[test]
fn test_merge_sort_random_array() {
    let mut input = utils::generate_random_vector(1000);
    let mut unsorted = [0; 1000];

    unsorted.copy_from_slice(&input);
    input.sort();

    assert_eq!(input, mergesort::merge_sort(&unsorted))
}
