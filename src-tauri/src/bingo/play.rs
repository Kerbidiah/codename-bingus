use serde::{Deserialize, Serialize};

use crate::auto_serde::AutoSerde;
use crate::bingo::board::BingoBoard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayableBingo {
	pub board: BingoBoard,
	// pub assets_base_path: PathBuf,
}

impl PlayableBingo {
	// pub fn base_path_as_string(&self) -> String {
	// 	self.assets_base_path.to_str().unwrap().to_string()
	// }
}

pub mod commands {
	use super::*;

	#[tauri::command]
	pub fn open_play(path: String) -> PlayableBingo {
		PlayableBingo::open(path).unwrap()
	}

	#[tauri::command]
	pub fn save_play(path: String, obj: PlayableBingo) {
		obj.write(path).unwrap();
	}
}
