use yew::prelude::*;

use crate::components::ui::the_input::TheInput;
use crate::components::ui::the_select::{TheSelect, SelectOption};
use crate::helpers::parse_string_to_i32_or_default;

#[derive(Clone, PartialEq)]
pub struct SortConfigValues {
    pub items_count: i32,
    pub time_overall: i32,
    pub current_algorithm_name: String,
    alg_options: Vec<SelectOption>,
}

impl SortConfigValues {
    pub fn new() -> Self {
        let default_algorithm = "merge_sort".to_string();
        Self {
            items_count: MAX_ITEMS,
            time_overall: 10,
            current_algorithm_name: default_algorithm.clone(),
            alg_options: vec![
                SelectOption { value: default_algorithm, label: String::from("Merge Sort") },
                SelectOption { value: String::from("bubble_sort"), label: String::from("Bubble Sort") },
                SelectOption { value: String::from("heap_sort"), label: String::from("Heap Sort") },
                SelectOption { value: String::from("quick_sort"), label: String::from("Quick Sort") },
                SelectOption { value: String::from("insertion_sort"), label: String::from("Insertion Sort") },
                SelectOption { value: String::from("shell_sort"), label: String::from("Shell Sort") },
                // SelectOption { value: String::from("bucket_sort"), label: String::from("Bucket Sort") }
            ]
        }
    }
}

const MAX_ITEMS: i32 = 100;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(SortConfigValues::new())]
    pub value: SortConfigValues, 
    #[prop_or_default]
    pub on_change: Callback<SortConfigValues>,
}

#[function_component(SortingConfig)]
pub fn sorting_config(props: &Props) -> Html {
    let config = use_state(|| SortConfigValues::new());
    let change_items_count = {
        let config = config.clone();
        Callback::from(move |value: String| {
            let mut config_value = (*config).clone();
            let result = parse_string_to_i32_or_default(value, 0);
            config_value.items_count = result;
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
        use_effect_with_deps(move |_| {
            on_change.emit((*config).clone());
        }, config_value);
    }

    html! {
        <div>
            <TheInput
                label="Items Count"
                value={config.items_count.to_string()}
                set_value={change_items_count}
            />
            <TheInput
                label="Time to run (seconds)"
                value={config.time_overall.to_string()}
                set_value={change_time_overall}
            />
            <TheSelect 
                label="Sorting Algorithm"
                value={(*config).current_algorithm_name.clone()}
                on_change={change_current_algorithm}
                options={config.alg_options.clone()}
            />
        </div>
    }
}
