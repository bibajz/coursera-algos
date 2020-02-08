use std::fs::File;
use std::io::{BufRead, BufReader};

use std::clone::Clone;
use std::cmp::PartialOrd;

use crate::pivot_pick_strategies::pivot_pick_strategy_dmap;

pub fn sort_and_count_comps_by_strat_from_file(fname: &str, strategy: &str) -> (Vec<u64>, usize) {
    let file = File::open(fname).unwrap();
    let f = BufReader::new(file).lines();

    let mut numbers: Vec<u64> = f
        .map(|x| u64::from_str_radix(&x.unwrap(), 10).unwrap())
        .collect();

    let comp_count = sort_and_count_comps_by_strategy(& mut numbers, strategy);

    return (numbers, comp_count);
}

fn sort_and_count_comps_by_strategy<T: Copy + Ord>(list: & mut [T], strategy: &str) -> usize {
    let len = list.len();

    if len == 0 || len == 1 {
        return 0;
    }
    if len == 2 {
        if list[0] > list[1] {
            _swap_items(list, 0, 1);
        }
        return 1;
    }
    swap_pivot_with_first(list, strategy);

    let mut i = 1;
    let mut j = 1;

    loop {
        if j == len {
            break
        }
        if list[0] > list[j] {
            if i <= j {
                _swap_items(list, i, j);
                i += 1;
            }
        }
        j += 1;

    }
    _swap_items(list, 0, i-1);

    let mut comp_count = len - 1;

    comp_count += sort_and_count_comps_by_strategy(&mut list[0..i-1], strategy);
    comp_count += sort_and_count_comps_by_strategy(&mut list[i..], strategy);

    comp_count

}

fn swap_pivot_with_first<T: Copy + Ord>(list: & mut [T], strategy: &str) -> () {
    let (_, pos) = pivot_pick_strategy_dmap()[strategy](list);
    if pos != 0 {
        _swap_items(list, pos, 0)
    }
}

fn _swap_items<T: Copy + Ord>(list: & mut [T], ind1: usize, ind2: usize) -> () {
    if ind1 != ind2 {
        let tmp = list[ind1];
        list[ind1] = list[ind2];
        list[ind2] = tmp;
    }
}