use crate::pages::sort::Sort;
use yew_router::prelude::*;
use yew::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/sort")]
    Sort,
    #[at("/maze")]
    Maze,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
pub fn secure() -> Html {
    let navigator = use_navigator().unwrap();

    let onclick = Callback::from(move |_| navigator.push(&Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <h1>{ "Home" }</h1> },
        Route::Sort => html! { <Sort /> },
        Route::Maze => html! { <h1>{ "Maze" }</h1> },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
