use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{Device, SupportedStreamConfig};

pub fn get_audio_device() -> Device {
    let host = cpal::default_host();
    host.default_input_device()
        .expect("no audio input device available")
}

pub fn get_audio_config(device: &Device) -> SupportedStreamConfig {
    // get supported config
    let mut supported_configs_range = device
        .supported_input_configs()
        .expect("error while querying audio configs");
    supported_configs_range
        .next()
        .expect("no supported audio config?!")
        .with_max_sample_rate()
}
