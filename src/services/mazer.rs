use std::collections::VecDeque;

use crate::{components::{sorting_page::sorting_config::SortConfigValues, maze_page::maze_view_canvas::Coords}, maze_solver_algorithms::dfs::is_path_between, helpers::MAX_REFRESH_RATE};

use super::maze_generator::{Maze, Cell};

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
            },
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

    pub fn do_sort(&self, sort_config: &SortConfigValues, items: &mut Vec<i32>, steps: &mut VecDeque<RunType>) {
        (MazeAlgorithm::from(sort_config))(items, steps)
    }

    pub fn from(sort_config: &SortConfigValues) -> fn(&mut Vec<i32>, &mut VecDeque<RunType>) {
        let result = MazeAlgorithmsEnum::from_string(sort_config.current_algorithm_name.clone());
        match result {
            Ok(v) => MazeAlgorithm::from_enum(v),
            Err(_) => MazeAlgorithm::from_enum(MazeAlgorithmsEnum::Dijkstra)
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
    initial_cells: Vec<Vec<Cell>>,
    current_step: usize,
    active_step: u32,
    is_playing: bool,
}

impl Mazer {
    pub fn new() -> Mazer {
        let width = 35;
        let height = 35;
        Self {
            steps: VecDeque::new(),
            current_step: 0,
            active_step: 0,
            steps_time: 10.,
            is_playing: true,
            height,
            width,
            maze: Maze::new(width, height),
            path: Vec::new(),
            visited: Vec::new(),
            initial_cells: Vec::new(),
        }
    }

    pub fn solve(&mut self) {
        self.initial_cells = self.maze.cells.clone();
        let (path, _, visited, steps) = is_path_between(&self.maze, self.maze.entry(), self.maze.exit());
        self.path = path;
        self.steps = steps;
        self.visited = visited;
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
        self.maze.cells = self.get_output_by_step(new_step_index);
        self.active_step = new_step_index;
    }

    fn get_output_by_step(&mut self, step: u32) -> Vec<Vec<Cell>> {
        let mut steps = self.steps.clone();
        let mut data = self.initial_cells.clone();
        for _ in 0..step {
            let MazeStep { coords, cell_type } = steps.pop_back().unwrap();
            data[coords.y][coords.x] = cell_type;
        }
        data
    }
}
