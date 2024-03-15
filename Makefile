.PHONY: all run build release clean bundle bundle-windows

all: build run

run: src
	cargo run

build: src
	cargo vcpkg build
	cargo build

release: src
	cargo build --release

clean: src
	cargo clean
	rm -rf target

bundle: src
	cargo bundle --release