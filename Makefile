build_exe:
	cargo build --release

build:
	cargo run --release main.morth > main.fasm
	fasm main.fasm main

run:
	./main
