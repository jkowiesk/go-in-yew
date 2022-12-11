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

enum EventAction {
    place
}

struct Event {
    action: EventAction,
    payload: i32,
}

impl Reducible for Game {
    type Action = Event;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            EventAction::place => Self {size: self.size, liberties}.into()
        }
    }
}

pub fn init_liberties(size: Size) -> Vec<Liberty> {
    match size {
        Size::nine => (0..100).map(|i| Liberty { idx: i, owner: Some(Stone::white) }).collect()
    }
}