use std::env;

mod inversions_count;
mod mergesort;
mod quicksort;
mod pivot_pick_strategies;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!(
        "Number of inversions: {}",
        inversions_count::sort_and_count_inv_from_file(filename).1
    );

    println!(
        "Number of comps first: {:?}",
        quicksort::sort_and_count_comps_by_strat_from_file(filename, "first").1
    );
    println!(
        "Number of comps last: {:?}",
        quicksort::sort_and_count_comps_by_strat_from_file(filename, "last").1
    );
    println!(
        "Number of comps median: {:?}",
        quicksort::sort_and_count_comps_by_strat_from_file(filename, "median_of_three").1
    );
}

#[test]
fn test_merge_sort_random_array() {
    let mut input = utils::generate_random_vector(1000);
    let mut unsorted = [0; 1000];

    unsorted.copy_from_slice(&input);
    input.sort();

    assert_eq!(input, mergesort::merge_sort(&unsorted))
}
