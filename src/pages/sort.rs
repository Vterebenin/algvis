use std::cell::RefCell;
use std::collections::VecDeque;

use gloo_timers::callback::Interval;
use rand::seq::SliceRandom;
use rand::thread_rng;
use web_sys::console;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::components::sorting_graph_canvas::SortingGraphCanvas;
use crate::components::the_button::TheButton;
use crate::components::ui::the_input::TheInput;
use crate::helpers::parse_string_to_i32_or_default;
use crate::sorting_algorithms::merge_sort::merge_sort;

const MAX_ITEMS: i32 = 50;

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

    use_interval(move || {
        console::log_1(&format!("test").into());
    }, 1);
    let handle_sort = {
        let data = data.clone();
        let time_overall = time_overall.clone();
        let steps = steps.clone();
        Callback::from(move |_| {
            let mut items = (*data).clone();
            let mut alg_steps = VecDeque::new();
            merge_sort(&mut items, &mut alg_steps);

            let items = data.clone();
            let mut arr = (*data).clone();
            let time = *time_overall / alg_steps.len() as i32;
            let time = time as u32;
            let time = 0;
            let steps = steps.clone();
            let interval_value = Interval::new(time, move || {
                let item = alg_steps.pop_back();
                steps.set(alg_steps.clone());
                if item.is_some() {
                    let (index, val) = item.unwrap();
                    arr[index] = val;
                    items.set(arr.clone());
                }
            });
            interval_value.forget();
        })
    };
    {
        use_effect_with_deps(|v| {
            console::log_1(&format!("v: {:?}", v).into());
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
