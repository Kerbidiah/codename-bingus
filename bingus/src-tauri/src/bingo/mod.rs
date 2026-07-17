pub mod board;
pub mod completion;
pub mod item;
pub mod play;
pub mod project;

use anyhow;

use crate::auto_serde::AutoSerde;
use crate::bingo::play::PlayableBingo;
use crate::bingo::project::BingoProject;

use std::fs;
use std::fs::File;
use std::path::Path;

#[non_exhaustive]
#[derive(Debug, Clone)]
pub enum BingoType {
	Editable(BingoProject),
	Playable(PlayableBingo),
}

const BINGO_EDIT_STORE_PATH: &str = "~/bingus/edit/";
const BINGO_PLAY_STORE_PATH: &str = "~/bingus/play/";

#[tauri::command]
pub fn get_bingos() -> anyhow::Result<Vec<BingoType>> {
	let path_edit = Path::new(BINGO_EDIT_STORE_PATH);
	let path_play = Path::new(BINGO_PLAY_STORE_PATH);

	let edit_files: Vec<BingoProject> = fs::read_dir(path_edit)?
		.filter_map(|res| res.ok().map(|dir| File::open(dir.path())))
		.filter_map(|f| f.ok().map(|x| x)) // filter out Errors and extract the value out of Oks
		.map(|mut f| BingoProject::from_ron_file(&mut f))
		.filter_map(|f| f.ok().map(|x| x)) // filter out Errors and extract the value out of Oks
		.collect();
	let play_files: Vec<PlayableBingo> = fs::read_dir(path_edit)?
		.filter_map(|res| res.ok().map(|dir| File::open(dir.path())))
		.filter_map(|f| f.ok().map(|x| x)) // filter out Errors and extract the value out of Oks
		.map(|mut f| PlayableBingo::from_ron_file(&mut f))
		.filter_map(|f| f.ok().map(|x| x)) // filter out Errors and extract the value out of Oks
		.collect();

	let mut ans = Vec::with_capacity(edit_files.len() + play_files.len());

	edit_files.iter().for_each(|project| ans.push(BingoType::Editable(project.clone())));
	play_files.iter().for_each(|play| ans.push(BingoType::Playable(play.clone())));

	Ok(ans)
}
