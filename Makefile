all::
	cargo run --release
	open -W output.png

test::
	cargo check
	cargo fmt
	cargo test

lint::
	cargo clippy

time::
	@date
	cargo run --release
	@date
