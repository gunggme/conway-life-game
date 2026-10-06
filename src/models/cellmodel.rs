#[derive(Clone, Debug)]
pub struct CellModel{
    is_alive: bool,
    can_change: bool,
}

impl CellModel {
    pub fn new(is_alive: bool) -> Self {
        CellModel { 
            is_alive,
            can_change: false,  
        }
    }

    pub fn get_is_alive(&self) -> bool { self.is_alive }

    pub fn get_can_change(&self) -> bool { self.can_change }

    pub fn set_is_alive(&mut self, alive: bool) {
        self.is_alive = alive;
    }

    pub fn set_can_change(&mut self){
        match self.get_is_alive() {
            true => {
                self.can_change = true;
            }
            false => {
                return;
            }
        }
    }

    // 인접한 8방향(상하좌우대각선)을 검사해 생존을 업데이트 하는 로직
}