use yew::prelude::*;

#[function_component(Home)]
pub fn home() -> Html {
    let value = use_state(|| 0i64);
    let on_add = {
        let value = value.clone();
        Callback::from(move |_| value.set(*value + 1))
    };
    let on_reset = {
        let value = value.clone();
        Callback::from(move |_| value.set(0))
    };

    html! {
        <div>
            <button onclick={on_add}>{ "+1" }</button>

            <button onclick={on_reset}>{ "Reset" }</button>
            <p>{ *value }</p>
        </div>
    }
}
