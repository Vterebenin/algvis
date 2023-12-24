use yew::prelude::*;
use crate::router::Route;
use yew_router::prelude::*;

#[function_component(SortingLink)]
fn sorting_link() -> Html {
    let navigator = use_navigator().unwrap();

    let handle_route = Callback::from(move |route: &Route| {
        navigator.push(route);
    });
    return html! {
        <a class="av-simple-link" onclick={handle_route.clone().reform(|_| &Route::Sort)}>{ "sorting" }</a>
    }
}
#[function_component(About)]
pub fn about() -> Html {
    html! {
        <main class="flex flex-col gap-6 text-center mx-auto w-full max-w-[560px]">
            <h1 class="text-xl mb-5">{"Algvis project"}</h1>
            <section>
                <h2 class="text-lg mb-4">{"Overview"}</h2>
                <p class="mb-2 text-justify whitespace-normal">{"Algvis is an application about visualisations of different algorithms.\n\
                    If im not that lazy, i will implement visualizations for algorithms like "}<SortingLink />{" and maze running.\n\
                    Furthermore, im planning to have a simple pseudocodes for them and maybe a little explanations."}</p>
            </section>
            <section>
                <h2 class="text-lg mb-4">{"Tech specs"}</h2>
                <p class="mb-2 text-justify whitespace-normal">{"Algvis developed using following technologies:"}</p>
                <ul class="text-justify">
                    <li><a class="av-simple-link" href="https://www.rust-lang.org/">{"Rust programming language"}</a></li>
                    <li><a class="av-simple-link" href="https://yew.rs/">{"Yew frontend framework"}</a></li>
                    <li><a class="av-simple-link" href="https://tailwindcss.com/">{"Tailwind"}</a></li>
                </ul>
            </section>
            <section>
                <h2 class="text-lg mb-4">{"Why"}</h2>
                <p class="mb-2 text-justify whitespace-normal">{"Algvis project was created primarly for practicing rust and testing out its perfomance.\n\
                    It's also a practice for common algorithms and data structures.\n\
                    The project is not pretending to be the best algorithm visualisation out there, it just some version, highly inspired by "}
                    <a class="av-simple-link" href="https://algorust.dev/">
                        {"Algorust"}
                    </a>{"."}</p>
            </section>
            <section>
                <h2 class="text-lg mb-4">{"About the author"}</h2>
                <p class="mb-2 text-justify whitespace-normal">{"Author of the project, Valentin Terebenin, is a professional text editor of files with extensions .js, .vue, .rs, .ts, .py, .rb and many more."}</p>
                <p class="mb-2 text-justify whitespace-normal">{"Check out his socials:"}</p>
                <ul class="text-justify">
                    <li><a class="av-simple-link" href="https://github.com/Vterebenin">{"Github"}</a></li>
                    <li><a class="av-simple-link" href="https://www.linkedin.com/in/vterebenin/">{"LinkedIn"}</a></li>
                </ul>
            </section>
        </main>
    }
}
