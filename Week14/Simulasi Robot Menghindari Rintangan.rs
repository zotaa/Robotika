use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

type Point = (usize, usize);

fn is_valid(x: i32, y: i32, rows: usize, cols: usize, grid: &Vec<Vec<i32>>, visited: &Vec<Vec<bool>>) -> bool {
    x >= 0 && y >= 0 && x < rows as i32 && y < cols as i32 && grid[x as usize][y as usize] == 0 && !visited[x as usize][y as usize]
}

fn find_path(grid: Vec<Vec<i32>>, start: Point, goal: Point) -> Option<(Vec<Point>, usize)> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = vec![vec![false; cols]; rows];
    let mut queue = VecDeque::new();
    let mut parent = vec![vec![None; cols]; rows];

    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == goal {
            let mut path = vec![];
            let mut current = Some(goal);
            let mut steps = 0;

            while let Some((cx, cy)) = current {
                path.push((cx, cy));
                current = parent[cx][cy];
                steps += 1;
            }

            path.reverse();
            return Some((path, steps - 1));
        }

        for &(dx, dy) in &DIRECTIONS {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;

            if is_valid(nx, ny, rows, cols, &grid, &visited) {
                let nx = nx as usize;
                let ny = ny as usize;

                visited[nx][ny] = true;
                queue.push_back((nx, ny));
                parent[nx][ny] = Some((x, y));
            }
        }
    }

    None
}

fn main() {
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
    ];

    let start = (0, 0);
    let goal = (4, 4);

    match find_path(grid, start, goal) {
        Some((path, steps)) => {
            println!("Jalur ditemukan: {:?}", path);
            println!("Langkah yang diambil: {}", steps);
        }
        None => {
            println!("Tidak ada jalur yang ditemukan.");
        }
    }
}
