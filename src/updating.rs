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
    if state.held_for > 20 && state.held_for.is_multiple_of(4) {
        old_dpad = ff::DPad4::None;
    }
    let pressed = dpad.just_pressed(old_dpad);
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

    match pressed {
        ff::DPad4::Left => move_avatar_to(state, -1, 0),
        ff::DPad4::Right => move_avatar_to(state, 1, 0),
        ff::DPad4::Up => move_avatar_to(state, 0, -1),
        ff::DPad4::Down => move_avatar_to(state, 0, 1),
        ff::DPad4::None => {}
    }
}

fn read_dpad() -> ff::DPad4 {
    let buttons = ff::read_buttons(ff::Peer::COMBINED);
    if buttons.s {
        return ff::DPad4::Down;
    }
    if buttons.e {
        return ff::DPad4::Right;
    }
    if buttons.w {
        return ff::DPad4::Left;
    }
    if buttons.n {
        return ff::DPad4::Up;
    }

    match ff::read_pad(ff::Peer::COMBINED) {
        Some(pad) => pad.as_dpad4(),
        None => ff::DPad4::None,
    }
}

pub fn advance_actions(state: &mut State) {
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
