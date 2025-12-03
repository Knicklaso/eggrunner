use bracket_lib::prelude::{BTerm, INPUT};
use specs::prelude::*;

use crate::components::Position;
use crate::map::Map;

fn try_move_player(delta_x: i32, delta_y: i32, map: &Map, hero_pos: &mut Position) {
    if delta_x == 0 && delta_y == 0 {
        return;
    }

    let target_x = hero_pos.x + delta_x;
    let target_y = hero_pos.y + delta_y;

    if target_x < 0 || target_x >= map.width || target_y < 0 || target_y >= map.height {
        return;
    }

    let destination_idx = map.xy_idx(target_x, target_y);
    if map.tiles[destination_idx] != crate::map::TileType::Wall {
        hero_pos.x = target_x;
        hero_pos.y = target_y;
    }
}

pub fn player_input(
    hero: Entity,
    map: &Map,
    positions: &mut WriteStorage<Position>,
    ctx: &mut BTerm,
) {
    let mouse_primary = {
        let input = INPUT.lock();
        input.is_mouse_button_pressed(0)
    };

    if mouse_primary {
        let (mouse_x, mouse_y) = ctx.mouse_pos();

        if let Some(hero_pos) = positions.get_mut(hero) {
            let dx = (mouse_x - hero_pos.x).signum();
            let dy = (mouse_y - hero_pos.y).signum();

            try_move_player(dx, dy, map, hero_pos);
        }
    }
}
