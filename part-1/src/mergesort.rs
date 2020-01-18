use std::clone::Clone;
use std::cmp::PartialOrd;

pub fn merge_sort<T: PartialOrd + Clone + Copy>(list: &[T]) -> Vec<T> {
    let l = list.len();

    if l == 0 || l == 1 {
        return list.to_vec();
    }
    return merge(&merge_sort(&list[..l / 2]), &merge_sort(&list[l / 2..]));
}

fn merge<T: PartialOrd + Clone + Copy>(list1: &[T], list2: &[T]) -> Vec<T> {
    let mut res = Vec::new();
    let mut i1 = 0;
    let mut i2 = 0;

    if list1.len() == 0 {
        // Clone trait necessary here
        return list2.to_vec();
    }
    if list2.len() == 0 {
        return list1.to_vec();
    }
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
            i2 += 1;
            if i2 == list2.len() {
                res.extend_from_slice(&list1[i1..]);
                break;
            }
        }
    }

    return res;
}

#[test]
fn test_merge_one_empty() {
    assert_eq!(vec![1, 2, 3], merge(&[1, 2, 3], &[]))
}

#[test]
fn test_merge_two_empty() {
    let empty: Vec<u64> = Vec::new();
    assert_eq!(empty, merge(&[], &[]))
}

#[test]
fn test_merge_two_nonempty_integer() {
    assert_eq!(vec![1, 2, 2, 4, 5, 6], merge(&[1, 2, 6], &[2, 4, 5]))
}

#[test]
fn test_merge_two_nonempty_float() {
    assert_eq!(
        vec![1.0, 2.0, 2.1, 4.3],
        merge(&vec![1.0, 2.1], &vec![2.0, 4.3])
    )
}

#[test]
fn test_merge_sort() {
    assert_eq!(vec![1, 2, 3, 4, 5, 9], merge_sort(&[5, 3, 4, 2, 9, 1]))
}

#[test]
fn test_merge_sort_empty() {
    let empty: Vec<u64> = Vec::new();
    assert_eq!(empty, merge_sort(&[]))
}
