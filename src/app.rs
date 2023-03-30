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
               <main>
                <div>
                <Title text="UNO Rust"/>
                    <Routes>
                        <Route path="" view=|cx| view! {
                            cx,
                            <Home/>
                        }/>
                        <Route path="/test" view=|cx| view! {
                            cx,
                            <Todos/>
                        }/>
                    </Routes>
                    </div>
                </main>
            </Router>
        </div>
    }
}
