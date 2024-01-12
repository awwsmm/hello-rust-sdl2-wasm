use std::cell::RefCell;
use std::process;
use std::rc::Rc;

use sdl2::event::Event;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

#[cfg(target_family = "wasm")]
pub mod emscripten;

pub fn main_loop(ctx: Rc<RefCell<Sdl>>, rect: Rc<RefCell<Rect>>, canvas: Rc<RefCell<WindowCanvas>>) -> impl FnMut() {

    let mut events = ctx.borrow_mut().event_pump().unwrap();

    move || {

        let black = sdl2::pixels::Color::RGB(0, 0, 0);
        let white = sdl2::pixels::Color::RGB(255, 255, 255);

        for event in events.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    process::exit(1);
                },
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    rect.borrow_mut().x -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    rect.borrow_mut().x += 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                    rect.borrow_mut().y -= 10;
                },
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    rect.borrow_mut().y += 10;
                },
                _ => {}
            }
        }

        let _ = canvas.borrow_mut().set_draw_color(black);
        let _ = canvas.borrow_mut().clear();
        let _ = canvas.borrow_mut().set_draw_color(white);
        let _ = canvas.borrow_mut().fill_rect(rect.borrow().clone());
        let _ = canvas.borrow_mut().present();
    }

}