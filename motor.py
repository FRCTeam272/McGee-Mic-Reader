import yaml
import motoron.motoron

def read_config(file_path: str):
    with open(file_path, 'r') as file:
        config = yaml.safe_load(file)
    return config

motor_power = read_config('config.yaml')['motor_power']
mc = None

def set_speed(speed: int):
    mc.set_speed(1, speed)
    mc.set_speed(2, speed)

def configure_Pi():
    global mc
    global port
    mc = motoron.motoron.MotoronI2C()
    mc.reinitialize()
    mc.disable_crc()
    mc.clear_reset_flag()
        
    mc.set_max_acceleration(1, 140)
    mc.set_max_deceleration(1, 300)


    mc.set_max_acceleration(2, 140)
    mc.set_max_deceleration(2, 300)

    set_speed(0)
    
        
def go_forward():
    try:
        set_speed(motor_power)
    except:
        configure_Pi()
        go_forward()

def stop():
    try:
        set_speed(0)
    except:
        configure_Pi()
        stop()

if __name__ == '__main__':
    configure_Pi()
    go_forward()
    stop()
    pass