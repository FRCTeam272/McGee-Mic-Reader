# McGee-Mic-Reader

Short and sweet this project takes the import from a mic plugged into a raspi and uses it to generate a signal to fire 
a motor

## Installing

This guy runs on rust with minimal use of External Libraries make sure your pi is connected to the internet

```bash
git clone https://github.com/FRCTeam272/McGee-Mic-Reader
cd McGee-Mic-Reader
cargo build
```
`cargo build` should grab all necessary dependency's

## Executing

to actually run the application use `cargo run`

I'd recommend setting it as a boot program  as well
follow [this guide](https://www.dexterindustries.com/howto/run-a-program-on-your-raspberry-pi-at-startup/) 
to configure your pi I'd recommend the rc.local method

## Electronics

@Todo this section is coming as the project progresses

## Using

For the first 10 seconds the program builds a sample rate, during this time the motors will be dead
on the 11th second the bot will begin to create its current sample
speaking, beatboxing, making noise of any kind into a mic can trigger the motor