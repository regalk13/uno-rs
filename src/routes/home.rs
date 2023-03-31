use leptos::*;
use crate::components::navbar::{NavBar, NavBarProps};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section>
            <NavBar />
        </section>
    }

}
