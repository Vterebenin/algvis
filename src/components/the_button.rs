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
            class="flex w-full justify-center rounded-md bg-indigo-500 px-3 py-1.5 text-sm font-semibold leading-6 text-white shadow-sm hover:bg-indigo-500 focus-visible:outline focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-indigo-600"
            onclick={onclick}
        >
            {props.children.clone()}
        </button>
    }
}
