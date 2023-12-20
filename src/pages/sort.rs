use std::collections::VecDeque;
use rand::seq::SliceRandom;
use rand::thread_rng;
use web_sys::console;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::components::sorting_page::sorting_graph_canvas::SortingGraphCanvas;
use crate::components::sorting_page::sorting_config::{SortingConfig, SortConfigValues};
use crate::components::ui::the_button::TheButton;
use crate::sorting_algorithms::merge_sort::merge_sort;

const MS_IN_SECS: f32 = 1000.;
const MAX_REFRESH_RATE: f32 = 33.33;

pub fn get_output_by_step(items: &mut [i32], steps: VecDeque<(usize, i32)>, step_index: u32) {
    let mut steps = steps.clone();
    for _ in 0..step_index {
        let (index, val) = steps.pop_back().unwrap();
        items[index] = val
    }
}

pub fn shuffle(mut data: Vec<i32>) -> Vec<i32> {
    let mut rng = thread_rng();
    data.shuffle(&mut rng);
    data
}
pub fn get_new_generation(items_count: &i32) -> Vec<i32> {
    shuffle((1..=*items_count).collect())
}

#[function_component(Sort)]
pub fn sort() -> Html {
    let config = use_state(|| SortConfigValues::new());
    let change_config = {
        let config = config.clone();
        Callback::from(move |value: SortConfigValues| {
            config.set(value)
        })
    };
    let _current_algorithm = {
        let config_value = (*config).clone();
        use_memo(|_| config_value.current_algorithm_name.clone(), config_value.clone())
    };
    let items_count = {
        let config_value = (*config).clone();
        use_memo(|_| config_value.items_count.clone(), config_value.clone())
    };
    let time_overall = {
        let config_value = (*config).clone();
        use_memo(|_| config_value.time_overall.clone(), config_value.clone())
    };

    let data: UseStateHandle<Vec<i32>> = use_state(|| get_new_generation(&items_count));
    let steps: UseStateHandle<VecDeque<(usize, i32)>> = use_state(|| VecDeque::new());
    let steps_time: UseStateHandle<f32> = use_state(|| 0.0);
    let active_step_index: UseStateHandle<u32> = use_state(|| 0);

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
                steps_time.set(0.);
            } else {
                let step_increment = (MAX_REFRESH_RATE / steps_time_value).ceil() as usize;
                let new_step_index = *active_step_index + step_increment as u32;
                let new_step_index = if new_step_index >= max as u32 {
                    max as u32
                } else {
                    new_step_index
                };
                console::log_1(&format!("test call").into());
                get_output_by_step(&mut arr, steps_value.clone(), new_step_index);
                items.set(arr.clone());
                active_step_index.set(new_step_index);
            }
        }, steps_time_value.max(MAX_REFRESH_RATE) as u32);
    }

    let handle_sort = {
        let steps = steps.clone();
        let data_value = (*data).clone();

        Callback::from(move |_| {
            let mut data_value = data_value.clone();
            let mut alg_steps = VecDeque::new();

            // should be a computed algorithm by enum
            merge_sort(&mut data_value, &mut alg_steps);

            let time = *time_overall as f32 / alg_steps.len() as f32 * MS_IN_SECS;

            steps.set(alg_steps);
            steps_time.set(time);
            active_step_index.set(0);
        })
    };
    let handle_generate = {
        let data = data.clone();
        Callback::from(move |_| {
            data.set(get_new_generation(&items_count));
            steps.set(VecDeque::new());
        })
    };

    html! {
        <div class="mx-auto flex justify-center items-center gap-6">
            <div class="flex flex-col justify-between gap-3 p-5 border-2 border-accent rounded-lg h-full">
                <SortingConfig value={(*config).clone()} on_change={change_config} />
                <div class="flex flex-col gap-2 my-5">
                    <TheButton onclick={handle_generate}>
                        {"Generate"}
                    </TheButton>
                    <TheButton onclick={handle_sort}>
                        {"Sort it"}
                    </TheButton>
                </div>
            </div>
            <SortingGraphCanvas data={(*data).clone()} />
        </div>
}
}
