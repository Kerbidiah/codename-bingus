PATH := "src-tauri/Cargo.toml"

run:
	cargo tauri dev

release:
	cargo tauri build

fmt:
	cargo fmt --manifest-path {{PATH}}

doc:
	cargo doc --manifest-path {{PATH}}

clean:
	cargo clean --manifest-path {{PATH}}