use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: String,
    pub options: Vec<SelectOption>,
    pub on_change: Callback<String>,
}

#[function_component(TheSelect)]
pub fn the_select(props: &Props) -> Html {
    let label = if !props.label.is_empty() {
        html! {
            <label
                for="name"
                    class="form__label">
                    {props.label.clone()}
            </label>
        }
    } else {
        html! { <div></div> }
    };
    let on_change = {
        let onchange_cb = props.on_change.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
            onchange_cb.emit(input.value());
        })
    };
    let options = {
        props
            .options
            .iter()
            .map(|opt| {
                html! {
                   <option
                       class="form__option"
                       selected={opt.value.clone() == props.value.clone()}
                       value={opt.value.clone()}
                   >
                   {opt.label.clone()}
                   </option>
                }
            })
            .collect::<Html>()
    };

    html! {
        <div class="form__group field">
           <select class="form__field select" value={props.value.clone()} onchange={on_change.clone()}>
             {options}
           </select>
           {label}
        </div>
    }
}
