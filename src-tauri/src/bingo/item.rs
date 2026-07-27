use serde::{Deserialize, Serialize};

use anyhow;

// use super::completion::BingoCompletionInfo;

use crate::auto_serde::AutoSerde;
use log::info;
use std::fs;
use std::fs::{DirEntry, File};
use std::io;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BingoItem {
	pub title: String,
	pub emoji: Option<char>,
	pub short_description: Option<String>,
	pub url: Option<String>,
	pub completion_info: bool,
}

impl BingoItem {
	fn read_samples<P>(path: P) -> anyhow::Result<Vec<Self>>
	where
		P: AsRef<std::path::Path>,
	{
		let example_paths: Vec<Result<DirEntry, io::Error>> = fs::read_dir(path)?.collect(); // get list of files in examples/items directory
		let mut ans = Vec::with_capacity(example_paths.len()); // create vector preallocated with enough space

		for p in example_paths {
			let mut f = File::open(p?.path())?;
			ans.push(Self::from_file(&mut f)?);
		}

		Ok(ans)
	}

	pub fn vienna_samples() -> anyhow::Result<Vec<Self>> {
		Self::read_samples("../examples/items/")
	}

	pub fn prauge_samples() -> anyhow::Result<Vec<Self>> {
		Self::read_samples("../examples/prauge_items/")
	}

	pub fn london_samples() -> anyhow::Result<Vec<Self>> {
		Self::read_samples("../examples/london_items/")
	}

	pub fn luzern_samples() -> anyhow::Result<Vec<Self>> {
		Self::read_samples("../examples/luzern_items/")
	}

	pub fn budapest_samples() -> anyhow::Result<Vec<Self>> {
		Self::read_samples("../examples/budapest_items/")
	}
}

pub mod commands {
	use super::*;

	#[tauri::command]
	pub fn example_bingo_items() -> Vec<BingoItem> {
		info!("example_bingo_items ran");
		BingoItem::vienna_samples().unwrap()
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
		BingoItem::vienna_samples().unwrap()
	}

	#[tauri::command]
	pub fn get_prauge_samples() -> Vec<BingoItem> {
		BingoItem::prauge_samples().unwrap()
	}

	#[tauri::command]
	pub fn get_london_samples() -> Vec<BingoItem> {
		BingoItem::london_samples().unwrap()
	}

	#[tauri::command]
	pub fn get_luzern_samples() -> Vec<BingoItem> {
		BingoItem::luzern_samples().unwrap()
	}

	#[tauri::command]
	pub fn get_budapest_samples() -> Vec<BingoItem> {
		BingoItem::budapest_samples().unwrap()
	}
}
