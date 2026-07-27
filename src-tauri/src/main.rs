// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
	env_logger::Builder::new()
		.filter_level(log::LevelFilter::Warn)
		.format_timestamp_millis()
		.init();

	log::info!("Starting up");

	bingus_lib::run()
}
