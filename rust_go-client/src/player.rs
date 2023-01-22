use crate::game::Stone;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub name: Option<String>,
    pub side: Option<Stone>,
    pub your_turn: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: None,
            side: None,
            your_turn: false,
        }
    }

    pub fn set_player(&mut self, name: String) -> Result<(), ()> {
        self.name = Some(name.clone());
        if name == "first" {
            self.side = Some(Stone::White);
        } else {
            self.side = Some(Stone::Black);
        }
        Ok(())
    }
}
