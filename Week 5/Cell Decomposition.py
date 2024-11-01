import matplotlib.pyplot as plt
from queue import Queue

# Fungsi untuk melakukan Cell Decomposition dan mencari jalur aman
def cell_decomposition(grid, start, goal):
    rows, cols = len(grid), len(grid[0])
    
    # Fungsi untuk mengecek apakah cell berada dalam grid dan bebas dari rintangan
    def is_valid(cell):
        x, y = cell
        return 0 <= x < rows and 0 <= y < cols and grid[x][y] == 0
    
    # BFS untuk menemukan jalur aman
    queue = Queue()
    queue.put(start)
    visited = {start}
    parent = {start: None}
    
    # Arah pergerakan (atas, bawah, kiri, kanan)
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    
    # Cari jalur
    while not queue.empty():
        current = queue.get()
        
        if current == goal:
            # Rekonstruksi jalur dari tujuan ke awal
            path = []
            while current:
                path.append(current)
                current = parent[current]
            path.reverse()
            return path
        
        # Periksa tetangga di empat arah
        for direction in directions:
            neighbor = (current[0] + direction[0], current[1] + direction[1])
            
            if is_valid(neighbor) and neighbor not in visited:
                visited.add(neighbor)
                parent[neighbor] = current
                queue.put(neighbor)

    # Jika tidak ada jalur yang ditemukan, kembalikan None
    return None

# Fungsi untuk menampilkan grid dan jalur yang ditemukan
def plot_grid_path(grid, path):
    plt.figure(figsize=(6, 6))
    for x in range(len(grid)):
        for y in range(len(grid[0])):
            if grid[x][y] == 1:
                plt.fill_between([y, y + 1], x, x + 1, color='black')
            else:
                plt.fill_between([y, y + 1], x, x + 1, color='white')
    
    if path:
        path_x, path_y = zip(*path)
        plt.plot([y + 0.5 for y in path_y], [x + 0.5 for x in path_x], 'ro-')
    
    plt.gca().invert_yaxis()
    plt.show()

# Grid contoh: 0 = jalan, 1 = rintangan
grid = [
    [0, 1, 0, 0, 0],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 1, 0],
    [0, 1, 1, 1, 0],
    [0, 0, 0, 0, 0]
]

# Titik awal dan tujuan
start = (0, 0)
goal = (4, 4)

# Cari jalur
path = cell_decomposition(grid, start, goal)

# Tampilkan hasil
if path:
    print("Jalur ditemukan:", path)
else:
    print("Tidak ada jalur yang tersedia.")

# Plot grid dan jalur
plot_grid_path(grid, path)
