use yew::prelude::*;
use yew_hooks::use_interval;

use crate::components::sorting_page::sorting_config::{SortConfigValues, SortingConfig};
use crate::components::sorting_page::sorting_graph_canvas::SortingGraphCanvas;
use crate::components::ui::the_button::TheButton;
use crate::components::ui::the_slider::TheSlider;
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

    let handle_play = {
        let sorter = sorter.clone();
        let config = config.clone();

        Callback::from(move |_| {
            let mut sorter_value = (*sorter).clone();
            sorter_value.play(&config);
            sorter.set(sorter_value);
        })
    };

    let handle_pause = {
        let sorter = sorter.clone();

        Callback::from(move |_| {
            let mut sorter_value = (*sorter).clone();
            sorter_value.stop();
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

    let change_current_step = {
        let sorter = sorter.clone();
        Callback::from(move |value: u32| {
            let mut sorter_value = (*sorter).clone();
            sorter_value.set_step(value);
            sorter.set(sorter_value);
        })
    };

    let steps_info = {
        let steps_total = format!("Steps total: {}", sorter.get_steps_len_string());
        let active_step_index = format!("Active step: {}", sorter.get_active_step_string());
        let active_step = sorter.get_active_step_item().to_string();
        html! {
            <>
                <div>{steps_total}</div>
                <div>{active_step_index}{" "}{active_step}</div>
            </>
        }
    };

    html! {
        <div class="w-full flex justify-center items-center gap-6 mt-[100px]">
            <div class="flex flex-col justify-between gap-3 p-5 border-2 border-accent rounded-lg h-full w-full max-w-[320px]">
                <SortingConfig value={(*config).clone()} on_change={change_config} />
                <div class="flex flex-col gap-2 my-5">
                    <TheButton onclick={handle_generate}>
                        {"Generate"}
                    </TheButton>
                    {
                        if (*sorter).is_playing {
                            html! {
                                <TheButton onclick={handle_pause}>
                                    {"Pause"}
                                </TheButton>
                            }
                        } else {
                            html! {
                                <TheButton onclick={handle_play}>
                                    {"Play"}
                                </TheButton>
                            }
                        }
                    }
                </div>
            </div>
            <div class="w-full">
                {steps_info}
                <SortingGraphCanvas 
                    data={(*sorter).data.clone()} 
                    active_step_item={(*sorter).get_active_step_item()} 
                />
                <TheSlider 
                    max={sorter.get_steps_len_string()} 
                    value={(*sorter).active_step} 
                    set_value={change_current_step} 
                />
            </div>
        </div>
    }
}
