/// Demonstrates the simultaneous mixing of raw streams effects.
use sdl2::mixer::{AUDIO_S16LSB, DEFAULT_CHANNELS};
use std::env;
use std::fs::File;
use std::io::Read;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;

const MAX_MIXER_CHANNELS: i32 = 4;

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!(
            "Usage: ./{} stream0.pcm_s16le stream1.pcm_s16le ...",
            args.get(0).unwrap()
        );
    } else {
        // Load all stream data to memory from file.
        let streams: Vec<_> = args
            .iter()
            .skip(1)
            .map(|x| {
                File::open(x)
                    .and_then(|mut f| {
                        let mut buffer = Vec::new();
                        let r = f.read_to_end(&mut buffer);
                        r.map(|_| buffer)
                    })
                    .map_or(vec![], |b| b)
            })
            .collect();
        // Play with mixer.
        demo(streams)?;
    }

    Ok(())
}

fn demo(streams: Vec<Vec<u8>>) -> Result<(), String> {
    println!(
        "mixer linked version: {}",
        sdl2::mixer::get_linked_version()
    );

    let sdl = sdl2::init()?;
    let _audio = sdl.audio()?;
    let mut timer = sdl.timer()?;

    let frequency = 44_100;
    let format = AUDIO_S16LSB; // signed 16 bit samples, in little-endian byte order
    let channels = DEFAULT_CHANNELS; // Stereo
    let chunk_size = 1_024;
    sdl2::mixer::open_audio(frequency, format, channels, chunk_size)?;

    // Number of mixing channels available for sound effect `Chunk`s to play
    // simultaneously.
    sdl2::mixer::allocate_channels(MAX_MIXER_CHANNELS);

    let active_channels = Arc::new(AtomicI32::new(MAX_MIXER_CHANNELS));
    let active_channels_cloned = Arc::clone(&active_channels);

    // Print the channel finished state.
    sdl2::mixer::set_channel_finished(move |channel| {
        active_channels_cloned.fetch_sub(1, Ordering::SeqCst);
        println!("{:?} finished!", channel);
    });

    // Convert stream data from byte array to Chunk.
    let mixer_chunks: Vec<_> = streams
        .into_iter()
        .map(|x| {
            sdl2::mixer::Chunk::from_raw_buffer(x.into_boxed_slice())
                .expect("Failed to create chunk!")
        })
        .collect();

    // Make the Channel objects.
    let mixer_channels: Vec<_> = (0..MAX_MIXER_CHANNELS).map(sdl2::mixer::Channel).collect();

    // Play each Chunk on Channel.
    for (channel, chunk) in mixer_channels.iter().zip(mixer_chunks.iter()) {
        channel.play(chunk, 0)?;
    }

    // Wait all channel finished.
    loop {
        if active_channels.load(Ordering::SeqCst) == 0 {
            break;
        }
        timer.delay(20);
    }

    Ok(())
}
