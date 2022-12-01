mod components;
mod router;

use router::{switch, Route};

use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    // Comment this line if you don't need logs
    wasm_logger::init(wasm_logger::Config::default());
    // Render main app
    yew::Renderer::<App>::new().hydrate();
}
