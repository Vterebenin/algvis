use web_sys::console;
use yew::prelude::*;

use crate::{
    components::maze_page::maze_view_canvas::{MazeItem, MazeViewCanvas, Coords},
    services::mazer::Mazer,
};

#[function_component(Maze)]
pub fn maze() -> Html {
    let mut mazer = Mazer::new();
    mazer.solve();
    let mazer: UseStateHandle<Mazer> = use_state(|| mazer);
    let on_cell_click = {
        let mazer = mazer.clone();

        Callback::from(move |cell: MazeItem| {
            let mut mazer_value = (*mazer).clone();
            console::log_1(&format!("test {:?}", cell).into());
            let new_entry = Coords::from(cell.col, cell.row);
            mazer_value.maze.change_entry(new_entry);
            mazer_value.solve();
            mazer.set(mazer_value);
        })
    };

    html! {
        <MazeViewCanvas
            mazer={(*mazer).clone()}
            on_cell_click={on_cell_click}
        />
    }
}
