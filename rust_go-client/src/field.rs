use yew::prelude::*;

use crate::game::{GameAction, EventType, Field, Game, Payload, Stone};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub field: Field,
    pub game: UseReducerHandle<Game>,
    pub size: u64,
}

#[function_component(FieldFC)]
pub fn field_fc(props: &Props) -> Html {
    let Props { field, game, size } = props;
    let field = field.clone();

    let place_stone = {
        let game = game.clone();
        Callback::from(move |_| {
            game.dispatch(GameAction {
                event_type: EventType::Place,
                payload: Payload::Usize(field.idx),
            })
        })
    };

    let class_name = match &field.owner {
        Some(stone) => match stone {
            Stone::Black => if *size == 9 {"field--9x9 field--black"} else {"field--19x19 field--black"},
            Stone::White => if *size == 9 {"field--9x9 field--white"} else {"field--19x19 field--white"},
        },
        None => if *size == 9 {"field--9x9 field--empty"} else {"field--19x19 field--empty"},
    };

    html! {
        <button class={class_name} onclick={place_stone}> </button>
    }
}
