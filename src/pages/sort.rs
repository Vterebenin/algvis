use yew::prelude::*;
use yew_hooks::use_interval;

use crate::components::sorting_page::sorting_config::{SortConfigValues, SortingConfig};
use crate::components::sorting_page::sorting_graph_canvas::SortingGraphCanvas;
use crate::components::ui::the_button::TheButton;
use crate::services::sorter::Sorter;

#[function_component(Sort)]
pub fn sort() -> Html {
    let config = use_state(|| SortConfigValues::new());
    let change_config = {
        let config = config.clone();
        Callback::from(move |value: SortConfigValues| config.set(value))
    };

    let sorter: UseStateHandle<Sorter> = use_state(|| Sorter::new(&config));

    {
        let sorter = sorter.clone();
        let tick_time = sorter.tick_time();

        use_interval(
            move || {
                let mut sorter_value = (*sorter).clone();
                sorter_value.tick();
                sorter.set(sorter_value);
            },
            tick_time,
        );
    }

    let handle_sort = {
        let sorter = sorter.clone();
        let config = config.clone();

        Callback::from(move |_| {
            let mut sorter_value = (*sorter).clone();
            sorter_value.sort(&config);
            sorter.set(sorter_value);
        })
    };

    let handle_generate = {
        let sorter = sorter.clone();
        let config = (*config).clone();
        Callback::from(move |_| {
            let mut sorter_value = (*sorter).clone();
            sorter_value.generate(&config);
            sorter.set(sorter_value);
        })
    };

    html! {
        <div class="mx-auto flex justify-center items-center gap-6">
            <div class="flex flex-col justify-between gap-3 p-5 border-2 border-accent rounded-lg h-full">
                <SortingConfig value={(*config).clone()} on_change={change_config} />
                <div class="flex flex-col gap-2 my-5">
                    <TheButton onclick={handle_generate}>
                        {"Generate"}
                    </TheButton>
                    <TheButton onclick={handle_sort}>
                        {"Sort it"}
                    </TheButton>
                </div>
            </div>
            <SortingGraphCanvas data={(*sorter).data.clone()} />
        </div>
    }
}
