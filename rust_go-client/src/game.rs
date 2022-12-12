use std::{rc::Rc};

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    nine
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum Stone {
    black,
    white
}

#[derive(Clone, Debug, PartialEq, Copy)]
pub struct Liberty {
    pub idx: u32,
    pub owner: Option<Stone>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub size: Size,
    pub liberties: Vec<Liberty>,
}

pub enum EventAction {
    place
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
            EventAction::place => match &liberties[action.payload].owner {
                Some(stone) => match &stone {
                    Stone::black => {liberties[action.payload].owner = Some(Stone::white);},
                    Stone::white => {liberties[action.payload].owner = Some(Stone::black);}
                },
                None => {}
            }
        };

        Self {size: self.size.clone(), liberties}.into()
    }
}

pub fn init_liberties(size: Size) -> Vec<Liberty> {
    match size {
        Size::nine => (0..100).map(|i| Liberty { idx: i, owner: Some(Stone::white) }).collect()
    }
}