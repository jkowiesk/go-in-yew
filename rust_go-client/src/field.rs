use yew::prelude::*;

use crate::game::{Event, EventAction, Game, Field, Stone};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub field: Field,
    pub game: UseReducerHandle<Game>,
}

#[function_component]
pub fn FieldFC(props: &Props) -> Html {
    let Props { field, game } = props;
    let game = game.clone();
    let field = field.clone();

    let place_stone = {
        let game = game.clone();
        Callback::from(move |_| {
            game.dispatch(Event {
                event_type: EventAction::Place,
                payload: field.idx,
            })
        })
    };

    let class_name = match &field.owner {
        Some(stone) => match stone {
            Stone::Black => "field field--black",
            Stone::White => "field field--white",
        },
        None => "field field--empty",
    };

    html! {
        <button class={class_name} onclick={place_stone}> </button>
    }
}
