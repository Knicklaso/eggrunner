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
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello from Bracket!");
    }
}

fn main() -> BError {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Renderable>();

    let context = BTermBuilder::simple80x50()
        .with_title("Egg Runner")
        .with_fps_cap(30.0)
        .build()?;

    let gs = State { ecs: world };

    main_loop(context, gs)
}
