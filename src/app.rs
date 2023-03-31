use crate::routes::todo::*;
use crate::routes::home::*;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    provide_meta_context(cx);
    view! {
        cx,
        <div>
            <Router>
                <header>
                    <h1>"My Tasks"</h1>
                </header>
                <main>
                <div>
                <Title text="Leptos Heavy Metal Stack"/>
                    <Routes>
                        <Route path="" view=|cx| view! {
                            cx,
                            <Home/>
                        }/>
                    </Routes>
                    </div>
                </main>
            </Router>
        </div>
    }
}
