use std::collections::VecDeque;

use crate::{
    components::maze_page::{maze_config::MazeConfigValues, maze_view_canvas::Coords},
    helpers::{MAX_REFRESH_RATE, MS_IN_SECS},
    maze_solver_algorithms::{dfs::solve_maze_by_dfs, dijkstra::solve_maze_by_dijkstra},
};

use super::{maze_generator::{Cell, Maze}, playable::Playable};

#[derive(Clone, PartialEq, Debug)]
enum MazeAlgorithmsEnum {
    Dijkstra,
    DFS,
}

impl MazeAlgorithmsEnum {
    fn from_string(s: String) -> Result<MazeAlgorithmsEnum, &'static str> {
        match s.as_str() {
            "dijkstra" => Ok(MazeAlgorithmsEnum::Dijkstra),
            "dfs" => Ok(MazeAlgorithmsEnum::DFS),
            _ => Err("Invalid variant"),
        }
    }
}

pub type MazeSolverReturnType = (
    Vec<(usize, usize)>,
    VecDeque<MazeStep>,
);
pub type MazeSolverType = fn(&Maze, Coords<usize>, Coords<usize>) -> MazeSolverReturnType;

struct MazeAlgorithm;

impl MazeAlgorithm {
    pub fn run(
        config: &MazeConfigValues,
        maze: &Maze,
        start: Coords<usize>,
        end: Coords<usize>,
    ) -> MazeSolverReturnType {
        (MazeAlgorithm::from(config))(maze, start, end)
    }

    fn from(config: &MazeConfigValues) -> MazeSolverType {
        let result = MazeAlgorithmsEnum::from_string(config.current_algorithm_name.clone());
        match result {
            Ok(v) => MazeAlgorithm::from_enum(v),
            Err(_) => MazeAlgorithm::from_enum(MazeAlgorithmsEnum::Dijkstra),
        }
    }

    fn from_enum(enum_value: MazeAlgorithmsEnum) -> MazeSolverType {
        match enum_value {
            MazeAlgorithmsEnum::Dijkstra => solve_maze_by_dijkstra,
            MazeAlgorithmsEnum::DFS => solve_maze_by_dfs,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
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
            initial_cells: Vec::new(),
        }
    }

    pub fn show_path_line(&self) -> bool {
        !self.is_playing && self.active_step == self.get_final_step()
    }

    pub fn play_or_pause(&mut self, config: &MazeConfigValues) {
        if self.is_playing {
            self.stop();
        } else {
            self.play(config);
        }
    }

    pub fn generate_new_maze(&mut self, config: &MazeConfigValues) {
        self.width = config.size;
        self.height = config.size;
        self.maze.reset(config);
        self.is_playing = false;
        self.active_step = 0;
    }

    pub fn get_final_step(&self) -> u32 {
        self.max_steps().max(1) - 1
    }

    pub fn drop_cells(&mut self) {
        self.maze.cells = self.initial_cells.clone();
    }

    pub fn solve(&mut self, config: &MazeConfigValues) {
        self.initial_cells = self.maze.cells.clone();
        let (path, steps) =
            MazeAlgorithm::run(config, &self.maze, self.maze.entry(), self.maze.exit());
        self.steps = steps;
        self.path = path;
        self.set_step(self.get_final_step());
    }

    fn max_steps(&self) -> u32 {
        return self.steps.len() as u32;
    }
}

impl Playable<MazeConfigValues, MazeStep, Vec<Vec<Cell>>> for Mazer {
    fn reset(&mut self, config: &MazeConfigValues) {
        self.maze.reset(config);
        self.is_playing = false;
        self.set_step(self.get_final_step());
        self.calculate_time(config);
    }

    fn play(&mut self, config: &MazeConfigValues) {
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

    fn stop(&mut self) {
        self.is_playing = false;
        self.steps_time = 0.;
    }

    fn tick(&mut self) {
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

    fn set_step(&mut self, step: u32) {
        self.maze.cells = self.get_output_by_step(step);
        self.active_step = step;
    }

    fn get_active_step_string(&self) -> String {
        self.active_step.to_string()
    }

    fn tick_time(&self) -> u32 {
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
                if data[coords.y][coords.x] != Cell::Entry && data[coords.y][coords.x] != Cell::Exit {
                    data[coords.y][coords.x] = cell_type;
                }
            } else {
                break; // If steps are exhausted, stop updating data
            }
        }
        data
    }

    fn get_steps_len_string(&self) -> String {
        self.get_final_step().to_string()
    }

    fn get_active_step_item(&self) -> MazeStep {
        self.steps[self.active_step as usize]
    }

    fn calculate_time(&mut self, config: &MazeConfigValues) {
        self.steps_time = config.time_overall as f32 / self.steps.len() as f32 * MS_IN_SECS;
    }
}
