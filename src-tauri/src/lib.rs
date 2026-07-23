pub mod auto_serde;
pub mod bingo;

use crate::bingo::board::commands::generate_dummy_bingo_board;
use crate::bingo::{get_bingo_projects, get_bingo_games};
use crate::bingo::item::commands::example_bingo_items;
use crate::bingo::project::commands::{generate_random_board, open_project, save_project};

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
	let bingo_proj = BingoProject {
		title: "TEST".to_string(),
		city: "Fuckin Linz Man".to_string(),
		items: BingoItem::vienna_samples().unwrap(),
		last_board: None,
	};

	let mut f = File::create("../examples/projects/TEST.BingoProject").unwrap();
	bingo_proj.to_file(&mut f).unwrap();

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
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
