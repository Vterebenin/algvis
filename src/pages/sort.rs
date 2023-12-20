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
    active_step: u32,
    steps: VecDeque<(usize, i32)>,
    steps_time: f32,
}

impl Sorter {
    pub fn new(sort_config: &SortConfigValues) -> Sorter {
        Self {
            algorithm: SortingAlgorithmEnum::MergeSort,
            data: get_new_generation(&sort_config.items_count),
            active_step: 0,
            steps: VecDeque::new(),
            steps_time: 0.
        }
    }

    pub fn sort(&mut self, sort_config: &SortConfigValues) {
        let mut data = self.data.clone();
        self.steps = VecDeque::new();
        self.active_step = 0;

        // should be a computed algorithm by enum
        merge_sort(&mut data, &mut self.steps);

        self.steps_time = sort_config.time_overall as f32 / self.steps.len() as f32 * MS_IN_SECS;
    }

    pub fn _set_algorithm(&mut self, s: String) {
        self.algorithm = SortingAlgorithmEnum::from_string(s).unwrap_or(SortingAlgorithmEnum::MergeSort);
    }

    pub fn tick(&mut self) {
        let max_steps = self.steps.len() as u32;
        if self.active_step >= max_steps {
            // Clear interval when the end is reached.
            self.steps_time = 0.;
            return ();
        } 
        let step_increment = (MAX_REFRESH_RATE / self.steps_time).ceil() as u32;
        let new_step_index = self.active_step + step_increment;
        let new_step_index = if new_step_index >= max_steps {
            max_steps
        } else {
            new_step_index
        };
        get_output_by_step(&mut self.data, self.steps.clone(), new_step_index);
        self.active_step = new_step_index;
    }
    pub fn generate(&mut self, sort_config: &SortConfigValues) {
        self.data = get_new_generation(&sort_config.items_count);
        self.steps = VecDeque::new();
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
    let sorter: UseStateHandle<Sorter> = use_state(|| Sorter::new(&config));

    let _current_algorithm = {
        let config_value = (*config).clone();
        use_memo(|_| config_value.current_algorithm_name.clone(), config_value.clone())
    };


    {
        let sorter = sorter.clone();

        let steps_time_value = (*sorter).steps_time.clone();

        use_interval(move || {
            let mut sorter_value = (*sorter).clone();
            sorter_value.tick();
            sorter.set(sorter_value);
        }, steps_time_value.max(MAX_REFRESH_RATE) as u32);
    }

    let handle_sort = {
        let sorter = sorter.clone();
        let config = config.clone();

        Callback::from(move |_| {
            let mut sorter_value = (*sorter).clone();
            sorter_value.sort(&config);
            sorter.set(sorter_value);
        })
    };

    let handle_generate = {
        let sorter = sorter.clone();
        let config = (*config).clone();
        Callback::from(move |_| {
            let mut sorter_value = (*sorter).clone();
            sorter_value.generate(&config);
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
