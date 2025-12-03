mod components;
mod map;
mod player;

use bracket_lib::prelude::*;
use specs::prelude::*;

use components::{Position, Renderable};
use map::{draw_map, new_map, Map};
use player::player_input;

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
        player_input(self.hero, &map, &mut positions, ctx);

        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
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
