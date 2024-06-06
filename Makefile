.PHONY: all run build install release clean bundle bundle-windows

all: build run

run: src
	cargo run

install:
	cargo install cargo-vcpkg
	cargo vcpkg build

build: src
	./prepare.sh debug
	cargo build --debug

release: src
	./prepare.sh release
	cargo build --release

clean: src
	cargo clean
	rm -rf target

bundle: src
	cargo bundle --release
