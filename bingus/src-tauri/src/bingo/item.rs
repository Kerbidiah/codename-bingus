use serde::{Serialize, Deserialize};

use super::completion::BingoCompletion as BingoCompletionInfo;

use fake::{faker, Fake};

#[derive(Debug, Deserialize, Serialize)]
pub struct BingoItem {
	pub title: String,
	pub emoji: Option<char>,
	pub short_description: Option<String>,
	pub long_description: Option<String>,
	pub url: Option<String>,
	pub completion_info: Option<BingoCompletionInfo>
}

impl BingoItem {
	const VIENNA_SAMPLES: [Self; 3] = [
		// 🌭 Eat a Käsekrainer or a döner / falafel wrap
		Self {
			title: "Eat a Käsekrainer",
			emoji: Option<'🌭'>,
			short_description: Some(String::from("or a döner / falafel wrap")),
			long_description: None,
			url: Some(String::from("https://www.wien.info/en/dine-drink/viennese-cuisine/hot-dog-stands-348128")),
			completion_info: None,
		},
		// 🎻 Find the golden Strauss
		Self {
			title: "Find the golden Strauss",
			emoji: Option<'🎻'>,
			short_description: None,
			long_description: None,
			url: Some(String::from("https://www.wien.info/en/art-culture/music-stage-shows/strauss-monument-360274")),
			completion_info: None,
		},
		// 🫙 Sample food at the Brunnenmarkt
		Self {
			title: "Sample food at the Brunnenmarkt",
			emoji: Option<'🫙'>,
			short_description: None,
			long_description: None,
			url: Some(String::from("https://www.wien.info/en/dine-drink/markets/yppenplatz-restaurants-366538")),
			completion_info: None,
		},
	]
}