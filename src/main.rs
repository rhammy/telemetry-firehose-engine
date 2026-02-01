mod bfs;
use crate::bfs::RobotPathFinder;

fn main() {
    println!("Hello, world!");
    let grid = vec![
            vec![0, 0, 1, 0],
            vec![1, 0, 1, 0],
            vec![0, 0, 0, 0],
            vec![0, 1, 1, 0]
        ];
    let finder = RobotPathFinder::new(grid);
    // let result = finder.shortest_path_length((0,0), (2,3));
    let result2 = finder.shortest_path_length((0,3), (3,0));
    println!("Result {}", result2);
}
