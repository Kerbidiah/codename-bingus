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

#[non_exhaustive]
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
pub fn get_bingo_projects() -> Vec<BingoProject> {
	let path_edit = resolve_path(BINGO_EDIT_PATH);

	fs::read_dir(path_edit)
		.unwrap()
		.filter_map(|res| res.ok().map(|dir| File::open(dir.path())))
		.filter_map(|f| f.ok().map(|x| x)) // filter out Errors and extract the value out of Oks
		.map(|mut f| BingoProject::from_file(&mut f))
		.filter_map(|f| f.ok().map(|x| x)) // filter out Errors and extract the value out of Oks
		.collect()
}

#[tauri::command]
pub fn get_bingo_games() -> Vec<PlayableBingo> {
	let path_play = resolve_path(BINGO_PLAY_PATH);

	fs::read_dir(path_play)
		.unwrap()
		.filter_map(|res| res.ok().map(|dir| File::open(dir.path())))
		.filter_map(|f| f.ok().map(|x| x)) // filter out Errors and extract the value out of Oks
		.map(|mut f| PlayableBingo::from_file(&mut f))
		.filter_map(|f| f.ok().map(|x| x)) // filter out Errors and extract the value out of Oks
		.collect()
}