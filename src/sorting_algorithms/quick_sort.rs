use std::collections::VecDeque;

use crate::services::sorter::SortType;

pub fn quick_sort<T: Copy + Clone + Ord>(
    arr: &mut Vec<T>, 
    steps: &mut VecDeque<SortType<T>>, 
) {
    _quick_sort(arr, steps, 0)
}

pub fn _quick_sort<T: Copy + Clone + Ord>(
    arr: &mut Vec<T>, 
    steps: &mut VecDeque<SortType<T>>, 
    divide_index: usize
) {
    let len = arr.len();
    if len < 2 {
        return; // Already sorted if the array has 0 or 1 element
    }

    let pivot_index = partition(arr, steps, divide_index);
    _quick_sort(&mut arr[0..pivot_index].to_vec(), steps, divide_index);
    _quick_sort(&mut arr[pivot_index + 1..].to_vec(), steps, divide_index + pivot_index + 1);
}

fn partition<T: Ord>(arr: &mut Vec<T>, steps: &mut VecDeque<SortType<T>>, divide_index: usize) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;

    arr.swap(pivot_index, len - 1);
    steps.push_front(SortType::Swap(divide_index + pivot_index, divide_index + len - 1));

    let mut i = 0;
    for j in 0..len - 1 {
        if arr[j] <= arr[len - 1] {
            arr.swap(i, j);
            steps.push_front(SortType::Swap(divide_index + i, divide_index + j));
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    steps.push_front(SortType::Swap(divide_index + i, divide_index + len - 1));
    i
}

