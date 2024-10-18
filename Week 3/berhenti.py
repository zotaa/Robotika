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

# Mendapatkan sensor proximity
proximity_sensors = []
for i in range(8):
    sensor_name = 'ps' + str(i)
    proximity_sensor = robot.getDevice(sensor_name)
    proximity_sensor.enable(TIME_STEP)
    proximity_sensors.append(proximity_sensor)

# Fungsi untuk mendeteksi objek di depan (sensor 0 dan 7 berada di depan robot)
def is_object_in_front():
    front_left_sensor = proximity_sensors[0].getValue()  # Sensor depan kiri
    front_right_sensor = proximity_sensors[7].getValue()  # Sensor depan kanan
    threshold = 80.0  # Ambang batas deteksi sensor proximity (bisa disesuaikan)
    
    return front_left_sensor > threshold or front_right_sensor > threshold

# Loop simulasi
while robot.step(TIME_STEP) != -1:
    # Jika ada objek di depan, hentikan robot
    if is_object_in_front():
        left_motor.setVelocity(0.0)
        right_motor.setVelocity(0.0)
    else:
        # Jika tidak ada objek, terus bergerak maju
        left_motor.setVelocity(MAX_SPEED)
        right_motor.setVelocity(MAX_SPEED)
