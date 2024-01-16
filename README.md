# hello-rust-sdl2-wasm

A minimal working "game" written in Rust with SDL2, compiled to WASM.

![a screen recording of the "game"](https://raw.githubusercontent.com/awwsmm/hello-rust-sdl2-wasm/master/demo.gif)

Note: these instructions are written for macOS and Ubuntu. If you'd like to submit instructions for any other OS, please open a PR.

## Prerequisites

- [`rustup`](https://rustup.rs/) must be installed
- [`git`](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git) must be installed

### install [the SDL2 development libraries](https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#requirements)

On macOS, this is as easy as

```shell
brew install sdl2
```

Make sure these libraries are on your `LIBRARY_PATH`, as well. If you installed with `brew`, run

```shell
export LIBRARY_PATH="$LIBRARY_PATH:$(brew --prefix)/lib"
```

So you don't have to do this in each new shell you open, maybe also add this to your `~/.bashrc` or `~/.zshrc` (or other shell startup script).

On Ubuntu:

```shell
sudo apt install libsdl2-dev
```
(setting `LIBRARY_PATH` is not needed on Ubuntu)

### clone this repo and `cd` into it

```shell
git clone https://github.com/awwsmm/hello-rust-sdl2-wasm.git && cd hello-rust-sdl2-wasm
```

## Building locally

To build a desktop (standalone binary executable) app, run

```shell
cargo run
```

## Building for WASM

To build the WASM app instead, there are a few more setup steps.

### install the target architecture toolchain

```shell
rustup target add asmjs-unknown-emscripten
```

We use [emscripten](https://emscripten.org/) to convert Rust LLVM bytecode to WASM.

### install the `emscripten` compiler

On MacOS:

```shell
brew install emscripten
```

(This will take a few minutes.)

On Ubuntu, follow emscripten's [recommended installation instructions](https://emscripten.org/docs/getting_started/downloads.html):

```shell
git clone https://github.com/emscripten-core/emsdk.git && cd emsdk
./emsdk install latest
./emsdk activate latest
source ./emsdk_env.sh
```

### set this one environment variable

```shell
export EMCC_CFLAGS="-s USE_SDL=2"
```

### build

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
