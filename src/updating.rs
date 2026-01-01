use crate::*;
use firefly_rust as ff;

/// The number of tiles in a row.
const TILES_X: u8 = 30;
/// The number of tiles in a column.
const TILES_Y: u8 = 20;

pub fn update_state(state: &mut State) {
    handle_pad(state);
}

fn handle_pad(state: &mut State) {
    let dpad = read_dpad();
    if dpad.any() {
        state.held_for += 1;
    } else {
        state.held_for = 0;
    }
    let mut old_dpad = state.dpad;
    if state.held_for > 14 && state.held_for.is_multiple_of(4) {
        old_dpad = ff::DPad::default();
    }
    let pressed = dpad.just_pressed(&old_dpad);
    state.dpad = dpad;

    if state.msg.is_some() {
        if pressed.any() {
            state.msg = None;
            advance_actions(state);
        }
        return;
    }

    if state.script.end {
        return;
    }

    if pressed.left {
        move_avatar_to(state, -1, 0);
    } else if pressed.right {
        move_avatar_to(state, 1, 0);
    } else if pressed.up {
        move_avatar_to(state, 0, -1);
    } else if pressed.down {
        move_avatar_to(state, 0, 1);
    }
}

fn read_dpad() -> firefly_rust::DPad {
    let mut dpad = match ff::read_pad(ff::Peer::COMBINED) {
        Some(pad) => to_dpad(pad),
        None => ff::DPad::default(),
    };
    let buttons = ff::read_buttons(ff::Peer::COMBINED);
    if buttons.s {
        dpad.down = true;
    }
    if buttons.e {
        dpad.right = true;
    }
    if buttons.w {
        dpad.left = true;
    }
    if buttons.n {
        dpad.up = true;
    }
    dpad
}

fn to_dpad(pad: ff::Pad) -> ff::DPad {
    let mut dpad = ff::DPad::default();
    let x = pad.x;
    let y = pad.y;
    if y > 100 && y > x.abs() {
        dpad.up = true
    } else if y < -100 && -y > x.abs() {
        dpad.down = true
    } else if x > 100 && x > y.abs() {
        dpad.right = true
    } else if x < -100 && -x > y.abs() {
        dpad.left = true
    }
    dpad
}

fn advance_actions(state: &mut State) {
    state.dirty = true;
    loop {
        let Some(action) = state.script.pop() else {
            break;
        };
        use bulb_parser::Action::*;
        if let Say(msg) = action {
            state.msg = Some(msg);
            break;
        } else {
            state.script.apply(&action);
        }
    }
}

fn move_avatar_to(state: &mut State, dx: i8, dy: i8) {
    let old_pos = state.script.pos;
    let x = old_pos.x.saturating_add_signed(dx).min(TILES_X - 1);
    let y = old_pos.y.saturating_add_signed(dy).min(TILES_Y - 1);
    let new_pos = bulb_parser::Pos {
        room: old_pos.room,
        x,
        y,
    };
    let room = &state.script.sections.rooms[new_pos.room];
    let tile_id = room.tiles[usize::from(y)][usize::from(x)];
    let tile = &state.script.sections.tiles[tile_id];
    state.script.tile_pos = new_pos;
    if !tile.wall {
        state.dirty = true;
        state.script.pos = new_pos;
    }
    if let Some(action_id) = tile.action {
        state.script.enqueue(action_id);
        advance_actions(state);
    }
}
