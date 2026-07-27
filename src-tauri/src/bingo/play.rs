use serde::{Deserialize, Serialize};

use crate::auto_serde::AutoSerde;
use crate::bingo::board::BingoBoard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoGame {
	pub board: BingoBoard,
	// pub assets_base_path: PathBuf,
}

impl BingoGame {
	// pub fn base_path_as_string(&self) -> String {
	// 	self.assets_base_path.to_str().unwrap().to_string()
	// }
}

pub mod commands {
	use super::*;
	use log::info;
	#[tauri::command]
	pub fn open_play(path: String) -> BingoGame {
		info!("open_play ran");
		info!("open_play path: {path}");
		BingoGame::open(path).unwrap()
	}

	#[tauri::command]
	pub fn save_play(path: String, obj: BingoGame) {
		info!("save_play ran");
		obj.write(path).unwrap();
	}
}
