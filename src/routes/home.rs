use crate::components::navbar::{NavBar, NavBarProps};
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section>
            <NavBar />
        </section>
    }
}
