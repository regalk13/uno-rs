mod components;
mod router;

use router::{switch, Route};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(Model)]
fn model_functional() -> Html {
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

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
