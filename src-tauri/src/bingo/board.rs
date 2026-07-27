use serde::{Deserialize, Serialize};

use crate::bingo::item::BingoItem;
use crate::auto_serde::AutoSerde;

use fake::{Fake, faker};

use rand::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BingoBoard {
	pub city: String,
	pub items: Vec<BingoItem>,
}

impl BingoBoard {
	/// generate a dummy bingo board with 25 randomly selected and ordered items
	pub fn dummy() -> Self {
		// TODO: error handling!!!!

		let mut rng = rand::rng();
		let items: Vec<BingoItem> = BingoItem::vienna_samples()
			.unwrap()
			.sample(&mut rng, 25)
			.cloned()
			.collect();

		Self {
			city: faker::address::de_de::CityName().fake(),
			items: items,
		}
	}
}

pub mod commands {
	use super::*;

	#[tauri::command]
	pub fn generate_dummy_bingo_board() -> BingoBoard {
		BingoBoard::dummy()
	}

	#[tauri::command]
	pub fn open_board(path: String) -> BingoBoard {
		BingoBoard::open(path).unwrap()
	}

	#[tauri::command]
	pub fn save_board(path: String, obj: BingoBoard) {
		obj.write(path).unwrap();
	}
}
