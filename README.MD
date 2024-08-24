# McGee-Mic-Reader

Short and sweet this project takes the import from a mic plugged into a raspi and uses it to generate a signal to fire 
a motor

## Installing

This guy runs on python with minimal use of External Libraries make sure your pi is connected to the internet

```bash
git clone https://github.com/FRCTeam272/McGee-Mic-Reader
cd McGee-Mic-Reader
pip install -r requirments.txt
```

make sure the following is also installed
```bash
sudo apt-get remove libportaudio2
sudo apt-get install libasound2-dev
git clone -b alsapatch https://github.com/gglockner/portaudio
cd portaudio
./configure && make
sudo make install
sudo ldconfig
cd ..
```

you will also have to restart the pi once all these scripts are installed
```bash
sudo shutdown -r -t 0
```

## Executing

to actually run the application use `chmod +x execute.sh && ./execute.sh`

I'd recommend setting it as a boot program  as well
follow [this guide](https://www.dexterindustries.com/howto/run-a-program-on-your-raspberry-pi-at-startup/) 
to configure your pi I'd recommend the rc.local method

## Electronics

We used a [Raspberry Pi 4b](https://www.raspberrypi.com/products/raspberry-pi-4-model-b/) as the computer, it should be noted that this software should work on any modern pi system

We also used a [Motoron M2H18v18 Dual High-Power Motor Controller for Raspberry Pi (No Connectors or Standoffs)](https://www.pololu.com/product/5050) from Pololu which slides right onto the pi

we run power through a 12 volt ac adapter into the Motoron Controller, and then run power from the motor control to an [FRC BAG Motor](https://www.vexrobotics.com/217-3351.html) 

the software will read from 1 and only 1 usb connected micrphone (decided at startup) when the user hits over a deciable level the motor will spin and the whatever it is rigged to (a drum pedal)

## Using

For the first 10 seconds the program builds a sample rate, during this time the motors will be dead
on the 11th second the bot will begin to create its current sample
speaking, beatboxing, making noise of any kind into a mic can trigger the motor