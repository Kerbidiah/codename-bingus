pub mod auto_serde;
pub mod bingo;

use crate::bingo::{get_bingo_projects, get_bingo_games};
use crate::bingo::board::commands::*;
use crate::bingo::item::commands::*;
use crate::bingo::project::commands::*;
use crate::bingo::play::commands::*;

use crate::bingo::item::BingoItem;
use crate::bingo::project::BingoProject;

use auto_serde::AutoSerde;

use std::fs::File;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	// let bingo_proj = BingoProject {
	// 	title: "TEST".to_string(),
	// 	city: "Fuckin Linz Man".to_string(),
	// 	items: BingoItem::vienna_samples().unwrap(),
	// 	last_board: None,
	// };

	// let mut f = File::create("../examples/projects/TEST.BingoProject").unwrap();
	// bingo_proj.to_file(&mut f).unwrap();

	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			greet,
			generate_dummy_bingo_board,
			generate_random_board,
			example_bingo_items,
			get_bingo_projects,
			get_bingo_games,
			open_project,
			save_project,
			open_board,
			save_board,
			open_play,
			save_play
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
