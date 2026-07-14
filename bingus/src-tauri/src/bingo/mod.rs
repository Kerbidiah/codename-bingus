pub mod board;
pub mod completion;
pub mod item;
pub mod project;

use serde::{Deserialize, Serialize};

use crate::bingo::project::BingoProject;

#[non_exhaustive]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BingoType {
	Editable(BingoProject),
}