use web_sys::console;
use yew::prelude::*;
use yew_hooks::use_interval;

use crate::components::maze_page::maze_config::{MazeConfig, MazeConfigValues};
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
    let config = MazeConfigValues::new();
    let mut mazer = Mazer::new(&config);
    mazer.solve();
    let mazer: UseStateHandle<Mazer> = use_state(|| mazer);
    let config: UseStateHandle<MazeConfigValues> = use_state(|| config);

    let on_cell_click = {
        let mazer = mazer.clone();
        let config = config.clone();

        Callback::from(move |cell: MazeItem| {
            let config_value = (*config).clone();
            let cell_type_value = config_value.cell_type;
            let mut mazer_value = (*mazer).clone();
            console::log_1(&format!("test {:?}", cell).into());
            let new_cell = Coords::from(cell.col, cell.row);
            match cell_type_value {
                Cell::Visited => unreachable!(),
                Cell::Path => unreachable!(),
                Cell::Empty => mazer_value.maze.create_wall_or_empty(new_cell),
                Cell::Wall => mazer_value.maze.create_wall_or_empty(new_cell),
                Cell::Entry => mazer_value.maze.change_entry(new_cell),
                Cell::Exit => mazer_value.maze.change_exit(new_cell),
            };
            mazer_value.solve();
            mazer.set(mazer_value);
        })
    };
    let clear_walls = {
        let mazer = mazer.clone();
        Callback::from(move |_| {
            let mut mazer_value = (*mazer).clone();
            mazer_value.maze.clear_walls();
            mazer_value.solve();
            mazer.set(mazer_value);
        })
    };
    let regenerate = {
        let mazer = mazer.clone();
        Callback::from(move |_| {
            let mut mazer_value = (*mazer).clone();
            mazer_value.maze.reset();
            mazer_value.solve();
            mazer.set(mazer_value);
        })
    };

    let set_config = {
        let config = config.clone();
        Callback::from(move |v: MazeConfigValues| {
            config.set(v);
        })
    };

    let play_or_pause = {
        let mazer = mazer.clone();
        let config_value = (*config).clone();
        Callback::from(move |_| {
            let mut mazer_value = (*mazer).clone();
            mazer_value.play_or_pause(&config_value);
            mazer.set(mazer_value);
        })
    };

    {
        let mazer = mazer.clone();
        let tick_time = (*mazer).steps_time as u32;

        use_interval(
            move || {
                let mut mazer_value = (*mazer).clone();
                mazer_value.tick();
                mazer.set(mazer_value);
            },
            tick_time,
        );
    }
    let is_playing = (*mazer).is_playing;

    html! {
        <div class="flex justify-between gap-10">
            <div>
                <MazeConfig value={(*config).clone()} on_change={set_config} />
                <div class="flex gap-5">
                    <TheButton class="mt-5" onclick={clear_walls}>
                        {"Clear All Walls"}
                    </TheButton>
                    <TheButton class="mt-5" onclick={regenerate}>
                        {"Regenerate"}
                    </TheButton>
                </div>
                <div>
                    <TheButton class="my-5" onclick={play_or_pause}>
                     {
                         if is_playing {
                            {"Pause"}
                         } else {
                            {"Play"}
                         }
                     }
                    </TheButton>
                </div>
                <MazeLegend />
            </div>
            <MazeViewCanvas
                mazer={(*mazer).clone()}
                on_cell_click={on_cell_click}
            />
        </div>
    }
}
