use std::rc::Rc;
use yew::prelude::*;

use crate::{player::Player, utils::format_msg, web_service::WebsocketService};

/// represents the size of the board, which can be chosen at the beginng of the game
#[derive(Clone, Debug, PartialEq)]
pub enum BoardSize {
    Nine,
    Nineteen,
}

/// reprezents a stone placed on the board, which can be either black or white
#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Stone {
    Black,
    White,
}

impl Stone {
    pub fn from_str(stone: String) -> Self {
        if stone == "white" {
            Stone::White
        } else {
            Stone::Black
        }
    }

    fn decode(&self) -> u64 {
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

impl Field {
    pub fn from_num(idx: usize, num: u64) -> Self {
        let mut owner: Option<Stone> = None;
        if num == 1 {
            owner = Some(Stone::Black);
        } else if num == 2 {
            owner = Some(Stone::White);
        }

        Self { idx, owner }
    }
}

/// represents the state of the game
#[derive(Debug, PartialEq)]
pub struct Game {
    pub size: Option<BoardSize>,
    pub fields: Vec<Field>,
    pub wss: WebsocketService,
    pub player: Player,
}

fn code_fields(board: &Vec<u64>) -> Vec<Field> {
    let mut new_fields: Vec<Field> = Vec::new();
    for (idx, num) in board.iter().enumerate() {
        new_fields.push(Field::from_num(idx, num.clone()));
    }
    new_fields
}

fn decode_fields(fields: &Vec<Field>) -> Vec<u64> {
    let mut new_fields: Vec<u64> = Vec::new();
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
    let new_fields = decode_fields(&fields);
    format_msg(
        "board_state",
        &format!("\"board\": {:?}", new_fields),
    )
}

/// represents an action that a player can take during the game
pub enum EventType {
    Place,
    Board,
    Player,
    BoardSize,
}

pub enum Payload {
    Text(String),
    Usize(usize),
    Size(BoardSize),
    Player(String),
    Vector(Vec<u64>),
    BoardState((Vec<u64>, bool)),
    None,
}

/// represents an even happening in the game, which has an action type and action details
pub struct GameAction {
    pub event_type: EventType,
    pub payload: Payload,
}

impl Reducible for Game {
    type Action = GameAction;

    fn reduce(self: Rc<Self>, event: Self::Action) -> Rc<Self> {
        let mut fields = self.fields.clone();

        match event.event_type {
            EventType::Place => {
                if let Payload::Usize(payload) = event.payload {
                    if fields[payload].owner.is_none() && self.player.your_turn {
                        if let Some(s) = self.player.side {
                            fields[payload].owner = Some(s);
                            if let Ok(_) = self
                                .wss
                                .tx
                                .clone()
                                .try_send(format_fields_to_string(&fields))
                            {
                            };
                        }
                    }
                }
            }
            EventType::Board => {
                if let Payload::BoardState((server_fields, your_turn)) = event.payload {
                    let mut player = self.player.clone();
                    player.your_turn = your_turn;

                    if let None = &self.size {
                        let size = if *&server_fields.len() == 100 {
                            BoardSize::Nine
                        } else {
                            BoardSize::Nineteen
                        };

                        return Self {
                            size: Some(size),
                            fields: code_fields(&server_fields),
                            wss: self.wss.clone(),
                            player: player,
                        }
                        .into();
                    }
                    return Self {
                        size: self.size.clone(),
                        fields: code_fields(&server_fields),
                        wss: self.wss.clone(),
                        player: player,
                    }
                    .into();
                }
            }
            EventType::Player => {
                if let Payload::Player(name) = event.payload {
                    let mut player = self.player.clone();
                    if let Ok(_) = player.set_player(name) {};
                    return Self {
                        size: self.size.clone(),
                        fields: self.fields.clone(),
                        wss: self.wss.clone(),
                        player,
                    }
                    .into();
                }
            }
            EventType::BoardSize => {
                if let Payload::None = event.payload {
                    return Self {
                        size: self.size.clone(),
                        fields: self.fields.clone(),
                        wss: self.wss.clone(),
                        player: self.player.clone(),
                    }
                    .into();
                }
            }
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
        BoardSize::Nineteen => (0..196)
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
        let fields = init_fields(BoardSize::Nineteen);
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

    #[test]
    fn test_code_fields() {
        let final_fields = vec![
            Field {
                idx: 0,
                owner: None,
            },
            Field {
                idx: 1,
                owner: None,
            },
            Field {
                idx: 2,
                owner: Some(Stone::White),
            },
            Field {
                idx: 3,
                owner: Some(Stone::Black),
            },
        ];
        let from_fields: Vec<u64> = vec![0, 0, 2, 1];
        assert!(final_fields == code_fields(&from_fields));
    }

    #[test]
    fn test_decode_fields() {
        let final_fields: Vec<u64> = vec![0, 0, 2, 1];
        let from_fields = vec![
            Field {
                idx: 0,
                owner: None,
            },
            Field {
                idx: 1,
                owner: None,
            },
            Field {
                idx: 2,
                owner: Some(Stone::White),
            },
            Field {
                idx: 3,
                owner: Some(Stone::Black),
            },
        ];
        assert!(final_fields == decode_fields(&from_fields));
    }

    #[test]
    fn test_field_from_num() {
        let final_field = Field {
            idx: 0,
            owner: Some(Stone::Black),
        };
        assert!(final_field == Field::from_num(0, 1));
    }

    #[test]
    fn test_stone_from_str() {
        let final_stone = Stone::White;
        let stone = Stone::from_str(String::from("white"));
        assert!(final_stone == stone);
    }

    #[test]
    fn test_format_fields_to_string() {
        let final_string = "{\"message_type\": \"board_state\", \"board\": [0, 0, 2, 1]}";
        let fields = vec![
            Field {
                idx: 0,
                owner: None,
            },
            Field {
                idx: 1,
                owner: None,
            },
            Field {
                idx: 2,
                owner: Some(Stone::White),
            },
            Field {
                idx: 3,
                owner: Some(Stone::Black),
            },
        ];
        assert!(final_string == format_fields_to_string(&fields));
    }
}
