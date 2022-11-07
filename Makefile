BUILD_MODE ?= --release
FILE_PATH ?= main.morth
OUT_PATH ?= main.fasm

build_exe:
	cargo build $(BUILD_MODE)

build:
	cargo run $(BUILD_MODE) $(FILE_PATH) > $(OUT_PATH)
	fasm $(OUT_PATH) main

run:
	./main
