use std::fs::File;
use std::io::{BufRead, BufReader};

use std::clone::Clone;
use std::cmp::PartialOrd;

pub fn sort_and_count_comps_by_strat_from_file(fname: &str, strategy: &str) -> (Vec<u64>, usize) {
    let file = File::open(fname).unwrap();
    let f = BufReader::new(file).lines();

    let mut numbers: Vec<u64> = f
        .map(|x| u64::from_str_radix(&x.unwrap(), 10).unwrap())
        .collect();

    let comp_count = sourt_and_count_comps(& mut numbers);

    return (numbers, comp_count);
}