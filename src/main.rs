extern crate cpal;
use std::process::exit;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

fn main() {
    //initialise the host
    let host = cpal::default_host();

    //choose available device
    let device = host.default_input_device().expect("no input device found");
    println!("Input device: {:?}", device.name());

    let config = device
        .default_input_config()
        .expect("Failed to get input config");
    println!("Default input config: {:?} ", config);

    // A flag to indicate that recording is in progress.
    println!("Listening...");

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| match write_input_data::<f32>(data) {
                    true => exit(1),
                    false => (),
                },
                move |err| {},
            )
            .expect("Failed to process stream"),

        cpal::SampleFormat::I16 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| match write_input_data::<f32>(data) {
                    true => exit(1),
                    false => (),
                },
                move |err| {},
            )
            .expect("Failed to process stream"),
        cpal::SampleFormat::U16 => device
            .build_input_stream(
                &config.into(),
                move |data, _: &_| match write_input_data::<f32>(data) {
                    true => exit(1),
                    false => (),
                },
                move |err| {},
            )
            .expect("Failed to process stream"),
    };
    loop {
        match stream.play() {
            Ok(()) => {}
            Err(err) => eprintln!("Failed to play stream: {}", err),
        }
    }
}

//return false if the RMS level is higher than silence? (keep recording...)
fn write_input_data<T>(input: &[T]) -> bool
where
    T: cpal::Sample,
{
    let mut rms: usize = 0;
    let shorts = input.len() / 2;
    for elem in 0..shorts {
        let normal = elem;
        rms += normal * normal;
    }
    rms = rms / shorts; //find square root of right side
    println!("Listening, rms is {}", rms);
    if rms < 0.1 {};
    true
}
