use crate::components::navbar::{NavBar, NavBarProps};
use leptos::*;

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <section>
            <NavBar />
            <main class="flex">
                <div>
                    <h1>{"The Uno Game"}</h1>
                    <p>{"Invite your friends and rustify the world of UNO"}</p>
                    <input type="button" value="Play now" />
                </div>
                <div>
                    <img src="main-image.png" />
                </div>
            </main>
        </section>
    }
}
