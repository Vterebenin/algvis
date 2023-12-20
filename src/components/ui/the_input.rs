use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: String, 
    pub set_value: Callback<String>,
}

#[function_component(TheInput)]
pub fn the_input(props: &Props) -> Html {
    let label = if !props.label.is_empty() {
        html ! {
            <label
                for="name"
                    class="form__label">
                    {props.label.clone()}
            </label>
        }                              
    } else { 
        html! { <div></div> }
    };
    let set_value = {
        let oninput_cb = props.set_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            oninput_cb.emit(input.value());
        })
    };
    
    html! {
        <div class="form__group field">
            <input
                type="input"
                class="form__field"
                placeholder={props.label.clone()}
                name="name"
                id="name"
                value={props.value.clone()}
                oninput={set_value}
                required={true}
            />
            {label}
        </div>
    }
}
