use std::collections::VecDeque;
use rand::seq::SliceRandom;
use rand::thread_rng;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::components::sorting_graph_canvas::SortingGraphCanvas;
use crate::components::the_button::TheButton;
use crate::components::ui::the_input::TheInput;
use crate::helpers::parse_string_to_i32_or_default;
use crate::sorting_algorithms::merge_sort::merge_sort;

const MAX_ITEMS: i32 = 500;

pub fn get_output_by_step(items: &mut [i32], steps: VecDeque<(usize, i32)>, step_index: u32) {
    let mut steps = steps.clone();
    for _ in 0..step_index {
        let (index, val) = steps.pop_back().unwrap();
        items[index] = val
    }
}

#[function_component(Sort)]
pub fn sort() -> Html {
    let items_count = use_state(|| MAX_ITEMS);
    let change_items_count = {
        let items_count = items_count.clone();
        Callback::from(move |value: String| {
            let result = parse_string_to_i32_or_default(value, 0);
            items_count.set(result)
        })
    };

    let time_overall = use_state(|| 1000);
    let change_time_overall = {
        let time_overall = time_overall.clone();
        Callback::from(move |value: String| {
            let result = parse_string_to_i32_or_default(value, 0);
            time_overall.set(result)
        })
    };

    let mut data: Vec<i32> = (1..=*items_count).collect(); // Create a vector with numbers from 1 to 100

    let mut rng = thread_rng();
    data.shuffle(&mut rng);
    let data: UseStateHandle<Vec<i32>> = use_state(|| data);
    let steps: UseStateHandle<VecDeque<(usize, i32)>> = use_state(|| VecDeque::new());
    let steps_time: UseStateHandle<f32> = use_state(|| 0.0);
    let interval_ms: UseStateHandle<u32> = use_state(|| 0);
    let active_step_index: UseStateHandle<u32> = use_state(|| 0);
    let max_refresh_rate_ms: f32 = 33.33;

    {
        let items = data.clone();
        let mut arr = (*data).clone();

        let steps = steps.clone();
        let steps_value = (*steps).clone();

        let steps_time = steps_time.clone();
        let steps_time_value = (*steps_time).clone();

        let active_step_index = active_step_index.clone();
        use_interval(move || {
            let max = steps.len();
            if *active_step_index as usize >= max {
                // Clear interval when the end is reached.
                steps_time.set(0.0);
            } else {
                let step_increment = (max_refresh_rate_ms / steps_time_value).ceil() as usize;
                let new_step_index = *active_step_index + step_increment as u32;
                let new_step_index = if new_step_index >= max as u32 {
                    max as u32
                } else {
                    new_step_index
                };
                get_output_by_step(&mut arr, steps_value.clone(), new_step_index);
                items.set(arr.clone());
                active_step_index.set(new_step_index);
            }
        }, *interval_ms);
    }

    let handle_sort = {
        let data = data.clone();
        let time_overall = time_overall.clone();
        let steps_time = steps_time.clone();
        let interval_ms = interval_ms.clone();
        let steps = steps.clone();

        Callback::from(move |_| {
            let mut items = (*data).clone();
            let mut alg_steps = VecDeque::new();
            merge_sort(&mut items, &mut alg_steps);

            let time = *time_overall as f32 / alg_steps.len() as f32;

            steps.set(alg_steps);
            steps_time.set(time);
            interval_ms.set(time.max(max_refresh_rate_ms) as u32);
        })
    };
    {
        use_effect_with_deps(|v| {
        }, steps)
    }
    fn shuffle(mut data: Vec<i32>) -> Vec<i32> {
        let mut rng = thread_rng();
        data.shuffle(&mut rng);
        data
    }
    let handle_shuffle = {
        let data = data.clone();
        Callback::from(move |_| {
            let items = shuffle(data.to_vec());
            data.set(items);
        })
    };
    let update = {
        let data = data.clone();
        let items_count = items_count.clone();
        Callback::from(move |_| {
            let items: Vec<i32> = (1..=*items_count).collect();
            let items = shuffle(items);
            data.set(items);
        })
    };

    html! {
            <div class="mx-auto flex justify-center items-center gap-6">
                <div class="flex flex-col justify-between gap-3 p-5 border-2 border-sky-500 rounded-lg h-full">
                    <div>
                        <TheInput
                            label="Items Count"
                            value={items_count.to_string()}
                            set_value={change_items_count}
                        />
                        <TheInput
                            label="Time Overall"
                            value={time_overall.to_string()}
                            set_value={change_time_overall}
                        />
                    </div>
                    <div class="flex flex-col gap-2 my-5">
                        <TheButton onclick={update}>
                            {"Update"}
                        </TheButton>
                        <TheButton onclick={handle_sort}>
                            {"Sort it"}
                        </TheButton>
                        <TheButton onclick={handle_shuffle}>
                            {"Shuffle"}
                        </TheButton>
                    </div>
                </div>
                <SortingGraphCanvas data={(*data).clone()} />
            </div>
    }
}
