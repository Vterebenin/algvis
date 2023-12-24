use std::collections::VecDeque;

use web_sys::console;

use crate::components::sorting_page::sorting_config::SortConfigValues;
use crate::helpers::get_new_generation;
use crate::sorting_algorithms::bubble_sort::bubble_sort;
use crate::sorting_algorithms::heap_sort::heap_sort;
use crate::sorting_algorithms::insertion_sort::insertion_sort;
use crate::sorting_algorithms::merge_sort::merge_sort;
use crate::sorting_algorithms::quick_sort::quick_sort;
use crate::sorting_algorithms::shell_sort::shell_sort;

const MS_IN_SECS: f32 = 1000.;
const MAX_REFRESH_RATE: f32 = 33.33;

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum SortType<T> {
    Set(usize, T),
    Swap(usize, usize),
}

impl ToString for SortType<i32> {
    fn to_string(&self) -> String {
        match &self {
            SortType::Swap(idx1, idx2) => {
                if idx1 == idx2 && idx1 == &0 {
                    String::new()
                } else {
                    format!("Swap indexes {} and {}", idx1, idx2)
                }
            },
            SortType::Set(idx, value) => format!("Set index {} to {}", idx, value),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum SortingAlgorithmEnum {
    MergeSort,
    BubbleSort,
    HeapSort,
    QuickSort,
    InsertionSort,
    ShellSort,
    BucketSort,
}

impl SortingAlgorithmEnum {
    fn from_string(s: String) -> Result<SortingAlgorithmEnum, &'static str> {
        match s.as_str() {
            "merge_sort" => Ok(SortingAlgorithmEnum::MergeSort),
            "bubble_sort" => Ok(SortingAlgorithmEnum::BubbleSort),
            "heap_sort" => Ok(SortingAlgorithmEnum::HeapSort),
            "quick_sort" => Ok(SortingAlgorithmEnum::QuickSort),
            "insertion_sort" => Ok(SortingAlgorithmEnum::InsertionSort),
            "shell_sort" => Ok(SortingAlgorithmEnum::ShellSort),
            "bucket_sort" => Ok(SortingAlgorithmEnum::BucketSort),
            _ => Err("Invalid variant"),
        }
    }
}

struct SortAlgorithm;

impl SortAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn do_sort(&self, sort_config: &SortConfigValues, items: &mut Vec<i32>, steps: &mut VecDeque<SortType<i32>>) {
        (SortAlgorithm::from(sort_config))(items, steps)
    }

    pub fn from(sort_config: &SortConfigValues) -> fn(&mut Vec<i32>, &mut VecDeque<SortType<i32>>) {
        let result = SortingAlgorithmEnum::from_string(sort_config.current_algorithm_name.clone());
        match result {
            Ok(v) => SortAlgorithm::from_enum(v),
            Err(_) => SortAlgorithm::from_enum(SortingAlgorithmEnum::MergeSort)
        }
    }

    fn from_enum(enum_value: SortingAlgorithmEnum) -> fn(&mut Vec<i32>, &mut VecDeque<SortType<i32>>) {
        match enum_value {
            SortingAlgorithmEnum::MergeSort => merge_sort::<i32>,
            SortingAlgorithmEnum::BubbleSort => bubble_sort::<i32>,
            SortingAlgorithmEnum::HeapSort => heap_sort::<i32>,
            SortingAlgorithmEnum::QuickSort => quick_sort::<i32>,
            SortingAlgorithmEnum::InsertionSort => insertion_sort::<i32>,
            SortingAlgorithmEnum::ShellSort => shell_sort::<i32>,
            SortingAlgorithmEnum::BucketSort => todo!(),
        }
    }
}

// test
#[derive(Clone, PartialEq)]
pub struct Sorter {
    pub data: Vec<i32>,
    algorithm: SortingAlgorithmEnum,
    pub active_step: u32,
    pub steps: VecDeque<SortType<i32>>,
    steps_time: f32,
    initial_data: Vec<i32>,
    pub is_playing: bool,
}

impl Sorter {
    pub fn new(sort_config: &SortConfigValues) -> Sorter {
        let generation = get_new_generation(&sort_config.items_count);
        let mut sorter = Self {
            algorithm: SortingAlgorithmEnum::MergeSort,
            data: generation.clone(),
            active_step: 0,
            steps: VecDeque::new(),
            steps_time: 0.,
            initial_data: generation,
            is_playing: false,
        };
        sorter.generate(sort_config);
        sorter
    }

    pub fn sort(&mut self, sort_config: &SortConfigValues) {
        let mut data = self.initial_data.clone();
        self.data = self.initial_data.clone();
        self.steps = VecDeque::new();
        self.active_step = 0;

        let algorithm = SortAlgorithm::new();
        algorithm.do_sort(sort_config, &mut data, &mut self.steps);
    }

    fn _set_algorithm(&mut self, s: String) {
        self.algorithm = SortingAlgorithmEnum::from_string(s).unwrap_or(SortingAlgorithmEnum::MergeSort);
    }

    fn reset(&mut self, sort_config: &SortConfigValues) {
        self.data = self.initial_data.clone();
        self.set_step(0);
        self.calculate_time(sort_config);
    }

    pub fn play(&mut self, sort_config: &SortConfigValues) {
        self.is_playing = true;
        if self.active_step as usize == self.steps.len() {
            self.reset(sort_config);
            return;
        }
        if self.is_playing == false && self.steps_time > 0. {
            return;
        }

        self.calculate_time(sort_config);
    }

    fn calculate_time(&mut self, sort_config: &SortConfigValues) {
        self.steps_time = sort_config.time_overall as f32 / self.steps.len() as f32 * MS_IN_SECS;
    }

    pub fn stop(&mut self) {
        self.is_playing = false;
    }

    pub fn tick(&mut self) {
        let max_steps = self.steps.len() as u32;
        if self.active_step >= max_steps {
            // Clear interval when the end is reached.
            self.steps_time = 0.;
            self.is_playing = false;
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
        self.initial_data = self.data.clone();
        self.sort(sort_config);
    }

    pub fn set_step(&mut self, step: u32) {
        self.data = self.get_output_by_step(step);
        self.active_step = step;
    }
    
    pub fn get_active_step_string(&self) -> String {
        self.active_step.to_string()
    }

    pub fn tick_time(&self) -> u32 {
        if self.steps_time == 0. || !self.is_playing {
            return 0
        }
        self.steps_time.max(MAX_REFRESH_RATE) as u32
    }

    fn get_output_by_step(&mut self, step: u32) -> Vec<i32> {
        let mut steps = self.steps.clone();
        let mut data = self.initial_data.clone();
        for _ in 0..step {
            match steps.pop_back().unwrap() {
                SortType::Set(index, val) => data[index] = val,
                SortType::Swap(index1, index2) => {
                    data.swap(index1, index2);
                },
            }
        }
        data
    }

    pub fn get_steps_len_string(&self) -> String {
        self.steps.len().to_string()
    }

    pub fn get_active_step_item(&self) -> SortType<i32> {
        match self.steps.get(self.steps.len() - self.active_step as usize) {
            Some(v) => *v,
            None => SortType::Swap(0, 0)
        }
    }
}
