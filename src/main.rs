mod models;

use crate::models::{engine::Engine, cellmodel::CellModel};

fn main() {
    let width = 30;
    let height = 30;

    let grid: Vec<Vec<CellModel>> = vec![vec![CellModel::new(false); width]; height];   
    let mut engine = Engine::new(10); // 30 FPS로 엔진 초기화

    engine.run();


    // // 먼저 격자판만 출력
    // for row in 0..height {
    //     for col in 0..width {
    //         if grid[row][col].get_is_alive() {
    //             print!("O ");
    //         } else {
    //             print!(". ");
    //         }
    //     }
    //     println!();
    // }   
}
