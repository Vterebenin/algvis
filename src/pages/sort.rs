use std::collections::VecDeque;
use web_sys::{console};
use wasm_bindgen::prelude::*;

use rand::seq::SliceRandom;
use rand::thread_rng;
use yew::prelude::*;
use gloo_timers::callback::Interval;

use crate::components::sorting_graph::SortingGraph;
use crate::sorting_algorithms::merge_sort::merge_sort;

const MAX_ITEMS: i32 = 50;

#[function_component(Sort)]
pub fn sort() -> Html {
    let items_count = use_state(|| MAX_ITEMS.to_string());
    let change_items_count = {
        let items_count = items_count.clone();
        Callback::from(move |e: InputEvent| {
             let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
             items_count.set(input.value())
        })
    };
    let mut data: Vec<i32> = (1..=items_count.parse::<i32>().unwrap()).collect(); // Create a vector with numbers from 1 to 100

    let mut rng = thread_rng();
    data.shuffle(&mut rng);
    let data: UseStateHandle<Vec<i32>> = use_state(|| data);
    
    // todo: calculate based on time overall provided 
    let time = 10;

    let handle_sort = {
        let data = data.clone();
        Callback::from(move |_| {
            let mut items = (*data).clone();
            let mut steps = VecDeque::new();
            merge_sort(&mut items, &mut steps);

            let items = data.clone();
            let mut arr = (*data).clone();
            let interval = Interval::new(time, move || {
                let item = steps.pop_back();
                if item.is_some() {
                    let (index, val) = item.unwrap();
                    arr[index] = val;
                    items.set(arr.clone());
                }
            });
            interval.forget();
        })
    };
    let handle_shuffle = {
        let data = data.clone();
        Callback::from(move |_| {
            let mut rng = thread_rng();
            let mut items = (*data).clone();
            items.shuffle(&mut rng);
            data.set(items);
        })
    };
    html! {
        <>
            <div class="mx-auto flex-col justify-center items-center gap-6"> 
                <div class="flex justify-center gap-3">
                    <button onclick={handle_sort}>{ "Sort it!" }</button>
                    <button onclick={handle_shuffle}>{ "Shuffle!" }</button>
                    <input value={(*items_count).clone()} oninput={change_items_count} />
                    {(*items_count).clone()}
                </div>
                <SortingGraph data={(*data).clone()} />
            </div>
        </>
    }
}
