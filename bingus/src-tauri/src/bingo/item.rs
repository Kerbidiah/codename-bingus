use serde::{Deserialize, Serialize};

use anyhow;

use super::completion::BingoCompletionInfo;

use crate::auto_serde::AutoSerde;

use std::fs;
use std::fs::{DirEntry, File};
use std::io;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BingoItem {
	pub title: String,
	pub emoji: Option<char>,
	pub short_description: Option<String>,
	pub url: Option<String>,
	pub completion_info: Option<BingoCompletionInfo>,
}

impl BingoItem {
	pub fn vienna_samples() -> anyhow::Result<Vec<Self>> {
		// TODO: rewrite to pure iterator based code for SPEED

		let example_paths: Vec<Result<DirEntry, io::Error>> =
			fs::read_dir("../../examples/items/")?.collect(); // get list of files in examples/items directory
		let mut ans = Vec::with_capacity(example_paths.len()); // create vector preallocated with enough space

		for p in example_paths {
			let mut f = File::open(p?.path())?;
			ans.push(Self::from_ron_file(&mut f)?);
		}

		Ok(ans)
	}
}

pub mod commands {
	use super::*;

	#[tauri::command]
	pub fn example_bingo_items() -> Vec<BingoItem> {
		BingoItem::vienna_samples().unwrap()
	}
}
