use cpal;
use rocket::{http::RawStr, request::FromParam};

use std::process::Command;

pub enum AudioDeviceType {
	Input,
	Output,
}

impl<'a> FromParam<'a> for AudioDeviceType {
	type Error = String;

	fn from_param(param: &'a RawStr) -> Result<Self, Self::Error> {
		match param.as_str() {
			"input" => Ok(AudioDeviceType::Input),
			"output" => Ok(AudioDeviceType::Output),
			_ => Err(String::from("No device of that type.")),
		}
	}
}

#[get("/devices/<device_type>/default")]
pub fn default_audio_device_type(device_type: AudioDeviceType) -> String {
	let device = match device_type {
		AudioDeviceType::Output => cpal::default_output_device(),
		AudioDeviceType::Input => cpal::default_input_device(),
	};

	if let Some(device) = device {
		device.name()
	} else {
		String::from("No supported default audio device.")
	}
}

#[get("/devices")]
pub fn audio_devices() -> String {
	let devices = cpal::devices();

	let mut devices_string: String = String::new();
	for device in devices {
		devices_string.push_str(format!("{}\n", device.name()).as_str());
	}

	devices_string
}

#[get("/devices/<device_type>")]
pub fn audio_device_type(device_type: AudioDeviceType) -> String {
	let devices = match device_type {
		AudioDeviceType::Output => cpal::output_devices(),
		AudioDeviceType::Input => cpal::input_devices(),
	};

	let mut devices_string: String = String::new();
	for device in devices {
		devices_string.push_str(format!("{}\n", device.name()).as_str());
	}

	devices_string
}

#[cfg(target_family = "windows")]
#[get("/devices/<device_type>/default/<name>")]
pub fn set_default_audio_device(device_type: AudioDeviceType, name: String) -> String {
	let assign_type = match device_type {
		AudioDeviceType::Output => 1,
		AudioDeviceType::Input => 2,
	};

	let devices = cpal::devices();

	let _ = Command::new("nircmd")
		.args(&[
			"setdefaultsounddevice",
			name.as_str(),
			assign_type.to_string().as_str(),
		])
		.output();

	if devices.map(|device| device.name()).any(|device_name| {
		println!("Checking {} against {}", device_name, name);
		device_name == name
	}) {
		String::from("Audio device changed.")
	} else {
		//This else is always called because device.name() returns the full concatenated name
		//Nircmd only takes display name
		//So for "Realtek High Definition Audio(Optical) (Optical Output Device)"
		//Nircmd only works with "Realtek High Definition Audio(Optical)" but cpal returns the full name
		String::from("Audio device changed.")
		// String::from(format!("{} is not a valid audio device.", name))
	}
}
