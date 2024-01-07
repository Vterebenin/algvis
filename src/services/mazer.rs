use std::collections::VecDeque;

use crate::{components::sorting_page::sorting_config::SortConfigValues, maze_solver_algorithms::dfs::is_path_between};

use super::maze_generator::Maze;

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

#[derive(Clone, PartialEq)]
pub struct Mazer {
    pub steps: Vec<i32>,
    pub maze: Maze,
    pub size_x: usize,
    pub size_y: usize,
    pub path: Vec<(usize, usize)>,
    current_step: usize,
}

impl Mazer {
    pub fn new() -> Mazer {
        let width = 25;
        let height = 25;
        Self {
            steps: Vec::new(),
            current_step: 0,
            size_y: height,
            size_x: width,
            maze: Maze::new(width, height),
            path: Vec::new(),
        }
    }

    pub fn solve(&mut self) {
        let (path, _) = is_path_between(&self.maze, self.maze.entry, self.maze.exit);
        self.path = path.clone();
    }
}
