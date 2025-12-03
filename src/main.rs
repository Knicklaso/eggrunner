use bracket_lib::prelude::*;
use specs::prelude::*;
use specs_derive::Component;

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

        let mut positions = self.ecs.write_storage::<Position>();

        if ctx.left_click {
            let (mouse_x, mouse_y) = ctx.mouse_pos();

            if let Some(hero_pos) = positions.get_mut(self.hero) {
                let dx = mouse_x - hero_pos.x;
                let dy = mouse_y - hero_pos.y;

                hero_pos.x = (hero_pos.x + dx.signum()).clamp(0, 79);
                hero_pos.y = (hero_pos.y + dy.signum()).clamp(0, 49);
            }
        }

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
        .build()?;

    let gs = State { ecs: world, hero };

    main_loop(context, gs)
}
