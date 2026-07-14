use serde::{Deserialize, Serialize};

use rand::prelude::*;

use crate::bingo::board::BingoBoard;
use crate::bingo::item::BingoItem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BingoProject {
	pub title: String,
	pub city: String,
	pub items: Vec<BingoItem>,
}

impl BingoProject {
	/// Serialize to RON format
	pub fn to_ron(&self) -> anyhow::Result<String> {
		// this looks a little weird, but it to convert from ron::result to anyhow::result
		// anyhow result is awesome and lets us mix all the errors into 1 type
		Ok(ron::to_string(self)?)
	}

	/// Deserialize from RON format
	pub fn from_ron(input: &str) -> anyhow::Result<Self> {
		// this looks a little weird, but it to convert from ron::result to anyhow::result
		// anyhow result is awesome and lets us mix all the errors into 1 type
		Ok(ron::from_str(input)?)
	}

	/// generate a random bingo board from the bingo board project by shuffling 25 items into a `BingoBoard`
	pub fn generate_random_board(&self) -> BingoBoard {
		let mut rng = rand::rng();

		BingoBoard {
			city: self.city.clone(),
			items: self.items.sample(&mut rng, 25).cloned().collect(),
		}
	}
}

pub mod commands {
	use super::*;

	#[tauri::command]
	pub fn generate_random_board(project: BingoProject) -> BingoBoard {
		project.generate_random_board()
	}
}
