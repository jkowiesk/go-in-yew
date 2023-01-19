use serde::{Deserialize, Serialize};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{Bridge, Bridged};
use crate::game::Stone;
#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub name: Option<u64>,
    pub side: Option<Stone>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: None,
            side: None
        }
    }

    pub fn set_player(&mut self, name: u64, side: Stone) -> Result<(), ()> {
        self.name = Some(name);
        self.side = Some(side);
        Ok(())
    }
}