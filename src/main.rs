#![no_std]
#![no_main]
extern crate alloc;

mod rendering;
mod state;
mod updating;

use crate::rendering::*;
use crate::state::*;
use crate::updating::*;

#[unsafe(no_mangle)]
extern "C" fn boot() {
    load_state();
}

#[unsafe(no_mangle)]
extern "C" fn update() {
    let state = get_state();
    update_state(state);
}

#[unsafe(no_mangle)]
extern "C" fn render() {
    let state = get_state();
    render_room(state);
}
