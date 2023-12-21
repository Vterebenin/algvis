use std::collections::VecDeque;
use crate::components::sorting_page::sorting_config::SortConfigValues;
use crate::helpers::get_new_generation;
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

// test
#[derive(Clone, PartialEq)]
pub struct Sorter {
    pub data: Vec<i32>,
    algorithm: SortingAlgorithmEnum,
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
        self.data = self.get_output_by_step(new_step_index);
        self.active_step = new_step_index;
    }

    pub fn generate(&mut self, sort_config: &SortConfigValues) {
        self.data = get_new_generation(&sort_config.items_count);
        self.steps = VecDeque::new();
    }

    pub fn tick_time(&self) -> u32 {
        if self.steps_time == 0. {
            return 0
        }
        self.steps_time.max(MAX_REFRESH_RATE) as u32
    }

    fn get_output_by_step(&mut self, step: u32) -> Vec<i32> {
        let mut steps = self.steps.clone();
        let mut data = self.data.clone();
        for _ in 0..step {
            let (index, val) = steps.pop_back().unwrap();
            data[index] = val
        }
        data
    }
}
