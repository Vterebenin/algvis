use yew::prelude::*;

use crate::{services::mazer::Mazer, components::maze_page::maze_view_canvas::MazeViewCanvas};

#[function_component(Maze)]
pub fn maze() -> Html {
    let mut mazer = Mazer::new();
    mazer.solve();
    let mazer: UseStateHandle<Mazer> = use_state(|| mazer);

    html! {
        <MazeViewCanvas
            mazer={(*mazer).clone()}
        />
    }
}
