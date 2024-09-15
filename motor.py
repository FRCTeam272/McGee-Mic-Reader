import RPi.GPIO as GPIO

# left as variables for documentation
CH1 = 26
CH2 = 20
CH3 = 21

# give me an array that I can loop over
CHANNELS = [CH1, CH2, CH3]

def configure_Pi():
    GPIO.setwarnings(False)
    GPIO.setmode(GPIO.BCM)
    for channel in CHANNELS:
        GPIO.setup(channel, GPIO.OUT)

    pass        
def stop():
    for channel in CHANNELS:
        control(channel, GPIO.LOW)
def go_forward():
    for channel in CHANNELS:
        control(channel, GPIO.HIGH)
def control(channel, source):
    print(f"Turning {channel} to {source}")
    GPIO.output(channel, source)
    
if __name__ == '__main__':
    import time
    configure_Pi()
    go_forward()
    time.sleep(5)
    stop()
    print("exited successfully")