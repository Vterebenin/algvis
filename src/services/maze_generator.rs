use crate::helpers::shuffle;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Empty,
    Wall,
    Entry,
    Exit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Maze {
    pub cells: Vec<Vec<Cell>>,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        let mut cells = vec![vec![Cell::Wall; width]; height];
        Maze::generate(&mut cells, 1, 1);
        let mut maze = Maze {
            cells,
        };
        maze.set_entry_exit();
        maze
    }

    fn generate(cells: &mut Vec<Vec<Cell>>, x: i32, y: i32) {
        cells[y as usize][x as usize] = Cell::Empty;

        let directions = vec![(2, 0), (-2, 0), (0, 2), (0, -2)];
        let directions = shuffle::<(i32, i32)>(directions);

        for (dx, dy) in directions.iter() {
            let nx = x.wrapping_add(*dx);
            let ny = y.wrapping_add(*dy);

            let fit = ny > 0 && ny < cells.len() as i32 && nx > 0 && nx < cells[ny as usize].len() as i32;
            if fit && cells[ny as usize][nx as usize] == Cell::Wall {
                let row = (y + dy / 2) as usize;
                let col = (x + dx / 2) as usize;
                cells[row][col] = Cell::Empty;
                Maze::generate(cells, nx, ny);
            }
        }
    }

    fn set_entry_exit(&mut self) {
        self.cells[0][1] = Cell::Entry;
        let height = self.cells.len();
        let col = self.cells[height - 2].len() - 2;
        self.cells[height - 1][col] = Cell::Exit;
    }

    fn display(&self) {
        for row in &self.cells {
            for cell in row {
                let symbol = match cell {
                    Cell::Empty => ' ',
                    Cell::Wall => '1',
                    Cell::Entry => 'E',
                    Cell::Exit => 'X',
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maze_generation() {
        let width = 30;
        let height = 30;
        let mut maze = Maze::new(width, height);

        // Check that the entry and exit points are correctly set
        assert_eq!(maze.cells[0][1], Cell::Entry);
        let exit_row = maze.cells.len() - 1;
        let exit_col = maze.cells[exit_row].len() - 2;
        assert_eq!(maze.cells[exit_row][exit_col], Cell::Exit);
        maze.display();

        // Check that the maze has the correct dimensions
        assert_eq!(maze.cells.len(), height);
        for row in maze.cells.iter() {
            assert_eq!(row.len(), width);
        }

        // Check that there is a path from entry to exit
        let has_path = is_path_between(&maze, (0, 1), (exit_row, exit_col));
        assert!(has_path);
    }

    fn is_path_between(maze: &Maze, start: (usize, usize), end: (usize, usize)) -> bool {
        let mut visited = vec![vec![false; maze.cells[0].len()]; maze.cells.len()];
        dfs_path_exists(maze, &mut visited, start, end)
    }

    fn dfs_path_exists(maze: &Maze, visited: &mut Vec<Vec<bool>>, current: (usize, usize), end: (usize, usize)) -> bool {
        let (row, col) = current;
        if current == end {
            return true;
        }

        visited[row][col] = true;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dr, dc) in directions.iter() {
            let nr = (row as isize + dr) as usize;
            let nc = (col as isize + dc) as usize;

            if nr < maze.cells.len() && nc < maze.cells[nr].len() && !visited[nr][nc] && maze.cells[nr][nc] != Cell::Wall {
                if dfs_path_exists(maze, visited, (nr, nc), end) {
                    return true;
                }
            }
        }

        false
    }
}

