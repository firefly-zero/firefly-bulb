use crate::*;
use firefly_rust as ff;

const COLOR_BG: ff::Color = ff::Color::new(1);

pub fn render_room(state: &mut State) {
    clear_room(state);
    draw_tiles(state);
    draw_player(state);
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

fn draw_player(state: &State) {
    let Some(image_id) = state.script.sections.player else {
        return;
    };
    let image = &state.script.sections.images[image_id];
    let pos = state.script.pos;
    let p = ff::Point::new(pos.x as i32 * 8, pos.y as i32 * 8);
    let image = unsafe { ff::Image::from_bytes(&image.raw) };
    ff::draw_image(&image, p);
}
