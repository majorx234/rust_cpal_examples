use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, SampleFormat};

fn get_audio_device() -> Result<(Device, SampleFormat), String> {
    let host = cpal::default_host();
    let out_default_device = host
        .default_output_device()
        .ok_or("Error: fail to get output device")?;
    // "Error: fail to get output device"
    let mut sample_format = None;
    if let Ok(output_default_config) = out_default_device.default_output_config() {
        match output_default_config.sample_format() {
            SampleFormat::F32 => {
                println!("using F32");
                sample_format = Some(SampleFormat::F32);
            }
            SampleFormat::I16 => {
                println!("using I16");
                sample_format = Some(SampleFormat::I16);
            }
            SampleFormat::U16 => {
                println!("using U16");
                sample_format = Some(SampleFormat::U16);
            }
            _ => {
                println!("unsupported format");
            }
        }
    };
    if let Some(sample_format) = sample_format {
        Ok((out_default_device, sample_format))
    } else {
        Err("unsupported format".to_string())
    }
}

fn main() {
    println!("starting audio output");
    if let Ok((device, sampleformat)) = get_audio_device() {
        println!("ready to play");
    }
}
