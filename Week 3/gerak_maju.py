from controller import Robot

# Inisialisasi robot e-puck
robot = Robot()

# Waktu langkah simulasi (ms)
TIME_STEP = 64

# Kecepatan robot
MAX_SPEED = 6.28

# Mendapatkan motor kiri dan kanan
left_motor = robot.getDevice('left wheel motor')
right_motor = robot.getDevice('right wheel motor')

# Set motor ke mode velocity
left_motor.setPosition(float('inf'))
right_motor.setPosition(float('inf'))

# Set kecepatan awal
left_motor.setVelocity(0.0)
right_motor.setVelocity(0.0)

# Loop simulasi
while robot.step(TIME_STEP) != -1:
    # Set motor untuk bergerak maju dengan kecepatan maksimum
    left_motor.setVelocity(MAX_SPEED)
    right_motor.setVelocity(MAX_SPEED)
