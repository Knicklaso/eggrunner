use bracket_lib::prelude::*;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
}

pub fn new_map() -> Map {
    let mut map = Map {
        tiles: vec![TileType::Floor; (MAP_WIDTH * MAP_HEIGHT) as usize],
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
    };

    for x in 0..map.width {
        let idx_top = map.xy_idx(x, 0);
        let idx_bottom = map.xy_idx(x, map.height - 1);
        map.tiles[idx_top] = TileType::Wall;
        map.tiles[idx_bottom] = TileType::Wall;
    }

    for y in 0..map.height {
        let idx_left = map.xy_idx(0, y);
        let idx_right = map.xy_idx(map.width - 1, y);
        map.tiles[idx_left] = TileType::Wall;
        map.tiles[idx_right] = TileType::Wall;
    }

    let mut rng = RandomNumberGenerator::new();
    for _ in 0..400 {
        let x = rng.range(1, map.width - 1);
        let y = rng.range(1, map.height - 1);
        let idx = map.xy_idx(x, y);
        map.tiles[idx] = TileType::Wall;
    }

    map
}

pub fn draw_map(map: &Map, ctx: &mut BTerm) {
    for y in 0..map.height {
        for x in 0..map.width {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx] == TileType::Wall {
                ctx.set(x, y, RGB::named(GREEN), RGB::named(BLACK), to_cp437('#'));
            }
        }
    }
}
