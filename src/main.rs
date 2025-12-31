#![no_std]
#![no_main]
extern crate alloc;

mod rendering;
mod state;

use crate::rendering::*;
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
    let state = get_state();
    render_room(state);
}
