import yaml
from witmotion import IMU

with open("config.yaml", 'r') as config_file:
    config = yaml.safe_load(config_file)

bluetooth_address = config['bluetooth_address']
print("Bluetooth Address:", bluetooth_address)

imu = IMU()
imu.connect()
imu.start()

def trigger():
    return imu.get_acceleration() > 20

print("IMU connected and started.")
while True:
    try:
        print(imu.get_acceleration())
    except:
        print("Error occurred while reading IMU data.")
        break