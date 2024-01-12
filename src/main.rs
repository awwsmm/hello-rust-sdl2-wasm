extern crate sdl2;

use std::cell::RefCell;
use std::rc::Rc;
use std::thread::sleep;
use std::time::Duration;
use sdl2::rect::Rect;
use hello_rust_sdl2_wasm::main_loop;

// this demo is based on
// https://puddleofcode.com/story/definitive-guide-to-rust-sdl2-and-emscriptem/

// I used this to get started, but basically all of it had to change
// https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_Wasm

// downloaded this, but I don't think it's needed
// https://github.com/emscripten-core/emsdk

// TODO to run locally: cargo run
// TODO to run on web:
//     export EMCC_CFLAGS="-s USE_SDL=2"
//     cargo build --target asmjs-unknown-emscripten && open index.html

// need to add this target first with rustup target add asmjs-unknown-emscripten

fn main() {
    let ctx = sdl2::init().unwrap();
    let video_ctx = ctx.video().unwrap();

    let window  = match video_ctx
        .window("rust_to_js", 640, 480)
        .position_centered()
        .opengl()
        .build() {
        Ok(window) => window,
        Err(err)   => panic!("failed to create window: {}", err)
    };

    let mut canvas = match window
        .into_canvas()
        .present_vsync()
        .build() {
        Ok(canvas) => canvas,
        Err(err) => panic!("failed to create canvas: {}", err)
    };

    let mut rect = Rect::new(0, 0, 10, 10);

    // see https://stackoverflow.com/a/58599995/2925434
    let ctx = Rc::new(RefCell::new(ctx));
    let rect = Rc::new(RefCell::new(rect));
    let canvas = Rc::new(RefCell::new(canvas));

    // changed this from (target_os = "emscripten") because I'm not sure if that's still valid
    // https://doc.rust-lang.org/reference/conditional-compilation.html
    // https://doc.rust-lang.org/nightly/rustc/platform-support.html
    #[cfg(target_family = "wasm")]
    use hello_rust_sdl2_wasm::emscripten;

    println!("made it here");

    #[cfg(target_family = "wasm")]
    emscripten::set_main_loop_callback(main_loop(Rc::clone(&ctx), Rc::clone(&rect), Rc::clone(&canvas)));

    println!("made it here 2");

    #[cfg(not(target_family = "wasm"))]
    loop {
        main_loop(Rc::clone(&ctx), Rc::clone(&rect), Rc::clone(&canvas))();
        sleep(Duration::from_millis(10))
    }
}