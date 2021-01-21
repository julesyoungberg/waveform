use cpal;
use cpal::traits::DeviceTrait;
use nannou::prelude::*;
use std::sync::mpsc::channel;
use std::time;

mod util;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;

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
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

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

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PURPLE);

    if let Ok(data) = model.rx.recv_timeout(time::Duration::from_millis(1)) {
        let num_samples = data.len();

        let points = (0..num_samples).map(|i| {
            let x = ((i as f32 / num_samples as f32) - 0.5) * WIDTH as f32;
            let y = data[i] * HEIGHT as f32;
            (pt2(x, y), YELLOW)
        });

        draw.polyline().weight(3.0).points_colored(points);
    }

    draw.to_frame(app, &frame).unwrap();
}
