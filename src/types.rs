use axum_template::engine::Engine;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};

pub type HbsEngine = Engine<Handlebars<'static>>;

#[derive(Debug, Serialize)]
pub struct Page {
	pub content: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
	pub server: Server,
	pub bluetooth: Bluetooth,
	pub devices: Vec<Device>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Server {
	pub port: u16,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Bluetooth {
	pub adapter: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Device {
	pub name: String,
	pub address: String,
	pub handle: u16,
	pub order: String,
}
