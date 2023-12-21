use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(Callback::from(|_| ()))]
    pub onclick: Callback<MouseEvent>
}


#[function_component(TheButton)]
pub fn the_button(props: &Props) -> Html {
    let onclick = {
        let cb = props.onclick.clone();
        Callback::from(move |e: MouseEvent| {
            cb.emit(e);
        })
    };
    html! {
        <button 
            type="submit" 
            class="flex transition-background duration-300 ease-in-out w-full justify-center rounded-md bg-accent px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-accentSecond focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary"
            onclick={onclick}
        >
            {props.children.clone()}
        </button>
    }
}
