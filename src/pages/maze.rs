use web_sys::console;
use yew::prelude::*;

use crate::{
    components::maze_page::maze_legend::MazeLegend,
    components::{
        maze_page::maze_view_canvas::{Coords, MazeItem, MazeViewCanvas},
        ui::the_button::TheButton,
    },
    services::{maze_generator::Cell, mazer::Mazer},
};
#[function_component(Maze)]
pub fn maze() -> Html {
    let mut mazer = Mazer::new();
    mazer.solve();
    let mazer: UseStateHandle<Mazer> = use_state(|| mazer);
    let cell_type: UseStateHandle<Cell> = use_state(|| Cell::Entry);

    let on_cell_click = {
        let mazer = mazer.clone();
        let cell_type_value = (*cell_type).clone();

        Callback::from(move |cell: MazeItem| {
            let mut mazer_value = (*mazer).clone();
            console::log_1(&format!("test {:?}", cell).into());
            let new_cell = Coords::from(cell.col, cell.row);
            match cell_type_value {
                Cell::Visited => todo!(),
                Cell::Path => todo!(),
                Cell::Empty => mazer_value.maze.create_wall_or_empty(new_cell),
                Cell::Wall => mazer_value.maze.create_wall_or_empty(new_cell),
                Cell::Entry => mazer_value.maze.change_entry(new_cell),
                Cell::Exit => mazer_value.maze.change_exit(new_cell),
            };
            mazer_value.solve();
            mazer.set(mazer_value);
        })
    };

    let set_entry = {
        let cell_type = cell_type.clone();
        Callback::from(move |cell: &Cell| {
            cell_type.set(*cell);
        })
    };
    let current_type_name = if (*cell_type).clone() == Cell::Wall {
        "Wall or Empty"
    } else {
        (*cell_type).as_name()
    };

    html! {
        <div class="flex justify-between gap-10">
            <div>
                <MazeLegend />
                <div>{"You can change the maze entry or exit by clicking on a cell"}</div>
                <div>{"If you click right now, you will change "}<b>{current_type_name}</b></div>
                <div>{"Change cell type on click here:"}</div>
                <div class="flex mt-5 gap-2 justify-between">
                    <TheButton onclick={set_entry.clone().reform(|_| &Cell::Entry)}>
                        {"Entry"}
                    </TheButton>
                    <TheButton onclick={set_entry.clone().reform(|_| &Cell::Exit)}>
                        {"Exit"}
                    </TheButton>
                    <TheButton onclick={set_entry.clone().reform(|_| &Cell::Wall)}>
                        {"Wall"}
                    </TheButton>
                </div>
            </div>
            <MazeViewCanvas
                mazer={(*mazer).clone()}
                on_cell_click={on_cell_click}
            />
        </div>
    }
}
