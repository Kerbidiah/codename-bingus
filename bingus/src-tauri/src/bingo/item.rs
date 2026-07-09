use serde::{Deserialize, Serialize};

use ron;
use anyhow;

use super::completion::BingoCompletion as BingoCompletionInfo;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BingoItem {
	pub title: String,
	pub emoji: Option<char>,
	pub short_description: Option<String>,
	pub url: Option<String>,
	pub completion_info: Option<BingoCompletionInfo>,
}

impl BingoItem {
	pub fn vienna_samples() -> Vec<Self> {
		vec![
			// 🌭 Eat a Käsekrainer or a döner / falafel wrap
			Self {
				title: "Eat a Käsekrainer".to_string(),
				emoji: Some('🌭'),
				short_description: Some("or a döner / falafel wrap".to_string()),
				url: Some("https://www.wien.info/en/dine-drink/viennese-cuisine/hot-dog-stands-348128".to_string()),
				completion_info: None,
			},
			// 🎻 Find the golden Strauss
			Self {
				title: "Find the golden Strauss".to_string(),
				emoji: Some('🎻'),
				short_description: None,
				url: Some("https://www.wien.info/en/art-culture/music-stage-shows/strauss-monument-360274".to_string()),
				completion_info: None,
			},
			// 🫙 Sample food at the Brunnenmarkt
			Self {
				title: "Sample food at the Brunnenmarkt".to_string(),
				emoji: Some('🫙'),
				short_description: None,
				url: Some("https://www.wien.info/en/dine-drink/markets/yppenplatz-restaurants-366538".to_string()),
				completion_info: None,
			},
			// ☕ Order a Melange in a traditional Kaffeehaus
			Self {
				title: "Order a Melange in a traditional Kaffeehaus".to_string(),
				emoji: Some('☕'),
				short_description: Some("Café Central or Café Sperl are classics".to_string()),
				url: None,
				completion_info: None,
			},
			// 🍰 Try a slice of Sachertorte
			Self {
				title: "Try a slice of Sachertorte".to_string(),
				emoji: Some('🍰'),
				short_description: Some("at Café Sacher or Demel".to_string()),
				url: None,
				completion_info: None,
			},
			// ⛪ Climb the South Tower of Stephansdom
			Self {
				title: "Climb the South Tower of Stephansdom".to_string(),
				emoji: Some('⛪'),
				short_description: Some("343 steps for a view over the rooftops".to_string()),
				url: None,
				completion_info: None,
			},
			// 🎡 Ride the Wiener Riesenrad
			Self {
				title: "Ride the Wiener Riesenrad".to_string(),
				emoji: Some('🎡'),
				short_description: Some("the giant Ferris wheel in the Prater".to_string()),
				url: None,
				completion_info: None,
			},
			// 🏰 Visit Schönbrunn Palace
			Self {
				title: "Visit Schönbrunn Palace".to_string(),
				emoji: Some('🏰'),
				short_description: Some("stroll the gardens up to the Gloriette".to_string()),
				url: None,
				completion_info: None,
			},
			// 🐎 Watch the Lipizzaner stallions
			Self {
				title: "Watch the Lipizzaner stallions".to_string(),
				emoji: Some('🐎'),
				short_description: Some("at the Spanish Riding School".to_string()),
				url: None,
				completion_info: None,
			},
			// 🏠 See the Hundertwasserhaus
			Self {
				title: "See the Hundertwasserhaus".to_string(),
				emoji: Some('🏠'),
				short_description: Some("Vienna's most colorful apartment building".to_string()),
				url: None,
				completion_info: None,
			},
			// 🎨 Explore the MuseumsQuartier
			Self {
				title: "Explore the MuseumsQuartier".to_string(),
				emoji: Some('🎨'),
				short_description: Some("grab a spot on the Enzi loungers outside".to_string()),
				url: None,
				completion_info: None,
			},
			// 🖼️ See Klimt's "The Kiss" at the Belvedere
			Self {
				title: "See Klimt's \"The Kiss\" at the Belvedere".to_string(),
				emoji: Some('🖼'),
				short_description: None,
				url: None,
				completion_info: None,
			},
			// 🍷 Spend an evening at a Heuriger
			Self {
				title: "Spend an evening at a Heuriger".to_string(),
				emoji: Some('🍷'),
				short_description: Some("a wine tavern out in Grinzing or Nussdorf".to_string()),
				url: None,
				completion_info: None,
			},
			// 🎭 Catch a show at the Vienna State Opera
			Self {
				title: "Catch a show at the Vienna State Opera".to_string(),
				emoji: Some('🎭'),
				short_description: Some("standing-room tickets are cheap and easy".to_string()),
				url: None,
				completion_info: None,
			},
			// 🥨 Buy a fresh pretzel from a market stall
			Self {
				title: "Buy a fresh pretzel from a market stall".to_string(),
				emoji: Some('🥨'),
				short_description: None,
				url: None,
				completion_info: None,
			},
			// 🛍️ Browse the Naschmarkt
			Self {
				title: "Browse the Naschmarkt".to_string(),
				emoji: Some('🛍'),
				short_description: Some("Vienna's biggest and liveliest market".to_string()),
				url: None,
				completion_info: None,
			},
			// 🚋 Ride the historic tram around the Ringstrasse
			Self {
				title: "Ride the historic tram around the Ringstrasse".to_string(),
				emoji: Some('🚋'),
				short_description: Some("Tram 1 or 2 circles the old city walls".to_string()),
				url: None,
				completion_info: None,
			},
			// 🌳 Relax on Danube Island
			Self {
				title: "Relax on Danube Island".to_string(),
				emoji: Some('🌳'),
				short_description: Some("Donauinsel, Vienna's summer playground".to_string()),
				url: None,
				completion_info: None,
			},
			// 🗼 Get a bird's-eye view from the Donauturm
			Self {
				title: "Get a bird's-eye view from the Donauturm".to_string(),
				emoji: Some('🗼'),
				short_description: Some("the Danube Tower revolving restaurant".to_string()),
				url: None,
				completion_info: None,
			},
			// 🌹 Wander the Volksgarten rose garden
			Self {
				title: "Wander the Volksgarten rose garden".to_string(),
				emoji: Some('🌹'),
				short_description: None,
				url: None,
				completion_info: None,
			},
			// 🎬 Follow The Third Man film locations
			Self {
				title: "Follow The Third Man film locations".to_string(),
				emoji: Some('🎬'),
				short_description: Some("including the famous sewer tour".to_string()),
				url: None,
				completion_info: None,
			},
			// 🍽️ Eat a proper Wiener Schnitzel
			Self {
				title: "Eat a proper Wiener Schnitzel".to_string(),
				emoji: Some('🍽'),
				short_description: Some("Figlmüller is a famous spot for it".to_string()),
				url: None,
				completion_info: None,
			},
			// 🕯️ Visit the Zentralfriedhof
			Self {
				title: "Visit the Zentralfriedhof".to_string(),
				emoji: Some('🕯'),
				short_description: Some("Vienna's Central Cemetery, resting place of Beethoven and Strauss".to_string()),
				url: None,
				completion_info: None,
			},
			// 🎪 Wander the Wurstelprater funfair
			Self {
				title: "Wander the Wurstelprater funfair".to_string(),
				emoji: Some('🎪'),
				short_description: Some("rides, games, and old-school carnival charm".to_string()),
				url: None,
				completion_info: None,
			},
			// 🐴 Take a Fiaker ride through the Altstadt
			Self {
				title: "Take a Fiaker ride through the Altstadt".to_string(),
				emoji: Some('🐴'),
				short_description: Some("the classic horse-drawn carriage tour".to_string()),
				url: None,
				completion_info: None,
			},
			// 🕌 Admire Karlskirche's dome
			Self {
				title: "Admire Karlskirche's dome".to_string(),
				emoji: Some('🕌'),
				short_description: Some("take the elevator up close to the frescoes".to_string()),
				url: None,
				completion_info: None,
			},
			// 🎨 Visit the 'Woman in a Green Hat'
			Self {
				title: "Visit the 'Woman in a Green Hat'".to_string(),
				emoji: Some('🎨'),
				short_description: None,
				url: Some("https://www.albertina.at/en/exhibitions/monet-to-picasso/".to_string()),
				completion_info: None,
			},
			// 🚦 Discover a diversity-themed traffic light
			Self {
				title: "Discover a diversity-themed traffic light".to_string(),
				emoji: Some('🚦'),
				short_description: None,
				url: Some("https://www.wien.gv.at/en/transportation/diversity-themed-traffic-lights".to_string()),
				completion_info: None,
			},
			// 🐢 Visit the turtles at Haus des Meeres
			Self {
				title: "Visit the turtles at Haus des Meeres".to_string(),
				emoji: Some('🐢'),
				short_description: None,
				url: Some("https://www.haus-des-meeres.at/en/zoo/puppigeschichte".to_string()),
				completion_info: None,
			},
			// 🎹 Find Beethoven's grave
			Self {
				title: "Find Beethoven's grave".to_string(),
				emoji: Some('🎹'),
				short_description: None,
				url: Some("https://www.visitingvienna.com/footsteps/beethovens-grave/".to_string()),
				completion_info: None,
			},
			// 🏛️ Visit an Otto Wagner site
			Self {
				title: "Visit an Otto Wagner site".to_string(),
				emoji: Some('🏛'),
				short_description: None,
				url: Some("https://www.visitingvienna.com/footsteps/otto-wagner-in-vienna/".to_string()),
				completion_info: None,
			},
			// 🛍️ Discover the shops on Spittelberggasse
			Self {
				title: "Discover the shops on Spittelberggasse".to_string(),
				emoji: Some('🛍'),
				short_description: None,
				url: Some("https://spittelberg.at/en/arts-and-crafts/".to_string()),
				completion_info: None,
			},
			// 🥤 Drink an Almdudler
			Self {
				title: "Drink an Almdudler".to_string(),
				emoji: Some('🥤'),
				short_description: None,
				url: Some("https://almdudler.com/en/".to_string()),
				completion_info: None,
			},
			// 🍫 Eat a Mozartkugel
			Self {
				title: "Eat a Mozartkugel".to_string(),
				emoji: Some('🍫'),
				short_description: None,
				url: Some("https://www.timetravel-vienna.at/en/the-history-of-the-original-mozartkugel/".to_string()),
				completion_info: None,
			},
			// 🏛️ Visit the Pestsäule on the Graben
			Self {
				title: "Visit the Pestsäule on the Graben".to_string(),
				emoji: Some('🏛'),
				short_description: None,
				url: Some("https://www.visitingvienna.com/sights/plague-column-pestsaule/".to_string()),
				completion_info: None,
			},
			// 🍦 Cool off with gelato
			Self {
				title: "Cool off with gelato".to_string(),
				emoji: Some('🍦'),
				short_description: None,
				url: Some("https://www.zanoni.co.at/eiskarte/coppas/".to_string()),
				completion_info: None,
			},
			// 🥾 Take a hike in the Vienna Woods or countryside
			Self {
				title: "Take a hike in the Vienna Woods or countryside".to_string(),
				emoji: Some('🥾'),
				short_description: None,
				url: Some("https://www.wienerwald.info/en/".to_string()),
				completion_info: None,
			},
			// 🚂 Take a side trip
			Self {
				title: "Take a side trip".to_string(),
				emoji: Some('🚂'),
				short_description: Some("Krems, Graz, Znojmo…".to_string()),
				url: Some("https://www.austria.info/en-gb/inspiration/day-trips-from-vienna/".to_string()),
				completion_info: None,
			},
		]
	}

	/// Serialize to RON format
	pub fn to_ron(&self) -> anyhow::Result<String> {
		// this looks a little weird, but it to convert from ron::result to anyhow::result
		// anyhow result is awesome and lets us mix all the errors into 1 type
		Ok(ron::to_string(self)?)
	}

	/// Deserialize from RON format
	pub fn from_ron(input: &str) -> anyhow::Result<Self> {
		// this looks a little weird, but it to convert from ron::result to anyhow::result
		// anyhow result is awesome and lets us mix all the errors into 1 type
		Ok(ron::from_str(input)?)
	}
}
