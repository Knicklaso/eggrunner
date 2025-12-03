use bracket_lib::prelude::*;

use crate::components::Position;

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
    let mut builder = CityBlocksBuilder::new();
    builder.build_map();
    builder.get_map()
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

pub trait MapBuilder {
    fn build_map(&mut self);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
}

pub struct CityBlocksBuilder {
    pub map: Map,
    pub starting_position: Position,
}

impl CityBlocksBuilder {
    pub fn new() -> Self {
        Self {
            map: Map {
                tiles: vec![TileType::Wall; (MAP_WIDTH * MAP_HEIGHT) as usize],
                width: MAP_WIDTH,
                height: MAP_HEIGHT,
            },
            starting_position: Position { x: 0, y: 0 },
        }
    }
}

impl MapBuilder for CityBlocksBuilder {
    fn build_map(&mut self) {
        for tile in self.map.tiles.iter_mut() {
            *tile = TileType::Wall;
        }

        let top_street_start = 10;
        let bottom_street_start = self.map.height - 10 - 6;

        for y in top_street_start..top_street_start + 6 {
            for x in 0..self.map.width {
                let idx = self.map.xy_idx(x, y);
                self.map.tiles[idx] = TileType::Floor;
            }
        }

        for y in bottom_street_start..bottom_street_start + 6 {
            for x in 0..self.map.width {
                let idx = self.map.xy_idx(x, y);
                self.map.tiles[idx] = TileType::Floor;
            }
        }

        let left_street_start = 12;
        let right_street_start = self.map.width - 12 - 6;
        let center_street_start = (self.map.width / 2) - 3;

        for x in left_street_start..left_street_start + 6 {
            for y in 0..self.map.height {
                let idx = self.map.xy_idx(x, y);
                self.map.tiles[idx] = TileType::Floor;
            }
        }

        for x in right_street_start..right_street_start + 6 {
            for y in 0..self.map.height {
                let idx = self.map.xy_idx(x, y);
                self.map.tiles[idx] = TileType::Floor;
            }
        }

        for x in center_street_start..center_street_start + 6 {
            for y in 0..self.map.height {
                let idx = self.map.xy_idx(x, y);
                self.map.tiles[idx] = TileType::Floor;
            }
        }

        self.starting_position = Position {
            x: self.map.width / 2,
            y: top_street_start + 3,
        };
    }

    fn get_map(&self) -> Map {
        Map {
            tiles: self.map.tiles.clone(),
            width: self.map.width,
            height: self.map.height,
        }
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position
    }
}
