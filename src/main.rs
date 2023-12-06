/* Disclaimer, I am learning Rust right now with simple projects in nannou like this (this is the first "big one" I tryied to do), so this program will
contain unoptimized code and probably bad programming practices

I'm an absolute noob in digital audio processing and I still don't know many of the concepts behind it

this simple project was made by "stealing some code" from:
- /examples/audio/feedback.rs from the official nannou examples repository https://github.com/nannou-org/nannou
- the program made by this guy, which (if I understood right) visualizes sound without differentiating between left and right channels https://github.com/julesyoungberg/waveform/tree/main

the audio is captured from the microphone (I still don't know how to change input device from code), but by using programs like qjackctl and editing the audio graph you
can redirect inputs from other sources (and visualize them)
for example from youtube running in a browser (it has to be running to be visible), from spotify or from an analog instrument connected to a sound card
if you connected the L and R channels correctly you should be able to see the differences (but you actually have to reproduce stereo audio, try for example with the song
California Dreamin' from The Mamas & The Papas, in the beginning you can clearly see differences in both channels) */

use nannou_audio as audio;
use nannou::prelude::*;
use ringbuf::{Consumer, RingBuffer, Producer};
use audio::Buffer;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 800;
const FRAME_SIZE: usize = 512;      // lower means less accurate sampling and so less accurate visualization (???)

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

// dead_code is needed to hold a reference to stream (I think)
#[allow(dead_code)]
struct Model {
    buffer: Vec<f32>,                           // used for visualization purposes (can be accessed in the view function)
    in_stream: audio::Stream<InputModel>,       // used to get audio data (input)
    consumer: Consumer<f32>,                    // used to "consume" audio data
}

struct InputModel {
    pub producer: Producer<f32>                 // used to "fill" the buffer with audio data
}

fn model(app: &App) -> Model {
    app.new_window()
        .size(WIDTH, HEIGHT)
        .transparent(true)          // if you want transparent background look in the first lines of the draw function
        .view(view)
        .build()
        .unwrap();

    // Initialise the audio host to spawn an audio stream.
    let audio_host = audio::Host::new();

    // Create a ring buffer and split it into producer and consumer
    let ring_buffer = RingBuffer::<f32>::new(FRAME_SIZE * 2);   // Add some latency (why is that? I don't know why multiply by 2, but even by not multiplying it doesn't seem to affect anything)
    
    let (mut producer, consumer) = ring_buffer.split();     // producer will "fill" the buffer, consumer will "drain" it
    
    // initialize the buffer with zeros
    for _ in 0..FRAME_SIZE {
        producer.push(0.0).unwrap();
    }

    let in_model = InputModel { producer };
    let in_stream = audio_host
        .new_input_stream(in_model)
        .capture(pass_in)           // this callback is run everytime a new sound is captured
        .build()
        .unwrap();
        
    Model {
        buffer: vec![],
        in_stream,
        consumer,
    }
}

// function called everytime a new sound is captured
fn pass_in(model: &mut InputModel, buffer: &Buffer) {
    // fill the ringbuffer (only the producer can add items to it) with samples from the captured audio
    for frame in buffer.frames() {
        for sample in frame {
            model.producer.push(*sample).ok();
        }
    }
}

// this function basically "transfers" data from the ringbuffer (audio in) to a secondary buffer that can be accessed by the view function and used for visualization
fn update(_app: &App, model: &mut Model, _update: Update) {
    model.buffer = (0..FRAME_SIZE)
        .map(|_| match model.consumer.pop() {
            Some(f) => f,
            None => 0.0,
        })
        .collect::<Vec<f32>>();
}

// this function separates audio data in left and right channel (the audio vector is interleaved, like [L, R, L, R, L, R, ...])
fn separate_channels(input: Vec<f32>) -> (Vec<f32>, Vec<f32>) {
    let mut left_channel = Vec::new();
    let mut right_channel = Vec::new();

    // left channel data is in position i*2, right channel data is in position i*2 + 1
    for i in (0..input.len()).step_by(2) {
        // Extract left and right channel values
        let left_value = input[i];
        let right_value = input[i + 1];

        // Append values to respective channels
        left_channel.push(left_value);
        right_channel.push(right_value);
    }

    (left_channel, right_channel)
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    //frame.clear(rgba(0.0, 0.0, 0.0, 0.0));        // uncomment this (and comment the line above) if you want transparent background (pretty cool in my opinion, but you may have to change left and right channel colors for better visualization)
    
    let left_channel_color = YELLOW;
    let right_channel_color = CYAN;

    let channels = separate_channels(model.buffer.clone());     // I still don't understand Rust concepts very well, clone is probably needed to not take ownership of the buffer (which is used in the update function) ???

    let win = app.window_rect();

    // top_rect refers to the "higher half" of the screen
    let top_rect = Rect::from_x_y_w_h(0.0, win.h() / 4.0, win.w(), (win.h() - 1.0) / 2.0);
    
    // bot_rect refers to the "lower half" of the screen
    let bot_rect = Rect::from_x_y_w_h(0.0, (-win.h() + 1.0) / 4.0, win.w(), win.h() / 2.0);

    // visualize the "higher half" of the screen as a no_fill rectangle
    draw.rect()
        .xy(top_rect.xy())
        .wh(top_rect.wh())
        .no_fill()
        .stroke_weight(1.0)
        .stroke_color(left_channel_color);

    // visualize the "lower half" of the screen as a no_fill rectangle
    draw.rect()
        .xy(bot_rect.xy())
        .wh(bot_rect.wh())
        .no_fill()
        .stroke_weight(1.0)
        .stroke_color(right_channel_color);

    // mapping of the values from the left audio channel into coordinates
    let left_points = channels.0.iter()
        .enumerate()
        .map(|(i, sample)| {
            let x = ((i as f32 / FRAME_SIZE as f32) - 0.5) * (top_rect.w() * 2.0) + (top_rect.w() / 2.0);
            let y = sample * (top_rect.h() / 2.0) / 2.0;
            (pt2(x, y), left_channel_color)
        });

    // mapping of the values from the right audio channel into coordinates
    let right_points = channels.1.iter()
        .enumerate()
        .map(|(i, sample)| {
            let x = ((i as f32 / FRAME_SIZE as f32) - 0.5) * (bot_rect.w() * 2.0) + (bot_rect.w() / 2.0);
            let y = sample * (bot_rect.h() / 2.0) / 2.0;
            (pt2(x, y), right_channel_color)
        });

    // points to the "center" of the "higher half" of the screen
    let left_rect = top_rect.middle_of(top_rect);
    
    left_points.for_each(|p| {
        let pos = p.0;
        let col = p.1;

        draw.line()
            .xy(left_rect.xy())   // centers the line in the "higher half" of the screen, start and end positions are "off-setted" from this
            .start(pos)
            .end(pt2(pos.x, 0.0))
            .color(col);

    });

    // points to the "center" of the "lower half" of the screen
    let right_rect = bot_rect.middle_of(bot_rect);
    
    right_points.for_each(|p| {
        let pos = p.0;
        let col = p.1;

        draw.line()
            .xy(right_rect.xy())   // centers the line in the "lower half" of the screen, start and end positions are "off-setted" from this
            .start(pos)
            .end(pt2(pos.x, 0.0))
            .color(col);
    });

    draw.to_frame(app, &frame).unwrap();
}
