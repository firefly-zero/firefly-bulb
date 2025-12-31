#![no_std]
#![no_main]
extern crate alloc;

mod state;

use crate::state::*;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    load_state();
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    // ...
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    // ...
}
