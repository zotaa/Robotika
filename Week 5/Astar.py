import heapq

# Fungsi untuk menghitung heuristik (menggunakan jarak Manhattan)
def heuristic(node, goal):
    return abs(node[0] - goal[0]) + abs(node[1] - goal[1])

# Fungsi untuk mengimplementasikan Algoritma A*
def a_star(grid, start, goal):
    rows, cols = len(grid), len(grid[0])
    
    # Inisialisasi variabel untuk menyimpan jarak dan jalur
    open_set = []
    heapq.heappush(open_set, (0, start))
    came_from = {}
    g_score = {start: 0}
    f_score = {start: heuristic(start, goal)}

    # Arah pergerakan: atas, bawah, kiri, kanan
    directions = [(0, 1), (0, -1), (1, 0), (-1, 0)]
    
    while open_set:
        # Ambil node dengan nilai f_score terendah
        current = heapq.heappop(open_set)[1]

        # Jika telah mencapai tujuan, rekonstruksi jalur
        if current == goal:
            path = []
            while current in came_from:
                path.append(current)
                current = came_from[current]
            path.append(start)
            path.reverse()
            return path
        
        # Periksa tetangga dari node saat ini
        for direction in directions:
            neighbor = (current[0] + direction[0], current[1] + direction[1])
            
            # Abaikan jika keluar dari grid atau jika ada rintangan
            if (0 <= neighbor[0] < rows and 0 <= neighbor[1] < cols and grid[neighbor[0]][neighbor[1]] == 0):
                # Hitung g_score (biaya aktual)
                tentative_g_score = g_score[current] + 1
                
                if neighbor not in g_score or tentative_g_score < g_score[neighbor]:
                    # Perbarui g_score, f_score, dan jalur
                    came_from[neighbor] = current
                    g_score[neighbor] = tentative_g_score
                    f_score[neighbor] = tentative_g_score + heuristic(neighbor, goal)
                    heapq.heappush(open_set, (f_score[neighbor], neighbor))

    # Jika tidak ada jalur, kembalikan None
    return None

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

# Panggil fungsi a_star dan tampilkan hasilnya
path = a_star(grid, start, goal)
if path:
    print("Jalur ditemukan:", path)
else:
    print("Tidak ada jalur yang tersedia.")
