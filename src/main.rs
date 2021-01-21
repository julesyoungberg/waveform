use cpal;
use cpal::traits::DeviceTrait;
use nannou::prelude::*;
use std::sync::mpsc::channel;
use std::time;

mod util;

fn main() {
    nannou::app(model).run();
}

#[allow(dead_code)]
struct Model {
    rx: std::sync::mpsc::Receiver<Vec<f32>>,
    stream: cpal::Stream,
}

fn model(app: &App) -> Model {
    // Create a window to receive key pressed events.
    app.new_window().view(view).build().unwrap();

    // Initialise the audio host so we can spawn an audio stream.
    println!("configuring audio input device");
    let audio_device = util::get_audio_device();
    let mic_config = util::get_mic_config(&audio_device);
    let cpal::SampleRate(sample_rate) = mic_config.sample_rate();
    println!("sample_rate: {:?}", sample_rate);

    let (tx, rx) = channel();

    // create the stream
    let stream = audio_device
        .build_input_stream(
            &mic_config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| match tx.send(data.to_vec()) {
                Ok(()) => (),
                Err(e) => panic!(e),
            },
            move |err| {
                panic!(err);
            },
        )
        .unwrap();

    Model { rx, stream }
}

fn view(_app: &App, model: &Model, frame: Frame) {
    frame.clear(PURPLE);

    let data = match model.rx.recv_timeout(time::Duration::from_millis(1)) {
        Ok(m) => m,
        Err(_) => return,
    };

    println!("received: {:?}", data.len());
}
