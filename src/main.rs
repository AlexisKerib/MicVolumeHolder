use clap::Parser;
use std::{thread, time::Duration};
use windows::{
    Win32::{
        Media::Audio::Endpoints::IAudioEndpointVolume,
        Media::Audio::{DEVICE_STATE_ACTIVE, IMMDeviceEnumerator, MMDeviceEnumerator, eCapture},
        System::Com::{CLSCTX_ALL, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx},
    },
    core::*,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'v', long, default_value_t = 100, value_parser = validate_volume)]
    volume: u32,

    #[arg(short = 't', long, default_value_t = 5.0, value_parser = validate_time)]
    time: f32,
}

fn validate_volume(s: &str) -> std::result::Result<u32, String> {
    let v: u32 = s
        .parse()
        .map_err(|_| format!("{} is not a valid integer", s))?;
    if v > 100 {
        Err(format!("volume must be between 0 and 100, got {}", v))
    } else {
        Ok(v)
    }
}

fn validate_time(s: &str) -> std::result::Result<f32, String> {
    let t: f32 = s
        .parse()
        .map_err(|_| format!("{} is not a valid float", s))?;
    if t <= 0.0 {
        Err(format!("time must be greater than 0, got {}", t))
    } else {
        Ok(t)
    }
}

fn set_mic_volume(target_volume: u32) -> Result<()> {
    unsafe {
        let device_enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;

        let devices = device_enumerator.EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE)?;
        let count = devices.GetCount()?;

        if count == 0 {
            println!("No active microphone found.");
        } else {
            println!("Found {} active microphone(s).", count);
        }

        for i in 0..count {
            let device = devices.Item(i)?;
            let volume: IAudioEndpointVolume = device.Activate(CLSCTX_ALL, None)?;

            let volume_scalar = target_volume as f32 / 100.0;
            volume.SetMasterVolumeLevelScalar(volume_scalar, &GUID::zeroed())?;
            println!("Set the volume of device {} to {}%", i, target_volume);
        }
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    let adjust_period = cli.time;
    let target_volume = cli.volume;

    unsafe {
        if CoInitializeEx(None, COINIT_MULTITHREADED).is_err() {
            println!("COM initialization failed");
            return;
        }
    }

    println!(
        "Service started. The microphone volume will be adjusted to {}% every {} seconds.",
        adjust_period, target_volume
    );
    println!("Press Ctrl+C to stop the service.");

    loop {
        if let Err(err) = set_mic_volume(target_volume) {
            println!("An error occurred while setting the volume: {:?}", err);
        }

        thread::sleep(Duration::from_secs_f32(adjust_period));
    }
}
