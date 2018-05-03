#![feature(plugin)]
#![feature(use_extern_macros)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate cpal;

mod audio_devices;
use audio_devices::*;

use rocket::Rocket;
use std::process::{Command, Output};
use std::io::Result;


#[get("/shutdown")]
fn shutdown() -> String {

	let result: Result<Output> = Command::new("shutdown")
			.args(&["/p"])
			.output();
	
	match result {
		Ok(x) => String::from(format!("Process exited with code {} - and output {}", x.status, std::str::from_utf8(&x.stdout[0..x.stdout.len()]).expect("Status output wasn't in UTF8, for some reason."))),
		Err(z) => String::from(format!("Process exited with error code {:?}", z.raw_os_error())),
	}

}


fn rocket() -> Rocket {
	rocket::ignite()
}

#[cfg(target_family = "windows")]
fn main() {

	rocket()
		.mount("/", routes![shutdown])
		.mount("/audio", routes![audio_devices, audio_device_type, default_audio_device_type])
		.mount("/audio", routes![set_default_audio_device]) //Windows only (currently)
			.launch();

}

#[cfg(target_family = "unix")]
fn main() {
		rocket()
		.mount("/audio", routes![audio_devices, audio_device_type, default_audio_device_type])
			.launch();
}