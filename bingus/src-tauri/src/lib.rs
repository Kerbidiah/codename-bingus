pub mod bingo;

use crate::bingo::board::commands::generate_dummy_bingo_board;
use crate::bingo::item::commands::example_bingo_items;
use crate::bingo::project::commands::generate_random_board;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			greet,
			generate_dummy_bingo_board,
			generate_random_board,
			example_bingo_items,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
