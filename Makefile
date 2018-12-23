SHELL := /bin/bash
COMPILER = rustc
COMPILER_FLAGS = -O
RUSTDOC = rustdoc
UPX := $(shell command -v upx 2> /dev/null)

.PHONY: all
all:
	cargo build --release 
	strip target/release/matrix-rs
ifdef UPX
		upx target/release/matrix-rs
endif
	cargo install --path . 

.PHONY: build
build:
	cargo build --release 
	strip target/release/matrix-rs
	upx target/release/matrix-rs 

.PHONY: run
run:
	cargo run

.PHONY: install
install:
	cargo install --path .

.PHONY: clean
clean:
	cargo clean