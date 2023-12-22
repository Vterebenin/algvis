use std::collections::VecDeque;

use crate::services::sorter::SortType;

pub fn insertion_sort<T: Copy + Clone + Ord>(arr: &mut Vec<T>, steps: &mut VecDeque<SortType<T>>) {
    let len = arr.len();

    for i in 1..len {
        let mut j = i;

        while j > 0 && arr[j] < arr[j - 1] {
            arr.swap(j, j - 1);
            steps.push_front(SortType::Swap(j, j - 1));
            j -= 1;
        }
    }
}
