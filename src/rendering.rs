use crate::*;
use firefly_rust as ff;

const TILES_X: u8 = 30;
const TILES_Y: u8 = 20;
/// Controls how fast, relative to the `update` speed, the room and word animations play.
const ANIMATION_DELAY: u16 = 25;
/// Control how fast, relative to the `update` speed, the new dialog words are printed.
const DIALOG_DELAY: u16 = 3;

const COLOR_BG: ff::Color = ff::Color::new(1);

pub fn render_room(state: &mut State) {
    let render_frame = state.update_frame / ANIMATION_DELAY;
    state.render_frame = render_frame;
    state.room_dirty = false;
    clear_room(state);
    draw_tiles(state);
}

fn clear_room(_state: &State) {
    ff::clear_screen(COLOR_BG);
}

fn draw_tiles(state: &State) {
    let room = &state.script.sections.rooms[state.script.pos.room];
    for (y, line) in room.tiles.iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            let tile = &state.script.sections.tiles[*tile];
            let Some(image_id) = tile.image else {
                continue;
            };
            let image = &state.script.sections.images[image_id];
            let p = ff::Point::new(x as i32 * 8, y as i32 * 8);
            let image = unsafe { ff::Image::from_bytes(&image.raw) };
            ff::draw_image(&image, p);
        }
    }
}
