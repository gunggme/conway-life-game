use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};
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
}

impl Engine {
    // 생명주기: Initialization (초기화)
    pub fn new(fps: u32) -> Self {
        Self {
            state: AppState {
                counter: 0,
                is_running: true,
            },
            fps,
        }
    }

    // 생명주기: Run Loop (엔진 시작)
    pub fn run(&mut self, ) {
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
    }

    // 생명주기: Render (화면 그리기)
    fn render(&self) {
        // ANSI 이스케이프 코드: 커서를 맨 위(0,0)로 이동하여 덮어쓰기 (깜빡임 최소화)
        print!("\x1B[H");

        // 터미널에 출력할 내용 작성
        // println!("========================================");
        // println!("  Rust 스타일 터미널 엔진 가동 중       ");
        // println!("========================================");
        // println!("  현재 카운트: [{}]", self.state.counter);
        // println!("  목표 FPS:    {} FPS", self.fps);
        // println!("========================================");
        // println!("  종료하려면 대기하거나 프로그램(Ctrl+C)을 종료하세요.");

        // 버퍼를 즉시 비워서 화면에 출력되도록 보장
        io::stdout().flush().unwrap();
    }

    // 터미널 화면 전체 삭제 함수
    fn clear_terminal(&self) {
        // \x1B[2J: 화면 전체 삭제, \x1B[H: 커서를 홈 위치로 이동
        print!("\x1B[2J\x1B[H");
        io::stdout().flush().unwrap();
    }
}
