use yew::prelude::*;

use crate::game::{Action, EventType, Field, Game, Payload, Stone};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub field: Field,
    pub game: UseReducerHandle<Game>,
}

#[function_component(FieldFC)]
pub fn FieldC(props: &Props) -> Html {
    let Props { field, game } = props;
    let game = game.clone();
    let field = field.clone();

    let place_stone = {
        let game = game.clone();
        Callback::from(move |_| {
            game.dispatch(Action {
                event_type: EventType::Place,
                payload: Payload::Number(field.idx),
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
