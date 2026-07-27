use serde::{Deserialize, Serialize};

use crate::auto_serde::AutoSerde;
use crate::bingo::item::BingoItem;
use log::info;
use fake::{Fake, faker};

use rand::prelude::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BingoBoard {
	pub title: String,
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
			title: faker::address::en::CityName().fake(),
			city: faker::address::de_de::CityName().fake(),
			items: items,
		}
	}
}

pub mod commands {
	use super::*;

	#[tauri::command]
	pub fn generate_dummy_bingo_board() -> BingoBoard {
		info!("generate_dummy_bingo_board ran");
		BingoBoard::dummy()
	}

	#[tauri::command]
	pub fn open_board(path: String) -> BingoBoard {
		info!("open_board ran");
		BingoBoard::open(path).unwrap()
	}

	#[tauri::command]
	pub fn save_board(path: String, obj: BingoBoard) {
		info!("save_board ran");
		obj.write(path).unwrap();
	}

	#[tauri::command]
	pub fn new_bingo_board(title: String, city: String, items: Vec<BingoItem>) -> BingoBoard {
		info!("new_bingo_board ran");
		BingoBoard { title, city, items }
	}
}
