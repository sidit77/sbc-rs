use std::iter;
use std::thread::sleep;
use std::time::Duration;
use anyhow::Context;
use cpal::{default_host, SampleFormat};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use sbc_rs::Decoder;

fn main() -> anyhow::Result<()> {
    let host = default_host();
    let device = host
        .default_output_device()
        .context("failed to find output device")?;

    let config = device.supported_output_configs()?
        .find(|config| config.sample_format() == SampleFormat::I16)
        .context("failed to find output config")?
        .with_max_sample_rate()
        .config();

    let file = std::fs::read("../bluefang/target/sbc/output.sbc")?;
    let mut decoder = Decoder::new(file);
    let mut source = iter::from_fn(move || decoder.next_frame().map(Vec::from))
        .inspect(|frame| println!("decoded {} samples", frame.len()))
        .flat_map(|frame| frame.into_iter())
        .chain(iter::repeat(0));

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [i16], _info| {
            println!("playing {} samples", data.len());
            data.into_iter().for_each(|d| *d = source.next().unwrap());
        },
        move |err| {
            eprintln!("an error occurred on the output stream: {}", err);
        },
        None,
    )?;

    stream.play()?;

    sleep(Duration::from_secs(30));

    stream.pause()?;

    Ok(())
}
