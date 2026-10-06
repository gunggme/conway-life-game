mod models;

use crate::models::{engine::Engine, pattern};

fn main() {
    let width = 60;
    let height = 28;
    let fps = 4;

    let mut engine = Engine::new(fps, width, height);

    engine.place_pattern(&pattern::BLINKER, 4, 3);
    engine.place_pattern(&pattern::TOAD, 12, 3);
    engine.place_pattern(&pattern::BEACON, 22, 3);
    engine.place_pattern(&pattern::PULSAR, 34, 3);
    engine.place_pattern(&pattern::R_PENTOMINO, 4, 20);
    engine.place_pattern(&pattern::GLIDER, 50, 20);
    engine.place_pattern(&pattern::LWSS, 20, 22);

    engine.run();
}
