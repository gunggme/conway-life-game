mod models;

use crate::models::{engine::Engine};

fn main() {
    let width = 30;
    let height = 30;

    let mut engine = Engine::new(1, width, height); // 30 FPS로 엔진 초기화

    engine.run();
}
