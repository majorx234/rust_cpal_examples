use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, SampleFormat};

fn get_audio_device() -> Result<Device, String> {
    let host = cpal::default_host();
    let out_default_device = host
        .default_output_device()
        .ok_or("Error: fail to get output device")?;
    // "Error: fail to get output device"
    Ok(out_default_device)
}

fn main() {
    println!("starting audio output");
    let device = get_audio_device();
}
