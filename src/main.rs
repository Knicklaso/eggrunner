use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 50;

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

struct Map {
    tiles: Vec<TileType>,
    width: i32,
    height: i32,
}

impl Map {
    fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }
}

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
    if map.tiles[destination_idx] != TileType::Wall {
        hero_pos.x = target_x;
        hero_pos.y = target_y;
    }
}

fn new_map() -> Map {
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

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(VecStorage)]
struct Renderable {
    glyph: FontCharType,
    fg: RGB,
    bg: RGB,
}

struct State {
    ecs: World,
    hero: Entity,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        let map = self.ecs.fetch::<Map>();
        draw_map(&map, ctx);

        let mut positions = self.ecs.write_storage::<Position>();

        let mouse_primary = {
            let input = INPUT.lock();
            input.is_mouse_button_pressed(0)
        };

        if mouse_primary {
            let (mouse_x, mouse_y) = ctx.mouse_pos();

            if let Some(hero_pos) = positions.get_mut(self.hero) {
                let dx = (mouse_x - hero_pos.x).signum();
                let dy = (mouse_y - hero_pos.y).signum();

                try_move_player(dx, dy, &map, hero_pos);
            }
        }

        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn draw_map(map: &Map, ctx: &mut BTerm) {
    for y in 0..map.height {
        for x in 0..map.width {
            let idx = map.xy_idx(x, y);
            if map.tiles[idx] == TileType::Wall {
                ctx.set(x, y, RGB::named(GREEN), RGB::named(BLACK), to_cp437('#'));
            }
        }
    }
}

fn main() -> BError {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Renderable>();

    let map = new_map();
    world.insert(map);

    let hero = world
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::named(YELLOW),
            bg: RGB::named(BLACK),
        })
        .build();

    for i in 0..5 {
        world
            .create_entity()
            .with(Position {
                x: 10 + (i * 8),
                y: 40,
            })
            .with(Renderable {
                glyph: to_cp437('#'),
                fg: RGB::named(RED),
                bg: RGB::named(BLACK),
            })
            .build();
    }

    let context = BTermBuilder::simple80x50()
        .with_title("Egg Runner")
        .with_fps_cap(30.0)
        .with_advanced_input(true)
        .build()?;

    let gs = State { ecs: world, hero };

    main_loop(context, gs)
}
