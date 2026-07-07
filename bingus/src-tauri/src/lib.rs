pub mod bingo;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
	format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn generate_board() -> Vec<Vec<String>> {
	// FIXME:
	return vec![
		vec![
			"Cell 1".into(),
			"Cell 2".into(),
			"Cell 3".into(),
			"Cell 4".into(),
			"Cell 5".into(),
		],
		vec![
			"Cell 6".into(),
			"Cell 7".into(),
			"Cell 8".into(),
			"Cell 9".into(),
			"Cell 10".into(),
		],
		vec![
			"Cell 11".into(),
			"Cell 12".into(),
			"Cell 13".into(),
			"Cell 14".into(),
			"Cell 15".into(),
		],
		vec![
			"Cell 16".into(),
			"Cell 17".into(),
			"Cell 18".into(),
			"Cell 19".into(),
			"Cell 20".into(),
		],
		vec![
			"Cell 21".into(),
			"Cell 22".into(),
			"Cell 23".into(),
			"Cell 24".into(),
			"Cell 25".into(),
		],
	];
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.plugin(tauri_plugin_opener::init())
		.invoke_handler(tauri::generate_handler![greet, generate_board])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
