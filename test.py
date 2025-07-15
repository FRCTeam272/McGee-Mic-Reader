from mbientlab.metawear import MetaWear, libmetawear, parse_value
from mbientlab.metawear.cbindings import *
from time import sleep
from datetime import now as datetime_now
import yaml

start_time = datetime_now()
with open("config.yaml", 'r') as config_file:
    config = yaml.safe_load(config_file)

DEVICE_ADDRESS = config['metawear_address']

class State:
    def __init__(self, device, trigger_func: function = None):
        self.device = device
        self.trigger_func = trigger_func
        # Define the function that will handle incoming data
        self.callback = FnVoid_VoidP_DataP(self.data_handler) # from cbindings

    # THIS IS THE CALLBACK FUNCTION
    def data_handler(self, ctx, data):
        """
        This function is called every time new gyroscope data arrives.
        """
        # Parse the data from the sensor to get usable values
        parsed_data = parse_value(data)
        
        # Get the angular velocity for the Z-axis
        angular_velocity_z = parsed_data.z

        print(f"{(datetime_now() - start_time).total_seconds():.2f} Gyro Z: {angular_velocity_z:.2f}")
        
        if abs(angular_velocity_z) > 150.0:
            self.trigger_custom_action()
            
    def trigger_custom_action(self):
        print("!! ACTION TRIGGERED: Fast rotation detected! !!")
        self.trigger_func() if self.trigger_func else print("triggered without a custom function")

if __name__ == "__main__":
    # Main connection and setup logic
    try:
        # Connect to the device
        device = MetaWear(DEVICE_ADDRESS)
        device.connect()
        print(f"Connected to {device.address}")
        
        state = State(device) #We'll need to pass a function here specifically one that triggers the motor to enable

        # this is a little inside baseball, the metawear has a bunch of sensors I picked the gyroscope arbitrarily but an accelerometer may be better
        # Get the gyroscope data signal
        gyro_signal = libmetawear.mbl_mw_gyro_bmi270_get_rotation_data_signal(state.device.board)

        # Subscribe to the gyroscope signal and link it to our data_handler
        libmetawear.mbl_mw_datasignal_subscribe(gyro_signal, None, state.callback)

        # Enable and start the gyroscope
        libmetawear.mbl_mw_gyro_bmi270_enable_rotation_sampling(state.device.board)
        libmetawear.mbl_mw_gyro_bmi270_start(state.device.board)

        # Keep the script running to receive data
        print("Streaming data... Press Ctrl+C to stop.")
        while True:
            sleep(.2)

    except Exception as e:
        print(f"Error: {e}")

    finally:
        # Stop the gyroscope and disconnect
        if 'device' in locals() and device.is_connected:
            print("Stopping sensor and disconnecting...")
            libmetawear.mbl_mw_gyro_bmi270_stop(state.device.board)
            libmetawear.mbl_mw_gyro_bmi270_disable_rotation_sampling(state.device.board)
            libmetawear.mbl_mw_datasignal_unsubscribe(gyro_signal)
            device.disconnect()
            print("Disconnected.")