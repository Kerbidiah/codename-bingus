PATH := "src-tauri/Cargo.toml"

run:
	cargo tauri dev

run_clean: wipe run

release:
	cargo tauri build

release_windows:
	cargo tauri build -t x86_64-pc-windows-gnu

release_signed:
	#!/usr/bin/env bash
	set -euo pipefail
	if [ ! -f .env.release ]; then
		echo "Missing .env.release — copy .env.release.example and fill in your Apple credentials"
		exit 1
	fi
	set -a
	source .env.release
	set +a
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
	cp -r examples/items ~/bingus

peek:
	ls ~/bingus/edit

wipe:
	rm -r ~/bingus/edit
	mkdir ~/bingus/edit