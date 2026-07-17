use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::bingo::board::BingoBoard;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayableBingo {
	pub board: BingoBoard,
	pub assets_base_path: PathBuf,
}

impl PlayableBingo {
	pub fn base_path_as_string(&self) -> String {
		self.assets_base_path.to_str().unwrap().to_string()
	}
}
