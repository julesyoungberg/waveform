use cpal;
use cpal::traits::DeviceTrait;
use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer};

mod util;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 400;
const FRAME_SIZE: usize = 512;

fn main() {
    nannou::app(model).update(update).run();
}

#[allow(dead_code)]
struct Model {
    buffer: Vec<f32>,
    consumer: Consumer<f32>,
    stream: cpal::Stream,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .view(view)
        .build()
        .unwrap();

    // Initialise the audio host so we can spawn an audio stream.
    println!("configuring audio input device");
    let audio_device = util::get_audio_device();
    let mic_config = util::get_audio_config(&audio_device);
    let cpal::SampleRate(sample_rate) = mic_config.sample_rate();
    println!("sample_rate: {:?}", sample_rate);

    // Create a ring buffer and split it into producer and consumer
    let ring_buffer = RingBuffer::<f32>::new(FRAME_SIZE * 2); // Add some latency
    let (mut producer, consumer) = ring_buffer.split();
    for _ in 0..FRAME_SIZE {
        // The ring buffer has twice as much space as necessary to add latency here,
        // so this should never fail
        producer.push(0.0).unwrap();
    }

    // create the stream
    let stream = audio_device
        .build_input_stream(
            &mic_config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                for &sample in data {
                    producer.push(sample).ok();
                }
            },
            move |err| {
                panic!(err);
            },
        )
        .unwrap();

    Model {
        buffer: vec![],
        consumer,
        stream,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let buffer_size = FRAME_SIZE;
    model.buffer = (0..buffer_size)
        .map(|_| match model.consumer.pop() {
            Some(f) => f,
            None => 0.0,
        })
        .collect::<Vec<f32>>();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(PURPLE);

    let buffer_size = FRAME_SIZE;
    let points = model.buffer.iter().enumerate().map(|(i, sample)| {
        let x = ((i as f32 / buffer_size as f32) - 0.5) * WIDTH as f32;
        let y = sample * HEIGHT as f32 / 2.0;
        (pt2(x, y), YELLOW)
    });

    draw.polyline().weight(3.0).points_colored(points);

    draw.to_frame(app, &frame).unwrap();
}
