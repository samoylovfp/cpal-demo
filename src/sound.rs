use std::sync::mpsc::channel;
use std::time::Instant;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn get_sound() -> Option<Vec<f32>> {
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

    let (sound_sender, sound_receiver) = channel();

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                sound_sender.send(data.to_owned()).unwrap();
            },
            move |err| println!("Stream read error: {}", err),
        )
        .expect("Failed to process stream");
    match stream.play() {
        Ok(()) => {}
        Err(err) => eprintln!("Failed to play stream: {}", err),
    }

    // This indicates when silence in the mic stream starts
    let mut silence_start = None;
    let mut sound_from_start_till_silence = vec![];

    // Main loop that gathers sound from mic stream
    loop {
        let small_sound_chunk = sound_receiver.recv().unwrap();

        sound_from_start_till_silence.extend(&small_sound_chunk);

        let sound_as_ints = small_sound_chunk.iter().map(|f| (*f * 1000.0) as i32);
        let max_amplitude = sound_as_ints.clone().max().unwrap_or(0);
        let min_amplitude = sound_as_ints.clone().min().unwrap_or(0);

        let silence_detected = max_amplitude < 200 && min_amplitude > -200;
        if silence_detected {
            match silence_start {
                // There was no silence before
                None => silence_start = Some(Instant::now()),
                // Mic was silent for some time
                Some(s) => {
                    if s.elapsed().as_secs_f32() > 2.0 {
                        return Some(sound_from_start_till_silence);
                    }
                }
            }
        }
    }
}
