use serde::{Deserialize, Serialize};

use crate::bingo::item::BingoItem;

use fake::{Fake, faker};

use rand::prelude::*;
use rand::seq::SliceRandom;

#[derive(Debug, Deserialize, Serialize)]
pub struct BingoBoard {
	pub city: String,
	pub items: Vec<BingoItem>,
}

impl BingoBoard {
	pub fn fake() -> Self {
		let mut rng = rand::rng();
		let items: Vec<BingoItem> = BingoItem::vienna_samples()
			.sample(&mut rng, 25)
			.cloned()
			.collect();

		Self {
			city: faker::address::en::CityName().fake(),
			items: items,
		}
	}
}
