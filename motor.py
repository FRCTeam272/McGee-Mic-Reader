import motoron.motoron

mc = None
port = 1

def configure_Pi():
    global mc
    global port
    mc = motoron.motoron.MotoronI2C()
    mc.reinitialize()
    mc.disable_crc()
    mc.clear_reset_flag()
    try:
        mc.set_speed(port, 0)
    except:
        port = 2
        mc.set_speed(port, 0)
    
        
    mc.set_max_acceleration(port, 140)
    mc.set_max_deceleration(port, 300)
        
def go_forward():
    try:
        mc.set_speed(port, 100)
    except:
        configure_Pi()
        go_forward()

def stop():
    try:
        mc.set_speed(port, 0)
    except:
        configure_Pi()
        stop()

if __name__ == '__main__':
    configure_Pi()
    go_forward()
    stop()
    pass