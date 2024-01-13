// SOURCE: https://users.rust-lang.org/t/sdl2-emscripten-asmjs-and-invalid-renderer-panic/66567/2

// Based on emscripten.rs from https://github.com/therocode/rust_emscripten_main_loop
// This file interacts with the Emscripten API to provide a scheduling mechanism for main looping.

// Since Emscripten only schedules the looping to be executed later by the browser, we need to make sure that the
// data object looped upon lives as long as the looping is scheduled, as well as being properly destroyed afterwards.

// The Emscripten function used for this is emscripten_set_main_loop which will do the scheduling as well as terminate the current code flow
// to prevent scopes from being exited which would cause objects to be destroyed prematurely. To be able to destroy the data object properly
// as looping is terminated, the object is stored in thread_local storage.

use std::cell::RefCell;
use std::os::raw::c_int;

// Declare our FFI to the Emscripten functions we need. These will be linked in when building for Emscripten targets.
#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn();

extern "C" {
    pub fn emscripten_set_main_loop(
        func: em_callback_func,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
    pub fn emscripten_cancel_main_loop();
}

thread_local! {
    // This is where the data object will be kept during the scheduled looping. The storage structure is justified as follows

    // thread_local - we need it outside of function scope. thread_local is enough since we only expect interactions from the same thread.
    // RefCell<..> - allows for mutable access from anywhere which we need to store and then terminate. Still borrow-checked in runtime.
    // Option<..> - we don't always have anything scheduled
    // Box<dyn ...> - make it work generically for any closure passed in

    static MAIN_LOOP_CLOSURE: RefCell<Option<Box<dyn FnMut()>>> = RefCell::new(None);
}

// Schedules the given callback to be run over and over in a loop until it returns MainLoopEvent::Terminate.
// Retains ownership of the passed callback
pub fn set_main_loop_callback<F: FnMut() + 'static>(callback: F) {
    // Move the callback into the data storage for safe-keeping
    MAIN_LOOP_CLOSURE.with(|d| {
        *d.borrow_mut() = Some(Box::new(callback));
    });

    // Define a wrapper function that is compatible with the emscripten_set_main_loop function.
    // This function will take care of extracting and executing our closure.
    unsafe extern "C" fn wrapper<F: FnMut()>() {
        // Access and run the stashed away closure
        MAIN_LOOP_CLOSURE.with(|z| {
            if let Some(closure) = &mut *z.borrow_mut() {
                (*closure)();
            }
        });
    }

    // Schedule the above wrapper function to be called regularly with Emscripten
    unsafe {
        emscripten_set_main_loop(wrapper::<F>, 0, 1);
    }
}

// This is used to de-schedule the main loop function and destroy the kept closure object
pub fn cancel_main_loop() {
    // De-schedule
    unsafe {
        emscripten_cancel_main_loop();
    }

    // Remove the stored closure object
    MAIN_LOOP_CLOSURE.with(|d| {
        *d.borrow_mut() = None;
    });
}