use yew::prelude::*;

use crate::components::ui::the_button::TheButton;
use crate::components::ui::the_input::TheInput;
use crate::components::ui::the_select::{SelectOption, TheSelect};
use crate::helpers::parse_string_to_i32_or_default;
use crate::services::maze_generator::Cell;

#[derive(Clone, PartialEq)]
pub struct MazeConfigValues {
    pub time_overall: i32,
    pub current_algorithm_name: String,
    pub size: usize,
    pub cell_type: Cell,
    pub current_step: u32,
    alg_options: Vec<SelectOption>,
}

impl MazeConfigValues {
    pub fn new() -> Self {
        let default_algorithm = "dfs".to_string();
        Self {
            time_overall: 2,
            size: 35,
            cell_type: Cell::Entry,
            current_algorithm_name: default_algorithm.clone(),
            alg_options: vec![SelectOption {
                value: default_algorithm,
                label: String::from("DFS"),
            }],
            current_step: 0,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(MazeConfigValues::new())]
    pub value: MazeConfigValues,
    #[prop_or_default]
    pub on_change: Callback<MazeConfigValues>,
}

#[function_component(MazeConfig)]
pub fn maze_config(props: &Props) -> Html {
    let config = use_state(|| MazeConfigValues::new());

    // todo: somehow manage to update values via single function?
    let change_size = {
        let config = config.clone();
        Callback::from(move |value: String| {
            let mut config_value = (*config).clone();
            let result = parse_string_to_i32_or_default(value, 0);
            config_value.size = result as usize;
            config.set(config_value);
        })
    };


    let change_time_overall = {
        let config = config.clone();
        Callback::from(move |value: String| {
            let mut config_value = (*config).clone();
            let result = parse_string_to_i32_or_default(value, 0);
            config_value.time_overall = result;
            config.set(config_value);
        })
    };

    let change_current_algorithm = {
        let config = config.clone();
        Callback::from(move |value: String| {
            let mut config_value = (*config).clone();
            config_value.current_algorithm_name = value;
            config.set(config_value);
        })
    };

    {
        let on_change = props.on_change.clone();
        let config = config.clone();
        let config_value = (*config).clone();
        use_effect_with_deps(
            move |_| {
                on_change.emit((*config).clone());
            },
            config_value,
        );
    }
    
    let set_entry = {
        let config = config.clone();
        Callback::from(move |cell: &Cell| {
            let mut config_value = (*config).clone();
            config_value.cell_type = *cell;
            config.set(config_value);
        })
    };
    
    let current_type_name = {
        let config_value = (*config).clone();
        if config_value.cell_type == Cell::Wall {
            "Wall or Empty"
        } else {
            config_value.cell_type.as_name()
        }
    };

    html! {
        <>
            <h2 class="text-xl">{"Configuration: "}</h2>
            <div>
                <TheInput
                    label="Size"
                    value={config.size.to_string()}
                    set_value={change_size}
                />
                <TheInput
                    label="Time to run (seconds)"
                    value={config.time_overall.to_string()}
                    set_value={change_time_overall}
                />
                <TheSelect
                    label="Maze Walker Algorithm"
                    value={(*config).current_algorithm_name.clone()}
                    on_change={change_current_algorithm}
                    options={config.alg_options.clone()}
                />
            </div>
            <div class="mt-5">{"Current type on click: "}<b>{current_type_name}</b></div>
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
        </>
    }
}
