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
            <nav class="max-w-[1280px] mx-auto md:flex justify-between py-5 ">
                <div class="mb-2 md:mb-0 font-bold cursor-pointer" onclick={handle_route.clone().reform(|_| &Route::Home)}>
                    { "Algvis project" }
                </div>
                <div class="flex gap-4">
                    <ThemeToggle />
                    <div class="flex gap-4 font-medium">
                        <a class="av-link" onclick={handle_route.clone().reform(|_| &Route::Sort)}>{ "Sorting" }</a>
                        { 
                            // <a onclick={handle_route.clone().reform(|_| &Route::Maze)}>{ "Maze Runner" }</a> 
                            ""
                        }
                        <a class="av-link" onclick={handle_route.clone().reform(|_| &Route::Home)}>{ "About" }</a>
                        <a class="av-link" href="https://github.com/Vterebenin/algvis">{ "Github" }</a>
                    </div>
                </div>
            </nav>
        </header>
    }
}
