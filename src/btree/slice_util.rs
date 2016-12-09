use std::iter;
use std::mem::{swap, replace};
use super::B;


pub fn shift<T>(slice: &mut [T], value: T) {
    let mut tmp = value;
    for x in slice.iter_mut() {
        swap(&mut tmp, x);
    }
}


pub fn insert_split_spill<T: Default + Copy>(
    xs: &[T; B],
    insertion_point: usize,
    key: T
) -> (([T; B], u8), T, ([T; B], u8)) {
    let mut left: [T; B] = Default::default();
    let mut mid: T = Default::default();
    let mut right: [T; B] = Default::default();
    let left_fill = (B + 1) / 2;
    let right_fill = (B + 1) - left_fill - 1;

    let (lxs, rxs) = xs.split_at(insertion_point);
    {
        let source = lxs.iter().cloned()
            .chain(iter::once(key))
            .chain(rxs.iter().cloned());

        let destination = left[..left_fill].iter_mut()
            .chain(iter::once(&mut mid))
            .chain(right[..right_fill].iter_mut());

        for (dst, src) in destination.zip(source) {
            *dst = src
        }
    }

    return ((left, left_fill as u8), mid, (right, right_fill as u8))
}


pub fn insert_split<T: Default>(
    xs: &mut [T; B + 1],
    insertion_point: usize,
    key: T
) -> ([T; B + 1], [T; B + 1]) {
    let mut left: [T; B + 1] = Default::default();
    let mut right: [T; B + 1] = Default::default();
    let left_fill = (B + 2) / 2;
    let right_fill = (B + 2) - left_fill;

    let (lxs, rxs) = xs.split_at_mut(insertion_point);
    {
        let source = lxs.iter_mut().map(|p| replace(p, Default::default()))
            .chain(iter::once(key))
            .chain(rxs.iter_mut().map(|p| replace(p, Default::default())));

        let destination = left[..left_fill].iter_mut()
            .chain(right[..right_fill].iter_mut());

        for (dst, src) in destination.zip(source) {
            *dst = src
        }
    }

    return (left, right)
}


#[test]
fn test_insert_split_spill_start() {
    let xs = [1, 2, 3, 4, 5, 6];
    let x = 0;
    let ip = 0;
    let ((left, left_fill), mid, (right, right_fill)) = insert_split_spill(&xs, ip, x);

    assert_eq!(left, [0, 1, 2, 0, 0, 0]);
    assert_eq!(left_fill, 3);

    assert_eq!(mid, 3);

    assert_eq!(right, [4, 5, 6, 0, 0, 0]);
    assert_eq!(right_fill, 3);
}


#[test]
fn test_insert_split_spill_after_first() {
    let xs = [2, 4, 6, 8, 10, 12];
    let x = 3;
    let ip = 1;
    let ((left, left_fill), mid, (right, right_fill)) = insert_split_spill(&xs, ip, x);

    assert_eq!(left, [2, 3, 4, 0, 0, 0]);
    assert_eq!(left_fill, 3);

    assert_eq!(mid, 6);

    assert_eq!(right, [8, 10, 12, 0, 0, 0]);
    assert_eq!(right_fill, 3);
}


#[test]
fn test_insert_split_spill_mid() {
    let xs = [2, 4, 6, 8, 10, 12];
    let x = 7;
    let ip = 3;
    let ((left, left_fill), mid, (right, right_fill)) = insert_split_spill(&xs, ip, x);

    assert_eq!(left, [2, 4, 6, 0, 0, 0]);
    assert_eq!(left_fill, 3);

    assert_eq!(mid, 7);

    assert_eq!(right, [8, 10, 12, 0, 0, 0]);
    assert_eq!(right_fill, 3);
}


#[test]
fn test_insert_split_spill_last() {
    let xs = [2, 4, 6, 8, 10, 12];
    let x = 13;
    let ip = 6;
    let ((left, left_fill), mid, (right, right_fill)) = insert_split_spill(&xs, ip, x);

    assert_eq!(left, [2, 4, 6, 0, 0, 0]);
    assert_eq!(left_fill, 3);

    assert_eq!(mid, 8);

    assert_eq!(right, [10, 12, 13, 0, 0, 0]);
    assert_eq!(right_fill, 3);
}


#[test]
fn test_shift() {
    let mut xs = [1, 2, 3];
    shift(&mut xs, 92);
    assert_eq!(xs, [92, 1, 2]);
}


#[test]
fn test_insert_split_start() {
    let mut xs = [1, 2, 3, 4, 5, 6, 7];
    let x = 0;
    let ip = 0;
    let (left, right) = insert_split(&mut xs, ip, x);

    assert_eq!(left, [0, 1, 2, 3, 0, 0, 0]);
    assert_eq!(right, [4, 5, 6, 7, 0, 0, 0]);
}


#[test]
fn test_insert_split_after_first() {
    let mut xs = [2, 4, 6, 8, 10, 12, 14];
    let x = 3;
    let ip = 1;
    let (left, right) = insert_split(&mut xs, ip, x);

    assert_eq!(left, [2, 3, 4, 6, 0, 0, 0]);
    assert_eq!(right, [8, 10, 12, 14, 0, 0, 0]);
}


#[test]
fn test_insert_split_mid() {
    let mut xs = [2, 4, 6, 8, 10, 12, 14];
    let x = 7;
    let ip = 3;
    let (left, right) = insert_split(&mut xs, ip, x);

    assert_eq!(left, [2, 4, 6, 7, 0, 0, 0]);

    assert_eq!(right, [8, 10, 12, 14, 0, 0, 0]);
}


#[test]
fn test_insert_split_last() {
    let mut xs = [2, 4, 6, 8, 10, 12, 14];
    let x = 15;
    let ip = 7;
    let (left, right) = insert_split(&mut xs, ip, x);

    assert_eq!(left, [2, 4, 6, 8, 0, 0, 0]);
    assert_eq!(right, [10, 12, 14, 15, 0, 0, 0]);
}
