use std::{
    cmp::Ordering,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use itertools::Itertools;
use ordered_float::OrderedFloat;
use rayon::prelude::*;

pub fn transform<T, F>(arr: &[T], transform: F) -> Vec<T>
where
    T: Copy + Default,
    F: Fn((usize, &T)) -> T,
{
    arr.iter().enumerate().map(transform).collect_vec()
}

pub fn add<T>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Add<Output = T> + Copy + Default,
{
    if arr1.len() != arr2.len() {
        panic!("Can't add two arrays with a different length");
    }

    transform(arr1, |(index, value)| *value + *arr2.get(index).unwrap())
}

pub fn subtract<T>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Sub<Output = T> + Copy + Default,
{
    if arr1.len() != arr2.len() {
        panic!("Can't subtract two arrays with a different length");
    }

    transform(arr1, |(index, value)| *value - *arr2.get(index).unwrap())
}

pub fn multiply<T>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Mul<Output = T> + Copy + Default,
{
    if arr1.len() != arr2.len() {
        panic!("Can't multiply two arrays with a different length");
    }

    transform(arr1, |(index, value)| *value * *arr2.get(index).unwrap())
}

pub fn divide<T>(arr1: &[T], arr2: &[T]) -> Vec<T>
where
    T: Div<Output = T> + Copy + Default,
{
    if arr1.len() != arr2.len() {
        panic!("Can't divide two arrays with a different length");
    }

    transform(arr1, |(index, value)| *value / *arr2.get(index).unwrap())
}

pub fn cumulate<T>(arr: &[T]) -> Vec<T>
where
    T: Sum + Copy + Default + AddAssign,
{
    let mut sum = T::default();

    arr.iter()
        .map(|value| {
            sum += *value;
            sum
        })
        .collect_vec()
}

pub fn last_x_sum<T>(arr: &[T], x: usize) -> Vec<T>
where
    T: Sum + Copy + Default + AddAssign + SubAssign,
{
    let mut sum = T::default();

    arr.iter()
        .enumerate()
        .map(|(index, value)| {
            sum += *value;

            if index >= x - 1 {
                sum -= *arr.get(index + 1 - x).unwrap()
            }

            sum
        })
        .collect_vec()
}

pub fn net_change<T>(arr: &[T], offset: usize) -> Vec<T>
where
    T: Copy + Default + Sub<Output = T>,
{
    transform(arr, |(index, value)| {
        let previous = {
            if let Some(previous_index) = index.checked_sub(offset) {
                *arr.get(previous_index).unwrap()
            } else {
                T::default()
            }
        };

        *value - previous
    })
}

pub fn median(arr: &[f32], size: usize) -> Vec<Option<f32>>
// where
//     T: Copy + Default + Add<Output = T> + Ord,
{
    let even = size % 2 == 0;
    let median_index = size / 2;

    if size < 3 {
        panic!("Computing a median for a size lower than 3 is useless");
    }

    arr.par_iter()
        .enumerate()
        .map(|(index, _)| {
            if index >= size - 1 {
                let mut arr = arr[index - (size - 1)..index + 1]
                    .iter()
                    .map(|value| OrderedFloat(*value))
                    .collect_vec();

                arr.sort_unstable();

                if even {
                    Some(
                        **arr.get(median_index).unwrap()
                            + **arr.get(median_index - 1).unwrap() / 2.0,
                    )
                } else {
                    Some(**arr.get(median_index).unwrap())
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
}
