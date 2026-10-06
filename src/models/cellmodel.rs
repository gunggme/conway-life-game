#[derive(Clone, Debug)]
pub struct CellModel{
    is_alive: bool,
}

impl CellModel {
    pub fn new(is_alive: bool) -> Self {
        CellModel { is_alive }
    }

    pub fn get_is_alive(&self) -> bool {
        self.is_alive
    }

    pub fn set_is_alive(&mut self, alive: bool) {
        self.is_alive = alive;
    }
}