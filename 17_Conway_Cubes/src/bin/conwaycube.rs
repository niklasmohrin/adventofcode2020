use conway_cubes::ConwayCubeWorld;
use std::env;

fn main() {
    let filename = env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| String::from("input"));
    let mut world = ConwayCubeWorld::new(&filename).expect("Couldn't create world");
    eprintln!("Before any cycle:\n\n{:?}", world);
    for i in 1..=6 {
        world.cycle();
        eprintln!("After {} cycles:\n\n{:?}", i, world);
    }

    println!("Cubes left: {}", world.active_cubes());
}
