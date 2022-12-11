use crate::liberty::LibertyState;

pub enum Size {
    nine
}

pub struct Game {
    pub size: Size,
    pub liberties: Vec<LibertyState>,
}

impl Game {
    pub fn init(&self) {

    }
}