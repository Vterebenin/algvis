use std::collections::VecDeque;

use crate::services::sorter::SortType;

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize, steps: &mut VecDeque<SortType<T>>) {
    let mut largest = i;
    let left_child = 2 * i + 1;
    let right_child = 2 * i + 2;

    if left_child < n && arr[left_child] > arr[largest] {
        largest = left_child;
    }

    if right_child < n && arr[right_child] > arr[largest] {
        largest = right_child;
    }

    if largest != i {
        arr.swap(i, largest);
        steps.push_front(SortType::Swap(i, largest));
        heapify(arr, n, largest, steps);
    }
}

fn build_max_heap<T: Ord>(arr: &mut Vec<T>, steps: &mut VecDeque<SortType<T>>) {
    let n = arr.len();
    for i in (0..n / 2).rev() {
        heapify(arr, n, i, steps);
    }
}

pub fn heap_sort<T: Copy + Clone + Ord>(arr: &mut Vec<T>, steps: &mut VecDeque<SortType<T>>) {
    let n = arr.len();

    build_max_heap(arr, steps);

    for i in (0..n).rev() {
        arr.swap(0, i);
        steps.push_front(SortType::Swap(0, i));
        heapify(arr, i, 0, steps);
    }
}

