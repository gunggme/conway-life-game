use std::io::{self, Write};
use std::time::{Duration, Instant};
use std::{thread};
// 생명 주기
#[derive(Clone, Debug)]
struct AppState {
    counter: u32,
    is_running: bool,
}

// 엔진 구조체
#[derive(Clone, Debug)]
pub struct Engine {
    state: AppState,
    fps: u32,
    height: usize,
    width: usize,
    grid: Vec<Vec<crate::models::cellmodel::CellModel>>, // CellModel을 포함하는 2D 벡터 nullable
}

impl Engine {
    // 생명주기: Initialization (초기화)
    pub fn new(fps: u32, width: usize, height: usize) -> Self {
        let mut grid_data = vec![vec![crate::models::cellmodel::CellModel::new(false); width]; height];
        
        grid_data[5][5].set_is_alive(true);
        grid_data[5][6].set_is_alive(true);
        grid_data[5][7].set_is_alive(true);
        grid_data[4][4].set_is_alive(true);
        grid_data[4][5].set_is_alive(true);
        grid_data[4][6].set_is_alive(true);

        Self {
            state: AppState {
                counter: 0,
                is_running: true,
            },
            fps,
            height,
            width,
            grid: grid_data
        }
    }

    // 생명주기: Run Loop (엔진 시작)
    pub fn run(&mut self) {
        let target_dt = Duration::from_secs_f32(1.0 / self.fps as f32);

        // 터미널 화면 초기 청소
        self.clear_terminal();

        while self.state.is_running {
            let start_time = Instant::now();

            // 생명주기: Update (데이터 연산)
            self.update();

            // 생명주기: Render (터미널 출력)
            self.render();

            // FPS 제한을 위한 대기 시간 계산 (Delta Time)
            let elapsed = start_time.elapsed();
            if elapsed < target_dt {
                thread::sleep(target_dt - elapsed);
            }
        }

        println!("\n엔진이 성공적으로 종료되었습니다.");
    }

    // 생명주기: Update (상태 변경 로직)
    fn update(&mut self) {
        self.state.counter += 1;

        let mut arrived_list: Vec<(usize, usize)> = vec![];
        let mut dead_list: Vec<(usize, usize)> = vec![];

        // 생명주기 연산
        for (y, row) in self.grid.iter().enumerate() {
            for (x, _) in row.iter().enumerate() {
                let checking = self.check_current_eight_sight_can_arive(x as i32, y as i32);
                match checking {
                    None => { dead_list.push((x, y)); }
                    Some(arrive) => { arrived_list.push(arrive); }
                }
            }
        }

        // 살리거나 죽이기
        for arrive in arrived_list{
            self.grid[arrive.1][arrive.0].set_is_alive(true);
        }

        for dead in dead_list{
            self.grid[dead.1][dead.0].set_is_alive(false);
        }
    }

    // 생명주기: Render (화면 그리기)
    fn render(&mut self) {
        // ANSI 이스케이프 코드: 커서를 맨 위(0,0)로 이동하여 덮어쓰기 (깜빡임 최소화)
        print!("\x1B[H");

        // 그리드 출력
        for row in self.grid.iter_mut() {
            for cell in row.iter_mut() {
                match cell.get_is_alive() {
                    true => {
                        print!("⬛");
                    }
                    false => {
                        print!("⬜");
                    }
                };
            }
            println!();
        }

        // 버퍼를 즉시 비워서 화면에 출력되도록 보장
        io::stdout().flush().unwrap();
    }

    // 터미널 화면 전체 삭제 함수
    fn clear_terminal(&self) {
        // \x1B[2J: 화면 전체 삭제, \x1B[H: 커서를 홈 위치로 이동
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }

    // 현재 위치에서 8방향 체크
    fn check_current_eight_sight_can_arive(
        &self,
        cur_x: i32,
        cur_y: i32,
    ) -> Option<(usize, usize)> {
        // 인접한 8방향 검사하기
        let check_eight_sight = [
            [0, 1],
            [0, -1],
            [1, 0],
            [-1, 0],
            [1, 1],
            [1, -1],
            [-1, 1],
            [-1, -1],
        ];
        let mut count = 0;

        for sight in check_eight_sight {
            let check_x = cur_x + sight[0];
            let check_y = cur_y + sight[1];

            if check_x < 0 || check_y < 0 {
                continue;
            }

            let check_x = check_x as usize;
            let check_y = check_y as usize;

            if check_x >= self.width || check_y >= self.height {
                continue;
            }

            count += match self.grid[check_y][check_x].get_is_alive() {
                true => 1,
                false => 0,
            };

            if count > 4 {
                break;
            }
        }
        match self.grid[cur_y as usize][cur_x as usize].get_is_alive() {
            true => match count {
                0..=1 => {
                    return None;
                }
                2..=3 => return Some((cur_x as usize, cur_y as usize)),
                4..=7 => {
                    return None;
                }
                _ => {
                    return None;
                }
            },
            false => match count {
                0..=2 => {
                    return None;
                }
                3 => return Some((cur_x as usize, cur_y as usize)),
                ..=7 => {
                    return None;
                }
                _ => {
                    return None;
                }
            },
        }
    }
}
