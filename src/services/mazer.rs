use std::collections::{HashMap, VecDeque};

use web_sys::console;

use crate::{
    components::{
        maze_page::{maze_config::MazeConfigValues, maze_view_canvas::Coords},
        sorting_page::sorting_config::SortConfigValues,
    },
    helpers::{MAX_REFRESH_RATE, MS_IN_SECS},
    maze_solver_algorithms::dfs::is_path_between,
};

use super::maze_generator::{Cell, Maze};

#[derive(Clone, PartialEq, Debug, Copy)]
pub enum RunType {
    Search(i32),
    Path(i32),
}

impl ToString for RunType {
    fn to_string(&self) -> String {
        match &self {
            RunType::Search(step) => {
                format!("todo")
            }
            RunType::Path(path_step) => format!("todo"),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum MazeAlgorithmsEnum {
    Dijkstra,
}

impl MazeAlgorithmsEnum {
    fn from_string(s: String) -> Result<MazeAlgorithmsEnum, &'static str> {
        match s.as_str() {
            "dijkstra" => Ok(MazeAlgorithmsEnum::Dijkstra),
            _ => Err("Invalid variant"),
        }
    }
}

struct MazeAlgorithm;

impl MazeAlgorithm {
    pub fn new() -> Self {
        Self
    }

    pub fn do_sort(
        &self,
        sort_config: &SortConfigValues,
        items: &mut Vec<i32>,
        steps: &mut VecDeque<RunType>,
    ) {
        (MazeAlgorithm::from(sort_config))(items, steps)
    }

    pub fn from(sort_config: &SortConfigValues) -> fn(&mut Vec<i32>, &mut VecDeque<RunType>) {
        let result = MazeAlgorithmsEnum::from_string(sort_config.current_algorithm_name.clone());
        match result {
            Ok(v) => MazeAlgorithm::from_enum(v),
            Err(_) => MazeAlgorithm::from_enum(MazeAlgorithmsEnum::Dijkstra),
        }
    }

    fn from_enum(enum_value: MazeAlgorithmsEnum) -> fn(&mut Vec<i32>, &mut VecDeque<RunType>) {
        match enum_value {
            MazeAlgorithmsEnum::Dijkstra => todo!(),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct MazeStep {
    pub coords: Coords<usize>,
    pub cell_type: Cell,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Mazer {
    pub maze: Maze,
    pub width: usize,
    pub height: usize,
    pub path: Vec<(usize, usize)>,
    pub visited: Vec<Vec<bool>>,
    pub steps: VecDeque<MazeStep>,
    pub steps_time: f32,
    pub is_playing: bool,
    pub active_step: u32,
    initial_cells: Vec<Vec<Cell>>,
}

impl Mazer {
    pub fn new(config: &MazeConfigValues) -> Mazer {
        let width = config.size;
        let height = config.size;
        Self {
            steps: VecDeque::new(),
            active_step: 0,
            steps_time: 0.,
            is_playing: false,
            width,
            height,
            maze: Maze::new(width, height),
            path: Vec::new(),
            visited: Vec::new(),
            initial_cells: Vec::new(),
        }
    }

    pub fn pause(&mut self) {
        self.is_playing = false;
        self.steps_time = 0.;
    }

    pub fn show_path_line(&self) -> bool {
        !self.is_playing && self.active_step == self.get_final_step()
    }

    pub fn play_or_pause(&mut self, config: &MazeConfigValues) {
        if self.is_playing {
            self.pause();
        } else {
            self.play(config);
        }
    }

    pub fn play(&mut self, config: &MazeConfigValues) {
        self.is_playing = true;
        if self.active_step == self.get_final_step() {
            self.set_step(0);
        }
        if self.active_step >= self.max_steps() {
            self.reset(config);
            return;
        }
        if self.steps_time > 0. {
            return;
        }

        self.calculate_time(config);
    }

    pub fn generate_new_maze(&mut self, config: &MazeConfigValues) {
        self.width = config.size;
        self.height = config.size;
        self.maze.reset(config);
        self.is_playing = false;
        self.active_step = 0;
    }

    pub fn reset(&mut self, config: &MazeConfigValues) {
        self.maze.reset(config);
        self.is_playing = false;
        self.set_step(self.get_final_step());
        self.calculate_time(config);
    }
    pub fn get_final_step(&self) -> u32 {
        self.max_steps().max(1) - 1
    }

    pub fn drop_cells(&mut self) {
        self.maze.cells = self.initial_cells.clone();
    }

    pub fn solve(&mut self) {
        self.initial_cells = self.maze.cells.clone();
        let (path, _, visited, steps) =
            is_path_between(&self.maze, self.maze.entry(), self.maze.exit());
        self.path = path;
        self.steps = steps;
        self.visited = visited;
        self.set_step(self.get_final_step());
    }

    fn max_steps(&self) -> u32 {
        return self.steps.len() as u32;
    }

    pub fn tick(&mut self) {
        let max_steps = self.max_steps();
        if self.active_step >= max_steps {
            self.steps_time = 0.;
            self.is_playing = false;
            self.set_step(self.get_final_step());
            return;
        }
        let step_increment = (MAX_REFRESH_RATE / self.steps_time).ceil() as u32;
        let new_step_index = self.active_step + step_increment;
        let new_step_index = if new_step_index >= max_steps {
            max_steps
        } else {
            new_step_index
        };
        self.set_step(new_step_index);
    }

    pub fn set_step(&mut self, step: u32) {
        self.maze.cells = self.get_output_by_step(step);
        self.active_step = step;
    }

    fn calculate_time(&mut self, config: &MazeConfigValues) {
        self.steps_time = config.time_overall as f32 / self.steps.len() as f32 * MS_IN_SECS;
    }

    pub fn tick_time(&self) -> u32 {
        if self.steps_time == 0. || !self.is_playing {
            return 0;
        }
        self.steps_time.max(MAX_REFRESH_RATE) as u32
    }

    fn get_output_by_step(&mut self, step: u32) -> Vec<Vec<Cell>> {
        let mut steps = self.steps.clone();
        let mut data = self.initial_cells.clone();
        for _ in 0..step {
            if let Some(MazeStep { coords, cell_type }) = steps.pop_back() {
                if data[coords.y][coords.x] != Cell::Entry {
                    data[coords.y][coords.x] = cell_type;
                }
            } else {
                break; // If steps are exhausted, stop updating data
            }
        }
        data
    }

    pub fn get_steps_len_string(&self) -> String {
        self.get_final_step().to_string()
    }
}
