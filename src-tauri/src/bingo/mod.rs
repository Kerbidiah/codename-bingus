pub mod board;
pub mod completion;
pub mod item;
pub mod play;
pub mod project;

use serde::{Deserialize, Serialize};

use dirs;

use crate::auto_serde::AutoSerde;
use crate::bingo::play::PlayableBingo;
use crate::bingo::project::BingoProject;

use std::fs;
use std::fs::File;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BingoType {
	Editable(BingoProject),
	Playable(PlayableBingo),
}

const BINGO_EDIT_PATH: &str = "bingus/edit/";
const BINGO_PLAY_PATH: &str = "bingus/play/";

fn resolve_path(relative: &str) -> PathBuf {
	let mut path = dirs::home_dir().expect("could not determine home directory");
	path.push(relative);
	path
}

#[tauri::command]
pub fn get_bingo_projects() -> Vec<(BingoProject, String)> {
	let path_edit = resolve_path(BINGO_EDIT_PATH);

	fs::read_dir(path_edit)
		.unwrap()
		.filter_map(|res| {
			res.ok().map(|dir| {
				let p = dir.path();
				(File::open(&p), p.to_str().unwrap().to_string())
			})
		})
		.filter_map(|f| {
			if f.0.is_ok() {
				Some((f.0.unwrap(), f.1))
			} else {
				None
			}
		}) // filter out Errors and extract the value out of Oks
		.map(|mut f| (BingoProject::from_file(&mut f.0), f.1))
		.filter_map(|f| {
			if f.0.is_ok() {
				Some((f.0.unwrap(), f.1))
			} else {
				None
			}
		}) // filter out Errors and extract the value out of Oks
		.collect()
}

#[tauri::command]
pub fn get_bingo_games() -> Vec<(PlayableBingo, String)> {
	let path_play = resolve_path(BINGO_PLAY_PATH);

	fs::read_dir(path_play)
		.unwrap()
		.filter_map(|res| {
			res.ok().map(|dir| {
				let p = dir.path();
				(File::open(&p), p.to_str().unwrap().to_string())
			})
		})
		.filter_map(|f| {
			if f.0.is_ok() {
				Some((f.0.unwrap(), f.1))
			} else {
				None
			}
		}) // filter out Errors and extract the value out of Oks
		.map(|mut f| (PlayableBingo::from_file(&mut f.0), f.1))
		.filter_map(|f| {
			if f.0.is_ok() {
				Some((f.0.unwrap(), f.1))
			} else {
				None
			}
		}) // filter out Errors and extract the value out of Oks
		.collect()
}