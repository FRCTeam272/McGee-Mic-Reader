import RPi.GPIO as GPIO
import yaml
config = yaml.safe_load(open("config.yaml"))

power = config['motor_power']

AN2 = 13
AN1 = 12
DIG2 = 24
DIG1 = 26

motor = ''
def configure_Pi():
    global motor
    global setup_has_run
    GPIO.setmode(GPIO.BCM)
    GPIO.setwarnings(False)
    GPIO.setup(AN2, GPIO.OUT)
    GPIO.setup(AN1, GPIO.OUT)
    GPIO.setup(DIG2, GPIO.OUT)
    GPIO.setup(DIG1, GPIO.OUT)
    try:
        motor = GPIO.PWM(AN2, GPIO.HIGH)
    except:
        motor = GPIO.PWM(AN1, GPIO.HIGH)

def go_forward():
    global motor
    try:
        motor.start(power)
    except:
        configure_Pi()
        motor.start(power)

def stop():
    global motor
    try:
        motor.start(0)
    except:
        configure_Pi()
        motor.start(0)
