use std::collections::VecDeque;

use crate::services::sorter::SortType;

pub fn shell_sort<T: Copy + Clone + Ord>(arr: &mut Vec<T>, steps: &mut VecDeque<SortType<T>>) {
    let len = arr.len();
    let mut gap = len / 2;

    while gap > 0 {
        for i in gap..len {
            let mut j = i;
            let current_element = arr[i];

            while j >= gap && arr[j - gap] > current_element {
                arr[j] = arr[j - gap];
                steps.push_front(SortType::Set(j, arr[j - gap]));
                j -= gap;
            }

            arr[j] = current_element;
            steps.push_front(SortType::Set(j, current_element));
        }

        gap /= 2;
    }
}

