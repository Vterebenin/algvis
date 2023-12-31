use std::collections::VecDeque;

use crate::services::sorter::SortType;

pub fn merge_sort<T: Copy + Clone + Ord>(items: &mut Vec<T>, steps: &mut VecDeque<SortType<T>>) {
    _merge_sort(items, steps, 0);
}

fn _merge_sort<T: Copy + Clone + PartialOrd>(
    items: &mut Vec<T>,
    mut steps: &mut VecDeque<SortType<T>>,
    start_i: usize,
) {
    if items.len() > 1 {
        let mid = items.len() / 2;
        let mut left_half = items[0..mid].to_vec();
        let mut right_half = items[mid..].to_vec();
        _merge_sort(&mut left_half, &mut steps, start_i);
        _merge_sort(&mut right_half, &mut steps, start_i + mid);
        *items = merge(left_half, right_half, &mut steps, start_i);
    }
}

fn merge<T: Copy + Clone + PartialOrd>(
    a: Vec<T>,
    b: Vec<T>,
    steps: &mut VecDeque<SortType<T>>,
    start_i: usize,
) -> Vec<T> {
    let size = a.len() + b.len();
    let mut merged: Vec<T> = Vec::with_capacity(size);

    let mut i = 0; // Idx for a
    let mut j = 0; // Idx for b

    // Loop through a and b, adding the smallest values between them to `merged`
    while i < a.len() && j < b.len() {
        if a[i] < b[j] {
            merged.push(a[i]);
            steps.push_front(SortType::Set(start_i + merged.len() - 1, a[i]));
            i += 1;
        } else {
            merged.push(b[j]);
            steps.push_front(SortType::Set(start_i + merged.len() - 1, b[j]));
            j += 1;
        }
    }

    // Add all remaining values
    while i < a.len() {
        merged.push(a[i]);
        steps.push_front(SortType::Set(start_i + merged.len() - 1, a[i]));
        i += 1;
    }
    while j < b.len() {
        merged.push(b[j]);
        steps.push_front(SortType::Set(start_i + merged.len() - 1, b[j]));
        j += 1;
    }

    merged
}
