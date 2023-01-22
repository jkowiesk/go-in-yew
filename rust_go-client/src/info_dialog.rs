use yew::prelude::*;

use crate::game::{Game, Stone};

#[derive(Properties, PartialEq)]
pub struct Props {}

#[function_component(InfoDialog)]
pub fn info_dialog() -> Html {
    let game = use_context::<UseReducerHandle<Game>>().expect("no ctx found");

    let player = match &game.player.side {
        Some(stone) => match stone {
            Stone::Black => "Black",
            Stone::White => "White",
        },
        None => "None",
    };

    let player_turn: &str;

    let is_board = match &game.size {
        Some(_) => true,
        None => false,
    };

    if player == "None" || !is_board {
        player_turn = "None";
    } else if game.player.your_turn {
        player_turn = player;
    } else {
        player_turn = if player == "White" { "Black" } else { "White" };
    }

    html! {
         <section class="z-[100] absolute top-16 left-8 w-64 h-32 py-2 px-4 round-xl shadow-2xl bg-blue-200">
             <h2 class="font-bold text-xl w-full flex justify-center mx-auto mb-2">{"Game"}</h2>
             <div class="flex flex-col gap-4">
                 <div><span class="font-bold">{"Player: "}</span>{player}</div>
                 <div><span class="font-bold">{"Turn: "}</span>{player_turn}</div>
             </div>
         </section>
    }
}
