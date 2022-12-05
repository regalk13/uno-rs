use yew::prelude::*;

use crate::components::NavBar;


#[function_component(Home)]
pub fn home() -> Html {

    html! {
        <section>
            <NavBar />
        </section>
    }
}
