use crate::*;
use alloc::vec::Vec;
use core::cell::OnceCell;
use firefly_rust as ff;

static mut STATE: OnceCell<State> = OnceCell::new();

pub type Image = Vec<u8>;
pub type Images = Vec<Image>;

pub struct State {
    pub script: bulb_parser::State,
    pub update_frame: u16,
    pub render_frame: u16,
    pub dialog_frame: u16,
    pub room_dirty: bool,
    pub held_for: u32,
    pub dpad: ff::DPad,
    pub font: ff::FileBuf,
}

fn set_state(state: State) {
    #[allow(static_mut_refs)]
    unsafe { STATE.set(state) }.ok().unwrap();
}

pub fn get_state() -> &'static mut State {
    #[allow(static_mut_refs)]
    unsafe { STATE.get_mut() }.unwrap()
}

pub fn load_state() {
    let raw = ff::load_file_buf("main").unwrap();
    let raw = alloc::str::from_utf8(raw.data()).unwrap();
    let sections = match bulb_parser::parse(raw) {
        Ok(sections) => sections,
        Err(err) => {
            let msg = alloc::format!("line {}: {}", err.row + 1, err.kind.as_str());
            ff::log_error(&msg);
            panic!();
        }
    };
    let Some(font) = ff::load_file_buf("font") else {
        panic!("font not found")
    };
    let state = State {
        script: bulb_parser::State::new(sections),
        font,
        update_frame: 0,
        render_frame: 0,
        dialog_frame: 0,
        held_for: 0,
        room_dirty: true,
        dpad: ff::DPad::default(),
    };
    set_state(state);
}
