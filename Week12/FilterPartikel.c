import numpy as np
import matplotlib.pyplot as plt

# Simulasi parameter
np.random.seed(42)  # Seed untuk hasil yang reproduktif
time_steps = 50  # Jumlah langkah waktu
actual_velocity = 1.0  # Kecepatan sebenarnya (konstan)
measurement_noise_std = 1.0  # Standar deviasi noise pengukuran
process_noise_std = 0.1  # Standar deviasi noise proses
num_particles = 100  # Jumlah partikel untuk Particle Filter

# Inisialisasi variabel untuk Particle Filter
actual_positions = [0]
measured_positions = []
filtered_positions_pf = []

particles = np.random.uniform(-10, 10, num_particles)  # Posisi awal partikel
weights = np.ones(num_particles) / num_particles  # Bobot awal partikel

# Fungsi resampling partikel
def resample_particles(particles, weights):
    indices = np.random.choice(range(len(particles)), size=len(particles), p=weights)
    return particles[indices]

for t in range(1, time_steps + 1):
    # Posisi sebenarnya (dengan noise proses)
    actual_positions.append(actual_positions[-1] + actual_velocity + np.random.normal(0, process_noise_std))

    # Pengukuran posisi (dengan noise pengukuran)
    measured_positions.append(actual_positions[-1] + np.random.normal(0, measurement_noise_std))

    # Particle Filter
    # Prediction step
    particles += actual_velocity + np.random.normal(0, process_noise_std, num_particles)

    # Update step (menggunakan pengukuran)
    weights *= np.exp(-0.5 * ((measured_positions[-1] - particles) / measurement_noise_std) ** 2)
    weights += 1e-300  # Hindari pembagian dengan nol
    weights /= sum(weights)  # Normalisasi

    # Resample step
    particles = resample_particles(particles, weights)

    # Estimasi posisi berdasarkan rata-rata bobot partikel
    filtered_positions_pf.append(np.average(particles, weights=weights))

# Visualisasi hasil
plt.figure(figsize=(10, 6))
plt.plot(range(time_steps + 1), actual_positions, label="Posisi Sebenarnya", color="green", linewidth=2)
plt.scatter(range(1, time_steps + 1), measured_positions, label="Pengukuran", color="red", alpha=0.6)
plt.plot(range(1, time_steps + 1), filtered_positions_pf, label="Posisi Setelah Filtering (Particle Filter)", color="orange", linewidth=2)
plt.xlabel("Waktu (step)")
plt.ylabel("Posisi")
plt.title("Simulasi Estimasi Posisi Robot dengan Particle Filter")
plt.legend()
plt.grid()
plt.show()
