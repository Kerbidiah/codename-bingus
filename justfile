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

dir_setup:
	mkdir ~/bingus
	mkdir ~/bingus/edit
	mkdir ~/bingus/play
	cp examples/projects/TEST.BingoProject ~/bingus/edit/TEST.BingoProject
