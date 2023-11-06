use crate::components::the_button::TheButton;
use crate::components::navigation::Navigation;

use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Navigation />
            <main>
                <TheButton />
                <h1 class="">{ "Hello World!" }</h1>
                <span class="subtitle">{ "from Yew with " }<i class="heart" /></span>
            </main>
        </>
    }
}
