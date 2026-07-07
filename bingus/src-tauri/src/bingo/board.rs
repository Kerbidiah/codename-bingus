use std::time::Instant;

use serde::{Deserialize, Serialize};

use crate::bingo::item::BingoItem;

#[derive(Debug, Deserialize, Serialize)]
pub struct BingoBoard {
	pub city: String,
	pub items: [BingoItem; 25],
}
