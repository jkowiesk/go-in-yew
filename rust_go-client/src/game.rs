use std::{rc::Rc};

use yew::prelude::*;

/// represents the size of the board, which can be chosen at the beginng of the game
#[derive(Clone, Debug, PartialEq)]
pub enum BoardSize {
    Nine,
    Thirteen
}

/// reprezents a stone placed on the board, which can be either black or white
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Stone {
    Black,
    White
}

/// represents a liberty, which is a vacant point on the board
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Liberty {
    pub idx: usize,
    pub owner: Option<Stone>,
}

/// represents the state of the game
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub size: BoardSize,
    pub liberties: Vec<Liberty>,
}

/// represents an action that a player can take during the game
pub enum EventAction {
    Place
}

/// represents an even happening in the game, which has an action type and action details
pub struct Event {
    pub event_type: EventAction,
    pub payload: usize,
}

impl Reducible for Game {
    type Action = Event;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut liberties = self.liberties.clone();

        match action.event_type {
            EventAction::Place => match &liberties[action.payload].owner {
                Some(stone) => match &stone {
                    Stone::Black => {liberties[action.payload].owner = Some(Stone::White);},
                    Stone::White => {liberties[action.payload].owner = Some(Stone::Black);}
                },
                None => {liberties[action.payload].owner = Some(Stone::Black);}
            }
        };

        Self {size: self.size.clone(), liberties}.into()
    }
}

/// initializes all fields on the board with empty liberties
pub fn init_liberties(size: BoardSize) -> Vec<Liberty> {
    match size {
        BoardSize::Nine => (0..100).map(|i| Liberty { idx: i, owner: None }).collect(),
        BoardSize::Thirteen => (0..196).map(|i| Liberty { idx: i, owner: None }).collect()
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_init_liberties_9x9() {
        let liberties = init_liberties(BoardSize::Nine);
        assert!(liberties == (0..100).map(|i| Liberty { idx: i, owner: None }).collect::<Vec<Liberty>>());
    }

    #[test]
    fn test_init_liberties_13x13() {
        let liberties = init_liberties(BoardSize::Thirteen);
        assert!(liberties == (0..196).map(|i| Liberty { idx: i, owner: None }).collect::<Vec<Liberty>>());
    }
}