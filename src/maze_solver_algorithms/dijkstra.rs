use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;
use std::collections::VecDeque;
use web_sys::console;

use crate::services::mazer::MazeSolverReturnType;
use crate::{services::{maze_generator::{Maze, Cell}, mazer::MazeStep}, components::maze_page::maze_view_canvas::Coords};

#[derive(Debug, PartialEq, Eq)]
struct DijkstraNode {
    coords: Coords<usize>,
    distance: u32,
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance) // Invert comparison for min-heap
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Function to find the shortest path through the maze using Dijkstra's algorithm
pub fn dijkstra(
    maze: &Maze,
    start: Coords<usize>,
    end: Coords<usize>,
    maze_path: &mut Vec<(usize, usize)>,
    steps: &mut VecDeque<MazeStep>,
) -> (Option<Vec<Coords<usize>>>, Option<u32>) {
    let mut distance = vec![vec![u32::MAX; maze.cells[0].len()]; maze.cells.len()];
    let mut path = vec![vec![Coords::from(0, 0); maze.cells[0].len()]; maze.cells.len()];

    let mut min_heap = BinaryHeap::new();
    distance[start.y][start.x] = 0;
    min_heap.push(DijkstraNode {
        coords: start,
        distance: 0,
    });
    steps.push_front(MazeStep {
        coords: start,
        cell_type: Cell::Visited,
    });

    while let Some(DijkstraNode { coords, distance: dist }) = min_heap.pop() {
        if coords == end {
            // Reconstruct path
            let mut current = coords;
            let mut path_coords = VecDeque::new();
            while current != start {
                path_coords.push_front(current);
                steps.push_front(MazeStep {
                    coords: current,
                    cell_type: Cell::Path,
                });
                maze_path.push((current.y, current.x));
                current = path[current.y][current.x];
            }
            path_coords.push_front(start);
            steps.push_front(MazeStep {
                coords: start,
                cell_type: Cell::Path,
            });
            maze_path.push((start.y, start.x));
            let path_vec: Vec<_> = path_coords.into_iter().collect();
            return (Some(path_vec), Some(dist));
        }

        for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let nx = coords.x as isize + *dx;
            let ny = coords.y as isize + *dy;
            if nx >= 0 && ny >= 0 && nx < maze.cells[0].len() as isize && ny < maze.cells.len() as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                let new_dist = dist + maze.get_weight(); // Adjust weight function accordingly
                if new_dist < distance[ny][nx] && maze.cells[ny][nx] != Cell::Wall {
                    distance[ny][nx] = new_dist;
                    path[ny][nx] = coords;
                    let new_coords = Coords::from(nx, ny);
                    steps.push_front(MazeStep {
                        coords: new_coords,
                        cell_type: Cell::Visited,
                    });
                    min_heap.push(DijkstraNode {
                        coords: new_coords,
                        distance: new_dist,
                    });
                }
            }
        }
    }

    (None, None) // No path found
}

pub fn solve_maze_by_dijkstra(
    maze: &Maze,
    start: Coords<usize>,
    end: Coords<usize>,
) -> MazeSolverReturnType {
    let mut path = vec![];
    let mut steps: VecDeque<MazeStep> = VecDeque::new();
    dijkstra(maze, start, end, &mut path, &mut steps);
    (path, steps)
}

