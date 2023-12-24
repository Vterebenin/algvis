use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <main>
            <h1 class="text-xl mb-5">{"Rust algvis project"}</h1>
            <h2 class="text-lg mb-4">{"What is it?"}</h2>
            <p class="white-space-pre mb-2">{"Algvis is an application about visualisations of different algorithms.\n\
                If im not that lazy, i will implement visualizations for algorithms like sorting and maze running.\n\
                Furthermore, im planning to have a simple pseudocodes for them and maybe a little explanations."}</p>
        </main>
    }
}
