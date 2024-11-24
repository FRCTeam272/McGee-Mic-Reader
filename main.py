# import sound
import motor
import time
# import yaml
from witmotion import IMU

if __name__ == '__main__':
    imu = IMU()
    motor.configure_Pi()
    while True:
        acceleration = imu.get_acceleration()
        print(acceleration)
        if acceleration > 20:
            print("Shake detected")
            motor.go_forward()
        if acceleration < 5:
            print("Shake stopped")
            motor.stop()
            time.sleep(1)
    
            

# config = yaml.safe_load(open("config.yaml"))
# decible_diff = config['decible_diff']
# buffer_min = config['buffer_min']

# motor.configure_Pi()
# print("motor test power on")
# motor.go_forward()
# time.sleep(.1)
# print("motor test power off")
# motor.stop()
# time.sleep(2)
# print("end motor test")
# exit()

# if __name__ == '__main__':
#     print(config)
#     print("configuring the pi motor")
#     motor.configure_Pi()
#     print("configuring baseline of 10 secs")
#     base_line = sound.read_decibel_levels(44100, 10)
#     buffer = 0
#     print("starting life capture")
#     try:
#         while True:
#             test_line = sound.read_decibel_levels(44100, .2)
#             result = sound.compare(base_line, test_line)
#             if result > decible_diff and buffer < buffer_min:
#                 print(f"Noise detected {time.time()}")
#                 motor.go_forward()
#                 buffer += 1
#             else:
#                 print(f"Reseting buffer: {buffer}")
#                 motor.stop()
#                 buffer = 0
#     except:
#         motor.stop()
#         print("Exiting")
#         exit()
