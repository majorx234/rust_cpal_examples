use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, FromSample, Sample, SampleFormat, SizedSample, StreamConfig};
use std::f64;

struct SineSynth {
    freq: f64,
    phase: f64,
    step: usize,
}

impl SineSynth {
    // TODO work with phase instead of step
    fn sine_wave_step(&mut self, fsample_rate: f32) -> (f64, f64) {
        let value = (2.0 * f64::consts::PI * self.freq as f64 * (self.step as f64)
            / (fsample_rate as f64))
            .sin();
        self.step += 1;
        (value, value)
    }
}

fn get_audio_device() -> Result<(Device, SampleFormat), String> {
    let host = cpal::default_host();
    let out_default_device = host
        .default_output_device()
        .ok_or("Error: fail to get output device")?;
    // "Error: fail to get output device"
    let sample_format =
        if let Ok(output_default_config) = out_default_device.default_output_config() {
            match output_default_config.sample_format() {
                SampleFormat::F32 => {
                    println!("using F32");
                    Some(SampleFormat::F32)
                }
                SampleFormat::I16 => {
                    println!("using I16");
                    Some(SampleFormat::I16)
                }
                SampleFormat::U16 => {
                    println!("using U16");
                    Some(SampleFormat::U16)
                }
                _ => {
                    println!("unsupported format");
                    None
                }
            }
        } else {
            None
        };
    if let Some(sample_format) = sample_format {
        Ok((out_default_device, sample_format))
    } else {
        Err("unsupported format".to_string())
    }
}

fn write_data<T: SizedSample + FromSample<f64>>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> (f64, f64),
) {
    for frame in output.chunks_mut(channels) {
        let sample = next_sample();
        let left: T = T::from_sample(sample.0);
        let right: T = T::from_sample(sample.1);

        for (channel, sample) in frame.iter_mut().enumerate() {
            *sample = if channel & 1 == 0 { left } else { right };
        }
    }
}

fn main() {
    println!("starting audio output");
    if let Ok((device, sampleformat)) = get_audio_device() {
        let config = device.default_output_config().expect("config error").into();
        println!("ready to play");
        std::thread::spawn(move || {
            let sample_rate = 48000.0f32;
            let mut sine_obj = SineSynth {
                freq: 440.0f64,
                phase: 0.0f64,
                step: 0,
            };
            let mut next_value = move || sine_obj.sine_wave_step(sample_rate);
            let channels: usize = 2;
            let err_fn = |err| eprintln!("Error: {err} occured on stream");
            let stream = device
                .build_output_stream(
                    &config,
                    move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
                        write_data(data, channels, &mut next_value)
                    },
                    err_fn,
                    None,
                )
                .expect("build output stream failed");
        });
    };
}
