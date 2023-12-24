use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub value: u32, 
    pub set_value: Callback<u32>,
    pub max: String,
}

#[function_component(TheSlider)]
pub fn the_slider(props: &Props) -> Html {
    let set_value = {
        let oninput_cb = props.set_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            let value = input.value().parse::<u32>().unwrap_or(0);
            oninput_cb.emit(value);
        })
    };
    
    html! {
        <input
            class="w-full h-2 bg-borders rounded-lg appearance-none cursor-pointer accent-accent hover:accent-accentSecond"
            type="input"
            min="0"
            max={props.max.clone()}
            type="range"
            value={props.value.to_string().clone()}
            oninput={set_value}
        />
    }
}
