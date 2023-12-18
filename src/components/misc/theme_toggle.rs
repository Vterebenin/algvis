use yew::prelude::*;

use crate::components::icons::day_icon::DayIcon;
use crate::components::icons::night_icon::NightIcon;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let handle_theme_toggle = Callback::from(move |_| {
        let document = web_sys::window().unwrap().document().unwrap();
        let html = document
            .get_elements_by_tag_name("html")
            .get_with_index(0)
            .unwrap();
        if !html.has_attribute("class") {
            html.set_attribute("class", "dark").unwrap();
        } else {
            html.remove_attribute("class").unwrap();
        }
    });
    html! {
        <div>
            <button class="av-day-toggle" onclick={handle_theme_toggle.clone()}>
                <DayIcon />
            </button>
            <button class="av-night-toggle" onclick={handle_theme_toggle}>
                <NightIcon />
            </button>
        </div>
    }
}
