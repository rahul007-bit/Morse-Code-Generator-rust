use std::{io, thread, time::Duration};

use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    FromSample, Sample, StreamConfig,
};
mod marse_code;

const DOT_DURATION: u64 = 80;
const DASH_DURATION: u64 = DOT_DURATION * 3;

// use rand::Rng;

fn main() {
    println!("Enter your message: ");
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = morse_code_generator(input);
    println!("morse codes: {:?}", input);
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");

    let morse_code = input.chars().enumerate();
    let config = device.default_output_config().unwrap();
    let new_config: StreamConfig = config.into();
    for (_, element) in morse_code {
        match element {
            '.' => {
                run(&device, new_config.clone(), DOT_DURATION);
            }
            '-' => {
                run(&device, new_config.clone(), DASH_DURATION);
            }
            ' ' => {
                thread::sleep(Duration::from_millis(DOT_DURATION));
            }
            _ => {
                println!("{}", element)
            }
        }
        thread::sleep(Duration::from_millis(DOT_DURATION));
    }
}

pub fn run(device: &cpal::Device, config: cpal::StreamConfig, duration: u64) {
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 0.86) % sample_rate;
        (sample_clock * 800.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data(data, channels, &mut next_value)
            },
            err_fn,
            None,
        )
        .expect("Failed to build output stream");
    stream.play().expect("Failed to play audio stream");

    std::thread::sleep(std::time::Duration::from_millis(duration));

    // Stop the audio stream
    stream.pause().expect("Failed to pause audio stream");
}

fn morse_code_generator(input: String) -> String {
    let morse_code_table = marse_code::get_morse_codes();
    let mut result = String::new();
    for word in input.chars() {
        let value = match morse_code_table.get(&word) {
            Some(morse) => format!("{} ", morse),
            None => format!(" "),
        };
        result.push_str(value.as_str());
    }
    result
}

fn write_data<T>(output: &mut [T], channels: usize, next_sample: &mut dyn FnMut() -> f32)
where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value: T = T::from_sample(next_sample());
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
