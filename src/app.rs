use crate::components::the_button::TheButton;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    use web_sys::console;
    console::log_1(&format!("{:?} the button", 123).into());
    html! {
        <main>
            <TheButton />
            <h1>{ "Hello World!" }</h1>
            <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
        </main>
    }
}
