use yew::prelude::*;

use crate::game::{Game, Field, Stone};

#[derive(Properties, PartialEq)]
pub struct Props {

}

#[function_component(Start)]
pub fn start() -> Html {
    let is_btn_clicked = false;


    html! {
        if is_btn_clicked {
            <div class="btn_container">
                <section class="flex gap-64">
                    <button class="bg-green-600 rounded-md w-64 h-32 p-8 text-2xl shadow-sm">{"9x9"}</button>
                    <button class="bg-red-600 text-white rounded-md w-64 h-32 p-8 text-2xl shadow-xl">{"19x19"}</button>
                </section>
            </div>
        }
    }
}
