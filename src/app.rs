use crate::components::navigation::Navigation;
use crate::router::router::{Route, switch};
use yew_router::prelude::*;
use yew::prelude::*;



#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Navigation />
                <main class="max-w-[1280px] mx-auto flex justify-between py-5">
                    <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
                </main>
            </BrowserRouter>
        </>
    }
}
