use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample, SampleFormat};

fn audio_info() {
    let host = cpal::default_host();
    println!("============= list devices =============");
    if let Ok(devices) = host.output_devices() {
        for (idx, device) in devices.enumerate() {
            println!(
                "device[{}]: {}",
                idx,
                device
                    .name()
                    .unwrap_or_else(|_err| String::from("device error")),
            );
        }
    }
    println!("=========== default device ===========");
    let out_default_device = host
        .default_output_device()
        .expect("Error: fail to get output device");
    if let Ok(out_device_name) = out_default_device.name() {
        println!("Device: {}", out_device_name);
    }
    if let Ok(output_config_list) = out_default_device.supported_output_configs() {
        for (idx, outconf) in output_config_list.into_iter().enumerate() {
            println!("============== device config {} ==========", idx);
            println!("output buffersize: {:?}", outconf.buffer_size());
            println!("output num hannels: {}", outconf.channels());
            println!("output sample format: {}", outconf.sample_format());
            println!("output min sample rate: {:?}", outconf.min_sample_rate());
            println!("output max sample rate: {:?}", outconf.max_sample_rate());
            println!("=========================================");
        }
    }
    if let Ok(output_default_config) = out_default_device.default_output_config() {
        match output_default_config.sample_format() {
            SampleFormat::F32 => {
                println!("using F32");
            }
            SampleFormat::I16 => {
                println!("using I16");
            }
            SampleFormat::U16 => {
                println!("using U16");
            }
            _ => {
                println!("unsupported format");
            }
        }
    }
}

fn main() {
    println!("hello cpal");
    audio_info();
}
