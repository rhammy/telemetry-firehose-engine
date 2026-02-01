/*
You are given a 2D grid representing a factory floor.
Some cells are free, and some are obstacles.
The robot can move up, down, left, or right (no diagonal movement).
Task
Implement a class named RobotPathfinder with a method:
shortest_path_length(start, target)
This method should return the minimum number of steps required for the robot to move from start to target in the grid.
Requirements
•	Return the shortest number of steps.
•	If the robot cannot reach the target, return -1.
•	If start and target are the same, return 0.
•	You may design the class in any way you like (constructor, helper functions, etc.).
Example (for understanding only)
grid = [
   [0, 0, 1, 0],
   [1, 0, 1, 0],
   [0, 0, 0, 0],
   [0, 1, 1, 0]
]
 
 
start = (0, 0)
target = (2, 3)
 
 
finder = RobotPathfinder(grid)
result = finder.shortest_path_length(start, target)
*/
use std::collections::VecDeque;

pub struct RobotPathFinder {
    grid: Vec<Vec<i32>>,
}

impl RobotPathFinder {
    pub fn new(grid: Vec<Vec<i32>>) -> Self {
        Self { grid }
    }

    pub fn shortest_path_length(self, start: (isize, isize), end: (isize, isize)) -> i32 {
        // BFS
        let rows = self.grid.len();
        let cols = self.grid[0].len();
        let mut queue: VecDeque<(isize, isize, i32)> = VecDeque::new();
        // Faster than a hashset for looking for visited.
        let mut visited = vec![vec![false; rows]; cols];

        // enqueue the start position
        if start == end { return 0; }

        queue.push_back((start.0, start.1, 0));
        visited[start.0 as usize][start.1 as usize] = true;
        // up, down, left, right
        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        while let Some((row,col,dist)) = queue.pop_front() {
            println!("Popped {}, {} = {}", row, col, dist);
            if (row,col) == end {
                return dist;
            }
            for (dr, dc) in directions {
                let r = row + dr;
                let c = col + dc;
                // Bounds checking
                if r >= 0 && c >= 0 && r < rows as isize && c < cols as isize
                {
                    if !visited[r as usize][c as usize] &&self.grid[r as usize][c as usize] == 0 
                    {
                        visited[r as usize][c as usize] = true;
                        queue.push_back((r,c,dist+1));
                    }
                }
            }
        }
        -1
    }
}
