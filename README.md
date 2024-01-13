# hello-rust-sdl2-wasm

A minimal working "game" written in Rust with SDL2, compiled to WASM.

## Prerequisites

### install [the SDL2 development libraries](https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#requirements)

On macOS, this is as easy as

```shell
brew install sdl2
```

### install [Rust](https://rustup.rs/)

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### install [git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) and clone this repo

```shell
git clone https://github.com/awwsmm/hello-rust-sdl2-wasm.git
```

### install the target architecture toolchain

```shell
rustup target add asmjs-unknown-emscripten
```

We use [emscripten](https://emscripten.org/) to convert Rust LLVM bytecode to WASM.

### set this one environment variable

```shell
export EMCC_CFLAGS="-s USE_SDL=2"
```

## Building

To build a desktop (standalone binary executable) app, run

```shell
cargo run
```

within this repo (in the `hello-rust-sdl2-wasm` directory).

To build the WASM app instead, run

```shell
cargo build --target asmjs-unknown-emscripten
```

Open the browser app with

```shell
open index.html
```

## Resources

This work is largely based on [Michał Kalbarczyk's blog post here](https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/), with some help from [Greg Buchholz's message board post here](https://users.rust-lang.org/t/sdl2-emscripten-asmjs-and-invalid-renderer-panic/66567/2), which is itself based on [Tobias Widlund's work here](https://github.com/therocode/rust_emscripten_main_loop). Thanks, all!

See [Mozilla's Rust to WASM guide](https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm) if you want a simpler Rust-to-WASM example (which doesn't use SDL2).