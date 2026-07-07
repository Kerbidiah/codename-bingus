use std::time::Instant;

use serde::{Serialize, Deserialize};

use crate::bingo::item::BingoItem;


#[derive(Debug, Deserialize, Serialize)]
pub struct BingoBoard {
	pub city: String,
	pub items: [BingoItem; 25],
}