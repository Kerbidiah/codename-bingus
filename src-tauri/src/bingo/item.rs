use serde::{Deserialize, Serialize};

// use super::completion::BingoCompletionInfo;

use crate::auto_serde::AutoSerde;
use log::info;

use include_dir::{Dir, include_dir};

static PROJECT_DIR: Dir<'_> = include_dir!("../examples/");

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BingoItem {
	pub title: String,
	pub emoji: Option<char>,
	pub short_description: Option<String>,
	pub url: Option<String>,
	pub completion_info: bool,
}

impl BingoItem {
	fn read_samples<P>(path: P) -> Vec<Self>
	where
		P: AsRef<std::path::Path>,
	{
		PROJECT_DIR
			.get_dir(path)
			.unwrap()
			.files()
			.map(|f| BingoItem::from_dir_file(f).unwrap())
			.collect()
	}

	pub fn vienna_samples() -> Vec<Self> {
		Self::read_samples("vienna_items/")
	}

	pub fn prauge_samples() -> Vec<Self> {
		Self::read_samples("prauge_items/")
	}

	pub fn london_samples() -> Vec<Self> {
		Self::read_samples("london_items/")
	}

	pub fn luzern_samples() -> Vec<Self> {
		Self::read_samples("luzern_items/")
	}

	pub fn budapest_samples() -> Vec<Self> {
		Self::read_samples("budapest_items/")
	}
}

pub mod commands {
	use super::*;

	#[tauri::command]
	pub fn example_bingo_items() -> Vec<BingoItem> {
		info!("example_bingo_items ran");
		BingoItem::vienna_samples()
	}

	#[tauri::command]
	pub fn new_bingo_item(title: String, mut emoji: String) -> BingoItem {
		info!("new_bingo_item ran");
		BingoItem {
			title: title,
			emoji: emoji.pop(),
			short_description: None,
			url: None,
			completion_info: false,
		}
	}

	#[tauri::command]
	pub fn get_vienna_samples() -> Vec<BingoItem> {
		info!("get_vienna_samples run");
		BingoItem::vienna_samples()
	}

	#[tauri::command]
	pub fn get_prauge_samples() -> Vec<BingoItem> {
		info!("get_prauge_samples run");
		BingoItem::prauge_samples()
	}

	#[tauri::command]
	pub fn get_london_samples() -> Vec<BingoItem> {
		info!("get_london_samples run");
		BingoItem::london_samples()
	}

	#[tauri::command]
	pub fn get_luzern_samples() -> Vec<BingoItem> {
		info!("get_luzern_samples run");
		BingoItem::luzern_samples()
	}

	#[tauri::command]
	pub fn get_budapest_samples() -> Vec<BingoItem> {
		info!("get_budapest_samples run");
		BingoItem::budapest_samples()
	}
}
