use crate::*;
use firefly_rust as ff;

const COLOR_BG: ff::Color = ff::Color::new(1);
const COLOR_TEXT: ff::Color = ff::Color::new(13);
const SPRITE_SIZE: u8 = 16;

pub fn render_room(state: &mut State) {
    if !state.dirty {
        return;
    }
    state.dirty = false;
    if state.script.end {
        draw_end(state);
        return;
    }
    clear_room(state);
    draw_tiles(state);
    draw_player(state);
    draw_message(state);
}

fn clear_room(_state: &State) {
    ff::clear_screen(COLOR_BG);
}

fn draw_tiles(state: &State) {
    let room = &state.script.sections.rooms[state.script.pos.room];
    for y in 0..10 {
        for x in 0..15 {
            // Boundary checks are for cowards!
            // Skipping the checks makes the rendering cycle 8% faster.
            // If bulb-parser has no bugs in the ID resolution, all indices are valid.
            let image = unsafe {
                let tile_id = room.tiles.get_unchecked(y).get_unchecked(x);
                let tile = &state.script.sections.tiles.get_unchecked(*tile_id);
                let Some(image_id) = tile.image else {
                    continue;
                };
                state.script.sections.images.get_unchecked(image_id)
            };
            let p = ff::Point::new(x as u8 * SPRITE_SIZE, y as u8 * SPRITE_SIZE);
            let sub = {
                let atlas = state.atlas.as_image();
                let size = ff::Size::new(SPRITE_SIZE, SPRITE_SIZE);
                let atlas_pos =
                    ff::Point::new(image.pos.x * SPRITE_SIZE, image.pos.y * SPRITE_SIZE);
                atlas.sub(atlas_pos, size)
            };
            ff::draw_sub_image(&sub, p);
        }
    }
}

fn draw_player(state: &State) {
    let Some(image_id) = state.script.sections.player else {
        return;
    };
    let image = &state.script.sections.images[image_id];
    let pos = state.script.pos;
    let p = ff::Point::new(pos.x * SPRITE_SIZE, pos.y * SPRITE_SIZE);
    let sub = {
        let atlas = state.atlas.as_image();
        let size = ff::Size::new(SPRITE_SIZE, SPRITE_SIZE);
        let atlas_pos = ff::Point::new(image.pos.x * SPRITE_SIZE, image.pos.y * SPRITE_SIZE);
        atlas.sub(atlas_pos, size)
    };
    ff::draw_sub_image(&sub, p);
}

fn draw_message(state: &State) {
    let Some(msg) = &state.msg else {
        return;
    };
    let p = ff::Point::new(0, 140);
    let b = ff::Size::new(ff::WIDTH, ff::HEIGHT - p.y);
    let s = ff::Style::solid(COLOR_BG);
    ff::draw_rect(p, b, s);

    let font = state.font.as_font();
    let p = ff::Point::new(2, p.y + i32::from(font.char_height()));
    ff::draw_text(msg, &font, p, COLOR_TEXT);
}

/// Render "THE END" screen.
fn draw_end(state: &State) {
    ff::clear_screen(COLOR_BG);
    let font = state.font.as_font();
    let x = (ff::WIDTH - i32::from(font.char_width()) * 7) / 2;
    let y = (ff::HEIGHT + i32::from(font.char_height())) / 2;
    let point = ff::Point::new(x, y);
    ff::draw_text("THE END", &font, point, COLOR_TEXT);
}
