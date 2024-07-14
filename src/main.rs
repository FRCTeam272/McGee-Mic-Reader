use std::collections::VecDeque;
use rust_gpiozero as pi;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::sample::{Sample};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::error;

const ON_PI: bool = false;

fn main() {
    setup_pi();
    // grab the default audio devices
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to get default input device");
    let config = device.default_input_config().expect("Failed to get default input config");

    // build the format as a float to represent the decidable, this are used for the configureation
    let sample_format = config.sample_format();
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    // this is some math stuff regarding audio inputs
    let max_amplitude = Arc::new(Mutex::new(0.0));

    let max_amplitude_clone = Arc::clone(&max_amplitude);
    let err_fn = |err| eprintln!("An error occurred on stream: {}", err);

    // build out streams
    let stream = match sample_format {
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                process_input(data, channels, sample_rate, &max_amplitude_clone);
            },
            err_fn,
            None
        ),
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data: &[i16], _: &cpal::InputCallbackInfo| {
                let data: Vec<f32> = data.iter().map(|&s| s.to_sample()).collect();
                process_input(&data, channels, sample_rate, &max_amplitude_clone);
            },
            err_fn,
            None
        ),
        cpal::SampleFormat::U16 => device.build_input_stream(
            &config.into(),
            move |data: &[u16], _: &cpal::InputCallbackInfo| {
                let data: Vec<f32> = data.iter().map(|&s| s.to_sample()).collect();
                process_input(&data, channels, sample_rate, &max_amplitude_clone);
            },
            err_fn,
            None
        ),
        _ => panic!("Unsupported sample format"),
    }
        .expect("Failed to build input stream");

    stream.play().expect("Failed to play stream");


    let mut last_print = Instant::now(); // timestamp of the most recent iteration of grab
    let mut base_line_container:Vec<f32> = vec![]; // array of baseline readings
    let mut base_line_average: f32 = 0.0; // eventually holds the average of the baseline
    let amount_to_capture: i32 = 100;

    let mut decidable_buffer: VecDeque<f32> = VecDeque::new(); // holds and updates the last 10, so we know to go when good
    let mut safe_range = 0.0; // safe range is +/- std of the baseline and is calculated later
    let mut bounce_inputs: usize = 0;
    const MAX_BOUNCE_SIZE: usize = 2;
    loop {
        // audio math I guess
        let amplitude = *max_amplitude.lock().unwrap();
        let db_level = if amplitude > 0.0 {
            20.0 * amplitude.log10()
        } else {
            f32::NEG_INFINITY
        };

        if last_print.elapsed() >= Duration::from_millis(100) { // only fires every 100 milliseconds

            if base_line_container.len() < amount_to_capture as usize{
                // first if statement will build the sample vec, update the last pull and then hop out of the loop
                println!("Running base_capture {}", base_line_container.len());
                last_print = Instant::now();
                if db_level == f32::NEG_INFINITY { continue } // thread failed to capture any audio
                base_line_container.push(db_level);
                continue
            } else if base_line_container.len() < (amount_to_capture + 1) as usize{
                // on the final iteration of the loop it also calcualtes all averages and std we'll need
                base_line_container.push(db_level);
                let sum: f32 = base_line_container.iter().sum();
                base_line_average =  sum / base_line_container.len() as f32;
                safe_range = calculate_variance(base_line_container.clone(), base_line_average).sqrt();
                last_print = Instant::now();
                continue
            }
            if decidable_buffer.len() < 10{
                // this final if will build the inital buffer
                println!("Building Buffer {}", decidable_buffer.len());
                _ = decidable_buffer.push_back(db_level);
                last_print = Instant::now();
                continue
            }
            // remove the front item
            _ = decidable_buffer.pop_front();
            // add the new db
            _ = decidable_buffer.push_back(db_level);

            // calculate the new decible average
            let sum: f32 = decidable_buffer.iter().sum();
            let avg_decidable =  sum / decidable_buffer.len() as f32;


            // decide if the robot should relax the motor or fire
            // execute robot if we are +/- 20 outside the selected decidable range
            let go_robot = avg_decidable.abs() > base_line_average.abs() + safe_range ||
                avg_decidable.abs() < base_line_average.abs() - safe_range;
            // if it is deployed run the pi centric code
            if ON_PI == true{
                if go_robot {
                    if(bounce_inputs < MAX_BOUNCE_SIZE){
                        fire_robot();
                        bounce_inputs += 1;
                    } else {
                        bounce_inputs = 0;
                        release_robot();
                    }
                } else {
                    release_robot();
                }
            }
            // this is the final debug message
            println!(
                "Current decibel level: {:.2} dB\nBase line decibel: {:.2}\nExecute Robot: {},\nSize of Sample {}\n\n\n\n",
                db_level,
                base_line_average,
                go_robot,
                base_line_container.len()
            );
            last_print = Instant::now(); // update the last request time
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}

fn setup_pi() -> pi::Motor{
    // let test_result: pi::Motor = pi::Motor::new(13, 24);
    let test = match pi::Motor::new(13, 24) {
        Ok(motor) => motor,
        Err(e) => pi::Motor::new(12, 26)
    };
    return test;
}

fn fire_robot(){
    // TODO: Lockdown this Pin
    let mut motor = setup_pi();
    motor.set_speed(0.2);
}

fn release_robot(){
    // TODO: Lockdown this Pin
    let mut motor = setup_pi();
    motor.stop();
}

fn process_input(data: &[f32], channels: usize, _sample_rate: f32, max_amplitude: &Arc<Mutex<f32>>) {
    let mut max:f32 = 0.0;
    for frame in data.chunks(channels) {
        for &sample in frame {
            max = max.max(sample.abs());
        }
    }
    let mut amplitude = max_amplitude.lock().unwrap();
    *amplitude = max;
}

fn calculate_variance(data: Vec<f32>, mean: f32) -> f32 {
    let count = data.len();

    if count > 0 {
        let variance: f32 = data.iter().map(|value| {
            let diff = mean - *value;
            diff * diff
        }).sum();
        variance / count as f32
    } else {
        0.0
    }
}