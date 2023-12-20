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

#[derive(Clone, PartialEq)]
enum SortingAlgorithmEnum {
    MergeSort,
    BubbleSort,
}

impl SortingAlgorithmEnum {
    fn from_string(s: String) -> Result<SortingAlgorithmEnum, &'static str> {
        match s.as_str() {
            "merge_sort" => Ok(SortingAlgorithmEnum::MergeSort),
            "bubble_sort" => Ok(SortingAlgorithmEnum::BubbleSort),
            _ => Err("Invalid variant"),
        }
    }
}

#[derive(Clone, PartialEq)]
struct Sorter {
    algorithm: SortingAlgorithmEnum,
    data: Vec<i32>,
    steps: VecDeque<(usize, i32)>,
}

impl Sorter {
    pub fn new(items_count: &i32) -> Sorter {
        Self {
            algorithm: SortingAlgorithmEnum::MergeSort,
            data: get_new_generation(&items_count),
            steps: VecDeque::new(),
        }
    }
    pub fn sort(&mut self, sort_config: &SortConfigValues) -> f32 {
        let mut data = self.data.clone();
        self.steps = VecDeque::new();

        // should be a computed algorithm by enum
        merge_sort(&mut data, &mut self.steps);

        let time = sort_config.time_overall as f32 / self.steps.len() as f32 * MS_IN_SECS;
        time
    }
    pub fn set_algorithm(&mut self, s: String) {
        self.algorithm = SortingAlgorithmEnum::from_string(s).unwrap_or(SortingAlgorithmEnum::MergeSort);
    }
}



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

    let sorter: UseStateHandle<Sorter> = use_state(|| Sorter::new(&items_count));
    let steps_time: UseStateHandle<f32> = use_state(|| 0.0);
    let active_step_index: UseStateHandle<u32> = use_state(|| 0);

    {
        let sorter = sorter.clone();
        let mut data_value = sorter.data.clone();

        let steps_value = (*sorter).steps.clone();
        let max_steps = steps_value.len() as u32;

        let steps_time = steps_time.clone();
        let steps_time_value = (*steps_time).clone();

        let active_step_index = active_step_index.clone();
        let active_step_index_value = *active_step_index;
        use_interval(move || {
            let mut sorter_value = (*sorter).clone();
            if active_step_index_value >= max_steps {
                // Clear interval when the end is reached.
                steps_time.set(0.);
                return ();
            } 
            let step_increment = (MAX_REFRESH_RATE / steps_time_value).ceil() as u32;
            let new_step_index = active_step_index_value + step_increment;
            let new_step_index = if new_step_index >= max_steps {
                max_steps
            } else {
                new_step_index
            };
            get_output_by_step(&mut data_value, steps_value.clone(), new_step_index);
            sorter_value.data = data_value.clone();
            sorter.set(sorter_value);
            active_step_index.set(new_step_index);
        }, steps_time_value.max(MAX_REFRESH_RATE) as u32);
    }

    let handle_sort = {
        let sorter = sorter.clone();
        let config = config.clone();

        Callback::from(move |_| {
            let mut sorter_value = (*sorter).clone();
            let time = sorter_value.sort(&config);
            sorter.set(sorter_value);
            steps_time.set(time);
            active_step_index.set(0);
        })
    };
    let handle_generate = {
        let sorter = sorter.clone();
        Callback::from(move |_| {
            let mut sorter_value = (*sorter).clone();
            sorter_value.data = get_new_generation(&items_count);
            sorter_value.steps = VecDeque::new();
            sorter.set(sorter_value);
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
            <SortingGraphCanvas data={(*sorter).data.clone()} />
        </div>
}
}
