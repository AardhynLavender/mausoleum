# Metroidvania

Metroidvania inspired game written Rust using the SDL2 library.

## Download

A demo of this software will eventually be available on [itch.io](https://aardhyn-lavender.itch.io/metroidvania).

For now, download the latest precompiled executable for your platform
under [Releases](https://github.com/AardhynLavender/Tetris/releases).

## Installation

For building from source.

### Rust

You will need to [Install Rust](https://www.rust-lang.org/tools/install).

### SDL2

You will need to install the SDL2 libraries.
I've had little success with the SDL3 wrapper, plus it's still technically in beta.

I've found [Microsoft VCPKG](https://github.com/microsoft/vcpkg) to be a simple (and multiplatform) method.

```bash
cargo install cargo-vcpkg
cargo vcpkg build
cargo build
```

> There are other installation methods not involving VCPKG described in
> the [SDL2-Rust Repository](https://github.com/Rust-SDL2/rust-sdl2), but I've not tested any of these.

#### SDL2 `unsafe_textures` Flag

I've enabled the `unsafe_textures` feature for the `sdl2` crate in `Cargo.toml` which omits the generic lifetime
annotations for the `sdl2::rendering::Texture` struct.

This removes the need to propagate lifetime annotations throughout the codebase when dealing with textures.
As textures are dropped before their *owning* `TextureCreator`, there is no risk of dangling references or memory
leaks.

## Compilation

Build and run the executable on your platform.

```bash
make
# or
cargo run # compile and run
```

## Bundling

Bundling is the process of packaging the application, its dependencies, and resources into a single easily shareable
file.

### MacOS

Install all the SDL2 frameworks (`.framework` directories) **SDL2**, **SDL_image**, **SDL_ttf** and **SDL_mixer**.

Make sure these are stored in `/Library/Frameworks/` or wherever you store your frameworks.

And run

```bash
cargo bundle --release
# or
make bundle
```

### Windows

Unfortunately, I've not found any ways to successfully bundle a Rust application on Windows as **cargo-bundle** does not
build `.msi` files correctly.
I've not had success with the **WiX Toolset** or either.

To ship the application, you can provide the `tetris.exe` and the `asset/` directory.

## References

See [CREDITS](./CREDITS.md).

> ChatGPT 3.5 was used to aid my understanding of algorithmic concepts and design patterns while learning Rust.
> Not for the generation of production code.

> GitHub Copilot was used to generate code I understood and would have written myself. An autocompletion tool.