run:
	cargo update
	cargo build
	RUST_LOG=info cargo run
