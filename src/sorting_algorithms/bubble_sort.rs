use std::collections::VecDeque;

use crate::services::sorter::SortType;

pub fn bubble_sort<T: Copy + Clone + Ord>(items: &mut Vec<T>, steps: &mut VecDeque<SortType<T>>) {
    _bubble_sort(items, steps)
}

fn _bubble_sort<T: Ord>(arr: &mut Vec<T>, steps: &mut VecDeque<SortType<T>>) {
    let n = arr.len();
    
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                // Swap arr[j] and arr[j + 1]
                arr.swap(j, j + 1);
                steps.push_front(SortType::Swap(j, j + 1));
            }
        }
    }
}
