run:
	cargo tauri dev

release:
	cargo tauri build

fmt:
	cargo fmt --manifest-path bingus/src-tauri/Cargo.toml

doc:
	cargo doc --manifest-path bingus/src-tauri/Cargo.toml