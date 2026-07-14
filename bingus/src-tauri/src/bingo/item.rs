use serde::{Deserialize, Serialize};

use anyhow;
use ron;

use super::completion::BingoCompletionInfo;

use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

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

		let example_paths: Vec<Result<fs::DirEntry, io::Error>> =
			fs::read_dir("../../examples/items/")?.collect(); // get list of files in examples/items directory
		let mut ans = Vec::with_capacity(example_paths.len()); // create vector preallocated with enough space

		for p in example_paths {
			let mut f = File::open(p?.path())?;
			let mut buf = String::new();
			f.read_to_string(&mut buf)?;

			drop(f);

			ans.push(BingoItem::from_ron(&buf)?);
		}

		Ok(ans)
	}

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
}

pub mod commands {
	use super::*;

	#[tauri::command]
	pub fn example_bingo_items() -> Vec<BingoItem> {
		BingoItem::vienna_samples().unwrap()
	}
}