use std::{rc::Rc};

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    Nine
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Stone {
    Black,
    White
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Liberty {
    pub idx: usize,
    pub owner: Option<Stone>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub size: Size,
    pub liberties: Vec<Liberty>,
}

pub enum EventAction {
    Place
}

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

pub fn init_liberties(size: Size) -> Vec<Liberty> {
    match size {
        Size::Nine => (0..100).map(|i| Liberty { idx: i, owner: None }).collect()
    }
}