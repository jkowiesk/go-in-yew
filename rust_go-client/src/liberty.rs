use yew::prelude::*;

use crate::game::{Liberty, Stone, Game, Event, EventAction};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub liberty: Liberty,
    pub game: UseReducerHandle<Game>,
}

#[function_component]
pub fn LibertyFC(props: &Props) -> Html {
    let Props { liberty, game } = props;
    let game = game.clone();
    let liberty = liberty.clone();

    let place_stone = {
        let game = game.clone();
        Callback::from(move |_| game.dispatch(Event {event_type: EventAction::Place, payload: liberty.idx }))
    };

    let class_name = match &liberty.owner {
        Some(stone) => {
            match stone {
                Stone::Black => "liberty liberty--black",
                Stone::White => "liberty liberty--white"
            }
        },
        None => "liberty liberty--empty"
    };

    html! {
        <button class={class_name} onclick={place_stone}> </button>
    }
}