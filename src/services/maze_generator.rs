use std::ops::RangeInclusive;
use std::slice::Iter;

use rand::distributions::uniform::SampleRange;
use rand::distributions::uniform::SampleUniform;
use rand::Rng;

use crate::components::maze_page::maze_config::MazeConfigValues;
use crate::components::maze_page::maze_view_canvas::Coords;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Visited,
    Path,
    Empty,
    Wall,
    Entry,
    Exit,
}

impl Cell {
    pub fn iterator() -> Iter<'static, Cell> {
        static CELL_COLORS: [Cell; 6] = [
            Cell::Visited,
            Cell::Path,
            Cell::Empty,
            Cell::Wall,
            Cell::Entry,
            Cell::Exit,
        ];
        CELL_COLORS.iter()
    }
    pub fn as_name(&self) -> &'static str {
        match self {
            Cell::Empty => "Empty",
            Cell::Visited => "Visited",
            Cell::Path => "Path",
            Cell::Wall => "Wall",
            Cell::Entry => "Entry",
            Cell::Exit => "Exit",
        }
    }

    pub fn as_color(&self) -> &'static str {
        match self {
            Cell::Empty => "#f5f5f5",   
            Cell::Visited => "#ff9933",
            Cell::Path => "#a0e6f2",  
            Cell::Wall => "#cc0000", 
            Cell::Entry => "#ffd700",
            Cell::Exit => "#00ff00",
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Orientation {
    Horizontal,
    Vertical,
}

fn rand_num<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    rand::thread_rng().gen_range(range)
}
fn float_even(num: f32) -> f32 {
    (num / 2.0).ceil() * 2.0
}

fn generate_new_maze(width: usize, height: usize) -> Maze {
    let cells = vec![vec![Cell::Empty; width]; height];
    let mut maze = Maze {
        cells,
        width,
        height,
    };
    maze.generate_side_walls();
    maze.generate(1, width - 2, 1, height - 2);
    maze.set_default_entry_exit();
    maze
}

#[derive(Debug, Clone, PartialEq)]
pub struct Maze {
    pub cells: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Maze {
        generate_new_maze(width, height)
    }

    pub fn get_weight(&self) -> u32 {
        return 1;
    }

    pub fn reset(&mut self, config: &MazeConfigValues) {
        self.width = config.size;
        self.height = config.size;
        self.cells = vec![vec![Cell::Empty; self.width]; self.height];
        self.generate_side_walls();
        self.generate(1, self.width - 2, 1, self.height - 2);
        self.set_default_entry_exit();
    }

    fn get_first_coords_of(&self, cell_type: Cell) -> Coords<usize> {
        for (y, row) in self.cells.iter().enumerate() {
            for (x, col) in row.iter().enumerate() {
                if *col == cell_type {
                    return Coords::from(x, y);
                }
            }
        }
        Coords::from(0, 0)
    }
    pub fn exit(&self) -> Coords<usize> {
        self.get_first_coords_of(Cell::Exit)
    }

    pub fn entry(&self) -> Coords<usize> {
        self.get_first_coords_of(Cell::Entry)
    }

    fn generate_side_walls(&mut self) {
        for col in 0..self.width {
            self.cells[0][col] = Cell::Wall;
            self.cells[self.height - 1][col] = Cell::Wall;
        }
        for row in 1..self.height - 1 {
            self.cells[row][0] = Cell::Wall;
            self.cells[row][self.width - 1] = Cell::Wall;
        }
    }

