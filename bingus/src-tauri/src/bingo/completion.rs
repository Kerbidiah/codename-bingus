use serde::{Deserialize, Serialize};

use std::time::SystemTime;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BingoCompletion {
	pub notes: String,
	pub asset_ids: Vec<u16>,
	pub time: Option<SystemTime>,
}
