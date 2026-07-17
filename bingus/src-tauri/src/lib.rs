pub mod auto_serde;
pub mod bingo;

use crate::bingo::board::commands::generate_dummy_bingo_board;
use crate::bingo::item::commands::example_bingo_items;
use crate::bingo::project::commands::generate_random_board;
use crate::bingo::get_bingos;

use crate::bingo::BingoType;
use crate::bingo::project::BingoProject;
use crate::bingo::item::BingoItem;
use std::fs::File;
use auto_serde::AutoSerde;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	let bingo_proj = BingoType::Editable(BingoProject {
		title: "TEST".to_string(),
		city: "Fuckin Linz Man".to_string(),
		items: BingoItem::vienna_samples().unwrap(),
		last_board: None
	});

	let mut f = File::create("../../examples/projects/TEST.BingoProject").unwrap();
	bingo_proj.to_file(&mut f).unwrap();
	
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
			greet,
			generate_dummy_bingo_board,
			generate_random_board,
			example_bingo_items,
			get_bingos,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
