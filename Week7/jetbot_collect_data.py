import os
import os.path

from controller import Supervisor
import jetbot_train

# Buat instance robot
robot = Supervisor()

# Print instruksi
print('Collect data to build the training dataset for AI collision avoidance classifier\n'
      'Manually move the robot to the desired position and press:\n'
      '- "F": if the robot can safely move forward\n'
      '- "B": if the robot should turn\n'
      'Datasets images will be automatically stored in two different folders named "free" and "blocked".\n'
      'At least 20 images per category are needed.\n'
      'When dataset is ready, press "C" to compute the model.\n'
      'Copy the resulting "best_model.pth" file in in the "jetbot_collision_avoidance" controller directory to use the model.')

# Ambil waktu timestep dari dunia saat ini
timestep = int(robot.getBasicTimeStep())

# Aktifkan kamera
camera = robot.getDevice('camera')
camera.enable(timestep)

# Aktifkan input keyboard
keyboard = robot.getKeyboard()
keyboard.enable(timestep)

# Path absolut untuk dataset
base_dir = r'C:\Users\Asus\Downloads\tugas7 robotika\webots-projects-master\projects\nvidia-jetbot-collision-avoidance\images\dataset'
index = 1

# Loop utama
while robot.step(4 * timestep) != -1:
    key = keyboard.getKey()
    dir_name = base_dir
    if not os.path.isdir(dir_name):
        os.mkdir(dir_name)
    if key == ord('F'):
        dir_name = os.path.join(base_dir, 'free')
    elif key == ord('B'):
        dir_name = os.path.join(base_dir, 'blocked')
    elif key == ord('C'):
        print('Start training model.\nThis could take a while...')
        robot.step(timestep)
        jetbot_train.train()
        print('Trained model ready.')
        continue
    else:
        continue

    if not os.path.isdir(dir_name):
        os.mkdir(dir_name)

    path = os.path.join(dir_name, 'image' + str(index) + '.jpg')
    camera.saveImage(path, 100)
    print(f"Image saved at {path}")
    index += 1
