pub mod board;
pub mod completion;
pub mod game;
pub mod item;
pub mod project;

use dirs;
use log::info;
use open;

use crate::auto_serde::AutoSerde;
use crate::bingo::game::BingoGame;
use crate::bingo::project::BingoProject;

use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::io::Write;

const BINGO_EDIT_PATH: &str = "bingus/edit/";
const BINGO_PLAY_PATH: &str = "bingus/edit/"; // look... this is kinda cursed, but it works because get_bingo_projects
// and get_bingo_play are different types and they ignore any files they don't understand

// please forgive me for my sins o Rust gods.

pub fn resolve_path(relative: &str) -> PathBuf {
	let mut path = dirs::home_dir().expect("could not determine home directory");
	path.push(relative);
	path
}

#[tauri::command]
pub fn get_bingo_projects() -> Vec<(BingoProject, String)> {
	info!("get_bingo_projects ran");
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
				let g = f.0.unwrap();
				let t = g.title.clone();
				info!("get_bingo_projects t: {t}");
				Some((g, f.1))
			} else {
				None
			}
		}) // filter out Errors and extract the value out of Oks
		.collect()
}

#[tauri::command]
pub fn get_bingo_games() -> Vec<(BingoGame, String)> {
	info!("get_bingo_games ran");
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
		.map(|mut f| (BingoGame::from_file(&mut f.0), f.1))
		.filter_map(|f| {
			if f.0.is_ok() {
				let g = f.0.unwrap();
				let t = g.board.title.clone();
				info!("get_bingo_games t: {t}");
				Some((g, f.1))
			} else {
				None
			}
		}) // filter out Errors and extract the value out of Oks
		.collect()
}

#[tauri::command]
pub fn convert_proj_path_to_game_path(input: String) -> String {
	info!("convert_proj_path_to_game_path ran");
	let mut p: PathBuf = input.into();

	let c = p.clone();
	let name = c.file_prefix().unwrap();
	p.pop();
	p.push(name);
	p.set_extension("BingoGame");

	p.to_str().unwrap().to_string()
}

#[tauri::command]
pub fn quick_export(proj_path: String) {
	info!("quick_export ran");
	info!("quick_export proj_path: {proj_path}");
	let proj = BingoProject::open(proj_path.clone()).unwrap();

	let game_path = convert_proj_path_to_game_path(proj_path);
	info!("quick_export game_path: {game_path}");

	let game: BingoGame = if proj.last_board.is_none() {
		BingoGame {
			board: proj.generate_random_board(),
		}
	} else if proj.last_board.clone().unwrap().len() == 0 {
		BingoGame {
			board: proj.generate_random_board(),
		}
	} else {
		BingoGame {
			board: proj.last_board.unwrap(),
		}
	};

	game.write(game_path).unwrap();
}

#[tauri::command]
pub fn delete(path: String) {
	info!("delete ran");

	if path.contains("bingus") {
		info!("delete p: {path}");
		fs::remove_file(&path).unwrap();
	}
}

#[tauri::command]
pub fn export_html(path: String, html: String){
	info!("export_html ran");
	info!("path: {path}");
	
	let mut f = File::create(&path).unwrap();
	let buf = html.into_bytes();
	f.write_all(&buf).unwrap();

	open::that_detached(path).unwrap();
}
