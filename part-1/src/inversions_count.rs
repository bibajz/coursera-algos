use std::fs::File;
use std::io::{BufRead, BufReader};

use std::clone::Clone;
use std::cmp::PartialOrd;

pub fn sort_and_count_inv_from_file(fname: &str) -> (Vec<u64>, usize) {
    let file = File::open(fname).unwrap();
    let f = BufReader::new(file).lines();

    let numbers: Vec<u64> = f
        .map(|x| u64::from_str_radix(&x.unwrap(), 10).unwrap())
        .collect();
    return sort_and_count_inv(&numbers);
}

fn sort_and_count_inv<T: PartialOrd + Clone + Copy>(list: &[T]) -> (Vec<T>, usize) {
    let l = list.len();

    if l == 0 || l == 1 {
        return (list.to_vec(), 0);
    }
    let (sorted_half_1, c1) = sort_and_count_inv(&list[..l / 2]);
    let (sorted_half_2, c2) = sort_and_count_inv(&list[l / 2..]);

    let (sorted, inv_split_count) = merge_and_count_inv(&sorted_half_1, &sorted_half_2);

    return (sorted, c1 + c2 + inv_split_count);
}

fn merge_and_count_inv<T: PartialOrd + Clone + Copy>(list1: &[T], list2: &[T]) -> (Vec<T>, usize) {
    let mut res = Vec::new();

    if list1.len() == 0 {
        // Clone trait necessary here
        return (list2.to_vec(), 0);
    }
    if list2.len() == 0 {
        return (list1.to_vec(), 0);
    }

    let mut i1 = 0;
    let mut i2 = 0;
    let mut count_inv = 0;

    loop {
        // Ord trait necessary here
        if list1[i1] <= list2[i2] {
            // Copy trait necessary here
            res.push(list1[i1]);
            i1 += 1;
            if i1 == list1.len() {
                res.extend_from_slice(&list2[i2..]);
                break;
            }
        } else {
            res.push(list2[i2]);
            count_inv += list1.len() - i1;
            i2 += 1;
            if i2 == list2.len() {
                res.extend_from_slice(&list1[i1..]);
                break;
            }
        }
    }

    return (res, count_inv);
}

#[test]
fn test_inv_count_on_sorted() {
    assert_eq!(3, merge_and_count_inv(&[1, 2, 4, 6], &[3, 5, 7]).1)
}

#[test]
fn test_inv_count() {
    assert_eq!((vec![1, 2, 3], 2), sort_and_count_inv(&[2, 3, 1]))
}
