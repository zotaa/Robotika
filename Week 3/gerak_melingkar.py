from controller import Robot

# Inisialisasi robot e-puck
robot = Robot()

# Waktu langkah simulasi (ms)
TIME_STEP = 64

# Kecepatan maksimum motor e-puck
MAX_SPEED = 6.28

# Mendapatkan motor kiri dan kanan
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Set motor ke mode velocity
left_motor.setPosition(float('inf'))
right_motor.setPosition(float('inf'))

# Set kecepatan awal dengan perbedaan yang signifikan antara roda kiri dan kanan
left_motor.setVelocity(0.2 * MAX_SPEED)  # Roda kiri lebih lambat (20% dari kecepatan maksimum)
right_motor.setVelocity(1.0 * MAX_SPEED) # Roda kanan dengan kecepatan maksimum

# Loop simulasi
while robot.step(TIME_STEP) != -1:
    # Robot akan terus bergerak melingkar
    pass
    
print(f"Kecepatan roda kiri: {left_motor.getVelocity()}, Kecepatan roda kanan: {right_motor.getVelocity()}")
