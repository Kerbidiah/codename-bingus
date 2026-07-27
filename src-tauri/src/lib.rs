pub mod auto_serde;
pub mod bingo;

use std::fs::File;

use crate::auto_serde::AutoSerde;
use crate::bingo::board::commands::*;
use crate::bingo::game::{BingoGame, commands::*};
use crate::bingo::item::{BingoItem, commands::*};
use crate::bingo::project::{BingoProject, commands::*};
use crate::bingo::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	{
		// create vienna bingo project
		let bingo_proj = BingoProject {
			title: "Vienna SWE 2026 Bingo 3".to_string(),
			city: "Wien, Österrich".to_string(),
			items: BingoItem::vienna_samples().unwrap(),
			last_board: None,
		};

		let mut f = File::create(resolve_path("bingus/edit/TEST.BingoProject")).unwrap();
		bingo_proj.to_file(&mut f).unwrap();
	}

	{
		// create prauge bingo game and project
		let proj = BingoProject {
			title: "PRAUGE TEST".to_string(),
			city: "Praha, Česká Republika".to_string(),
			items: BingoItem::prauge_samples().unwrap(),
			last_board: None,
		};

		let mut f = File::create(resolve_path("bingus/edit/PRAUGE_TEST.BingoProject")).unwrap();
		proj.to_file(&mut f).unwrap();
		drop(f);

		let game = BingoGame {
			board: proj.generate_random_board(),
		};
		let mut f = File::create(resolve_path("bingus/edit/PRAUGE_TEST.BingoGame")).unwrap();
		game.to_file(&mut f).unwrap();
	}

	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![
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
			save_play,
			new_bingo_board,
			convert_proj_path_to_game_path,
			new_bingo_item,
			quick_export,
			new_proj,
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
