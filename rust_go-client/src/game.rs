use std::rc::Rc;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use serde_json::{self, Value};

use crate::web_service::WebsocketService;


/// represents the size of the board, which can be chosen at the beginng of the game
#[derive(Clone, Debug, PartialEq)]
pub enum BoardSize {
    Nine,
    Thirteen,
}

/// reprezents a stone placed on the board, which can be either black or white
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Stone {
    Black,
    White,
}

/// represents a field, which is a vacant point on the board
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Field {
    pub idx: usize,
    pub owner: Option<Stone>,
}

/// represents the state of the game
#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub size: BoardSize,
    pub fields: Vec<Field>,
    pub wss: WebsocketService,
}

/// represents an action that a player can take during the game
pub enum EventAction {
    Place,
}

/// represents an even happening in the game, which has an action type and action details
pub struct Event {
    pub event_type: EventAction,
    pub payload: usize,
}

impl Reducible for Game {
    type Action = Event;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut fields = self.fields.clone();

        match action.event_type {
            EventAction::Place => match &fields[action.payload].owner {
                Some(stone) => match &stone {
                    Stone::Black => {
                        fields[action.payload].owner = Some(Stone::White);
                        let json: Value = serde_json::from_str(&format!("{{board: {:?}}}", fields)).unwrap();
                        if let Ok(_) = self.wss.tx.clone().try_send(json.to_string()){};
                    }
                    Stone::White => {
                        fields[action.payload].owner = Some(Stone::Black);
                    }
                },
                None => {
                    fields[action.payload].owner = Some(Stone::Black);
                }
            },
        };

        self
    }
}

/// initializes all fields on the board with empty fields
pub fn init_fields(size: BoardSize) -> Vec<Field> {
    match size {
        BoardSize::Nine => (0..100)
            .map(|i| Field {
                idx: i,
                owner: None,
            })
            .collect(),
        BoardSize::Thirteen => (0..196)
            .map(|i| Field {
                idx: i,
                owner: None,
            })
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_fields_9x9() {
        let fields = init_fields(BoardSize::Nine);
        assert!(
            fields
                == (0..100)
                    .map(|i| Field {
                        idx: i,
                        owner: None
                    })
                    .collect::<Vec<Field>>()
        );
    }

    #[test]
    fn test_init_fields_13x13() {
        let fields = init_fields(BoardSize::Thirteen);
        assert!(
            fields
                == (0..196)
                    .map(|i| Field {
                        idx: i,
                        owner: None
                    })
                    .collect::<Vec<Field>>()
        );
    }
}
