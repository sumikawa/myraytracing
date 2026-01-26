all::
	cargo run > output.ppm
	magick convert output.ppm output.png
	open -W output.ppm

test::
	cargo check
	cargo fmt
	cargo test

lint::
	cargo clippy
