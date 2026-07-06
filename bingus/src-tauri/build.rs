fn main() -> std::io::Result<()> {
	prost_build::compile_protos(
		&["proto/bingo_item.proto", "proto/bingo_board.proto"],
		&["proto/"], // include path, so `import "bingo_item.proto"` resolves
	)?;

	tauri_build::build();

	Ok(())
}
