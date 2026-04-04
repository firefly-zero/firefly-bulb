use crate::*;
use alloc::string::String;
use core::cell::OnceCell;
use firefly_rust as ff;

static mut STATE: OnceCell<State> = OnceCell::new();

pub struct State {
    pub script: bulb_parser::State,
    pub held_for: u32,
    pub dpad: ff::DPad4,
    pub font: ff::FileBuf,
    pub atlas: ff::FileBuf,
    pub dirty: bool,
    pub msg: Option<String>,
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
        ff::log_error("font not found");
        panic!();
    };
    let Some(atlas) = ff::load_file_buf("atlas") else {
        ff::log_error("atlas not found");
        panic!();
    };
    let mut state = State {
        script: bulb_parser::State::new(sections),
        font,
        atlas,
        dirty: true,
        held_for: 0,
        dpad: ff::DPad4::None,
        msg: None,
    };
    state.script.seed = ff::get_random();
    advance_actions(&mut state);
    set_state(state);
}
