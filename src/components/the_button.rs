use yew::prelude::*;

#[function_component]
pub fn TheButton() -> Html {
    let onclick = Callback::from(move |_| {
        let greeting = String::from("Hi there");
        web_sys::console::log_1(&greeting.into()); // if uncommented will print
    });

    html! {
        <button {onclick}>{ "Click" }</button>
    }
}
