#[macro_use]
extern crate queues;

use queues::*;
use rust_gpiozero as pi;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use dasp::sample::{Sample};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

fn main() {
    let host = cpal::default_host();
    let device = host.default_input_device().expect("Failed to get default input device");
    let config = device.default_input_config().expect("Failed to get default input config");

    let sample_format = config.sample_format();
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    let max_amplitude = Arc::new(Mutex::new(0.0));

    let max_amplitude_clone = Arc::clone(&max_amplitude);
    let err_fn = |err| eprintln!("An error occurred on stream: {}", err);

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

    let mut last_print = Instant::now();
    let mut base_line_container:Vec<f32> = vec![];
    let mut base_line_average: f32 = 0.0;
    let amount_to_capture: i32 = 100;
    let mut decidable_buffer: Queue<f32> = queue![];
    let safe_range = 50.0;
    loop {
        let amplitude = *max_amplitude.lock().unwrap();
        let db_level = if amplitude > 0.0 {
            20.0 * amplitude.log10()
        } else {
            f32::NEG_INFINITY
        };

        if last_print.elapsed() >= Duration::from_millis(100) {

            if base_line_container.len() < amount_to_capture as usize{
                println!("Running base_capture {}", base_line_container.len());
                if db_level == f32::NEG_INFINITY { continue } // thread failed to capture any audio
                base_line_container.push(db_level);
                last_print = Instant::now();
                continue
            } else if base_line_container.len() < (amount_to_capture + 1) as usize{
                base_line_container.push(db_level);
                let sum: f32 = base_line_container.iter().sum();
                base_line_average =  sum / base_line_container.len() as f32;
                last_print = Instant::now();
                continue
            }
            if decidable_buffer.size() < 10{
                println!("Building Buffer {}", decidable_buffer.size());
                _ = decidable_buffer.add(db_level);
                last_print = Instant::now();
                continue
            }

            _ = decidable_buffer.remove();
            _ = decidable_buffer.add(db_level);

            let sum: f32 = decidable_buffer.iter().sum();
            let avg_decidable =  sum / decidable_buffer.len() as f32;



            // execute robot if we are +/- 20 outside the selected decidable range
            let go_robot = avg_decidable.abs() > base_line_average.abs() + safe_range ||
                avg_decidable.abs() < base_line_average.abs() - safe_range;
            if go_robot {
                fire_robot();
            } else {
                release_robot();
            }

            println!(
                "Current decibel level: {:.2} dB\nBase line decibel: {:.2}\nExecute Robot: {},\nSize of Sample {}",
                db_level,
                base_line_average,
                go_robot,
                base_line_container.len()
            );
            last_print = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(10));
    }
}

fn fire_robot(){
    // TODO: Lockdown this Pin
    let mut motor = pi::Motor::new(0, 0);
    motor.set_speed(0.2);
}

fn release_robot(){
    // TODO: Lockdown this Pin
    let mut motor = pi::Motor::new(0, 0);
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
