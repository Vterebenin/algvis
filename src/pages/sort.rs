use std::collections::VecDeque;

use rand::seq::SliceRandom;
use rand::thread_rng;
use yew::prelude::*;
use gloo_timers::callback::Interval;

use crate::components::sorting_graph::SortingGraph;
use crate::sorting_algorithms::merge_sort::merge_sort;

#[function_component(Sort)]
pub fn sort() -> Html {
    let mut data: Vec<i32> = (1..=50).collect(); // Create a vector with numbers from 1 to 100

    let mut rng = thread_rng();
    data.shuffle(&mut rng);
    let data: UseStateHandle<Vec<i32>> = use_state(|| data);
    
    let time = 60;

    let sort_data = data.clone();
    let handle_sort = Callback::from(move |_| {
        let mut items = (*sort_data).clone();
        let mut steps = VecDeque::new();
        merge_sort(&mut items, &mut steps);

        let items = sort_data.clone();
        let mut arr = (*sort_data).clone();
        let interval = Interval::new(time, move || {
            let item = steps.pop_back();
            if item.is_some() {
                let (index, val) = item.unwrap();
                arr[index] = val;
                items.set(arr.clone());
            }
        });
        interval.forget();
    });
    let shuffle_data = data.clone();
    let handle_shuffle = Callback::from(move |_| {
        let mut rng = thread_rng();
        let mut items = (*shuffle_data).clone();
        items.shuffle(&mut rng);
        shuffle_data.set(items);
    });
    html! {
        <>
            <div class="mx-auto flex-col justify-center items-center gap-6"> 
                <div class="flex justify-center gap-3">
                    <button onclick={handle_sort}>{ "Sort it!" }</button>
                    <button onclick={handle_shuffle}>{ "Shuffle!" }</button>
                </div>
                <SortingGraph data={(*data).clone()} />
            </div>
        </>
    }
}
