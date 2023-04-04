use crate::routes::home::*;
use crate::routes::login::*;
use crate::routes::register::*;


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
                        <Route path="/login" view=|cx| view! {
                            cx,
                            <Login/>
                        }/>
                        <Route path="/register" view=|cx| view! {
                            cx,
                            <Register/>
                        }/>
                 </Routes>
                    </div>
                </main>
            </Router>
        </div>
    }
}
