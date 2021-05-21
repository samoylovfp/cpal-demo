use std::sync::mpsc::channel;
use std::time::Instant;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn get_mic_stream() -> Option<Vec<f32>> {
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
    let mut start = Instant::now();

    let (sound_sender, sound_receiver) = channel();

    let stream = device
        .build_input_stream(
            &config.into(),
            move |data: &[f32], _: &_| {
                sound_sender.send(data.to_owned()).unwrap();
            },
            move |err| {},
        )
        .expect("Failed to process stream");
    match stream.play() {
        Ok(()) => {}
        Err(err) => eprintln!("Failed to play stream: {}", err),
    }
    //starting with extend. That 'extends' the collection with the contents of the iterator. If we're creating an empty sound buffer here and extending it with
    //the sound_receiver.recv(), How's that not the same as cloning sound_recevier as sound buffer?
    let mut sound_buffer = vec![];
    loop {
        sound_buffer.extend(sound_receiver.recv().unwrap());
        if sound_buffer.len() >= 44100 {
            let int = sound_buffer.iter().map(|f| (*f * 1000.0) as i32);
            let max = int.clone().max();
            let min = int.clone().max();
            let max = max.expect("Stream Error");
            let min = min.expect("Stream Error");
            //what are the mix max values? How would I find the silence values.
            println!("Max is {}, min is {}", max, min);

            /*Silence detected */
            if max < 200 && min > -200 {
                let elapsed = start.elapsed();
                if elapsed.as_millis() > 2000 {
                    break;
                } else {
                    start = Instant::now();
                }
            }
            sound_buffer.clear();
        }
    }

    Some(sound_buffer)
}
