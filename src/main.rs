use std::{env::args, path::Path, time::Instant};

use cpal_demo::sound::get_mic_stream;
use deepspeech::Model;

fn main() {
    match get_mic_stream() {
        Some(data) => transcribe(&data),
        None => println!("No data"),
    };
}

fn transcribe(stream: &Vec<f32>) {
    let mut sound_buffer: Vec<f32> = vec![];
    let mut audio_buf: Vec<i16> = Vec::new();
    sound_buffer.extend(stream);
    if sound_buffer.len() >= 44100 {
        for i in stream {
            audio_buf.push(*i as i16);
        }
        let start = Instant::now();
        let model_dir_str = args().nth(1).expect("Please specify model dir");
        let dir_path = Path::new(&model_dir_str);
        let mut graph_name: Box<Path> = dir_path.join("output_graph.pb").into_boxed_path();
        let mut scorer_name: Option<Box<Path>> = None;
        //search for model in model dir

        for file in dir_path
            .read_dir()
            .expect("Specified model dir is not a dir")
        {
            if let Ok(f) = file {
                let file_path = f.path();
                if file_path.is_file() {
                    if let Some(ext) = file_path.extension() {
                        if ext == "pb" || ext == "pbmm" || ext == "tflite" {
                            graph_name = file_path.into_boxed_path();
                        } else if ext == "scorer" {
                            scorer_name = Some(file_path.into_boxed_path());
                        }
                    }
                }
            }
        }
        let mut m = Model::load_from_files(&graph_name).unwrap();
        if let Some(scorer) = scorer_name {
            println!("Using external scorer `{}`", scorer.to_str().unwrap());
            m.enable_external_scorer(&scorer).unwrap();
        }

        let initialised_time = Instant::now();
        println!("Model intialised in {:?}.", initialised_time - start);

        let len_seconds = audio_buf.len() as f64 / 44100 as f64;
        let decoded_time = Instant::now();

        println!(
            "Decoding done in {:?}. Sample length {}s. Initiating STT.",
            decoded_time - initialised_time,
            len_seconds
        );

        //Run speech to text algo:
        let result = m.speech_to_text(&audio_buf).unwrap();

        let text_time = Instant::now();

        let elapsed = text_time - decoded_time;

        let elapsed_f = elapsed.subsec_micros() as f64 / 1_000_000.0 + elapsed.as_secs() as f64;
        println!(
            "STT done in {:?}. Real time factor {:.5}",
            elapsed,
            elapsed_f / len_seconds
        );
        println!("{}", result);
    }
}
