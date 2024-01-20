use crate::{services::maze_generator::{Maze, Cell}, components::maze_page::maze_view_canvas::Coords};

pub fn is_path_between(
    maze: &Maze,
    start: Coords<usize>,
    end: Coords<usize>,
) -> (Vec<(usize, usize)>, bool) {
    let mut visited = vec![vec![false; maze.cells[0].len()]; maze.cells.len()];
    let mut path = vec![];
    let result = dfs_path_exists(maze, &mut visited, start, end, &mut path);
    (path, result)
}

pub fn dfs_path_exists(
    maze: &Maze,
    visited: &mut Vec<Vec<bool>>,
    current: Coords<usize>,
    end: Coords<usize>,
    path: &mut Vec<(usize, usize)>,
) -> bool {
    let (row, col) = (current.y, current.x);
    if current == end {
        return true;
    }

    visited[row][col] = true;

    let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for (dr, dc) in directions.iter() {
        let nr = (row as isize + dr) as usize;
        let nc = (col as isize + dc) as usize;

        if nr < maze.cells.len()
            && nc < maze.cells[nr].len()
            && !visited[nr][nc]
            && maze.cells[nr][nc] != Cell::Wall
        {
            if dfs_path_exists(maze, visited, Coords::from(nc, nr), end, path) {
                path.push((row, col));
                return true;
            }
        }
    }

    false
}