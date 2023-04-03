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
            <Html lang="en"/>
            <Stylesheet id="leptos" href="/pkg/uno_rs.css"/>
            <Router>
               <main>
                <div>
                <Title text="UNO Rust"/>
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
