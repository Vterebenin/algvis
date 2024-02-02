use std::collections::VecDeque;

use crate::{services::{maze_generator::{Maze, Cell}, mazer::MazeStep}, components::maze_page::maze_view_canvas::Coords};

pub fn is_path_between(
    maze: &Maze,
    start: Coords<usize>,
    end: Coords<usize>,
) -> (Vec<(usize, usize)>, bool, Vec<Vec<bool>>, VecDeque<MazeStep>) {
    let mut visited = vec![vec![false; maze.cells[0].len()]; maze.cells.len()];
    let mut path = vec![];
    let mut steps: VecDeque<MazeStep> = VecDeque::new();
    let result = dfs_path_exists(maze, &mut visited, start, end, &mut path, &mut steps);
    (path, result, visited, steps)
}

pub fn dfs_path_exists(
    maze: &Maze,
    visited: &mut Vec<Vec<bool>>,
    current: Coords<usize>,
    end: Coords<usize>,
    path: &mut Vec<(usize, usize)>,
    steps: &mut VecDeque<MazeStep>,
) -> bool {
    let (row, col) = (current.y, current.x);
    if current == end {
        return true;
    }

    visited[row][col] = true;
    steps.push_front(MazeStep {
        coords: Coords::from(col, row),
        cell_type: Cell::Visited,
    });
    

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dr, dc) in directions.iter() {
        let nr = (row as isize + dr) as usize;
        let nc = (col as isize + dc) as usize;

        if nr < maze.cells.len()
            && nc < maze.cells[nr].len()
            && !visited[nr][nc]
            && maze.cells[nr][nc] != Cell::Wall
        {
            if dfs_path_exists(maze, visited, Coords::from(nc, nr), end, path, steps) {
                path.push((row, col));
                steps.push_front(MazeStep {
                    coords: Coords::from(col, row),
                    cell_type: Cell::Path,
                });
                return true;
            }
        }
    }

    false
}
