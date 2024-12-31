use std::collections::VecDeque;

// Konstanta DIRECTIONS mendefinisikan arah gerakan dalam matriks: kanan, bawah, kiri, dan atas
const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

// Alias untuk pasangan koordinat (baris, kolom) dalam bentuk usize, memudahkan penggunaan tipe data
type Point = (usize, usize);

// Fungsi is_valid memeriksa apakah posisi (x, y) valid: berada dalam batas matriks, bukan rintangan, dan belum dikunjungi
fn is_valid(x: i32, y: i32, rows: usize, cols: usize, grid: &Vec<Vec<i32>>, visited: &Vec<Vec<bool>>) -> bool {
    x >= 0 && y >= 0 && x < rows as i32 && y < cols as i32 && grid[x as usize][y as usize] == 0 && !visited[x as usize][y as usize]
}

fn find_path(grid: Vec<Vec<i32>>, start: Point, goal: Point) -> Option<Vec<Point>> {
    let rows = grid.len();
    let cols = grid[0].len();

    // Membuat matriks 'visited' untuk melacak posisi yang sudah dikunjungi selama pencarian
    let mut visited = vec![vec![false; cols]; rows];
    // Menginisialisasi queue untuk BFS, digunakan untuk menyimpan posisi yang akan diperiksa
    let mut queue = VecDeque::new();
    // Matriks 'parent' menyimpan jejak posisi sebelumnya untuk merekonstruksi jalur setelah mencapai tujuan
    let mut parent = vec![vec![None; cols]; rows];

    queue.push_back(start);
    visited[start.0][start.1] = true;

    while let Some((x, y)) = queue.pop_front() {
        // Memeriksa apakah posisi saat ini adalah tujuan; jika iya, jalur direkonstruksi dan dikembalikan
        if (x, y) == goal {
            let mut path = vec![];
            let mut current = Some(goal);

            while let Some((cx, cy)) = current {
                path.push((cx, cy));
                current = parent[cx][cy];
            }

            path.reverse();
            return Some(path);
        }

        // Mengiterasi ke semua arah gerakan (kanan, bawah, kiri, atas) dari posisi saat ini
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
    // Inisialisasi matriks grid dengan 0 sebagai jalur bebas dan 1 sebagai rintangan
    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 1, 1, 1, 0],
        vec![0, 0, 0, 1, 0],
        vec![0, 1, 0, 0, 0],
        vec![0, 0, 0, 1, 0],
    ];

    let start = (0, 0); // Titik awal
    let goal = (4, 4); // Titik tujuan

    match find_path(grid, start, goal) {
        Some(path) => {
            // Mencetak jalur yang ditemukan dari titik awal ke tujuan jika ada
            println!("Path found: {:?}", path);
        }
        None => {
            // Mencetak pesan jika tidak ada jalur yang memungkinkan untuk mencapai tujuan
            println!("No path found.");
        }
    }
}
