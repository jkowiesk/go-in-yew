pub mod game;
pub mod liberty;
pub mod board;

use yew::{html, function_component, Html};
use board::BoardFC;

#[function_component]
fn App() -> Html {
    let test = "gitara";

    html! {
        <main>
            <BoardFC />
        </main>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

/* <section class="board--main--9x9">
                    <div class="board--main__row">
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                        <div class="board--main__liberty"></div>
                    </div>
                </section> */