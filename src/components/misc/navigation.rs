use crate::components::misc::theme_toggle::ThemeToggle;
use crate::router::Route;
use yew::prelude::*;
use yew_router::prelude::*;


#[function_component(Navigation)]
pub fn navigation() -> Html {
    let navigator = use_navigator().unwrap();

    let handle_route = Callback::from(move |route: &Route| {
        navigator.push(route);
    });

    html! {
        <header>
            <nav class="max-w-[1280px] mx-auto flex justify-between py-5">
                <div class="font-bold cursor-pointer" onclick={handle_route.clone().reform(|_| &Route::Home)}>{ "Algvis" }</div>
                <div class="flex gap-4">
                    <ThemeToggle />
                    <div class="flex gap-4 font-medium">
                        <a onclick={handle_route.clone().reform(|_| &Route::Sort)}>{ "Sorting" }</a>
                        <a onclick={handle_route.clone().reform(|_| &Route::Maze)}>{ "Maze Runner" }</a>
                        <a onclick={handle_route.clone().reform(|_| &Route::Home)}>{ "About" }</a>
                    </div>
                </div>
            </nav>
        </header>
    }
}