use futures::channel::mpsc::{Receiver, Sender};
use gloo_console::log;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::rc::Rc;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};

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

impl Stone {
    fn decode(&self) -> u8 {
        match self {
            Stone::Black => 1,
            Stone::White => 2,
        }
    }
}

/// represents a field, which is a vacant point on the board
#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Field {
    pub idx: usize,
    pub owner: Option<Stone>,
}

/// represents the state of the game
#[derive(Debug, PartialEq)]
pub struct Game {
    pub size: BoardSize,
    pub fields: Vec<Field>,
    pub wss: WebsocketService,
}

fn decode_fields(fields: &Vec<Field>) -> Vec<u8> {
    let mut new_fields: Vec<u8> = Vec::new();
    for field in fields.iter() {
        match &field.owner {
            Some(stone) => {
                new_fields.push(stone.decode());
            }
            _ => {
                new_fields.push(0);
            }
        }
    }
    new_fields
}

fn format_fields_to_string(fields: &Vec<Field>) -> String {
    let mut new_fields = decode_fields(&fields);
    format!("{{\"board\": {:?}}}", new_fields)
}

/// represents an action that a player can take during the game
pub enum EventType {
    Place,
    Board,
}

pub enum Payload {
    Text(String),
    Number(usize),
}

/// represents an even happening in the game, which has an action type and action details
pub struct Action {
    pub event_type: EventType,
    pub payload: Payload,
}

impl Reducible for Game {
    type Action = Action;

    fn reduce(self: Rc<Self>, event: Self::Action) -> Rc<Self> {
        let mut fields = self.fields.clone();

        match event.event_type {
            EventType::Place => {
                if let Payload::Number(payload) = event.payload {
                    match &fields[payload].owner {
                        Some(stone) => match &stone {
                            Stone::Black => {
                                fields[payload].owner = Some(Stone::White);
                                self
                            }
                            Stone::White => {
                                fields[payload].owner = Some(Stone::Black);
                                self
                            }
                        },
                        None => {
                            if let Payload::Number(payload) = event.payload {
                                fields[payload].owner = Some(Stone::White);
                                if let Ok(_) = self
                                    .wss
                                    .tx
                                    .clone()
                                    .try_send(format_fields_to_string(&fields))
                                {
                                };
                                self
                            } else {
                                self
                            }
                        }
                    }
                } else {
                    self
                }
            }
            EventType::Board => {
                if let Payload::Text(payload) = event.payload {
                    log!("IN GAME: ", payload);
                    self
                } else {
                    self
                }
            }
        }
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
