use yew::prelude::*;
use yew_hooks::use_interval;

use crate::components::maze_page::maze_config::{MazeConfig, MazeConfigValues};
use crate::components::ui::the_slider::TheSlider;
use crate::services::playable::Playable;
use crate::{
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
    mazer.solve(&config);
    let mazer: UseStateHandle<Mazer> = use_state(|| mazer);
    let config: UseStateHandle<MazeConfigValues> = use_state(|| config);

    let on_cell_click = {
        let mazer = mazer.clone();
        let config = config.clone();

        Callback::from(move |cell: MazeItem| {
            let config_value = (*config).clone();
            let cell_type_value = config_value.cell_type;
            let mut mazer_value = (*mazer).clone();
            let new_cell = Coords::from(cell.col, cell.row);
            mazer_value.drop_cells();
            match cell_type_value {
                Cell::Visited => unreachable!(),
                Cell::Path => unreachable!(),
                Cell::Empty => mazer_value.maze.create_wall_or_empty(new_cell),
                Cell::Wall => mazer_value.maze.create_wall_or_empty(new_cell),
                Cell::Entry => mazer_value.maze.change_entry(new_cell),
                Cell::Exit => mazer_value.maze.change_exit(new_cell),
            };
            mazer_value.solve(&config_value);
            mazer.set(mazer_value);
        })
    };

    let generate = {
        let mazer = mazer.clone();
        let config_value = (*config).clone();
        Callback::from(move |_| {
            let mut mazer_value = (*mazer).clone();
            mazer_value.generate_new_maze(&config_value);
            mazer_value.solve(&config_value);
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
        let tick_time = (*mazer).tick_time() as u32;

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

    let change_current_step = {
        let mazer = mazer.clone();
        Callback::from(move |value: u32| {
            let mut mazer_value = (*mazer).clone();
            mazer_value.set_step(value);
            mazer.set(mazer_value);
        })
    };
    let steps_info = {
        let steps_total = format!("Steps total: {}", mazer.get_steps_len_string());
        let active_step_index = format!("Active step: {}", mazer.get_active_step_string());
        html! {
            <div class="mb-2">
                <div>{steps_total}</div>
                <div>{active_step_index}</div>
            </div>
        }
    };

    html! {
        <div class="w-full flex flex-col-reverse md:flex-row justify-center items-center gap-6 md:mt-[100px]">
            <div class="flex flex-col justify-between gap-3 p-5 border-2 border-accent rounded-lg h-full w-full max-w-[320px]">
                <MazeConfig value={(*config).clone()} on_change={set_config} />
                <div class="flex flex-col gap-2">
                    <TheButton onclick={generate}>
                        {"Generate"}
                    </TheButton>
                    <TheButton onclick={play_or_pause}>
                     {
                         if is_playing {
                            {"Pause"}
                         } else {
                            {"Play"}
                         }
                     }
                    </TheButton>
                </div>
            </div>
            <div>
                {steps_info}
                <MazeViewCanvas
                    mazer={(*mazer).clone()}
                    on_cell_click={on_cell_click.clone()}
                />
                <TheSlider
                    max={mazer.get_steps_len_string()}
                    value={(*mazer).active_step}
                    set_value={change_current_step}
                />
            </div>
        </div>
    }
}
