use std::cmp::Ord;
use std::collections::HashMap;

type PivotPickStrategy<T: Copy + Ord> = fn(&[T]) -> (T, usize);

pub fn pivot_pick_strategy_dmap<T: Copy + Ord>() -> HashMap<String, PivotPickStrategy<T>> {
    let mut dmap: HashMap<String, PivotPickStrategy<T>> = HashMap::new();
    dmap.insert("first".to_string(), first_pivot_pick_strategy);
    dmap.insert("last".to_string(), last_pivot_pick_strategy);
    dmap.insert(
        "median_of_three".to_string(),
        median_of_three_pivot_pick_strategy,
    );

    dmap
}
pub fn first_pivot_pick_strategy<T: Copy>(list: &[T]) -> (T, usize) {
    (list[0], 0)
}

pub fn last_pivot_pick_strategy<T: Copy>(list: &[T]) -> (T, usize) {
    (list[list.len() - 1], list.len() - 1)
}

pub fn median_of_three_pivot_pick_strategy<T: Copy + Ord>(list: &[T]) -> (T, usize) {
    let len = list.len();

    if len % 2 == 0 {
        let med = _median_of_three(list[0], list[len / 2 - 1], list[len - 1]);
        if med == list[0] {
            (med, 0)
        } else if med == list[len / 2 - 1] {
            (med, len / 2 - 1)
        } else {
            (med, len - 1)
        }
    } else {
        let med = _median_of_three(list[0], list[(len - 1) / 2], list[len - 1]);
        if med == list[0] {
            (med, 0)
        } else if med == list[(len - 1) / 2] {
            (med, (len - 1) / 2)
        } else {
            (med, len - 1)
        }
    }
}

fn _median_of_three<T: Copy + Ord>(i1: T, i2: T, i3: T) -> T {
    let mut l = [i1, i2, i3];
    l.sort();
    l[1]
}

#[test]
fn test_first_pick_works_correctly() {
    assert_eq!(1, first_pivot_pick_strategy(&[1, 5, 9]).0)
}

#[test]
fn test_last_pick_works_correctly() {
    assert_eq!(9, last_pivot_pick_strategy(&[1, 5, 9]).0)
}

#[test]
fn test_median_odd_length_pick_works_correctly() {
    assert_eq!((3, 0), median_of_three_pivot_pick_strategy(&[3, 5, 1]))
}

#[test]
fn test_median_even_length_pick_works_correctly() {
    assert_eq!((4, 3), median_of_three_pivot_pick_strategy(&[3, 5, 1, 4]))
}

#[test]
fn test_pivot_pick_dispatch_map_works_correctly() {
    let dmap = pivot_pick_strategy_dmap();
    assert_eq!((4, 3), dmap["median_of_three"](&[3, 5, 1, 4]))
}

#[test]
#[should_panic]
fn test_pivot_pick_dispatch_map_fails_on_unknown_key() {
    let dmap = pivot_pick_strategy_dmap();
    let _i = dmap["invalid"](&[3, 5, 1, 4]);
}
