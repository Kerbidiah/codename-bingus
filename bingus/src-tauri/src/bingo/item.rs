use serde::{Deserialize, Serialize};

use super::completion::BingoCompletion as BingoCompletionInfo;

use fake::{Fake, faker};

#[derive(Debug, Deserialize, Serialize)]
pub struct BingoItem {
	pub title: String,
	pub emoji: Option<char>,
	pub short_description: Option<String>,
	pub long_description: Option<String>,
	pub url: Option<String>,
	pub completion_info: Option<BingoCompletionInfo>,
}

impl BingoItem {
	pub fn vienna_samples() -> [Self; 3] {
		[
			// 🌭 Eat a Käsekrainer or a döner / falafel wrap
			Self {
				title: "Eat a Käsekrainer".to_string(),
				emoji: Some('🌭'),
				short_description: Some("or a döner / falafel wrap".to_string()),
				long_description: None,
				url: Some("https://www.wien.info/en/dine-drink/viennese-cuisine/hot-dog-stands-348128".to_string()),
				completion_info: None,
			},
			// 🎻 Find the golden Strauss
			Self {
				title: "Find the golden Strauss".to_string(),
				emoji: Some('🎻'),
				short_description: None,
				long_description: None,
				url: Some("https://www.wien.info/en/art-culture/music-stage-shows/strauss-monument-360274".to_string()),
				completion_info: None,
			},
			// 🫙 Sample food at the Brunnenmarkt
			Self {
				title: "Sample food at the Brunnenmarkt".to_string(),
				emoji: Some('🫙'),
				short_description: None,
				long_description: None,
				url: Some("https://www.wien.info/en/dine-drink/markets/yppenplatz-restaurants-366538".to_string()),
				completion_info: None,
			},
		]
	}
}