    fn generate(&mut self, start_x: usize, width: usize, start_y: usize, height: usize) {
        let orientation = get_orientation(width, height);

        // Return when chamber is minimum size
        if orientation == Orientation::Horizontal && height <= 2
            || orientation == Orientation::Vertical && width <= 2
        {
            return;
        }

        let (from, to);
        let mut wall_points = Vec::new();
        let end = (start_x + width - 1, start_y + height - 1);

        match orientation {
            Orientation::Horizontal => {
                // Horizontal walls on even y-coordinates
                let range = start_y..start_y + height - 1;
                let y = float_even(rand_num(range) as f32) as usize;

                from = (start_x, y);
                to = (end.0, y);
                for x in start_x..=end.0 {
                    self.cells[y][x] = Cell::Wall;
                    wall_points.push((x, y));
                }
            }
            Orientation::Vertical => {
                // Vertical walls on even x-coordinates
                let range = start_x..start_x + width - 1;
                let x = float_even(rand_num(range) as f32) as usize;
                for y in start_y..=end.1 {
                    self.cells[y][x] = Cell::Wall;
                    wall_points.push((x, y));
                }
                from = (x, start_y);
                to = (x, end.1);
            }
        }

        let odd_wall_points = wall_points
            .iter()
            .filter(|coord| match orientation {
                // Passages off horizontal walls on odd x-coordinates
                Orientation::Horizontal => coord.0 % 2 != 0,
                // Passages off vertical walls on odd y-coordinates
                Orientation::Vertical => coord.1 % 2 != 0,
            })
            .collect::<Vec<&(usize, usize)>>();

        // Get random point from the vec of valid passage points
        if odd_wall_points.len() > 0 {
            let p_len = 0..(odd_wall_points.len());
            let passage = odd_wall_points[rand_num(p_len)];

            // // Remove the point from the wall to create a passage
            let passage_index = wall_points.iter().position(|coord| coord == passage);
            if let Some(passage_index) = passage_index {
                let point = wall_points[passage_index.min(wall_points.len() - 1)];
                self.cells[point.1][point.0] = Cell::Empty;
            }
        }

        match orientation {
            // If the wall is horizontal, recurse above and below
            Orientation::Horizontal => {
                // Top section
                self.generate(start_x, width, start_y, from.1 - start_y);
                // Bottom section
                self.generate(start_x, width, from.1 + 1, end.1 - from.1);
            }
            // If the wall is vertical, recurse left and right
            Orientation::Vertical => {
                // Left section
                self.generate(start_x, to.0 - start_x, start_y, height);
                // Bottom section
                self.generate(to.0 + 1, end.0 - to.0, start_y, height);
            }
        }
    }

    fn get_cell(&mut self, coords: Coords<usize>) -> Cell {
        let y = coords.y as usize;
        let x = coords.x as usize;
        self.cells[y][x]
    }

    fn modify_cell(&mut self, coords: Coords<usize>, value: Cell) -> Cell {
        let y = coords.y as usize;
        let x = coords.x as usize;
        self.cells[y][x] = value;
        self.cells[y][x]
    }

    fn set_default_entry_exit(&mut self) {
        let entry_point = Coords::from(2, 2);
        let exit_point = Coords::from(self.width - 3, self.height - 3);
        self.set_entry_exit(entry_point, exit_point);
    }

    fn set_entry_exit(&mut self, entry: Coords<usize>, exit: Coords<usize>) {
        self.modify_cell(entry, Cell::Entry);
        self.modify_cell(exit, Cell::Exit);
    }

    pub fn create_wall_or_empty(&mut self, coords: Coords<usize>) {
        let current = self.get_cell(coords);
        let next = if current == Cell::Wall {
            Cell::Empty
        } else {
            Cell::Wall
        };
        self.modify_cell(coords, next);
    }

    pub fn swap_cells(&mut self, cell1: Coords<usize>, cell2: Coords<usize>) {
        let old1 = self.get_cell(cell1);
        let old2 = self.get_cell(cell2);
        self.modify_cell(cell1, old2);
        self.modify_cell(cell2, old1);
    }

    pub fn change_entry(&mut self, entry: Coords<usize>) {
        let prev_entry = self.entry();
        if self.get_cell(entry) == Cell::Exit {
            self.swap_cells(entry, prev_entry);
            return;
        }
        self.modify_cell(prev_entry, Cell::Empty);
        self.modify_cell(entry, Cell::Entry);
    }

    pub fn change_exit(&mut self, exit: Coords<usize>) {
        let prev_exit = self.exit();
        if self.get_cell(exit) == Cell::Entry {
            self.swap_cells(exit, prev_exit);
            return;
        }
        self.modify_cell(prev_exit, Cell::Empty);
        self.modify_cell(exit, Cell::Exit);
    }
}

fn get_orientation(width: usize, height: usize) -> Orientation {
    if width < height {
        Orientation::Horizontal
    } else if height < width {
        Orientation::Vertical
    } else {
        if rand::thread_rng().gen_range::<u8, RangeInclusive<u8>>(0..=1) == 0 {
            Orientation::Horizontal
        } else {
            Orientation::Vertical
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::maze_solver_algorithms::dfs::is_path_between;

    use super::*;

    #[test]
    fn test_maze_generation() {
        let width = 30;
        let height = 30;
        let maze = Maze::new(width, height);

        // Check that the maze has the correct dimensions
        assert_eq!(maze.cells.len(), height);
        for row in maze.cells.iter() {
            assert_eq!(row.len(), width);
        }

        // Check that there is a path from entry to exit
        let (_path, has_path, ..) = is_path_between(&maze, maze.entry(), maze.exit());
        assert!(has_path);
    }
}
